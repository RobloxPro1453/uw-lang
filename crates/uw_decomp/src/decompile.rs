//! Native operation block -> UW text (docs/tasks/T-020).

use crate::{RawArg, RawOp};
use uw_atlas::Atlas;

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
}

impl<'a> Printer<'a> {
    fn new(atlas: &'a Atlas) -> Self {
        let code = |name: &str| atlas.op(name).map_or(0, |op| op.code);
        Printer { atlas, neg: code("neg"), this_or_next: code("this_or_next") }
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
        let Some(name) = self.op_name(op.code) else {
            return format!("// unknown operation {}", op.code);
        };
        let mut head = String::new();
        if self.has(op.code, self.neg) {
            head.push_str("neg|");
        }
        if self.has(op.code, self.this_or_next) {
            head.push_str("this_or_next|");
        }
        head.push_str(name);
        format!("{head}({})", self.args(&op.args))
    }

    fn args(&self, args: &[RawArg]) -> String {
        args.iter().map(|arg| self.arg(arg)).collect::<Vec<_>>().join(", ")
    }

    /// One operand: a number, `$global` / `:local` as written, a reference the atlas knows as a
    /// bare name, anything else as escaped text.
    fn arg(&self, arg: &RawArg) -> String {
        match arg {
            RawArg::Int(n) => n.to_string(),
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
                    // Anything not clean stays raw for that whole try block.
                    None => self.raw_into(&ops[index..=end], depth, out),
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

    /// `assign($x, v)` / `assign(:x, v)` as `x = v`; `None` for anything else.
    fn assign_line(&self, op: &RawOp) -> Option<String> {
        if self.has(op.code, self.neg) || self.has(op.code, self.this_or_next) {
            return None;
        }
        if self.op_name(op.code)? != "assign" || op.args.len() != 2 {
            return None;
        }
        let RawArg::Str(target) = &op.args[0] else { return None };
        if !(target.starts_with('$') || target.starts_with(':')) {
            return None;
        }
        Some(format!("{target} = {}", self.arg(&op.args[1])))
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
        let name = self.op_name(op.code)?;
        let negated = self.has(op.code, self.neg);
        if let Some(cmp) = comparison(name, negated) {
            if op.args.len() != 2 {
                return None;
            }
            return Some(format!("{} {cmp} {}", self.arg(&op.args[0]), self.arg(&op.args[1])));
        }
        let mut head = String::new();
        if negated {
            head.push_str("not ");
        }
        head.push_str(name);
        Some(format!("{head}({})", self.args(&op.args)))
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
            {"name":"assign","code":2133,"line":40,"signature":"","lhs":true,"can_fail":false,"global_lhs":false}
          ],
          "hooks": ["game_start"],
          "ranges": [],
          "constants": [
            {"name":"reg0","value":1,"module":"header_common","line":563}
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
        // The first arm starts with an assignment, so the whole block stays raw.
        let unclean = vec![
            op(4, vec![]),
            op(2133, vec![text("$g_x"), int(1)]),
            op(3, vec![]),
        ];
        assert_eq!(
            decompile(&atlas(), &unclean, Mode::Structured),
            "try_begin()\n    assign($g_x, 1)\ntry_end()"
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
}
