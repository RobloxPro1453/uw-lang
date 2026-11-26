//! `uw coverage [--all] [--json]`: how much of the Module System UW can write today, proven by a
//! compiled golden test.
//!
//! The inventory comes from the atlas: every record table × {new, like, override, disable, hook},
//! every hook kind (a constant named `ti_*`, `ti_once`/`ti_never` excepted) and every operation
//! name. A cell is silver only when a golden test's `input.uw` has a `// covers: <cell>` line, the
//! test is not an `error_` one and its `expected/` output shows the proof: the table's module file
//! for a table cell, the op/hook name for the others. A tag without that proof is listed and does
//! not count. A filled cell also listed in `docs/coverage-gold.txt` is gold (the in-game
//! proofs).

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_atlas::Atlas;
use uw_core::lower::{CoreData, Property};

const USAGE: &str = "usage:
  uw coverage [--all] [--json] [--atlas <atlas.json>] [--golden <dir>] [--gold <cells.txt>]
      how much of the Module System UW can write today, proven by a golden test: every record
      table x {new, like, override, disable, hook}, every ti_* hook kind and every operation name;
      --json: the same as one object (tables/hooks/ops {filled,total,percent,gold}, overall,
      gold_cells, empty, tags_without_proof);
      --all: list the empty hook and op cells too (the empty table cells are always listed);
      --golden: the golden test folder (default: crates/uw_cli/tests/golden);
      --gold: the cells proved in game, one per line (default: docs/coverage-gold.txt)";

/// The record table actions: one grid cell each.
const ACTIONS: [&str; 5] = ["new", "like", "override", "disable", "hook"];

/// `ti_*` constants that are not hook kinds: timing values of a trigger.
const NOT_HOOKS: [&str; 2] = ["ti_once", "ti_never"];

/// Where the golden tests and the in-game proofs live, from the UW folder.
const GOLDEN_REL: &str = "crates/uw_cli/tests/golden";
const GOLD_REL: &str = "docs/coverage-gold.txt";

/// What a `// covers:` tag has to show in the expected output to count.
enum Cell {
    /// `<table>.<action>`: the expected output holds the table's module file (`module_parties.py`).
    Table { file: String },
    /// `hook.<ti_name>` / `op.<name>`: the expected output mentions the name.
    Named { name: String },
}

/// One sub-grid: how many of its cells a golden test proves, and which are still empty.
struct Grid {
    filled: usize,
    total: usize,
    /// Filled cells that are also in the gold file.
    gold: usize,
    /// The cells no test proves, in grid order.
    empty: Vec<String>,
}

impl Grid {
    fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.filled as f64 / self.total as f64
        }
    }
}

/// A `// covers:` tag its own test does not prove.
struct Unproven {
    test: String,
    cell: String,
    reason: String,
}

/// How far the core language reaches into the kit: the words it can name and the ones that lower.
struct CoreWords {
    /// Kit properties a probe can be written for (not [`CoreWords::skipped`]).
    words: usize,
    /// Distinct Native operation names in the steps of [`CoreData::builtin`].
    reachable: usize,
    /// The atlas operation count.
    total: usize,
    /// Probeable words whose probe lowered without error.
    lowered: usize,
    /// Kit properties with an `{object}` subject other than `agent`: no probe can write them.
    skipped: usize,
    /// Every probeable word that did not lower, with its error message.
    failed: Vec<(String, String)>,
}

impl CoreWords {
    fn percent(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            100.0 * self.reachable as f64 / self.total as f64
        }
    }
}

/// The whole measure: three sub-grids, the weighted number and what did not count.
struct Coverage {
    tables: Grid,
    hooks: Grid,
    ops: Grid,
    /// The kit words of the core language.
    core: CoreWords,
    /// `0.5 × tables + 0.3 × hooks + 0.2 × ops`, in percent.
    overall: f64,
    /// Filled cells that are also in the gold file, in grid order.
    gold_cells: Vec<String>,
    unproven: Vec<Unproven>,
    /// List the empty hook and op cells too.
    all: bool,
    atlas: PathBuf,
    golden: PathBuf,
    gold: PathBuf,
}

