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
    /// Only the raw text came back identical.
    Raw,
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
            Outcome::Raw => self.raw += 1,
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
            _ => match attempt(Mode::Raw) {
                (text, None) => (text, Outcome::Raw),
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
    }

    #[test]
    fn first_difference_names_the_op() {
        assert_eq!(first_difference(&[op(1, 1)], &[op(1, 1)]), None);
        assert_eq!(first_difference(&[op(1, 1)], &[op(1, 2)]).unwrap(), "op 0: want RawOp { code: 1, args: [Int(1)] }, got RawOp { code: 1, args: [Int(2)] }");
    }
}
