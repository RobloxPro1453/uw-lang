//! `uw roundtrip`: every Native operation block -> UW text -> parse + check -> operations, compared
//! with the original (kaldıraç plan, docs/meclis/DEVIR.md). In memory; no kit, no Python.
//! Every record of a table that has a schema (`kit-meta/records/<table>.toml`) is printed and parsed
//! back the same way.

use std::collections::BTreeMap;
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
use uw_syntax::{Arg, Block as StmtBlock, Call, Cond, Expr, Ident, Stmt};

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
    let one_line = |s: String| s.lines().find(|l| l.contains("error")).unwrap_or(&s).trim().to_string();
    let stmts = uw_syntax::parse_block(text).map_err(|d| one_line(d.render(&file)))?;
    let mut expand = Expand { words: collect_words(sources), stack: Vec::new(), rename: 0, error: None };
    let stmts: Vec<Stmt> = stmts.iter().flat_map(|stmt| expand.statement(stmt)).collect();
    if let Some(error) = expand.error {
        return Err(error);
    }
    let ops = uw_check::check_block(atlas, &file, &stmts)
        .map_err(|ds| ds.first().map(|d| one_line(d.render(&file))).unwrap_or_default())?;
    uw_decomp::to_raw(atlas, &ops)
}

/// Deepest chain of `fn` calls expanded here, like `uw_check::check`'s own limit.
const FN_DEPTH: usize = 16;

/// One `fn` a project block may call: its parameters and body.
struct Word<'a> {
    params: &'a [Ident],
    body: &'a StmtBlock,
}

/// Every `fn` of `sources` by name; the first definition wins (`lib` comes before the project).
fn collect_words<'a>(sources: &'a [uw_check::Source]) -> BTreeMap<String, Word<'a>> {
    let mut words = BTreeMap::new();
    for source in sources {
        for item in &source.tree.items {
            if let uw_syntax::Item::Fn { name, params, body, .. } = item {
                words.entry(name.text.clone()).or_insert(Word { params, body });
            }
        }
    }
    words
}

/// Expands `fn` calls in a project's record block before it is checked: `uw_check::check_block`
/// checks one block alone and cannot see the project's fns (T-084).
struct Expand<'a> {
    words: BTreeMap<String, Word<'a>>,
    stack: Vec<String>,
    rename: usize,
    error: Option<String>,
}

impl Expand<'_> {
    fn block(&mut self, block: &StmtBlock) -> StmtBlock {
        StmtBlock {
            stmts: block.stmts.iter().flat_map(|stmt| self.statement(stmt)).collect(),
            span: block.span,
        }
    }

    fn statement(&mut self, stmt: &Stmt) -> Vec<Stmt> {
        match stmt {
            Stmt::Call(call) if self.words.contains_key(&call.op.text) => self.inline(call),
            Stmt::Assign { target: Arg::Name(name), op, value, span } if is_dotted(name) => {
                let verb = match op {
                    uw_syntax::AssignOp::Set => "set",
                    uw_syntax::AssignOp::Add => "add",
                    uw_syntax::AssignOp::Sub => "remove",
                    // `*=` `/=` `%=` have no object word; `check_block` reports them.
                    _ => return vec![stmt.clone()],
                };
                let Expr::Value(arg) = value else { return vec![stmt.clone()] };
                let call = Call {
                    op: Ident { text: format!("{}.{verb}", name.text), span: name.span },
                    args: vec![arg.clone()],
                    span: *span,
                };
                if self.words.contains_key(&call.op.text) {
                    self.inline(&call)
                } else {
                    vec![stmt.clone()]
                }
            }
            Stmt::If { arms, otherwise, span } => vec![Stmt::If {
                arms: arms.iter().map(|(cond, body)| (cond.clone(), self.block(body))).collect(),
                otherwise: otherwise.as_ref().map(|body| self.block(body)),
                span: *span,
            }],
            Stmt::Attempt { arms, span } => vec![Stmt::Attempt {
                arms: arms.iter().map(|body| self.block(body)).collect(),
                span: *span,
            }],
            Stmt::For { var, kind, args, body, span } => vec![Stmt::For {
                var: var.clone(),
                kind: kind.clone(),
                args: args.clone(),
                body: self.block(body),
                span: *span,
            }],
            _ => vec![stmt.clone()],
        }
    }

    fn inline(&mut self, call: &Call) -> Vec<Stmt> {
        let name = call.op.text.clone();
        let (params, body) = match self.words.get(&name) {
            Some(word) => (word.params.iter().map(|param| param.text.clone()).collect::<Vec<_>>(), word.body.clone()),
            None => return vec![Stmt::Call(call.clone())],
        };
        if self.error.is_some() {
            return Vec::new();
        }
        if call.args.len() != params.len() {
            let help = if params.is_empty() {
                format!("call it as `{name}()`")
            } else {
                format!("call it as `{name}({})`", params.join(", "))
            };
            self.error = Some(format!(
                "`fn {name}` takes {} value(s), got {}; {help}",
                params.len(),
                call.args.len()
            ));
            return Vec::new();
        }
        if self.stack.iter().any(|seen| seen == &name) {
            let mut chain = self.stack.clone();
            chain.push(name.clone());
            chain.push(name);
            self.error = Some(format!("`fn` never ends: {}", chain.join(" -> ")));
            return Vec::new();
        }
        if self.stack.len() >= FN_DEPTH {
            let mut chain = self.stack.clone();
            chain.push(name.clone());
            self.error = Some(format!("`fn` calls nest more than {FN_DEPTH} deep: {}", chain.join(" -> ")));
            return Vec::new();
        }
        let binds: Vec<(String, Arg)> = params.iter().cloned().zip(call.args.iter().cloned()).collect();
        let site = self.rename;
        self.rename += 1;
        let body = substituted_block(&body, &binds, site);
        self.stack.push(name);
        let out = body.stmts.iter().flat_map(|stmt| self.statement(stmt)).collect();
        self.stack.pop();
        out
    }
}

