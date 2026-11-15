//! The round-trip measure: each block is decompiled, compiled back and compared with the original.
//! Structured text is tried first; when it does not come back identical the raw text is tried, so
//! the report tells both how readable and how complete the decompiler is.

use std::collections::BTreeMap;

use crate::decompile::Mode;
use crate::{Block, RawOp};

/// What happened to one block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Structured text came back identical.
    Structured,
    /// Only the raw text came back identical. `structured_reason` is why the structured attempt
    /// did not (compile error or first difference) and `structured_text` is what it printed.
    Raw { structured_reason: String, structured_text: String },
    /// Neither did; `reason` is the compile error or the first difference (of the raw attempt).
    Failed { reason: String },
}

/// One block's result, with the text that was accepted (or the raw text when none was).
#[derive(Debug, Clone)]
pub struct Result1 {
    pub label: String,
    pub table: String,
    pub outcome: Outcome,
    pub text: String,
}

/// Counts for one table (or the total).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Counts {
    pub blocks: usize,
    pub structured: usize,
    pub raw: usize,
    pub failed: usize,
}

impl Counts {
    fn add(&mut self, outcome: &Outcome) {
        self.blocks += 1;
        match outcome {
            Outcome::Structured => self.structured += 1,
            Outcome::Raw { .. } => self.raw += 1,
            Outcome::Failed { .. } => self.failed += 1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub total: Counts,
    pub tables: BTreeMap<String, Counts>,
    pub results: Vec<Result1>,
}

impl Report {
    /// Plain-text summary: one line per table, the total, then the first `max_failures` failures.
    pub fn summary(&self, max_failures: usize) -> String {
        let pct = |n: usize, of: usize| if of == 0 { 0.0 } else { n as f64 * 100.0 / of as f64 };
        let line = |name: &str, c: &Counts| {
            format!(
                "{name:<20} {:>6} blocks  structured {:>6} ({:5.1}%)  raw {:>6}  failed {:>6}  round-trip {:5.1}%\n",
                c.blocks,
                c.structured,
                pct(c.structured, c.blocks),
                c.raw,
                c.failed,
                pct(c.structured + c.raw, c.blocks)
            )
        };
        let mut out = String::new();
        for (name, c) in &self.tables {
            out.push_str(&line(name, c));
        }
        out.push_str(&line("TOTAL", &self.total));
        let (lines, raw_lines) = self.line_counts();
        out.push_str(&format!(
            "lines {lines}, still Native-shaped {raw_lines} ({:.1}%: try_begin/else_try/try_end, neg|, this_or_next|)\n",
            pct(raw_lines, lines)
        ));
        let classes = self.shape_breakdown();
        if !classes.is_empty() {
            out.push_str(&format!("\nwhy those {raw_lines} lines stayed Native-shaped, by pattern:\n"));
            for (key, n) in &classes {
                out.push_str(&format!("  {:>6}  {key} ({:.1}%)\n", n, pct(*n, raw_lines)));
            }
        }
        let groups = self.raw_groups();
        if !groups.is_empty() {
            out.push_str(&format!("\nwhy {} blocks stayed raw (structured attempt), by group:\n", self.total.raw));
            for (key, labels) in &groups {
                let shown: Vec<&str> = labels.iter().take(3).map(String::as_str).collect();
                out.push_str(&format!("  {:>5}  {key}  e.g. {}\n", labels.len(), shown.join(", ")));
            }
        }
        let failures: Vec<&Result1> =
            self.results.iter().filter(|r| matches!(r.outcome, Outcome::Failed { .. })).collect();
        if !failures.is_empty() {
            out.push_str(&format!("\nfirst {} of {} failures:\n", failures.len().min(max_failures), failures.len()));
            for r in failures.into_iter().take(max_failures) {
                if let Outcome::Failed { reason } = &r.outcome {
                    out.push_str(&format!("  {}: {}\n", r.label, reason));
                }
            }
        }
        out
    }

    /// Non-empty lines of every accepted text, and how many of them still read like Native: a raw
    /// `try_begin()` / `else_try()` / `try_end()` line or a `neg|` / `this_or_next|` flag.
    pub fn line_counts(&self) -> (usize, usize) {
        let mut lines = 0;
        let mut raw = 0;
        for line in self.results.iter().flat_map(|r| r.text.lines()).map(str::trim).filter(|l| !l.is_empty()) {
            lines += 1;
            if native_shaped(line) {
                raw += 1;
            }
        }
        (lines, raw)
    }

    /// Why the [`Self::line_counts`] Native-shaped lines still read like Native, by pattern, with
    /// the counts of the patterns that occur (in the order they are listed here, classes with no
    /// line left out). Every Native-shaped line lands in exactly one class, so the counts add up to
    /// the second value of `line_counts`:
    ///
    /// 1. a raw `try_begin()` whose first statement is not an `if` head (compute-then-test arm),
    /// 2. a guard condition mid-block (`neg|`),
    /// 3. a `this_or_next|` chain outside an `if` head (a line carrying both flags counts here),
    /// 4. an `else_try()` inside a loop body (a `try_for_*` block still surrounds the line),
    /// 5. anything else (a shell's `try_end()`, a shell the first two rules do not explain).
    pub fn shape_breakdown(&self) -> Vec<(String, usize)> {
        let mut counts = [0usize; 5];
        for r in &self.results {
            let lines: Vec<&str> = r.text.lines().filter(|l| !l.trim().is_empty()).collect();
            // The indent of every open `try_begin` / `try_for_*` block, and whether it is a loop.
            let mut open: Vec<(usize, bool)> = Vec::new();
            for (index, line) in lines.iter().enumerate() {
                let text = line.trim();
                let indent = line.len() - line.trim_start().len();
                // A `}` / `else_try()` / `try_end()` line closes every block it steps out of.
                let steps_out =
                    text.starts_with("}") || text.starts_with("else_try(") || text.starts_with("try_end(");
                while matches!(open.last(), Some(&(at, _)) if at > indent || (steps_out && at == indent)) {
                    open.pop();
                }
                if native_shaped(text) {
                    let class = if text.contains("this_or_next|") {
                        2
                    } else if text.contains("neg|") {
                        1
                    } else if text.starts_with("else_try(") && open.iter().any(|&(_, is_loop)| is_loop) {
                        3
                    } else if text.starts_with("try_begin(")
                        && !lines.get(index + 1).is_some_and(|next| is_if_head(next.trim()))
                    {
                        0
                    } else {
                        4
                    };
                    counts[class] += 1;
                }
                if text.starts_with("try_begin(") || text.starts_with("try_for_") {
                    open.push((indent, text.starts_with("try_for_")));
                }
            }
        }
        let names = [
            "compute-then-test arm",
            "guard condition mid-block (neg|)",
            "this_or_next| chain outside an if head",
            "else_try inside a loop body",
            "other",
        ];
        names.iter().zip(counts).filter(|(_, n)| *n > 0).map(|(&name, n)| (name.to_string(), n)).collect()
    }

    /// Raw blocks grouped by [`reason_group`] of their structured failure, largest group first.
    pub fn raw_groups(&self) -> Vec<(String, Vec<String>)> {
        let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for r in &self.results {
            if let Outcome::Raw { structured_reason, .. } = &r.outcome {
                groups.entry(reason_group(structured_reason)).or_default().push(r.label.clone());
            }
        }
        let mut groups: Vec<_> = groups.into_iter().collect();
        groups.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));
        groups
    }
}

