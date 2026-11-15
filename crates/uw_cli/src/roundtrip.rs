//! `uw roundtrip`: every Native operation block -> UW text -> parse + check -> operations, compared
//! with the original (kaldıraç plan, docs/meclis/DEVIR.md). In memory; no kit, no Python.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_decomp::decompile::decompile;
use uw_decomp::roundtrip::{run, Outcome, Report};
use uw_decomp::RawOp;
use uw_diag::SourceFile;


/// Decompiles, compiles back and compares every block of `atlas`.
pub fn measure(atlas: &uw_atlas::Atlas) -> Report {
    let blocks = uw_decomp::extract::all(atlas);
    run(&blocks, |ops, mode| decompile(atlas, ops, mode), |text| compile(atlas, text))
}

/// UW block text -> operations, the way a mod's block is checked and lowered.
pub fn compile(atlas: &uw_atlas::Atlas, text: &str) -> Result<Vec<RawOp>, String> {
    let file = SourceFile::new("block.uw", text);
    let one_line = |s: String| s.lines().find(|l| l.contains("error")).unwrap_or(&s).trim().to_string();
    let stmts = uw_syntax::parse_block(text).map_err(|d| one_line(d.render(&file)))?;
    let ops = uw_check::check_block(atlas, &file, &stmts)
        .map_err(|ds| ds.first().map(|d| one_line(d.render(&file))).unwrap_or_default())?;
    uw_decomp::to_raw(atlas, &ops)
}

/// Writes one `<table>.uw` per table (every block under a `// label (outcome)` line) and
/// `failures.txt`, and `raw.txt` (each raw block's structured attempt and why it failed).
fn write_out(report: &Report, dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut per_table: std::collections::BTreeMap<&str, String> = Default::default();
    let mut failures = String::new();
    let mut raw = String::new();
    for r in &report.results {
        let tag = match &r.outcome {
            Outcome::Structured => "structured".to_string(),
            Outcome::Raw { structured_reason, structured_text } => {
                let _ = writeln!(raw, "{}: {structured_reason}\n{}\n", r.label, structured_text.trim_end());
                "raw".to_string()
            }
            Outcome::Failed { reason } => {
                let _ = writeln!(failures, "{}: {reason}\n{}\n", r.label, r.text);
                "FAILED".to_string()
            }
        };
        let out = per_table.entry(&r.table).or_default();
        let _ = writeln!(out, "// {} ({tag})\n{}\n", r.label, r.text.trim_end());
    }
    for (table, text) in per_table {
        std::fs::write(dir.join(format!("{table}.uw")), text)?;
    }
    std::fs::write(dir.join("failures.txt"), failures)?;
    std::fs::write(dir.join("raw.txt"), raw)?;
    std::fs::write(dir.join("summary.txt"), report.summary(usize::MAX))?;
    Ok(())
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut atlas_path: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut show = 20usize;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--atlas" => atlas_path = it.next().map(PathBuf::from),
            "--out" => out = it.next().map(PathBuf::from),
            "--failures" => match it.next().and_then(|n| n.parse().ok()) {
                Some(n) => show = n,
                None => return crate::fail("--failures needs a number"),
            },
            other => return crate::fail(&format!("unknown argument `{other}`")),
        }
    }
    let Some(path) = atlas_path
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(crate::find_atlas)
    else {
        return crate::fail(&format!("no atlas found ({} in this or a parent folder)", crate::ATLAS_REL));
    };
    let atlas = match uw_atlas::Atlas::from_path(&path) {
        Ok(a) => a,
        Err(e) => return crate::fail(&format!("{}: {e}", path.display())),
    };
    let report = measure(&atlas);
    print!("{}", report.summary(show));
    let dir = out.unwrap_or_else(|| {
        path.parent().and_then(Path::parent).unwrap_or(Path::new(".")).join("uw-native")
    });
    if let Err(e) = write_out(&report, &dir) {
        return crate::fail(&format!("{}: {e}", dir.display()));
    }
    println!("\ntext: {}", dir.display());
    if report.total.failed == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
