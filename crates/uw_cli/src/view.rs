//! `uw view` (ROADMAP D8a): Native read as the core language. Today: the measure of how much of Native's
//! operations the language can already write (`uw_core::view`), per table, with the operations most often
//! left in the kit's form; and one entry's blocks printed line by line.

use std::collections::BTreeMap;

use uw_atlas::Atlas;
use uw_core::view::Viewer;
use uw_core::{Arg, Op};
use uw_decomp::{RawArg, RawOp};

/// A Native operation from the atlas as the core language's [`Op`] (flags split off, operands typed).
pub fn core_op(atlas: &Atlas, raw: &RawOp) -> Option<Op> {
    let bit = |name: &str| atlas.op(name).map_or(0, |op| op.code);
    let (neg, ton) = (bit("neg"), bit("this_or_next"));
    let op = atlas.op_for_code(raw.code)?;
    let args = raw
        .args
        .iter()
        .map(|arg| match arg {
            RawArg::Int(n) => Arg::Int(*n),
            RawArg::Str(s) if s.starts_with(':') => Arg::Local(s.clone()),
            RawArg::Str(s) if s.starts_with('$') => Arg::Global(s.clone()),
            RawArg::Str(s) if s.starts_with('@') => Arg::Text(s.clone()),
            RawArg::Str(s) => Arg::Name(s.clone()),
        })
        .collect();
    Some(Op {
        name: op.name.clone(),
        args,
        neg: neg != 0 && raw.code & neg != 0,
        this_or_next: ton != 0 && raw.code & ton != 0,
    })
}

/// How many operations of each table the language writes, and which operations it leaves most often.
pub struct Measure {
    /// table → (written, total)
    pub tables: BTreeMap<String, (usize, usize)>,
    /// operation → times left in the kit's form
    pub left: BTreeMap<String, usize>,
}

pub fn measure(atlas: &Atlas, viewer: &Viewer) -> Measure {
    let mut tables: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut left: BTreeMap<String, usize> = BTreeMap::new();
    for block in uw_decomp::extract::all(atlas) {
        let count = tables.entry(block.table.clone()).or_default();
        for raw in &block.ops {
            count.1 += 1;
            match core_op(atlas, raw) {
                Some(op) if viewer.line(atlas, &op).is_some() => count.0 += 1,
                Some(op) => *left.entry(op.name).or_default() += 1,
                None => *left.entry(format!("code {}", raw.code)).or_default() += 1,
            }
        }
    }
    Measure { tables, left }
}

fn percent(part: usize, whole: usize) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 * 100.0 / whole as f64 }
}

/// `uw view [--atlas <atlas.json>] [--top N]`: the measure.
pub fn cmd(args: &[String]) -> Result<(), String> {
    let mut atlas_arg = None;
    let mut top = 20usize;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--atlas" => atlas_arg = it.next().map(std::path::PathBuf::from),
            "--top" => top = it.next().and_then(|n| n.parse().ok()).ok_or("--top needs a number")?,
            other => return Err(format!("uw view: unknown argument '{other}'")),
        }
    }
    let path = crate::atlas_path(atlas_arg)?;
    let atlas = Atlas::from_path(&path).map_err(|err| format!("{}: {err}", path.display()))?;
    let viewer = Viewer::new();
    let m = measure(&atlas, &viewer);
    let (mut written, mut total) = (0, 0);
    println!("Native operations the core language writes (D8a, read-only view):");
    for (table, (w, t)) in &m.tables {
        written += w;
        total += t;
        println!("  {table:<20} {w:>6}/{t:<6} {:>5.1}%", percent(*w, *t));
    }
    println!("  {:<20} {written:>6}/{total:<6} {:>5.1}%", "all", percent(written, total));
    let mut left: Vec<(&String, &usize)> = m.left.iter().collect();
    left.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    println!("most often left in the kit's form:");
    for (name, count) in left.into_iter().take(top) {
        println!("  {count:>6}  {name}");
    }
    Ok(())
}
