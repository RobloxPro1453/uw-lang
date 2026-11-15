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
            if ["try_begin(", "else_try(", "try_end("].iter().any(|p| line.starts_with(p))
                || line.contains("neg|")
                || line.contains("this_or_next|")
            {
                raw += 1;
            }
        }
        (lines, raw)
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
}
