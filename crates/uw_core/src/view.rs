//! Native as the core language, read-only (ROADMAP D8a): one Native operation written back as a line of the
//! language, with the same kit words the lowering reads (`lower::kit`), turned the other way round.
//!
//! `(jump_to_menu, "mnu_start_phase_2")` → `menu.start_phase_2.jump()`,
//! `(troop_set_name, "trp_player", "@Alex")` → `troop.player.name = "Alex"`,
//! `(troop_raise_attribute, "trp_player", ca_strength, 3)` → `troop.player.raise_attribute(ca.strength, 3)`.
//!
//! An operation whose shape or arguments the language cannot write yet (a Native global, a string register,
//! a condition) gives `None`: the view shows it as it is in the kit, and the share of `None`s is the measure
//! of what is left.

use std::collections::HashMap;

use uw_atlas::Atlas;

use crate::lower::kit::{kit_ops, kit_properties};
use crate::lower::{singular, Property};
use crate::{Arg, Op};

/// The kit words by Native operation, ready to write operations back as lines.
pub struct Viewer {
    /// Native operation → the kit word that writes or runs it.
    by_native: HashMap<String, Property>,
    /// Native constant → its easy name (`ca_strength` → `ca.strength`).
    constants: HashMap<String, String>,
    /// Native condition → the yes/no kit word that tests it (`agent_is_alive` → `agent.is_alive`).
    conditions: HashMap<String, Property>,
}

/// One line of a block written back: its indentation depth, its text, and whether the language wrote it
/// (`false`: the kit's own form, `(op, args…)`).
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub depth: usize,
    pub text: String,
    pub written: bool,
}

impl Viewer {
    /// The viewer of the committed kit vocabulary.
    pub fn new() -> Viewer {
        let mut by_native = HashMap::new();
        let mut conditions = HashMap::new();
        for prop in kit_properties(&kit_ops()) {
            for step in prop.run.iter().chain(&prop.set) {
                by_native.entry(step.op.clone()).or_insert_with(|| prop.clone());
            }
            if let [step] = prop.cond.as_slice() {
                conditions.entry(step.op.clone()).or_insert_with(|| prop.clone());
            }
        }
        Viewer { by_native, constants: crate::lower::kit::kit_constant_names(), conditions }
    }

    /// One operation as a line of the language, or `None` when the language cannot write it yet.
    pub fn line(&self, atlas: &Atlas, op: &Op) -> Option<String> {
        if op.neg || op.this_or_next {
            return None;
        }
        let prop = self.by_native.get(&op.name)?;
        let (step, is_set) = match prop.run.iter().find(|s| s.op == op.name) {
            Some(step) => (step, false),
            None => (prop.set.iter().find(|s| s.op == op.name)?, true),
        };
        if step.args.len() != op.args.len() {
            return None; // trailing optional parameters given: not a shape the word takes
        }
        let mut object = None;
        let mut value = None;
        let mut call_args = Vec::new();
        for (template, arg) in step.args.iter().zip(&op.args) {
            let text = self.arg(atlas, arg)?;
            match template.as_str() {
                "{object}" => object = Some(text),
                "{value}" if is_set => value = Some(text),
                _ => call_args.push(text),
            }
        }
        // `troop.raise_attribute({attribute_id}, {value})` → name `raise_attribute`, subject `troop`.
        let pattern = prop.path.split('(').next().unwrap_or(&prop.path);
        let (subject, name) = pattern.rsplit_once('.')?;
        let target = match object {
            Some(object) => {
                // The object must be a thing of the word's own kind (`troop.player` for `troop.*`).
                if object.split('.').next() != Some(subject) {
                    return None;
                }
                format!("{object}.{name}")
            }
            None => pattern.to_owned(),
        };
        if is_set {
            Some(format!("{target} = {}", value?))
        } else {
            Some(format!("{target}({})", call_args.join(", ")))
        }
    }

