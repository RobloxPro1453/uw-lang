//! party operations for the start dry run (see families/mod.rs for the rules).
//!
//! A party is a record row (`kit-meta/records/parties.toml`): `faction`, `party_template`, `menu`,
//! `party_flags`, `ai_behavior` and `ai_target` are numbers the record holds, so a `party_set_*`
//! writes that field and the read after it reads the write back. Two fields are text, not a number:
//! `initial_coordinates` (`(17, 52.5)`) and `stacks` (`[(trp_x, 5, 0)]`); the reads parse that text.
//! Where the engine would compute the new value from the game (members after an upgrade, the
//! position a spawned party gets) the cell is first written `unknown`, so later reads stay unknown.

use std::rc::Rc;

use serde_json::Value as Json;

use crate::{Cell, Flow, Machine, Val};

/// `pf_icon_mask` (header_parties.py:13): bits 0-7 of `party_flags` are the party's map icon.
const PF_ICON_MASK: i64 = 0xFF;
/// `pf_is_static` (header_parties.py:16): a static party never moves, so its
/// `initial_coordinates` stay where the record says it is.
const PF_IS_STATIC: i64 = 0x400;
/// `pmf_is_prisoner` (header_parties.py:48): member flag of a prisoner stack.
const PMF_IS_PRISONER: i64 = 0x1;
/// The `ai_bhvr_*` number fills the low byte of `ai_behavior` (kit-meta/records/parties.toml:103).
const AI_BEHAVIOR_MASK: i64 = 0xFF;