/// True when a trimmed, non-empty line still reads like Native: a raw `try_begin()` /
/// `else_try()` / `try_end()` line or a `neg|` / `this_or_next|` flag.
fn native_shaped(line: &str) -> bool {
    ["try_begin(", "else_try(", "try_end("].iter().any(|p| line.starts_with(p))
        || line.contains("neg|")
        || line.contains("this_or_next|")
}

/// True when a trimmed line is a structured `if` head, i.e. an arm that starts with a condition.
fn is_if_head(line: &str) -> bool {
    line.starts_with("if ") && line.ends_with('{')
}

/// A short key that puts failures of the same kind together: for a difference the operation codes
/// on both sides (`diff want 31 got 30`, `args of 2133`, `length`), for a compile error the message
/// with every quoted name, number and position blanked.
pub fn reason_group(reason: &str) -> String {
    let code_after = |s: &str, tag: &str| {
        s.find(tag).map(|i| s[i + tag.len()..].chars().take_while(|c| c.is_ascii_digit()).collect::<String>())
    };
    if reason.starts_with("op ") {
        if let Some(rest) = reason.split_once(": want ").map(|(_, r)| r) {
            let (want, got) = rest.split_once(", got ").unwrap_or((rest, ""));
            let w = code_after(want, "code: ").unwrap_or_default();
            let g = code_after(got, "code: ").unwrap_or_default();
            return if w == g { format!("args of {w}") } else { format!("diff want {w} got {g}") };
        }
    }
    if reason.starts_with("length:") {
        return "length".into();
    }
    let mut out = String::new();
    let mut quoted = false;
    for c in reason.chars() {
        if c == '`' {
            if !quoted {
                out.push_str("`…`");
            }
            quoted = !quoted;
        } else if !quoted {
            if c.is_ascii_digit() {
                if !out.ends_with('#') {
                    out.push('#');
                }
            } else {
                out.push(c);
            }
        }
    }
    out.chars().take(100).collect()
}

