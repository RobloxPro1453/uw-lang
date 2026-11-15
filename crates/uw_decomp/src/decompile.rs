//! Native operation block -> UW text (docs/tasks/T-020).

use std::cell::OnceCell;
use std::collections::HashMap;

use crate::{RawArg, RawOp};
use uw_atlas::{Atlas, Op};

/// How much structure the printer may recover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Clean `try_begin/else_try/try_end` -> `if/else`, flags -> `or`/`not`, `assign` -> `=`.
    Structured,
    /// One UW call per operation, flags written as `neg|op(...)`; must always round-trip.
    Raw,
}

/// UW statement text for `ops` (no surrounding braces), one statement per line.
pub fn decompile(atlas: &Atlas, ops: &[RawOp], mode: Mode) -> String {
    let printer = Printer::new(atlas);
    let mut lines = Vec::new();
    match mode {
        Mode::Raw => printer.raw_into(ops, 0, &mut lines),
        Mode::Structured => printer.structured_into(ops, 0, &mut lines),
    }
    lines.join("\n")
}

/// What a block operation does to the indentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// `try_begin` / `try_for_*`: opens a block, the lines after it go one level deeper.
    Open,
    /// `try_end` (`end_try`): closes a block.
    End,
    /// `else_try` (`else_try_begin`): starts another arm of a block, on the opener's level.
    Else,
    /// Any other operation.
    Other,
}

/// One arm of a clean `try_begin/…/try_end` block: `cond` empty means `else`.
struct Arm<'a> {
    /// The arm's conditions joined by `and`; empty for the trailing `else`.
    cond: String,
    /// The operations after the conditions.
    body: &'a [RawOp],
}

/// The block printer; holds the atlas plus the two flag bits taken from it.
struct Printer<'a> {
    atlas: &'a Atlas,
    neg: u64,
    this_or_next: u64,
    /// The kit's operand names; built the first time an operand may carry one.
    names: OnceCell<Names<'a>>,
}

impl<'a> Printer<'a> {
    fn new(atlas: &'a Atlas) -> Self {
        let code = |name: &str| atlas.op(name).map_or(0, |op| op.code);
        Printer { atlas, neg: code("neg"), this_or_next: code("this_or_next"), names: OnceCell::new() }
    }