/// Every cell of the inventory, in grid order: tables, then hooks, then ops.
struct Inventory {
    tables: Vec<(String, Cell)>,
    hooks: Vec<(String, Cell)>,
    ops: Vec<(String, Cell)>,
}

/// One golden test: its `// covers:` cells and its expected output, file by file.
struct Test {
    name: String,
    /// `error_*` golden: it proves no cell.
    error: bool,
    cells: Vec<String>,
    expected: Vec<(String, String)>,
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut all = false;
    let mut json = false;
    let mut atlas = None;
    let mut golden = None;
    let mut gold = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut path = |flag: &str| it.next().map(PathBuf::from).ok_or(format!("{flag} needs a path"));
        let result = match arg.as_str() {
            "--all" => {
                all = true;
                Ok(())
            }
            "--json" => {
                json = true;
                Ok(())
            }
            "--atlas" => path("--atlas").map(|path| atlas = Some(path)),
            "--golden" => path("--golden").map(|path| golden = Some(path)),
            "--gold" => path("--gold").map(|path| gold = Some(path)),
            _ => Err(format!("unexpected argument '{arg}'\n{USAGE}")),
        };
        if let Err(message) = result {
            return crate::fail(&message);
        }
    }
    let home = crate::uw_home().unwrap_or_else(|| PathBuf::from("."));
    let golden = golden.unwrap_or_else(|| home.join(GOLDEN_REL));
    let gold = gold.unwrap_or_else(|| home.join(GOLD_REL));
    let path = match crate::atlas_path(atlas) {
        Ok(path) => path,
        Err(message) => return crate::fail(&message),
    };
    let atlas = match Atlas::from_path(&path) {
        Ok(atlas) => atlas,
        Err(err) => return crate::fail(&format!("{}: {err}", path.display())),
    };
    let report = match measure(&atlas, &path, &golden, &gold, all) {
        Ok(report) => report,
        Err(message) => return crate::fail(&message),
    };
    if json {
        println!("{}", report.json());
    } else {
        print!("{}", report.text());
    }
    ExitCode::SUCCESS
}

/// Measures every cell of `atlas` against the golden tests in `golden` and the gold list in `gold`.
fn measure(atlas: &Atlas, source: &Path, golden: &Path, gold: &Path, all: bool) -> Result<Coverage, String> {
    let inventory = inventory(atlas);
    let golds = read_gold(gold)?;
    let tests = read_tests(golden)?;
    let mut lookup: Vec<(&str, &Cell)> = Vec::new();
    for grid in [&inventory.tables, &inventory.hooks, &inventory.ops] {
        lookup.extend(grid.iter().map(|(name, cell)| (name.as_str(), cell)));
    }
    let mut filled: BTreeSet<&str> = BTreeSet::new();
    let mut unproven = Vec::new();
    for test in &tests {
        for tag in &test.cells {
            let Some((name, cell)) = lookup.iter().find(|(name, _)| *name == tag.as_str()) else {
                unproven.push(Unproven {
                    test: test.name.clone(),
                    cell: tag.clone(),
                    reason: "not a cell of the atlas inventory".to_owned(),
                });
                continue;
            };
            match cell.proof(test) {
                Ok(()) => {
                    filled.insert(*name);
                }
                Err(reason) => unproven.push(Unproven { test: test.name.clone(), cell: tag.clone(), reason }),
            }
        }
    }
    let grid = |cells: &[(String, Cell)]| Grid {
        filled: cells.iter().filter(|(name, _)| filled.contains(name.as_str())).count(),
        total: cells.len(),
        gold: cells.iter().filter(|(name, _)| filled.contains(name.as_str()) && golds.contains(name)).count(),
        empty: cells
            .iter()
            .filter(|(name, _)| !filled.contains(name.as_str()))
            .map(|(name, _)| name.clone())
            .collect(),
    };
    let tables = grid(&inventory.tables);
    let hooks = grid(&inventory.hooks);
    let ops = grid(&inventory.ops);
    let core = core_words(atlas)?;
    let overall = 0.5 * tables.percent() + 0.3 * hooks.percent() + 0.2 * ops.percent();
    let gold_cells: Vec<String> = inventory
        .tables
        .iter()
        .chain(&inventory.hooks)
        .chain(&inventory.ops)
        .filter(|(name, _)| filled.contains(name.as_str()) && golds.contains(name))
        .map(|(name, _)| name.clone())
        .collect();
    Ok(Coverage {
        tables,
        hooks,
        ops,
        core,
        overall,
        gold_cells,
        unproven,
        all,
        atlas: source.to_path_buf(),
        golden: golden.to_path_buf(),
        gold: gold.to_path_buf(),
    })
}