/// First position where two blocks differ, described for a report.
pub fn first_difference(want: &[RawOp], got: &[RawOp]) -> Option<String> {
    for (i, (a, b)) in want.iter().zip(got).enumerate() {
        if a != b {
            return Some(format!("op {i}: want {a:?}, got {b:?}"));
        }
    }
    (want.len() != got.len()).then(|| format!("length: want {}, got {}", want.len(), got.len()))
}

/// Runs the measure. `print` is the decompiler, `compile` turns UW text back into operations
/// (parse + check + `to_raw`) or explains why it could not.
pub fn run(
    blocks: &[Block],
    mut print: impl FnMut(&[RawOp], Mode) -> String,
    mut compile: impl FnMut(&str) -> Result<Vec<RawOp>, String>,
) -> Report {
    let mut report = Report::default();
    for block in blocks {
        let mut attempt = |mode: Mode| {
            let text = print(&block.ops, mode);
            let verdict = match compile(&text) {
                Ok(got) => first_difference(&block.ops, &got),
                Err(e) => Some(e),
            };
            (text, verdict)
        };
        let (text, outcome) = match attempt(Mode::Structured) {
            (text, None) => (text, Outcome::Structured),
            (structured_text, Some(structured_reason)) => match attempt(Mode::Raw) {
                (text, None) => (text, Outcome::Raw { structured_reason, structured_text }),
                (text, Some(reason)) => (text, Outcome::Failed { reason }),
            },
        };
        report.total.add(&outcome);
        report.tables.entry(block.table.clone()).or_default().add(&outcome);
        report.results.push(Result1 { label: block.label.clone(), table: block.table.clone(), outcome, text });
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RawArg;

    fn op(code: u64, n: i64) -> RawOp {
        RawOp { code, args: vec![RawArg::Int(n)] }
    }

    fn block(table: &str, label: &str, ops: Vec<RawOp>) -> Block {
        Block { table: table.into(), index: 0, path: vec![1], label: label.into(), ops }
    }

    /// A report whose results carry exactly these accepted texts (the outcome does not matter).
    fn report_of(texts: &[&str]) -> Report {
        let mut report = Report::default();
        for text in texts {
            report.results.push(Result1 {
                label: "b".into(),
                table: "scripts".into(),
                outcome: Outcome::Structured,
                text: (*text).to_string(),
            });
        }
        report
    }

    /// A fake decompiler/compiler pair: structured text loses ops with code 9, raw text keeps
    /// everything except code 7, so each outcome can be produced on purpose.
    #[test]
    fn outcomes_are_counted_per_table() {
        let blocks = vec![
            block("scripts", "a", vec![op(1, 1)]),
            block("scripts", "b", vec![op(9, 2)]),
            block("menus", "c", vec![op(7, 3)]),
        ];
        let print = |ops: &[RawOp], mode: Mode| {
            ops.iter()
                .filter(|o| !(mode == Mode::Structured && o.code == 9) && o.code != 7)
                .map(|o| format!("{} {:?}", o.code, o.args[0]))
                .collect::<Vec<_>>()
                .join("\n")
        };
        let compile = |text: &str| -> Result<Vec<RawOp>, String> {
            text.lines()
                .map(|l| {
                    let (code, n) = l.split_once(" Int(").ok_or("bad line")?;
                    Ok(op(code.parse().unwrap(), n.trim_end_matches(')').parse().unwrap()))
                })
                .collect()
        };
        let report = run(&blocks, print, compile);
        assert_eq!(report.total, Counts { blocks: 3, structured: 1, raw: 1, failed: 1 });
        assert_eq!(report.tables["menus"], Counts { blocks: 1, structured: 0, raw: 0, failed: 1 });
        assert!(matches!(&report.results[2].outcome, Outcome::Failed { reason } if reason.contains("length")));
        let summary = report.summary(20);
        assert!(summary.contains("TOTAL"), "{summary}");
        assert!(summary.contains("  c: length: want 1, got 0"), "{summary}");
        assert!(matches!(&report.results[1].outcome, Outcome::Raw { structured_reason, .. } if structured_reason.contains("length")));
        assert_eq!(report.raw_groups(), vec![("length".to_string(), vec!["b".to_string()])]);
        assert!(summary.contains("      1  length  e.g. b"), "{summary}");
    }

    #[test]
    fn reasons_group_by_kind() {
        let d = first_difference(&[op(1, 1)], &[op(2, 1)]).unwrap();
        assert_eq!(reason_group(&d), "diff want 1 got 2");
        let d = first_difference(&[op(5, 1)], &[op(5, 2)]).unwrap();
        assert_eq!(reason_group(&d), "args of 5");
        assert_eq!(reason_group("error[E0201]: unknown name `foo` at 3:4"), reason_group("error[E0201]: unknown name `bar` at 9:12"));
    }

    #[test]
    fn first_difference_names_the_op() {
        assert_eq!(first_difference(&[op(1, 1)], &[op(1, 1)]), None);
        assert_eq!(first_difference(&[op(1, 1)], &[op(1, 2)]).unwrap(), "op 0: want RawOp { code: 1, args: [Int(1)] }, got RawOp { code: 1, args: [Int(2)] }");
    }

    #[test]
    fn breakdown_names_compute_then_test_arms() {
        // The first arm starts with an assignment, not a condition, so the shell cannot become an
        // `if`: the `try_begin()` is a compute-then-test arm, its `else_try()`/`try_end()` are not.
        let report = report_of(&[
            "try_begin()\n    $g_x = 1\n    if $g_x == 1 {\n        change_screen_map()\n    }\nelse_try()\n    :y = 2\ntry_end()",
        ]);
        assert_eq!(report.line_counts(), (8, 3));
        assert_eq!(
            report.shape_breakdown(),
            vec![("compute-then-test arm".to_string(), 1), ("other".to_string(), 2)]
        );
        // A shell whose first statement is an `if` head is not a compute-then-test arm.
        let report =
            report_of(&["try_begin()\n    if $g_x == 1 {\n        change_screen_map()\n    }\ntry_end()"]);
        assert_eq!(report.shape_breakdown(), vec![("other".to_string(), 2)]);
    }

    #[test]
    fn breakdown_names_guard_conditions() {
        let report = report_of(&["if $g_x == 1 {\n    neg|ge($g_x, 2)\n}"]);
        assert_eq!(report.line_counts(), (3, 1));
        assert_eq!(report.shape_breakdown(), vec![("guard condition mid-block (neg|)".to_string(), 1)]);
    }

    #[test]
    fn breakdown_names_this_or_next_chains() {
        let report =
            report_of(&["if $g_x == 1 {\n    this_or_next|ge($g_x, 2)\n    ge($g_x, 3)\n}"]);
        assert_eq!(report.line_counts(), (4, 1));
        assert_eq!(report.shape_breakdown(), vec![("this_or_next| chain outside an if head".to_string(), 1)]);
        // Both flags on one line: it is part of a chain, so it counts there once.
        let report = report_of(&["if $g_x == 1 {\n    neg|this_or_next|ge($g_x, 2)\n}"]);
        assert_eq!(report.line_counts(), (3, 1));
        assert_eq!(report.shape_breakdown(), vec![("this_or_next| chain outside an if head".to_string(), 1)]);
    }

    #[test]
    fn breakdown_names_else_try_in_a_loop_body() {
        let report = report_of(&[
            "try_for_range(:i, 0, 3)\n    try_begin()\n        $g_x = 1\n    else_try()\n        :y = 2\n    try_end()\ntry_end()",
        ]);
        assert_eq!(report.line_counts(), (7, 4));
        assert_eq!(
            report.shape_breakdown(),
            vec![
                ("compute-then-test arm".to_string(), 1),
                ("else_try inside a loop body".to_string(), 1),
                ("other".to_string(), 2),
            ]
        );
    }

    /// All five patterns in one text: the classes add up to the Native-shaped total and the
    /// summary shows the breakdown right below the `lines …` line.
    #[test]
    fn breakdown_classes_add_up_to_the_native_shaped_total() {
        let report = report_of(&[
            concat!(
                "try_begin()\n",
                "    if $g_x == 1 {\n",
                "        neg|ge($g_x, 2)\n",
                "        this_or_next|ge($g_x, 3)\n",
                "    }\n",
                "else_try()\n",
                "    :y = 2\n",
                "try_end()\n",
                "try_for_range(:i, 0, 3)\n",
                "    try_begin()\n",
                "        $g_x = 1\n",
                "    else_try()\n",
                "        :y = 3\n",
                "    try_end()\n",
                "try_end()"
            ),
        ]);
        let (_, raw_lines) = report.line_counts();
        assert_eq!(raw_lines, 9);
        let breakdown = report.shape_breakdown();
        assert_eq!(
            breakdown,
            vec![
                ("compute-then-test arm".to_string(), 1),
                ("guard condition mid-block (neg|)".to_string(), 1),
                ("this_or_next| chain outside an if head".to_string(), 1),
                ("else_try inside a loop body".to_string(), 1),
                ("other".to_string(), 5),
            ]
        );
        assert_eq!(breakdown.iter().map(|(_, n)| n).sum::<usize>(), raw_lines);
        let summary = report.summary(20);
        let lines_at = summary.find("lines 15, still Native-shaped 9").expect("lines line");
        let why_at = summary.find("why those 9 lines stayed Native-shaped").expect("breakdown");
        assert!(lines_at < why_at, "{summary}");
        assert!(summary.contains("       1  compute-then-test arm (11.1%)"), "{summary}");
        assert!(summary.contains("       1  else_try inside a loop body (11.1%)"), "{summary}");
        assert!(summary.contains("       5  other (55.6%)"), "{summary}");
    }
}