    fn names(&self) -> &Names<'a> {
        self.names.get_or_init(|| Names::build(self.atlas))
    }

    /// `code` with the `neg` / `this_or_next` flag bits cleared.
    fn base(&self, code: u64) -> u64 {
        code & !(self.neg | self.this_or_next)
    }

    /// True when `code` carries `flag` (and the atlas knows that flag).
    fn has(&self, code: u64, flag: u64) -> bool {
        flag != 0 && code & flag != 0
    }

    /// The atlas operation for `code`, its flags ignored.
    fn op_name(&self, code: u64) -> Option<&'a str> {
        self.atlas.op_for_code(self.base(code)).map(|op| op.name.as_str())
    }

    fn kind(&self, code: u64) -> Kind {
        match self.op_name(code) {
            Some("try_begin") => Kind::Open,
            Some(name) if name.starts_with("try_for_") => Kind::Open,
            Some("try_end") | Some("end_try") => Kind::End,
            Some("else_try") | Some("else_try_begin") => Kind::Else,
            _ => Kind::Other,
        }
    }

    /// One operation as a plain UW call, flags written as name prefixes.
    fn raw_line(&self, op: &RawOp) -> String {
        let Some(named) = self.atlas.op_for_code(self.base(op.code)) else {
            return format!("// unknown operation {}", op.code);
        };
        let mut head = String::new();
        if self.has(op.code, self.neg) {
            head.push_str("neg|");
        }
        if self.has(op.code, self.this_or_next) {
            head.push_str("this_or_next|");
        }
        head.push_str(&named.name);
        format!("{head}({})", self.args(named, &op.args))
    }

    fn args(&self, op: &Op, args: &[RawArg]) -> String {
        args.iter().enumerate().map(|(index, arg)| self.arg(op, index, arg)).collect::<Vec<_>>().join(", ")
    }

    /// One operand: the kit's name for a register, a position or a slot the operation asks for, a
    /// number, `$global` / `:local` as written, a reference the atlas knows as a bare name,
    /// anything else as escaped text.
    fn arg(&self, op: &Op, index: usize, arg: &RawArg) -> String {
        match arg {
            RawArg::Int(n) => match self.names().name(op, index, *n) {
                Some(name) => name.to_owned(),
                None => n.to_string(),
            },
            RawArg::Str(s) if s.starts_with('$') || s.starts_with(':') => s.clone(),
            RawArg::Str(s) if is_identifier(s) && self.atlas.resolve_ref(s).is_some() => s.clone(),
            RawArg::Str(s) => quote(s),
        }
    }

    /// `ops` as raw lines, indented inside `try_begin` / `try_for_*` blocks.
    fn raw_into(&self, ops: &[RawOp], indent: usize, out: &mut Vec<String>) {
        let mut depth = indent;
        for op in ops {
            let kind = self.kind(op.code);
            let line_depth = match kind {
                Kind::End | Kind::Else => depth.saturating_sub(1),
                Kind::Open | Kind::Other => depth,
            };
            out.push(indented(line_depth, &self.raw_line(op)));
            match kind {
                Kind::Open => depth += 1,
                Kind::End => depth = depth.saturating_sub(1),
                Kind::Else | Kind::Other => {}
            }
        }
    }

    /// `ops` as UW statements: clean `if` blocks, `assign` as `=`, everything else as a call.
    fn structured_into(&self, ops: &[RawOp], indent: usize, out: &mut Vec<String>) {
        let mut depth = indent;
        let mut index = 0;
        while index < ops.len() {
            let opened = match self.op_name(ops[index].code) {
                Some("try_begin") => self.matching_end(ops, index),
                _ => None,
            };
            if let Some(end) = opened {
                match self.if_arms(&ops[index + 1..end]) {
                    Some(arms) => self.if_into(&arms, depth, out),
                    // Not clean: the try_begin/else_try/try_end shell stays raw, each arm is
                    // still printed structured inside it.
                    None => self.raw_shell_into(&ops[index..=end], depth, out),
                }
                index = end + 1;
                continue;
            }
            let kind = self.kind(ops[index].code);
            let line_depth = match kind {
                Kind::End | Kind::Else => depth.saturating_sub(1),
                Kind::Open | Kind::Other => depth,
            };
            let line = match self.assign_line(&ops[index]) {
                Some(line) => line,
                None => self.raw_line(&ops[index]),
            };
            out.push(indented(line_depth, &line));
            match kind {
                Kind::Open => depth += 1,
                Kind::End => depth = depth.saturating_sub(1),
                Kind::Else | Kind::Other => {}
            }
            index += 1;
        }
    }

    /// A whole `try_begin … try_end` block (`block` includes both) whose arms are not a clean `if`:
    /// the opener, every top-level `else_try` and the closer as raw lines, the arms structured.
    fn raw_shell_into(&self, block: &[RawOp], indent: usize, out: &mut Vec<String>) {
        let last = block.len() - 1;
        out.push(indented(indent, &self.raw_line(&block[0])));
        let mut depth = 0usize;
        let mut start = 1;
        for (index, op) in block.iter().enumerate().take(last).skip(1) {
            match self.kind(op.code) {
                Kind::Open => depth += 1,
                Kind::End => depth = depth.saturating_sub(1),
                Kind::Else if depth == 0 => {
                    self.structured_into(&block[start..index], indent + 1, out);
                    out.push(indented(indent, &self.raw_line(op)));
                    start = index + 1;
                }
                Kind::Else | Kind::Other => {}
            }
        }
        self.structured_into(&block[start..last], indent + 1, out);
        out.push(indented(indent, &self.raw_line(&block[last])));
    }

    /// `assign($x, v)` / `assign(:x, v)` as `x = v`; `None` for anything else.
    fn assign_line(&self, op: &RawOp) -> Option<String> {
        if self.has(op.code, self.neg) || self.has(op.code, self.this_or_next) {
            return None;
        }
        let named = self.atlas.op_for_code(self.base(op.code))?;
        if named.name != "assign" || op.args.len() != 2 {
            return None;
        }
        let RawArg::Str(target) = &op.args[0] else { return None };
        if !(target.starts_with('$') || target.starts_with(':')) {
            return None;
        }
        Some(format!("{target} = {}", self.arg(named, 1, &op.args[1])))
    }

    /// The index of the `try_end` closing the opener at `start`, if the block is balanced.
    fn matching_end(&self, ops: &[RawOp], start: usize) -> Option<usize> {
        let mut depth = 0usize;
        for (offset, op) in ops[start..].iter().enumerate() {
            match self.kind(op.code) {
                Kind::Open => depth += 1,
                Kind::End => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        return Some(start + offset);
                    }
                }
                Kind::Else | Kind::Other => {}
            }
        }
        None
    }

    /// The arms of a block interior when it reads as `if / else if / else`, else `None`.
    /// Every arm but a trailing `else` must start with at least one condition operation.
    fn if_arms<'b>(&self, region: &'b [RawOp]) -> Option<Vec<Arm<'b>>> {
        let mut bounds = Vec::new();
        let mut start = 0;
        let mut depth = 0usize;
        for (index, op) in region.iter().enumerate() {
            match self.kind(op.code) {
                Kind::Open => depth += 1,
                Kind::End => depth = depth.checked_sub(1)?,
                Kind::Else if depth == 0 => {
                    bounds.push((start, index));
                    start = index + 1;
                }
                Kind::Else | Kind::Other => {}
            }
        }
        if depth != 0 {
            return None;
        }
        bounds.push((start, region.len()));
        let last = bounds.len() - 1;
        let mut arms = Vec::new();
        for (position, (from, to)) in bounds.iter().enumerate() {
            let (parts, body) = self.cond_parts(&region[*from..*to])?;
            // Only a trailing arm may carry no condition (it becomes `else`).
            if parts.is_empty() && (position < last || last == 0) {
                return None;
            }
            // `or` binds weaker than `and`: a group joined into a longer chain needs parentheses.
            let parts: Vec<String> = if parts.len() > 1 {
                parts.into_iter().map(|p| if p.contains(" or ") { format!("({p})") } else { p }).collect()
            } else {
                parts
            };
            arms.push(Arm { cond: parts.join(" and "), body });
        }
        Some(arms)
    }

    /// The leading condition operations of an arm as text, with the body that follows them.
    /// A run of `this_or_next` operations ending in one plain operation is one `or` group.
    fn cond_parts<'b>(&self, arm: &'b [RawOp]) -> Option<(Vec<String>, &'b [RawOp])> {
        let mut parts = Vec::new();
        let mut index = 0;
        while index < arm.len() && self.is_condition(&arm[index]) {
            let mut group = Vec::new();
            loop {
                let op = arm.get(index)?;
                if !self.is_condition(op) {
                    // A dangling `this_or_next|` continues into the body.
                    return None;
                }
                let chained = self.has(op.code, self.this_or_next);
                group.push(self.cond_text(op)?);
                index += 1;
                if !chained {
                    break;
                }
            }
            parts.push(group.join(" or "));
        }
        Some((parts, &arm[index..]))
    }

    /// True when the operation can start or continue an arm condition: it can fail, is a
    /// comparison, or is flagged.
    fn is_condition(&self, op: &RawOp) -> bool {
        if self.has(op.code, self.neg) || self.has(op.code, self.this_or_next) {
            return true;
        }
        let Some(name) = self.op_name(op.code) else {
            return false;
        };
        comparison(name, false).is_some()
            || self
                .atlas
                .op_for_code(self.base(op.code))
                .is_some_and(|named| named.can_fail)
    }

    /// One condition operation as expression text (`a == b`, `not op(...)`).
    fn cond_text(&self, op: &RawOp) -> Option<String> {
        let named = self.atlas.op_for_code(self.base(op.code))?;
        let name = named.name.as_str();
        let negated = self.has(op.code, self.neg);
        if let Some(cmp) = comparison(name, negated) {
            if op.args.len() != 2 {
                return None;
            }
            return Some(format!("{} {cmp} {}", self.arg(named, 0, &op.args[0]), self.arg(named, 1, &op.args[1])));
        }
        let mut head = String::new();
        if negated {
            head.push_str("not ");
        }
        head.push_str(name);
        Some(format!("{head}({})", self.args(named, &op.args)))
    }

    /// One `if / else if / else` chain; the bodies are printed again, recursively.
    fn if_into(&self, arms: &[Arm<'_>], indent: usize, out: &mut Vec<String>) {
        for (position, arm) in arms.iter().enumerate() {
            let head = if position == 0 {
                format!("if {} {{", arm.cond)
            } else if arm.cond.is_empty() {
                "} else {".to_owned()
            } else {
                format!("}} else if {} {{", arm.cond)
            };
            out.push(indented(indent, &head));
            self.structured_into(arm.body, indent + 1, out);
        }
        out.push(indented(indent, "}"));
    }
}

