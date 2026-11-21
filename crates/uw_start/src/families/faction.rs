//! Faction, relation, quest and time operations for the start dry run (see families/mod.rs for
//! the rules).

use std::rc::Rc;

use serde_json::Value as Json;

use crate::{Cell, Flow, Machine, Val};

/// A relation: the record holds -1..1, the operations the engine's integer -100..100 scale
/// (module_dialogs.py:25039-25041 clamps a relation at -100).
const RELATION_SCALE: f64 = 100.0;

/// Quest operations that move the engine's quest state (header_operations.py:549-557). No record
/// field holds that state, and no quest is active, finished, succeeded, failed or concluded when
/// the game starts; the fields below are the run's own cells.
const QUEST_WRITES: &[(&str, &[(&str, i64)])] = &[
    ("start_quest", &[("active", 1)]),
    ("complete_quest", &[("active", 0), ("finished", 1)]),
    (
        "succeed_quest",
        &[("active", 0), ("finished", 1), ("succeeded", 1), ("concluded", 1)],
    ),
    (
        "fail_quest",
        &[("active", 0), ("finished", 1), ("failed", 1), ("concluded", 1)],
    ),
    ("conclude_quest", &[("active", 0), ("finished", 1), ("concluded", 1)]),
    ("cancel_quest", &[("active", 0)]),
];

pub fn exec(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    // Display text only (the faction's name, the quest's text and giver): no number changes.
    if matches!(name, "faction_set_name" | "setup_quest_text" | "setup_quest_giver") {
        return Some(Flow::Done);
    }
    if let Some(&(_, fields)) = QUEST_WRITES.iter().find(|(op, _)| *op == name) {
        let quest = operand(m, args, 0);
        if quest.unknown {
            m.unknown_op(&format!("{name} (unknown target)"), false, args);
            return Some(Flow::Done);
        }
        for &(field, to) in fields {
            m.write(Cell::field("quests", quest.n, field), Val::num(to));
        }
        return Some(Flow::Done);
    }
    match name {
        // ---- relations -------------------------------------------------------------------
        "store_relation" => {
            let (a, b) = (operand(m, args, 1), operand(m, args, 2));
            let got = relation(m, &a, &b);
            if let Some(dest) = args.first() {
                m.set(dest, got);
            }
            Some(Flow::Done)
        }
        "set_relation" => {
            let (a, b, value) = (operand(m, args, 0), operand(m, args, 1), operand(m, args, 2));
            if a.unknown || b.unknown {
                m.unknown_op(&format!("{name} (unknown target)"), false, args);
                return Some(Flow::Done);
            }
            // header_operations.py:543 gives `set_relation` no symmetry note, so the run sets the
            // pair as written; the start values are the symmetric ones the records hold.
            let value = Val::derived(value.n, &[&value, &a, &b]);
            m.write(relation_cell(a.n, b.n), value);
            Some(Flow::Done)
        }
        // ---- quests ----------------------------------------------------------------------
        "check_quest_active" | "check_quest_finished" | "check_quest_succeeded"
        | "check_quest_failed" | "check_quest_concluded" => {
            let quest = operand(m, args, 0);
            let state = &name["check_quest_".len()..];
            Some(if quest_state(m, &quest, state) { Flow::Done } else { Flow::Fail })
        }
        "set_quest_progression" => {
            let (quest, value) = (operand(m, args, 0), operand(m, args, 1));
            if quest.unknown {
                m.unknown_op(&format!("{name} (unknown target)"), false, args);
                return Some(Flow::Done);
            }
            let value = Val::derived(value.n, &[&value, &quest]);
            m.write(Cell::field("quests", quest.n, "progression"), value);
            Some(Flow::Done)
        }
        // ---- time and colours ------------------------------------------------------------
        "store_current_hours" => {
            // A new game starts before anything has run: the first hour of day 1.
            if let Some(dest) = args.first() {
                m.set(dest, Val::num(0));
            }
            Some(Flow::Done)
        }
        "faction_set_color" => {
            let (faction, color) = (operand(m, args, 0), operand(m, args, 1));
            if faction.unknown {
                m.unknown_op(&format!("{name} (unknown target)"), false, args);
                return Some(Flow::Done);
            }
            let color = Val::derived(color.n, &[&color, &faction]);
            m.write(Cell::field("factions", faction.n, "faction_color"), color);
            Some(Flow::Done)
        }
        _ => None,
    }
}