/// Measures the core language's kit words against `atlas`: the Native names it can use, and how many
/// of its probeable words lower.
fn core_words(atlas: &Atlas) -> Result<CoreWords, String> {
    let data = CoreData::builtin().map_err(|err| err.to_string())?;
    let reachable: BTreeSet<&str> = data
        .properties
        .iter()
        .flat_map(|prop| prop.get.iter().chain(&prop.set).chain(&prop.cond).chain(&prop.run))
        .map(|step| step.op.as_str())
        .collect();
    let mut words = 0;
    let mut lowered = 0;
    let mut skipped = 0;
    let mut failed = Vec::new();
    for prop in data.properties.iter().filter(|prop| prop.kit) {
        let Some(text) = probe(prop) else {
            skipped += 1;
            continue;
        };
        words += 1;
        match lower_probe(&text, atlas, &data) {
            Ok(()) => lowered += 1,
            Err(message) => failed.push((prop.path.clone(), message)),
        }
    }
    Ok(CoreWords { words, reachable: reachable.len(), total: atlas.ops.len(), lowered, skipped, failed })
}

/// The probe program of one kit word, or `None` when its `{object}` is a subject other than `agent`
/// (only the player's agent can be written in a probe).
fn probe(prop: &Property) -> Option<String> {
    let object = prop
        .get
        .iter()
        .chain(&prop.set)
        .chain(&prop.cond)
        .chain(&prop.run)
        .flat_map(|step| step.args.iter())
        .any(|arg| arg == "{object}");
    let head = prop.path.split('(').next().unwrap_or(&prop.path);
    let base = if object {
        let (subject, name) = head.split_once('.')?;
        if subject != "agent" {
            return None;
        }
        format!("player.agent.{name}")
    } else {
        head.to_owned()
    };
    let expr = match prop.path.split_once('(').and_then(|(_, inner)| inner.strip_suffix(')')) {
        Some(inner) => {
            let args: Vec<&str> = inner
                .split(',')
                .map(|part| part.trim().trim_start_matches('{').trim_end_matches('}'))
                .filter(|name| !name.is_empty())
                .map(|name| {
                    if uw_core::lower::kit::param_unit(name) == "text" {
                        "\"uw\""
                    } else {
                        "1"
                    }
                })
                .collect();
            format!("{base}({})", args.join(", "))
        }
        None => base,
    };
    let mut text = String::new();
    if !prop.get.is_empty() {
        let _ = writeln!(text, "var agent.uw_probe = 0");
        let _ = writeln!(text, "while in mission {{\n  player.agent.uw_probe = {expr}\n}}");
    }
    if !prop.set.is_empty() {
        let _ = writeln!(text, "while in mission {{\n  {expr} = 1\n}}");
    }
    if !prop.cond.is_empty() {
        let _ = writeln!(text, "while {expr} {{\n}}");
    }
    if !prop.run.is_empty() {
        let _ = writeln!(text, "while in mission {{\n  {expr}\n}}");
    }
    Some(text)
}