/// The kit's readable names for numbers in a block (docs/tasks/T-048): tagged registers,
/// positions and object slots. A number is named only where the kit names it exactly once.
struct Names<'a> {
    /// `op_num_value_bits` and `tag_register`: a register operand's bit count and its tag.
    register: Option<(u32, i64)>,
    /// `reg<N>` constants by value.
    registers: HashMap<i64, &'a str>,
    /// `pos<N>` constants by value.
    positions: HashMap<i64, &'a str>,
    /// Every `slot_*` constant by value; the operation's object kind picks from them.
    slots: HashMap<i64, Vec<&'a str>>,
}

impl<'a> Names<'a> {
    fn build(atlas: &'a Atlas) -> Names<'a> {
        let register = match (atlas.constant("op_num_value_bits"), atlas.constant("tag_register")) {
            (Some(bits), Some(tag)) => match (bits.value.as_u64(), tag.value.as_i64()) {
                (Some(bits), Some(tag)) if bits > 0 && bits < 64 => Some((bits as u32, tag)),
                _ => None,
            },
            _ => None,
        };
        let mut slots: HashMap<i64, Vec<&'a str>> = HashMap::new();
        for constant in atlas.constants.iter().filter(|constant| constant.name.starts_with("slot_")) {
            if let Some(value) = constant.value.as_i64() {
                slots.entry(value).or_default().push(constant.name.as_str());
            }
        }
        Names { register, registers: numbered(atlas, "reg"), positions: numbered(atlas, "pos"), slots }
    }

    /// The kit name for operand `index` of `op`, when the kit names that number there.
    fn name(&self, op: &Op, index: usize, n: i64) -> Option<&'a str> {
        if let Some(name) = self.register_name(n) {
            return Some(name);
        }
        let placeholder = placeholder_at(&op.signature, index)?;
        if is_position_placeholder(placeholder) {
            return self.positions.get(&n).copied();
        }
        if placeholder == "slot_no" {
            return self.slot_name(n, slot_kind(&op.name)?);
        }
        None
    }

