//! `uw roundtrip`: every Native operation block -> UW text -> parse + check -> operations, compared
//! with the original. In memory; no kit, no Python.
//! Every record of a table that has a schema (`kit-meta/records/<table>.toml`) is printed and parsed
//! back the same way.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value as Json;
use uw_decomp::decompile::{decompile, Mode};
use uw_decomp::records::{
    ops_to_json, print_with_blocks, roundtrip_table_with_blocks, Schema, TableReport,
};
use uw_decomp::roundtrip::{first_difference, run, Outcome, Report};
use uw_decomp::{Block, RawOp};
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

/// The schema folder of `dir` or a parent folder, then of the current folder: `uw build <folder>`
/// runs outside the project too (T-092).
pub(crate) fn find_schemas_from(dir: &Path) -> Option<PathBuf> {
    let mut starts = vec![dir.to_path_buf()];
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    starts
        .iter()
        .flat_map(|start| start.ancestors().map(|dir| dir.join(SCHEMA_REL)).collect::<Vec<_>>())
        .find(|path| path.is_dir())
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
                    out.push_str(&format!(
                        "{name:<20} typed {}/{} fields, raw-rest {} values\n",
                        r.typed_fields, r.fields, r.raw_rest
                    ));
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

/// The block paths of every record, keyed by `(table, entry index)`.
pub(crate) fn block_paths(blocks: &[Block]) -> std::collections::BTreeMap<(&str, usize), Vec<Vec<usize>>> {
    let mut paths: std::collections::BTreeMap<(&str, usize), Vec<Vec<usize>>> = Default::default();
    for block in blocks {
        paths.entry((block.table.as_str(), block.index)).or_default().push(block.path.clone());
    }
    paths
}

/// One operation block as readable UW text: structured when that compiles back identical, raw
/// otherwise, the same choice [`run`] makes.
pub(crate) fn block_text(atlas: &uw_atlas::Atlas, block: &Json) -> String {
    let ops = uw_decomp::parse_ops(block).unwrap_or_default();
    let structured = decompile(atlas, &ops, Mode::Structured);
    match compile(atlas, &structured) {
        Ok(got) if first_difference(&ops, &got).is_none() => structured,
        _ => decompile(atlas, &ops, Mode::Raw),
    }
}

/// Round-trips every record of `atlas` through `schemas` and collects the texts to write.
/// Operation blocks (`extract::all`) print as `ops { … }`.
fn measure_records(atlas: &uw_atlas::Atlas, schemas: &[Schema]) -> Records {
    let blocks = uw_decomp::extract::all(atlas);
    let paths = block_paths(&blocks);
    let mut records = Records::default();
    for schema in schemas {
        let entries = atlas.tables.get(&schema.table).map(|table| table.entries.as_slice()).unwrap_or(&[]);
        let text = entries
            .iter()
            .map(|entry| {
                let paths = paths.get(&(schema.table.as_str(), entry.index)).map(Vec::as_slice).unwrap_or(&[]);
                print_with_blocks(atlas, schema, entry, paths, &mut |block| block_text(atlas, block))
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !text.is_empty() {
            records.texts.insert(schema.table.clone(), text);
        }
        let mut compile_block = |text: &str| compile(atlas, text).map(|ops| ops_to_json(&ops));
        let report = roundtrip_table_with_blocks(
            atlas,
            schema,
            &blocks,
            &mut |block| block_text(atlas, block),
            &mut compile_block,
        );
        records.tables.insert(schema.table.clone(), Some(report));
    }
    for name in atlas.tables.keys() {
        records.tables.entry(name.clone()).or_insert(None);
    }
    records
}

/// UW block text -> operations, the way a mod's block is checked and lowered.
pub fn compile(atlas: &uw_atlas::Atlas, text: &str) -> Result<Vec<RawOp>, String> {
    compile_with(atlas, text, &[])
}

/// [`compile`] with the `fn`s of `sources` visible: a project's record block may call a word from
/// `lib/*.uw` or its own `fns/*.uw` (T-084). Without sources this is exactly [`compile`].
pub fn compile_with(
    atlas: &uw_atlas::Atlas,
    text: &str,
    sources: &[uw_check::Source],
) -> Result<Vec<RawOp>, String> {
    let file = SourceFile::new("block.uw", text);
    // Keep the first diagnostic's position: a project record needs it to point back into its file
    // (records::parse_with_blocks shifts it to the record, project::load to the file).
    let one_line = |d: &uw_diag::Diagnostic| {
        let (line, column) = file.line_col(d.span.start);
        format!("line {line}:{column}: error[{}]: {}", d.code, d.message)
    };
    let stmts = uw_syntax::parse_block(text).map_err(|d| one_line(&d))?;
    let ops = uw_check::check_block_with(atlas, &file, &stmts, sources)
        .map_err(|ds| ds.first().map(one_line).unwrap_or_default())?;
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

/// `--names`: Native's own game names ([`uw_decomp::names::derive`]) so every block prints `praven`
/// instead of `p_town_6`. The round-trip has to survive that: the text must compile back to the same
/// operations. Returns how many names were installed.
fn install_derived_names(atlas: &mut uw_atlas::Atlas) -> usize {
    let names = uw_decomp::names::derive(atlas);
    atlas.set_game_names(&uw_decomp::names::pairs(&names));
    names.len()
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut atlas_path: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut schemas_dir: Option<PathBuf> = None;
    let mut show = 20usize;
    let mut names = false;
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
            // P5b: measure with the game names (`praven`), not only the kit references (`p_town_6`).
            "--names" => names = true,
            other => return crate::fail(&format!("unknown argument `{other}`")),
        }
    }
    let Some(path) = atlas_path
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(crate::find_atlas)
    else {
        return crate::fail(&format!("no atlas found ({} in this or a parent folder)", crate::ATLAS_REL));
    };
    let mut atlas = match uw_atlas::Atlas::from_path(&path) {
        Ok(a) => a,
        Err(e) => return crate::fail(&format!("{}: {e}", path.display())),
    };
    if names {
        install_derived_names(&mut atlas);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The golden tests' atlas: `p_town_6` is shown as "Praven", so `--names` derives `praven`.
    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");

    #[test]
    fn the_names_option_measures_with_game_names() {
        let mut atlas = uw_atlas::Atlas::from_path(FIXTURE).expect("the golden atlas fixture loads");
        // Four names: `praven` (Praven), `swadian_crossbowman` for the troop T-092 copies,
        // `king_harlaus` (King Harlaus) for the troop T-102's dialog talks to, and `iron_shield`
        // for T-114's condition-word example. T-119's faction adds `swadia` (Kingdom of Swadia).
        assert_eq!(install_derived_names(&mut atlas), 5);
        assert_eq!(atlas.reference_for("praven"), Some("p_town_6"));
        assert_eq!(atlas.reference_for("king_harlaus"), Some("trp_kingdom_1_lord"));
        assert_eq!(atlas.reference_for("iron_shield"), Some("itm_iron_shield"));
        assert_eq!(atlas.reference_for("swadia"), Some("fac_kingdom_1"));
        // The game name compiles like its kit reference and prints back as the game name.
        let text = "party_relocate_near_party(praven, p_main_party, 3)";
        let ops = compile(&atlas, text).expect("the game name is accepted");
        assert!(decompile(&atlas, &ops, Mode::Structured).contains("praven"));
        // And the whole measure still comes back identical with names installed.
        let report = measure(&atlas);
        assert_eq!(report.total.failed, 0);
    }
}