/// The value an operand stands for.
fn operand(m: &Machine, args: &[Json], i: usize) -> Val {
    args.get(i).map(|arg| m.get(arg)).unwrap_or_default()
}

/// The cell one direction of a relation pair lives in.
fn relation_cell(a: i64, b: i64) -> Cell {
    Cell::field("factions", a, &format!("relation {b}"))
}

/// The relation between two factions: what the run wrote, else the records' start value.
fn relation(m: &mut Machine, a: &Val, b: &Val) -> Val {
    if a.unknown || b.unknown {
        return Val::unknown();
    }
    let cell = relation_cell(a.n, b.n);
    if m.cells.contains_key(&cell) {
        let written = m.read(cell);
        return Val::derived(written.n, &[&written, a, b]);
    }
    match start_relation(m, a.n, b.n) {
        Some((value, from)) => from_field(value, from),
        None => Val::unknown(),
    }
}

/// The `relations` list's value for the pair, and the cell it came from.
fn start_relation(m: &mut Machine, a: i64, b: i64) -> Option<(i64, Cell)> {
    // process_factions.py writes every listed pair both ways, so either record may name it.
    for (own, other) in [(a, b), (b, a)] {
        let Some(text) = m.initial_field_text("factions", own, "relations") else {
            // No record for `own`: the engine's relation comes from outside the records.
            return None;
        };
        let Some(other) = m.entry("factions", other).and_then(|e| e.name.clone()) else {
            return None;
        };
        if let Some(given) = listed_relation(&text, &other) {
            return Some((runtime_relation(given)?, Cell::field("factions", own, "relations")));
        }
    }
    // Neither list names the pair: the uncompiled matrix is 0 there (process_factions.py).
    Some((0, Cell::field("factions", a, "relations")))
}

/// The value a `relations` record text gives `other`: `((outlaws, -0.05), (player_faction, 0.1))`
/// in the kit's tuple form, `[[outlaws, -0.05]]` in the atlas' list form.
fn listed_relation<'t>(text: &'t str, other: &str) -> Option<&'t str> {
    let mut rest = text;
    while let Some(open) = rest.find(['(', '[']) {
        let close = if rest.as_bytes()[open] == b'(' { ')' } else { ']' };
        let tail = &rest[open + 1..];
        let end = tail.find(close)?;
        let pair = &tail[..end];
        if let Some((name, value)) = pair.split_once(',') {
            let name = name.trim().trim_start_matches(['(', '[']).trim_matches('"');
            if name.eq_ignore_ascii_case(other) {
                return Some(value.trim());
            }
        }
        rest = &tail[end + 1..];
    }
    None
}

/// The engine's integer for a record's -1..1 relation value.
fn runtime_relation(given: &str) -> Option<i64> {
    let value: f64 = given.trim_matches('"').parse().ok()?;
    Some((value * RELATION_SCALE).round() as i64)
}

/// A number read from a record field: it depends on that field's cell.
fn from_field(n: i64, cell: Cell) -> Val {
    let mut value = Val::num(n);
    value.deps = Rc::new(std::iter::once(cell).collect());
    value
}

/// Whether the run's quest state has `quest` in `state`; nothing is in any state at game start.
fn quest_state(m: &Machine, quest: &Val, state: &str) -> bool {
    if quest.unknown {
        return false;
    }
    m.cells
        .get(&Cell::field("quests", quest.n, state))
        .is_some_and(|value| value.n != 0)
}