    /// The `reg<N>` name of a register-tagged number, when the kit has one for it.
    fn register_name(&self, n: i64) -> Option<&'a str> {
        let (bits, tag) = self.register?;
        if n >> bits != tag {
            return None;
        }
        self.registers.get(&n).copied()
    }

    /// The one `slot_<kind>_*` constant of `n`; `scene_prop_set_slot` must not take a
    /// `slot_scene_*` name, so the kind comes from the operation's own name.
    fn slot_name(&self, n: i64, kind: &str) -> Option<&'a str> {
        let prefix = format!("slot_{kind}_");
        let mut candidates = self.slots.get(&n)?.iter().filter(|name| name.starts_with(&prefix));
        let name = *candidates.next()?;
        candidates.next().is_none().then_some(name)
    }
}

/// Constants named `<prefix><digits>` (`reg0`, `pos1`) by value; a value named twice is left out,
/// because only an unambiguous name may replace the number.
fn numbered<'a>(atlas: &'a Atlas, prefix: &str) -> HashMap<i64, &'a str> {
    let mut found = HashMap::new();
    let mut shared = Vec::new();
    for constant in &atlas.constants {
        let Some(number) = constant.name.strip_prefix(prefix) else { continue };
        if number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let Some(value) = constant.value.as_i64() else { continue };
        if found.insert(value, constant.name.as_str()).is_some() {
            shared.push(value);
        }
    }
    for value in shared {
        found.remove(&value);
    }
    found
}