pub fn exec(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    let v = |m: &Machine, i: usize| args.get(i).map(|a| m.get(a)).unwrap_or_default();
    let dest = args.first().cloned().unwrap_or(Json::Null);
    match name {
        // New parties: Native puts them after its list; `reg0` gets the id (header_operations.py:476).
        "spawn_around_party" => {
            let (source, template) = (v(m, 0), v(m, 1));
            let new = m.party_count;
            m.party_count += 1;
            // Template and faction are the template record's; where the party appears is random
            // around the source party and no record cell holds that.
            let faction = if template.unknown {
                Val::unknown()
            } else {
                m.read(Cell::field("party_templates", template.n, "faction"))
            };
            m.write(
                Cell::field("parties", new, "party_template"),
                Val::derived(template.n, &[&template, &source]),
            );
            m.write(
                Cell::field("parties", new, "faction"),
                Val::derived(faction.n, &[&faction, &template]),
            );
            m.registers.insert(0, Val::derived(new, &[&source, &template, &faction]));
            Some(Flow::Done)
        }
        // Members: the record's stack list is only the start value, the engine keeps the live one.
        "party_add_members" | "party_remove_members" => {
            // Both store into `reg0` the number the engine managed to add or remove, which the
            // party's free capacity or its stacks bound (header_operations.py:651, 657).
            let (party, troop, number) = (v(m, 0), v(m, 1), v(m, 2));
            members_changed(m, &party);
            let mut added = Val::derived(number.n, &[&number, &party, &troop]);
            added.unknown = true;
            m.registers.insert(0, added);
            Some(Flow::Done)
        }
        "party_add_leader" | "party_upgrade_with_xp" | "party_add_template" => {
            // The engine picks the new stacks: template counts between min and max, the upgrade
            // path, the member limit.
            let party = v(m, 0);
            members_changed(m, &party);
            Some(Flow::Done)
        }
        "party_get_num_companions" => {
            let party = v(m, 1);
            let value = match companion_stacks(m, &party) {
                Some(stacks) => {
                    let n: i64 = stacks.iter().map(|(_, count)| count).sum();
                    let cell = Cell::field("parties", party.n, "stacks");
                    Val::derived(n, &[&party, &record_dep(&[cell])])
                }
                None => Val::unknown(),
            };
            m.set(&dest, value);
            Some(Flow::Done)
        }
        "party_stack_get_troop_id" => {
            let (party, index) = (v(m, 1), v(m, 2));
            let stack = match companion_stacks(m, &party) {
                Some(stacks) => usize::try_from(index.n).ok().and_then(|i| stacks.get(i).copied()),
                None => None,
            };
            let value = match stack {
                Some((troop, _)) if !index.unknown => Val::derived(troop, &[&party, &index]),
                _ => Val::unknown(),
            };
            m.set(&dest, value);
            Some(Flow::Done)
        }
        // Faction, template and the other plain record fields.
        "store_faction_of_party" => {
            let party = v(m, 1);
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let faction = m.read(Cell::field("parties", party.n, "faction"));
            m.set(&dest, Val::derived(faction.n, &[&faction, &party]));
            Some(Flow::Done)
        }
        "party_set_faction" => {
            let (party, faction) = (v(m, 0), v(m, 1));
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let cell = Cell::field("parties", party.n, "faction");
            m.write(cell, Val::derived(faction.n, &[&faction, &party]));
            Some(Flow::Done)
        }
        "party_get_template_id" => {
            let party = v(m, 1);
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let template = m.read(Cell::field("parties", party.n, "party_template"));
            m.set(&dest, Val::derived(template.n, &[&template, &party]));
            Some(Flow::Done)
        }
        "store_num_parties_of_template" => {
            let template = v(m, 1);
            let mut count = 0;
            let mut unknown = template.unknown;
            for party in 0..m.party_count {
                let value = m.read(Cell::field("parties", party, "party_template"));
                unknown |= value.unknown;
                if value.n == template.n && value.certain() {
                    count += 1;
                }
            }
            let mut out = Val::derived(count, &[&template]);
            out.unknown = unknown;
            m.set(&dest, out);
            Some(Flow::Done)
        }
        "party_set_flags" => {
            let (party, flag, clear_or_set) = (v(m, 0), v(m, 1), v(m, 2));
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let cell = Cell::field("parties", party.n, "party_flags");
            let flags = m.read(cell.clone());
            // `clear_or_set`: 0 clears the flag bits, anything else sets them (Native passes 1).
            let n = if clear_or_set.n == 0 { flags.n & !flag.n } else { flags.n | flag.n };
            m.write(cell, Val::derived(n, &[&flags, &flag, &clear_or_set, &party]));
            Some(Flow::Done)
        }
        "party_set_banner_icon" => {
            // The icon a party is drawn with is the low byte of its flags (`pf_icon_mask`); that
            // it is a banner rather than a plain icon is how the engine draws it, not a cell.
            let (party, icon) = (v(m, 0), v(m, 1));
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let cell = Cell::field("parties", party.n, "party_flags");
            let flags = m.read(cell.clone());
            let n = (flags.n & !PF_ICON_MASK) | (icon.n & PF_ICON_MASK);
            m.write(cell, Val::derived(n, &[&flags, &icon, &party]));
            Some(Flow::Done)
        }
        "party_set_ai_behavior" => {
            let (party, behavior) = (v(m, 0), v(m, 1));
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let cell = Cell::field("parties", party.n, "ai_behavior");
            let old = m.read(cell.clone());
            let n = (old.n & !AI_BEHAVIOR_MASK) | (behavior.n & AI_BEHAVIOR_MASK);
            m.write(cell, Val::derived(n, &[&old, &behavior, &party]));
            Some(Flow::Done)
        }
        "party_set_ai_object" => {
            let (party, target) = (v(m, 0), v(m, 1));
            if party.unknown {
                return unknown_party(m, name, args);
            }
            let cell = Cell::field("parties", party.n, "ai_target");
            m.write(cell, Val::derived(target.n, &[&target, &party]));
            Some(Flow::Done)
        }
        // Distances: only two parties that never move have a position the record can give.
        "store_distance_to_party_from_party" => {
            let (from, to) = (v(m, 1), v(m, 2));
            let value = match (position(m, &from), position(m, &to)) {
                (Some((ax, ay)), Some((bx, by))) => {
                    let n = ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt() as i64;
                    // The distance depends on both records: the flags say the parties stand
                    // still and the coordinates say where.
                    let cells = [
                        Cell::field("parties", from.n, "party_flags"),
                        Cell::field("parties", from.n, "initial_coordinates"),
                        Cell::field("parties", to.n, "party_flags"),
                        Cell::field("parties", to.n, "initial_coordinates"),
                    ];
                    Val::derived(n, &[&from, &to, &record_dep(&cells)])
                }
                _ => Val::unknown(),
            };
            m.set(&dest, value);
            Some(Flow::Done)
        }
        "party_is_active" => {
            let party = v(m, 0);
            if party.unknown {
                m.unknown_op("party_is_active (unknown party)", false, args);
                return Some(Flow::Fail);
            }
            // Native's list holds every record party and the run appends the ones it spawns.
            // Parties the run removes are not tracked: `remove_party` frees an id, not a cell.
            Some(if (0..m.party_count).contains(&party.n) { Flow::Done } else { Flow::Fail })
        }
        // The name is record text, not a number, so no cell changes.
        "party_set_name" => Some(Flow::Done),
        // Run-time AI state the records do not hold: initiative, radii, target positions,
        // the ignore switches and the radius the next spawn uses.
        "set_spawn_radius"
        | "party_set_ai_initiative"
        | "party_set_ai_target_position"
        | "party_set_ai_patrol_radius"
        | "party_ignore_player"
        | "party_set_bandit_attraction"
        | "party_set_helpfulness"
        | "party_set_ignore_with_player_party" => Some(Flow::Done),
        _ => None,
    }
}