    /// One argument as the language writes it: a kit thing (`party.town_6`), a kit constant (`ca.strength`),
    /// a number in the engine's units, a quick string.
    fn arg(&self, atlas: &Atlas, arg: &Arg) -> Option<String> {
        match arg {
            Arg::Int(n) => Some(n.to_string()),
            Arg::Text(text) => {
                let text = text.strip_prefix('@')?;
                (!text.contains(['"', '{'])).then(|| format!("\"{text}\""))
            }
            Arg::Name(name) => {
                if let Some(easy) = self.constants.get(name) {
                    if atlas.constant(name).is_some() {
                        return Some(easy.clone());
                    }
                }
                let (table, entry) = atlas.resolve_ref(name)?;
                let entry_name = entry.name.as_deref()?;
                // Only a name the language can read back as one path part.
                if !entry_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    return None;
                }
                Some(format!("{}.{}", singular(table), entry_name.to_lowercase()))
            }
            // A local of the block (`:town` → `town`).
            Arg::Local(name) => Some(name.trim_start_matches(':').to_owned()),
            Arg::Global(_) | Arg::Reg(_) => None,
        }
    }

    /// A condition as an expression: a comparison (`eq`, `gt`, `ge` and their `neg|` forms), a range test
    /// (`is_between`), or a yes/no kit word (`agent.is_alive`).
    pub fn condition(&self, atlas: &Atlas, op: &Op) -> Option<String> {
        let args = op.args.iter().map(|arg| self.arg(atlas, arg)).collect::<Option<Vec<_>>>()?;
        let compare = |sign: &str, negated: &str| -> Option<String> {
            match args.as_slice() {
                [a, b] => Some(format!("{a} {} {b}", if op.neg { negated } else { sign })),
                _ => None,
            }
        };
        match op.name.as_str() {
            "eq" => compare("==", "!="),
            "gt" => compare(">", "<="),
            "ge" => compare(">=", "<"),
            "is_between" => match args.as_slice() {
                [value, lo, hi] if !op.neg => Some(format!("{value} >= {lo} and {value} < {hi}")),
                _ => None,
            },
            name => {
                let prop = self.conditions.get(name)?;
                let [step] = prop.cond.as_slice() else { return None };
                if step.args.len() != args.len() {
                    return None;
                }
                let mut object = None;
                let mut call_args = Vec::new();
                for (template, text) in step.args.iter().zip(args) {
                    if template == "{object}" {
                        object = Some(text);
                    } else {
                        call_args.push(text);
                    }
                }
                let pattern = prop.path.split('(').next().unwrap_or(&prop.path);
                let (subject, word) = pattern.rsplit_once('.')?;
                let target = match object {
                    Some(object) => {
                        if object.split('.').next() != Some(subject) && object.contains('.') {
                            return None;
                        }
                        format!("{object}.{word}")
                    }
                    None => pattern.to_owned(),
                };
                let text = if prop.path.contains('(') {
                    format!("{target}({})", call_args.join(", "))
                } else {
                    target
                };
                Some(if op.neg { format!("not {text}") } else { text })
            }
        }
    }

    /// A whole operation block as lines of the language: `try_begin … else_try … try_end` become
    /// `if … { } else if … { } else { }`, `this_or_next` conditions join with `or`. What the language
    /// cannot write stays in the kit's form, one line each, so nothing is hidden.
    pub fn block(&self, atlas: &Atlas, ops: &[Op]) -> Vec<Line> {
        let mut out = Vec::new();
        let mut i = 0;
        self.statements(atlas, ops, &mut i, 0, &mut out);
        // An unbalanced `else_try`/`try_end` at the top: show the rest as it is.
        while i < ops.len() {
            out.push(Line { depth: 0, text: raw(&ops[i]), written: false });
            i += 1;
        }
        out
    }

    fn is_condition(atlas: &Atlas, op: &Op) -> bool {
        op.neg || op.this_or_next || atlas.op(&op.name).is_some_and(|o| o.can_fail)
    }

    /// Statements until an `else_try`/`try_end` of the enclosing block (left at `i`) or the end.
    fn statements(&self, atlas: &Atlas, ops: &[Op], i: &mut usize, depth: usize, out: &mut Vec<Line>) {
        while *i < ops.len() {
            let op = &ops[*i];
            match op.name.as_str() {
                "else_try" | "try_end" => return,
                "try_begin" => {
                    *i += 1;
                    self.try_block(atlas, ops, i, depth, out);
                }
                _ => {
                    let text = self.line(atlas, op);
                    out.push(Line { depth, written: text.is_some(), text: text.unwrap_or_else(|| raw(op)) });
                    *i += 1;
                }
            }
        }
    }

    /// The branches of a `try_begin` (already passed), through its `try_end`.
    fn try_block(&self, atlas: &Atlas, ops: &[Op], i: &mut usize, depth: usize, out: &mut Vec<Line>) {
        let mut first = true;
        loop {
            // The branch's leading conditions.
            let start = *i;
            while *i < ops.len() && Self::is_condition(atlas, &ops[*i]) {
                *i += 1;
            }
            let head = self.conditions_text(atlas, &ops[start..*i]);
            let keyword = if first { "if" } else { "} else if" };
            if first && start == *i {
                // A `try` with no condition only keeps a failure inside: its body reads as it is, in place.
                self.statements(atlas, ops, i, depth, out);
                while *i < ops.len() && ops[*i].name != "try_end" {
                    out.push(Line { depth, text: raw(&ops[*i]), written: false });
                    *i += 1;
                }
                *i += 1;
                return;
            }
            match (&head, start == *i) {
                (_, true) => out.push(Line { depth, text: "} else {".to_owned(), written: true }),
                (Some(head), false) => out.push(Line { depth, text: format!("{keyword} {head} {{"), written: true }),
                (None, false) => {
                    out.push(Line { depth, text: format!("{keyword} (kit conditions) {{"), written: false });
                    for op in &ops[start..*i] {
                        out.push(Line { depth: depth + 1, text: format!("// {}", raw(op)), written: false });
                    }
                }
            }
            self.statements(atlas, ops, i, depth + 1, out);
            first = false;
            match ops.get(*i).map(|op| op.name.as_str()) {
                Some("else_try") => *i += 1,
                Some("try_end") => {
                    *i += 1;
                    out.push(Line { depth, text: "}".to_owned(), written: true });
                    return;
                }
                _ => {
                    out.push(Line { depth, text: "} // (no try_end)".to_owned(), written: false });
                    return;
                }
            }
        }
    }

    /// Leading conditions as one expression: `and` between them, `or` after a `this_or_next` one.
    fn conditions_text(&self, atlas: &Atlas, ops: &[Op]) -> Option<String> {
        // Groups joined by `and`; inside a group the members are joined by `or` (a `this_or_next` member
        // joins the next one). `and` binds tighter than `or`, so a group of several is put in parentheses.
        let mut groups: Vec<Vec<String>> = vec![Vec::new()];
        for op in ops {
            let mut plain = op.clone();
            plain.this_or_next = false;
            let part = self.condition(atlas, &plain)?;
            groups.last_mut()?.push(part);
            if !op.this_or_next {
                groups.push(Vec::new());
            }
        }
        groups.retain(|group| !group.is_empty());
        let several = groups.len() > 1;
        let parts: Vec<String> = groups
            .into_iter()
            .map(|group| {
                // A member that is itself an `and` (`is_between`) keeps its own parentheses inside an `or`.
                let wrap_member = group.len() > 1;
                let members: Vec<String> = group
                    .into_iter()
                    .map(|m| if wrap_member && m.contains(" and ") { format!("({m})") } else { m })
                    .collect();
                let joined = members.join(" or ");
                if several && members_len(&joined) { format!("({joined})") } else { joined }
            })
            .collect();
        Some(parts.join(" and "))
    }
}

/// True when a joined group holds an `or` and so needs parentheses next to an `and`.
fn members_len(joined: &str) -> bool {
    joined.contains(" or ")
}

/// An operation in the kit's own form: `(neg|eq, $g_x, 1)`.
pub fn raw(op: &Op) -> String {
    let mut name = op.name.clone();
    if op.this_or_next {
        name = format!("this_or_next|{name}");
    }
    if op.neg {
        name = format!("neg|{name}");
    }
    let mut parts = vec![name];
    parts.extend(op.args.iter().map(|arg| match arg {
        Arg::Int(n) => n.to_string(),
        Arg::Local(s) | Arg::Global(s) | Arg::Name(s) => s.clone(),
        Arg::Reg(n) => format!("reg{n}"),
        Arg::Text(s) => format!("\"{s}\""),
    }));
    format!("({})", parts.join(", "))
}

impl Default for Viewer {
    fn default() -> Viewer {
        Viewer::new()
    }
}