/// The `index`-th `<...>` placeholder of a signature, in operand order:
/// `(troop_get_slot,<destination>,<troop_id>,<slot_no>)` gives `<slot_no>` at 2. `None` when the
/// signature names fewer operands (the kit sometimes writes no signature, or the next line).
fn placeholder_at(signature: &str, index: usize) -> Option<&str> {
    signature
        .split('<')
        .skip(1)
        .filter_map(|part| part.split_once('>'))
        .map(|(name, _)| name.trim())
        .nth(index)
}

/// True for the signature placeholders the kit marks as a position: `<position_no>`,
/// `<position_no_2>`, `<dest_position_no>`, `<source_position_no>`, `<position_register_no>`...
fn is_position_placeholder(placeholder: &str) -> bool {
    matches!(
        placeholder,
        "position_no"
            | "position_no_1"
            | "position_no_2"
            | "position_no_to_be_transformed"
            | "dest_position_no"
            | "source_position_no"
            | "position_register_no"
            | "hit_position_register"
            | "ray_position_register"
    )
}

/// The object kind of a slot operation, from its own name (`scene_prop_set_slot` -> `scene_prop`).
fn slot_kind(name: &str) -> Option<&str> {
    ["_set_slot", "_get_slot", "_slot_eq", "_slot_ge"]
        .iter()
        .find_map(|suffix| name.strip_suffix(suffix))
        .filter(|kind| !kind.is_empty())
}

/// The comparison operator for `eq` / `gt` / `ge`, flipped when the `neg` flag is set.
fn comparison(name: &str, negated: bool) -> Option<&'static str> {
    Some(match (name, negated) {
        ("eq", false) => "==",
        ("gt", false) => ">",
        ("ge", false) => ">=",
        ("eq", true) => "!=",
        ("gt", true) => "<=",
        ("ge", true) => "<",
        _ => return None,
    })
}

