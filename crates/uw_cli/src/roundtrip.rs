//! `uw roundtrip`: every Native operation block -> UW text -> parse + check -> operations, compared
//! with the original (kaldıraç plan, docs/meclis/DEVIR.md). In memory; no kit, no Python.
//! Every record of a table that has a schema (`kit-meta/records/<table>.toml`) is printed and parsed
//! back the same way.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_decomp::decompile::decompile;
use uw_decomp::records::{Schema, TableReport};
use uw_decomp::roundtrip::{run, Outcome, Report};
use uw_decomp::RawOp;
use uw_diag::SourceFile;

/// Where the record schemas live unless `--schemas` says otherwise (this or a parent folder).
const SCHEMA_REL: &str = "kit-meta/records";

/// Decompiles, compiles back and compares every block of `atlas`.
pub fn measure(atlas: &uw_atlas::Atlas) -> Report {
    let blocks = uw_decomp::extract::all(atlas);
    run(&blocks, |ops, mode| decompile(atlas, ops, mode), |text| compile(atlas, text))
}

/// The record schemas: every `kit-meta/records/*.toml` of `dir`.
pub(crate) fn load_schemas(dir: &Path) -> Result<Vec<Schema>, String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .collect();
    paths.sort();
    let mut schemas = Vec::new();
    for path in paths {
        schemas.push(Schema::from_path(&path)?);
    }
    Ok(schemas)
}

/// The schema folder of this or a parent folder, like the atlas and `kit-meta/aciklama`.
pub(crate) fn find_schemas() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    cwd.ancestors().map(|dir| dir.join(SCHEMA_REL)).find(|path| path.is_dir())
}

/// Every atlas table's record measure, and the record texts to write.
#[derive(Default)]
struct Records {
    /// Atlas table name -> its measure (`None`: no schema for that table), in table order.
    tables: std::collections::BTreeMap<String, Option<TableReport>>,
    /// Schema table name -> every record's text.
    texts: std::collections::BTreeMap<String, String>,
}

impl Records {
    /// How many records did not come back identical.
    fn failed(&self) -> usize {
        self.tables.values().flatten().map(|report| report.failed).sum()
    }

    /// Plain-text summary: `records N, exact N, failed N` per table with a schema, `no schema` for
    /// the rest, the total, then the first `max_failures` failures.
    fn summary(&self, max_failures: usize) -> String {
        let line = |name: &str, r: &TableReport| {
            format!("{name:<20} {:>6} records, exact {:>6}, failed {:>6}\n", r.records, r.exact, r.failed)
        };
        let mut total = TableReport::default();
        let mut out = String::from("\nrecords\n");
        for (name, report) in &self.tables {
            match report {
                Some(r) => {
                    total.records += r.records;
                    total.exact += r.exact;
                    total.failed += r.failed;
                    out.push_str(&line(name, r));
                }
                None => out.push_str(&format!("{name:<20} no schema\n")),
            }
        }
        out.push_str(&line("TOTAL", &total));
        let failures: Vec<(&str, &str)> = self
            .tables
            .iter()
            .filter_map(|(name, report)| report.as_ref().map(|r| (name.as_str(), r)))
            .flat_map(|(name, report)| report.first_failures.iter().map(move |m| (name, m.as_str())))
            .collect();
        if !failures.is_empty() {
            out.push_str(&format!(
                "\nfirst {} of {} record failures:\n",
                failures.len().min(max_failures),
                failures.len()
            ));
            for (table, message) in failures.into_iter().take(max_failures) {
                out.push_str(&format!("  {table}: {message}\n"));
            }
        }
        out
    }
}

/// Round-trips every record of `atlas` through `schemas` and collects the texts to write.
fn measure_records(atlas: &uw_atlas::Atlas, schemas: &[Schema]) -> Records {
    let mut records = Records::default();
    for schema in schemas {
        let text = match atlas.tables.get(&schema.table) {
            Some(table) => table
                .entries
                .iter()
                .map(|entry| uw_decomp::records::print(schema, entry))
                .collect::<Vec<_>>()
                .join("\n"),
            None => String::new(),
        };
        if !text.is_empty() {
            records.texts.insert(schema.table.clone(), text);
        }
        records.tables.insert(schema.table.clone(), Some(uw_decomp::records::roundtrip_table(atlas, schema)));
    }
    for name in atlas.tables.keys() {
        records.tables.entry(name.clone()).or_insert(None);
    }
    records
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

/// Writes every schema table's records, `<table>.uw` each, under `dir`.
fn write_records(records: &Records, dir: &Path) -> std::io::Result<()> {
    if records.texts.is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(dir)?;
    for (table, text) in &records.texts {
        std::fs::write(dir.join(format!("{table}.uw")), text)?;
    }
    Ok(())
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut atlas_path: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut schemas_dir: Option<PathBuf> = None;
    let mut show = 20usize;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--atlas" => atlas_path = it.next().map(PathBuf::from),
            "--out" => out = it.next().map(PathBuf::from),
            "--schemas" => match it.next() {
                Some(dir) => schemas_dir = Some(PathBuf::from(dir)),
                None => return crate::fail("--schemas needs a folder"),
            },
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
    let schemas = match schemas_dir.or_else(find_schemas) {
        Some(dir) => match load_schemas(&dir) {
            Ok(schemas) => schemas,
            Err(e) => return crate::fail(&e),
        },
        // Without a schema folder no table has a schema: the records block lists them all so.
        None => Vec::new(),
    };
    let report = measure(&atlas);
    let records = measure_records(&atlas, &schemas);
    print!("{}", report.summary(show));
    print!("{}", records.summary(show));
    let dir = out.unwrap_or_else(|| {
        path.parent().and_then(Path::parent).unwrap_or(Path::new(".")).join("uw-native")
    });
    if let Err(e) = write_out(&report, &dir) {
        return crate::fail(&format!("{}: {e}", dir.display()));
    }
    if let Err(e) = write_records(&records, &dir.join("records")) {
        return crate::fail(&format!("{}: {e}", dir.join("records").display()));
    }
    println!("\ntext: {}", dir.display());
    if report.total.failed == 0 && records.failed() == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