/// Counts an operation whose party operand the run cannot name (an unknown id) and changes nothing:
/// which record it would touch is not known.
fn unknown_party(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    m.unknown_op(&format!("{name} (unknown party)"), false, args);
    Some(Flow::Done)
}

/// Marks the party's member stacks `unknown`: an operation changed them and only the game knows the
/// new list. A party the run cannot name is left to the caller (nothing is written).
fn members_changed(m: &mut Machine, party: &Val) {
    if party.unknown {
        return;
    }
    m.write(Cell::field("parties", party.n, "stacks"), Val::unknown());
}

/// The party's companion stacks `(troop, number)`, in the order the record lists them, prisoners
/// (`pmf_is_prisoner`) left out as Native keeps them apart. `None` when the run changed the members
/// or the party has no such record field.
fn companion_stacks(m: &mut Machine, party: &Val) -> Option<Vec<(i64, i64)>> {
    if party.unknown || m.cells.contains_key(&Cell::field("parties", party.n, "stacks")) {
        return None;
    }
    let text = m.initial_field_text("parties", party.n, "stacks")?;
    let mut stacks = Vec::new();
    for stack in items(&text)? {
        let parts = items(&stack)?;
        let troop = m.resolve(parts.first()?)?;
        let number = parts.get(1)?.parse::<i64>().ok()?;
        let flags = match parts.get(2) {
            Some(flags) => flags.parse::<i64>().ok()?,
            None => 0,
        };
        if flags & PMF_IS_PRISONER == 0 {
            stacks.push((troop, number));
        }
    }
    Some(stacks)
}

/// A static party's position from the record's `initial_coordinates` (`(17, 52.5)`). A mobile
/// party's position is whatever its AI movement made it, so the record does not know it.
fn position(m: &mut Machine, party: &Val) -> Option<(f64, f64)> {
    if party.unknown {
        return None;
    }
    let flags = m.read(Cell::field("parties", party.n, "party_flags"));
    if !flags.certain() || flags.n & PF_IS_STATIC == 0 {
        return None;
    }
    let text = m.initial_field_text("parties", party.n, "initial_coordinates")?;
    let parts = items(&text)?;
    Some((parts.first()?.parse().ok()?, parts.get(1)?.parse().ok()?))
}

/// A value that only carries the record cells it came from: a family parsed the number out of the
/// field text, so the value depends on those cells.
fn record_dep(cells: &[Cell]) -> Val {
    Val { deps: Rc::new(cells.iter().cloned().collect()), ..Val::default() }
}

/// The items of a printed tuple or list (`(1, 2)`, `[(trp_x, 5, 0), (trp_y, 1, 0)]`, `[]`), each
/// one as text. Record text of these fields never holds a string with a comma in it.
fn items(text: &str) -> Option<Vec<String>> {
    let text = text.trim();
    let close = match text.chars().next()? {
        '(' => ')',
        '[' => ']',
        _ => return None,
    };
    let inner = text.strip_suffix(close)?.get(1..)?;
    let mut out = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for c in inner.chars() {
        match c {
            '(' | '[' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' => {
                depth = depth.saturating_sub(1);
                current.push(c);
            }
            ',' if depth == 0 => {
                out.push(current.trim().to_owned());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_owned());
    }
    Some(out)
}