/// `[A-Za-z_][A-Za-z0-9_]*`.
fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// `text` as a UW string literal: `\`, `"` and newline escaped.
fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// `line` behind `depth` four-space steps.
fn indented(depth: usize, line: &str) -> String {
    format!("{}{line}", "    ".repeat(depth))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atlas() -> Atlas {
        let text = r#"{
          "schema": 1, "kit_hash": "t",
          "ops": [
            {"name":"neg","code":2147483648,"line":440,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"this_or_next","code":1073741824,"line":441,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"try_end","code":3,"line":16,"signature":"(try_end),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"try_begin","code":4,"line":17,"signature":"(try_begin),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"else_try","code":5,"line":18,"signature":"(else_try),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"try_for_range","code":6,"line":19,"signature":"(try_for_range,<destination>,<lower_bound>,<upper_bound>),","lhs":true,"can_fail":false,"global_lhs":false},
            {"name":"ge","code":30,"line":30,"signature":"(ge,<value>,<value>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"eq","code":31,"line":31,"signature":"(eq,<value>,<value>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"gt","code":32,"line":32,"signature":"(gt,<value>,<value>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"party_is_active","code":132,"line":94,"signature":"(party_is_active,<party_id>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"display_message","code":1106,"line":482,"signature":"(display_message,<string_id>,[hex_colour_code]),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"change_screen_map","code":2052,"line":1093,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"player_item_slot_is_picked_up","code":461,"line":200,"signature":"(player_item_slot_is_picked_up,<player_id>,<item_slot_no>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"troop_set_slot","code":500,"line":239,"signature":"(troop_set_slot,<troop_id>,<slot_no>,<value>),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"scene_prop_set_slot","code":510,"line":249,"signature":"(scene_prop_set_slot,<scene_prop_instance_id>,<slot_no>,<value>),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"init_position","code":701,"line":302,"signature":"(init_position,<position_no>),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"assign","code":2133,"line":40,"signature":"","lhs":true,"can_fail":false,"global_lhs":false}
          ],
          "hooks": ["game_start"],
          "ranges": [],
          "constants": [
            {"name":"op_num_value_bits","value":56,"module":"header_common","line":357},
            {"name":"tag_register","value":1,"module":"header_common","line":359},
            {"name":"opmask_register","value":72057594037927936,"module":"header_common","line":387},
            {"name":"reg0","value":72057594037927936,"module":"header_common","line":563},
            {"name":"reg1","value":72057594037927937,"module":"header_common","line":564},
            {"name":"pos0","value":0,"module":"header_common","line":497},
            {"name":"pos1","value":1,"module":"header_common","line":498},
            {"name":"pos_belfry_begin","value":64,"module":"header_common","line":561},
            {"name":"slot_troop_x","value":7,"module":"module_constants","line":15},
            {"name":"slot_troop_amb_a","value":10,"module":"module_constants","line":16},
            {"name":"slot_troop_amb_b","value":10,"module":"module_constants","line":17},
            {"name":"slot_scene_q","value":9,"module":"module_constants","line":18},
            {"name":"slot_scene_prop_z","value":9,"module":"module_constants","line":19}
          ],
          "tables": {
            "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","ref_tag":"p",
              "entries": [
                {"index":0,"name":"main_party","id":"p_main_party","dup":null,"line":1,"value":[]},
                {"index":26,"name":"town_6","id":"p_town_6","dup":null,"line":82,"value":[]}
              ]}
          }
        }"#;
        Atlas::from_str(text).unwrap()
    }

    fn op(code: u64, args: Vec<RawArg>) -> RawOp {
        RawOp { code, args }
    }

    fn int(n: i64) -> RawArg {
        RawArg::Int(n)
    }

    fn text(s: &str) -> RawArg {
        RawArg::Str(s.to_owned())
    }

    const NEG_EQ: u64 = 2147483648 | 31;
    const THIS_OR_NEXT_EQ: u64 = 1073741824 | 31;

    #[test]
    fn decompile_raw_prints_flags_and_indents_blocks() {
        let ops = vec![
            op(4, vec![]),
            op(NEG_EQ, vec![text("$g_x"), int(1)]),
            op(THIS_OR_NEXT_EQ, vec![text(":a"), text("p_town_6")]),
            op(2147483648 | 1073741824 | 30, vec![text("$g_x"), int(-5)]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Raw),
            "try_begin()\n    neg|eq($g_x, 1)\n    this_or_next|eq(:a, p_town_6)\n    neg|this_or_next|ge($g_x, -5)\ntry_end()"
        );
    }

    #[test]
    fn decompile_raw_quotes_text_and_resolves_names() {
        let ops = vec![
            op(2052, vec![]),
            op(132, vec![text("p_town_6")]),
            op(1106, vec![text("p_town_9")]),
            op(1106, vec![text("@he said \"hi\"\\bye\n")]),
            op(999999, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Raw),
            concat!(
                "change_screen_map()\n",
                "party_is_active(p_town_6)\n",
                "display_message(\"p_town_9\")\n",
                "display_message(\"@he said \\\"hi\\\"\\\\bye\\n\")\n",
                "// unknown operation 999999"
            )
        );
    }

    #[test]
    fn decompile_structured_writes_assignments_as_equality() {
        let ops = vec![
            op(2133, vec![text("$g_x"), int(-5)]),
            op(2133, vec![text(":l_y"), text("p_town_6")]),
            op(2133, vec![int(1), int(2)]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Structured),
            "$g_x = -5\n:l_y = p_town_6\nassign(1, 2)"
        );
    }

    #[test]
    fn decompile_structured_if_else_if_else() {
        let ops = vec![
            op(4, vec![]),
            op(31, vec![text("$g_x"), int(1)]),
            op(2133, vec![text("$g_y"), int(2)]),
            op(5, vec![]),
            op(32, vec![text("$g_x"), int(3)]),
            op(2133, vec![text("$g_y"), int(4)]),
            op(5, vec![]),
            op(2133, vec![text("$g_y"), int(5)]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Structured),
            concat!(
                "if $g_x == 1 {\n",
                "    $g_y = 2\n",
                "} else if $g_x > 3 {\n",
                "    $g_y = 4\n",
                "} else {\n",
                "    $g_y = 5\n",
                "}"
            )
        );
    }

    #[test]
    fn decompile_structured_or_chain_joins_by_or() {
        let ops = vec![
            op(4, vec![]),
            op(THIS_OR_NEXT_EQ, vec![text(":a"), int(1)]),
            op(1073741824 | 32, vec![text(":a"), int(2)]),
            op(30, vec![text(":a"), int(3)]),
            op(2052, vec![]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Structured),
            "if :a == 1 or :a > 2 or :a >= 3 {\n    change_screen_map()\n}"
        );
    }

    #[test]
    fn decompile_structured_joins_conditions_by_and_not_by_not() {
        let and = vec![
            op(4, vec![]),
            op(31, vec![text("p_town_6"), text("reg0")]),
            op(132, vec![text("p_town_6")]),
            op(2052, vec![]),
            op(3, vec![]),
        ];
        // `reg0` is no reference the atlas resolves, so it is quoted text.
        assert_eq!(
            decompile(&atlas(), &and, Mode::Structured),
            "if p_town_6 == \"reg0\" and party_is_active(p_town_6) {\n    change_screen_map()\n}"
        );
        let not = vec![
            op(4, vec![]),
            op(2147483648 | 132, vec![text("p_town_6")]),
            op(2052, vec![]),
            op(5, vec![]),
            op(NEG_EQ, vec![text(":b"), int(3)]),
            op(2052, vec![]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &not, Mode::Structured),
            concat!(
                "if not party_is_active(p_town_6) {\n",
                "    change_screen_map()\n",
                "} else if :b != 3 {\n",
                "    change_screen_map()\n",
                "}"
            )
        );
    }

    #[test]
    fn decompile_structured_keeps_unclean_and_nested_tries() {
        // The first arm starts with an assignment, so the shell stays raw; the arms inside it
        // are still structured (a clean nested `if`, `=`).
        let unclean = vec![
            op(4, vec![]),
            op(2133, vec![text("$g_x"), int(1)]),
            op(4, vec![]),
            op(30, vec![text("$g_x"), int(1)]),
            op(2052, vec![]),
            op(3, vec![]),
            op(5, vec![]),
            op(2133, vec![text(":y"), int(2)]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &unclean, Mode::Structured),
            concat!(
                "try_begin()\n",
                "    $g_x = 1\n",
                "    if $g_x >= 1 {\n",
                "        change_screen_map()\n",
                "    }\n",
                "else_try()\n",
                "    :y = 2\n",
                "try_end()"
            )
        );
        // An `or` group inside a longer `and` chain is parenthesised (`or` binds weaker).
        let grouped = vec![
            op(4, vec![]),
            op(31, vec![text("$g_x"), int(0)]),
            op(THIS_OR_NEXT_EQ, vec![text(":a"), int(1)]),
            op(31, vec![text(":a"), int(2)]),
            op(2052, vec![]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &grouped, Mode::Structured),
            "if $g_x == 0 and (:a == 1 or :a == 2) {\n    change_screen_map()\n}"
        );
        // A nested clean block is printed inside the outer arm; a `try_for_*` stays raw but
        // still indents its body.
        let nested = vec![
            op(4, vec![]),
            op(31, vec![text("$g_x"), int(0)]),
            op(4, vec![]),
            op(30, vec![text("$g_x"), int(1)]),
            op(2052, vec![]),
            op(3, vec![]),
            op(5, vec![]),
            op(6, vec![text(":i"), int(0), int(3)]),
            op(2052, vec![]),
            op(3, vec![]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &nested, Mode::Structured),
            concat!(
                "if $g_x == 0 {\n",
                "    if $g_x >= 1 {\n",
                "        change_screen_map()\n",
                "    }\n",
                "} else {\n",
                "    try_for_range(:i, 0, 3)\n",
                "        change_screen_map()\n",
                "    try_end()\n",
                "}"
            )
        );
    }

    #[test]
    fn decompile_names_registers_positions_and_slots() {
        let ops = vec![
            op(2133, vec![int(72057594037927936), text("$g_multiplayer_team_1_faction")]),
            op(701, vec![int(1)]),
            op(500, vec![int(0), int(7), int(1)]),
            op(510, vec![int(72057594037927937), int(9), int(1)]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Raw),
            concat!(
                "assign(reg0, $g_multiplayer_team_1_faction)\n",
                "init_position(pos1)\n",
                "troop_set_slot(0, slot_troop_x, 1)\n",
                "scene_prop_set_slot(reg1, slot_scene_prop_z, 1)"
            )
        );
        // The same names in structured text, where `assign` to a number stays a plain call.
        assert_eq!(
            decompile(&atlas(), &ops[..2], Mode::Structured),
            "assign(reg0, $g_multiplayer_team_1_faction)\ninit_position(pos1)"
        );
    }

    #[test]
    fn decompile_names_only_where_the_kit_is_unambiguous() {
        let ops = vec![
            // `pos0` / `pos1` are plain numbers (0, 1): only a position-marked operand is named.
            op(31, vec![int(1), int(0)]),
            // `<item_slot_no>` is an inventory slot, not the object `<slot_no>`.
            op(461, vec![int(0), int(7)]),
            // Two `slot_troop_*` names share 10: the number stays.
            op(500, vec![int(0), int(10), int(0)]),
            // 9 names a `slot_scene_prop_*` and a `slot_scene_*` constant, not a `slot_troop_*`.
            op(500, vec![int(0), int(9), int(0)]),
            // Register-tagged but no `reg<N>` constant of that value: the number stays.
            op(2133, vec![int(72057594037928036), text("$g_x")]),
        ];
        assert_eq!(
            decompile(&atlas(), &ops, Mode::Raw),
            concat!(
                "eq(1, 0)\n",
                "player_item_slot_is_picked_up(0, 7)\n",
                "troop_set_slot(0, 10, 0)\n",
                "troop_set_slot(0, 9, 0)\n",
                "assign(72057594037928036, $g_x)"
            )
        );
    }

    /// The real kit: the two constants the naming reads must exist, every position placeholder in
    /// its signatures must be either a position this decompiler names or a known non-position, and
    /// the goal's own example numbers name.
    #[test]
    fn real_atlas_position_placeholders_and_names() {
        // Signature placeholders with "position" in the name that are NOT a position number: screen
        // coordinates, scene bounds and free text. A new one in the kit needs a decision here.
        const NOT_A_POSITION: &[&str] =
            &["position_x", "position_y", "position_min", "position_max", "starting_position", "position"];
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        assert!(atlas.constant("op_num_value_bits").is_some(), "no op_num_value_bits");
        assert!(atlas.constant("tag_register").is_some(), "no tag_register");
        let mut unknown = Vec::new();
        for op in &atlas.ops {
            let mut index = 0;
            while let Some(placeholder) = placeholder_at(&op.signature, index) {
                if placeholder.contains("position")
                    && !is_position_placeholder(placeholder)
                    && !NOT_A_POSITION.contains(&placeholder)
                {
                    unknown.push(format!("{}: <{placeholder}>", op.name));
                }
                index += 1;
            }
        }
        assert!(unknown.is_empty(), "{unknown:?}");
        let names = Names::build(&atlas);
        let assign = atlas.op("assign").expect("assign op");
        // target/uw-native/scripts.uw: assign(72057594037927936, $g_multiplayer_team_1_faction)
        assert_eq!(names.name(assign, 0, 72057594037927936), Some("reg0"));
        let init = atlas.op("init_position").expect("init_position op");
        assert_eq!(names.name(init, 0, 1), Some("pos1"));
    }
}