/// Parses and lowers one probe program; `Err` is the first line the compiler refuses.
fn lower_probe(text: &str, atlas: &Atlas, data: &CoreData) -> Result<(), String> {
    let file = uw_core::parse("uw_probe", "features/uw_probe.uw", text)
        .map_err(|errors| errors.into_iter().map(|err| err.to_string()).collect::<Vec<_>>().join("; "))?;
    uw_core::lower::lower(&[file], atlas, data).map(|_| ()).map_err(|errors| {
        errors.into_iter().map(|err| err.to_string()).collect::<Vec<_>>().join("; ")
    })
}

impl Cell {
    /// Proves the cell from `test`'s expected output, or says what is missing.
    fn proof(&self, test: &Test) -> Result<(), String> {
        if test.error {
            return Err("an error_ test proves no cell".to_owned());
        }
        match self {
            Cell::Table { file } => {
                if test.expected.iter().any(|(name, _)| name == file) {
                    Ok(())
                } else {
                    Err(format!("its expected output has no {file}"))
                }
            }
            Cell::Named { name } => {
                if test.expected.iter().any(|(_, text)| mentions(text, name)) {
                    Ok(())
                } else {
                    Err(format!("its expected output never mentions {name}"))
                }
            }
        }
    }
}

/// The cells of the atlas: every table × [`ACTIONS`], every `ti_*` hook kind, every op name.
fn inventory(atlas: &Atlas) -> Inventory {
    let mut tables = Vec::with_capacity(atlas.tables.len() * ACTIONS.len());
    for (name, table) in &atlas.tables {
        for action in ACTIONS {
            let file = format!("{}.py", table.module);
            tables.push((format!("{name}.{action}"), Cell::Table { file }));
        }
    }
    let mut hooks: Vec<(String, Cell)> = atlas
        .constants
        .iter()
        .filter(|constant| constant.name.starts_with("ti_") && !NOT_HOOKS.contains(&constant.name.as_str()))
        .map(|constant| {
            let name = constant.name.clone();
            (format!("hook.{name}"), Cell::Named { name })
        })
        .collect();
    hooks.sort_by(|a, b| a.0.cmp(&b.0));
    hooks.dedup_by(|a, b| a.0 == b.0);
    let mut ops: Vec<(String, Cell)> = atlas
        .ops
        .iter()
        .map(|op| (format!("op.{}", op.name), Cell::Named { name: op.name.clone() }))
        .collect();
    ops.sort_by(|a, b| a.0.cmp(&b.0));
    ops.dedup_by(|a, b| a.0 == b.0);
    Inventory { tables, hooks, ops }
}

impl Coverage {
    /// The cells the output lists as empty: the table grid alone, or all three grids with `--all`.
    fn empty(&self) -> Vec<&str> {
        let mut empty: Vec<&str> = self.tables.empty.iter().map(String::as_str).collect();
        if self.all {
            empty.extend(self.hooks.empty.iter().map(String::as_str));
            empty.extend(self.ops.empty.iter().map(String::as_str));
        }
        empty
    }