/// `x.y` used as a word (`player.gold += 100`).
fn is_dotted(name: &Ident) -> bool {
    name.text.contains('.')
}

/// A `fn` body copied for one call site: `binds` holds the call's argument per parameter and
/// `site` numbers the copy, so the body's own `:locals` stay apart from every other call.
fn substituted_stmt(stmt: &Stmt, binds: &[(String, Arg)], site: usize) -> Stmt {
    match stmt {
        Stmt::Call(call) => Stmt::Call(substituted_call(call, binds, site)),
        Stmt::Assign { target, op, value, span } => Stmt::Assign {
            target: substituted_arg(target, binds, site),
            op: *op,
            value: substituted_expr(value, binds, site),
            span: *span,
        },
        Stmt::If { arms, otherwise, span } => Stmt::If {
            arms: arms
                .iter()
                .map(|(cond, body)| (substituted_cond(cond, binds, site), substituted_block(body, binds, site)))
                .collect(),
            otherwise: otherwise.as_ref().map(|body| substituted_block(body, binds, site)),
            span: *span,
        },
        Stmt::Require { cond, span } => Stmt::Require { cond: substituted_cond(cond, binds, site), span: *span },
        Stmt::Attempt { arms, span } => Stmt::Attempt {
            arms: arms.iter().map(|body| substituted_block(body, binds, site)).collect(),
            span: *span,
        },
        Stmt::For { var, kind, args, body, span } => Stmt::For {
            var: substituted_arg(var, binds, site),
            kind: kind.clone(),
            args: args.iter().map(|arg| substituted_arg(arg, binds, site)).collect(),
            body: substituted_block(body, binds, site),
            span: *span,
        },
    }
}

fn substituted_block(block: &StmtBlock, binds: &[(String, Arg)], site: usize) -> StmtBlock {
    StmtBlock {
        stmts: block.stmts.iter().map(|stmt| substituted_stmt(stmt, binds, site)).collect(),
        span: block.span,
    }
}

fn substituted_call(call: &Call, binds: &[(String, Arg)], site: usize) -> Call {
    Call {
        op: call.op.clone(),
        args: call.args.iter().map(|arg| substituted_arg(arg, binds, site)).collect(),
        span: call.span,
    }
}

fn substituted_cond(cond: &Cond, binds: &[(String, Arg)], site: usize) -> Cond {
    match cond {
        Cond::Cmp { op, left, right, span } => Cond::Cmp {
            op: *op,
            left: substituted_expr(left, binds, site),
            right: substituted_expr(right, binds, site),
            span: *span,
        },
        Cond::Call(call) => Cond::Call(substituted_call(call, binds, site)),
        Cond::Not(inner, span) => Cond::Not(Box::new(substituted_cond(inner, binds, site)), *span),
        Cond::And(list) => Cond::And(list.iter().map(|cond| substituted_cond(cond, binds, site)).collect()),
        Cond::Or(list) => Cond::Or(list.iter().map(|cond| substituted_cond(cond, binds, site)).collect()),
    }
}

fn substituted_expr(expr: &Expr, binds: &[(String, Arg)], site: usize) -> Expr {
    match expr {
        Expr::Value(arg) => Expr::Value(substituted_arg(arg, binds, site)),
        Expr::Neg(inner, span) => Expr::Neg(Box::new(substituted_expr(inner, binds, site)), *span),
        Expr::Bin { op, left, right, span } => Expr::Bin {
            op: *op,
            left: Box::new(substituted_expr(left, binds, site)),
            right: Box::new(substituted_expr(right, binds, site)),
            span: *span,
        },
    }
}

/// A parameter becomes the call's argument; any other `:local` of the body gets a `__fn<site>`
/// suffix, like `uw_check::check` does for a mod's blocks.
fn substituted_arg(arg: &Arg, binds: &[(String, Arg)], site: usize) -> Arg {
    match arg {
        Arg::Local(ident) => match binds.iter().find(|(param, _)| param == &ident.text) {
            Some((_, value)) => value.clone(),
            None => Arg::Local(Ident { text: format!("{}__fn{site}", ident.text), span: ident.span }),
        },
        other => other.clone(),
    }
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
        assert_eq!(install_derived_names(&mut atlas), 1);
        assert_eq!(atlas.reference_for("praven"), Some("p_town_6"));
        // The game name compiles like its kit reference and prints back as the game name.
        let text = "party_relocate_near_party(praven, p_main_party, 3)";
        let ops = compile(&atlas, text).expect("the game name is accepted");
        assert!(decompile(&atlas, &ops, Mode::Structured).contains("praven"));
        // And the whole measure still comes back identical with names installed.
        let report = measure(&atlas);
        assert_eq!(report.total.failed, 0);
    }
}