    /// Plain output: the three grids, the weighted number, the tags without proof, the empty cells.
    fn text(&self) -> String {
        let mut out = String::from("coverage (silver: a golden test proves the cell; gold: proved in game too)\n");
        for (label, grid) in [("tables", &self.tables), ("hooks", &self.hooks), ("ops", &self.ops)] {
            let _ = writeln!(
                out,
                "  {label:<7} {:>4}/{:<4} {:>5.1}%   gold {}",
                grid.filled,
                grid.total,
                grid.percent(),
                grid.gold
            );
        }
        let _ = writeln!(out, "  overall (0.5 tables, 0.3 hooks, 0.2 ops): {:.1}%", self.overall);
        if !self.unproven.is_empty() {
            let _ = writeln!(out, "tag without proof: {}", self.unproven.len());
            for item in &self.unproven {
                let _ = writeln!(out, "  {}: {} ({})", item.test, item.cell, item.reason);
            }
        }
        let empty = self.empty();
        let label = if self.all { "empty cells" } else { "empty table cells" };
        let _ = writeln!(out, "{label}: {}", empty.len());
        for cell in empty {
            let _ = writeln!(out, "  {cell}");
        }
        let _ = writeln!(
            out,
            "core language: kit words {}, ops reachable {}/{} ({:.1}%), lowered {}/{}, skipped {}",
            self.core.words,
            self.core.reachable,
            self.core.total,
            self.core.percent(),
            self.core.lowered,
            self.core.words,
            self.core.skipped
        );
        if self.all {
            for (word, error) in &self.core.failed {
                let _ = writeln!(out, "  {word}: {error}");
            }
        }
        out
    }

    /// [`Coverage::text`] as one JSON object.
    fn json(&self) -> String {
        let grid = |grid: &Grid| {
            serde_json::json!({
                "filled": grid.filled,
                "total": grid.total,
                "percent": round1(grid.percent()),
                "gold": grid.gold,
            })
        };
        let unproven: Vec<serde_json::Value> = self
            .unproven
            .iter()
            .map(|item| serde_json::json!({"test": item.test, "cell": item.cell, "reason": item.reason}))
            .collect();
        serde_json::json!({
            "atlas": self.atlas.display().to_string(),
            "golden": self.golden.display().to_string(),
            "gold_file": self.gold.display().to_string(),
            "tables": grid(&self.tables),
            "hooks": grid(&self.hooks),
            "ops": grid(&self.ops),
            "overall": round1(self.overall),
            "core": {
                "words": self.core.words,
                "reachable": self.core.reachable,
                "total": self.core.total,
                "percent": round1(self.core.percent()),
                "lowered": self.core.lowered,
                "skipped": self.core.skipped,
            },
            "gold_cells": self.gold_cells,
            "empty": self.empty(),
            "tags_without_proof": unproven,
        })
        .to_string()
    }
}

/// One decimal place: the percentages the output prints (and the JSON holds).
fn round1(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

/// `name` in `text` as a whole word: several op names (`eq`, `ge`, `lt`) would otherwise match
/// inside a longer word and a tag would count without its proof.
fn mentions(text: &str, name: &str) -> bool {
    let hay = text.as_bytes();
    let needle = name.as_bytes();
    if needle.is_empty() || hay.len() < needle.len() {
        return false;
    }
    let word = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
    (0..=hay.len() - needle.len()).any(|start| {
        &hay[start..start + needle.len()] == needle
            && (start == 0 || !word(hay[start - 1]))
            && (start + needle.len() == hay.len() || !word(hay[start + needle.len()]))
    })
}

/// The cells a golden `input.uw` claims, from its `// covers: <cell>` lines.
fn covers(text: &str) -> Vec<String> {
    let mut cells = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.trim().strip_prefix("//") else {
            continue;
        };
        let Some(rest) = rest.trim_start().strip_prefix("covers:") else {
            continue;
        };
        cells.extend(
            rest.split(|c: char| c.is_whitespace() || c == ',')
                .filter(|cell| !cell.is_empty())
                .map(str::to_owned),
        );
    }
    cells
}

/// Every golden test folder under `golden`, sorted by name.
fn read_tests(golden: &Path) -> Result<Vec<Test>, String> {
    let entries = std::fs::read_dir(golden).map_err(|err| format!("{}: {err}", golden.display()))?;
    let mut dirs: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    dirs.sort();
    let mut tests = Vec::new();
    for dir in dirs {
        let input = dir.join("input.uw");
        if !input.is_file() {
            continue;
        }
        let text = std::fs::read_to_string(&input).map_err(|err| format!("{}: {err}", input.display()))?;
        let name = dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
        tests.push(Test {
            error: name.starts_with("error_"),
            cells: covers(&text),
            expected: read_expected(&dir.join("expected"))?,
            name,
        });
    }
    Ok(tests)
}

/// The files of a test's `expected/` folder, sorted: file name and text.
fn read_expected(dir: &Path) -> Result<Vec<(String, String)>, String> {
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let entries = std::fs::read_dir(dir).map_err(|err| format!("{}: {err}", dir.display()))?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_file())
        .collect();
    paths.sort();
    let mut expected = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
        expected.push((name, text));
    }
    Ok(expected)
}

/// The cells proved in game: one per line, `#` lines and blanks skipped. No file means
/// no gold cell (a fresh checkout keeps the silver number).
fn read_gold(path: &Path) -> Result<BTreeSet<String>, String> {
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    let text = std::fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uw_core::lower::Step;

    /// A kit word with the given steps; the tests write the ops and templates by hand.
    fn word(path: &str, get: &[(&str, &[&str])], set: &[(&str, &[&str])], cond: &[(&str, &[&str])]) -> Property {
        let steps = |raw: &[(&str, &[&str])]| -> Vec<Step> {
            raw.iter()
                .map(|(op, args)| Step { op: (*op).to_owned(), args: args.iter().map(|arg| (*arg).to_owned()).collect() })
                .collect()
        };
        Property {
            path: path.to_owned(),
            unit: None,
            get: steps(get),
            set: steps(set),
            cond: steps(cond),
            run: Vec::new(),
            aliases: Default::default(),
            vector: false,
            kit: true,
        }
    }

    #[test]
    fn the_probe_of_a_getter_a_setter_and_a_condition() {
        let getter = word("agent.hit_points", &[("store_agent_hit_points", &["{dest}", "{object}"])], &[], &[]);
        assert_eq!(
            probe(&getter).expect("an agent getter is probeable"),
            "var agent.uw_probe = 0\nwhile in mission {\n  player.agent.uw_probe = player.agent.hit_points\n}\n"
        );

        let setter = word("agent.speed_modifier", &[], &[("agent_set_speed_modifier", &["{object}", "{value}"])], &[]);
        assert_eq!(
            probe(&setter).expect("an agent setter is probeable"),
            "while in mission {\n  player.agent.speed_modifier = 1\n}\n"
        );

        let condition = word("agent.is_alive", &[], &[], &[("agent_is_alive", &["{object}"])]);
        assert_eq!(probe(&condition).expect("an agent condition is probeable"), "while player.agent.is_alive {\n}\n");

        // No `{object}`: the path stands for itself, whatever its subject.
        let global = word("mission.timer_c_msec", &[("store_mission_timer_c_msec", &["{dest}"])], &[], &[]);
        assert_eq!(
            probe(&global).expect("a subjectless getter is probeable"),
            "var agent.uw_probe = 0\nwhile in mission {\n  player.agent.uw_probe = mission.timer_c_msec\n}\n"
        );

        // Another subject with an `{object}` cannot be written in a probe.
        let other = word("troop.rank", &[("store_troop_rank", &["{dest}", "{object}"])], &[], &[]);
        assert_eq!(probe(&other), None);
    }

    #[test]
    fn a_short_op_name_only_counts_as_a_whole_word() {
        assert!(mentions("(eq, \"$x\", 1)", "eq"));
        assert!(!mentions("(sequence, \"$x\")", "eq"));
        assert!(mentions("(troop_add_gold, trp_player, 10)", "troop_add_gold"));
        assert!(!mentions("(troop_add_gold, trp_player, 10)", "add"));
    }

    #[test]
    fn covers_reads_the_tag_lines_only() {
        let text = "mod x\n// covers: menus.new, op.assign\n// covers: hook.ti_once\n// not a tag\n";
        assert_eq!(covers(text), ["menus.new", "op.assign", "hook.ti_once"]);
    }
}
