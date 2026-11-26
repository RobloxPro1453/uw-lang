//! `uw corpus <mods folder>`: inventory of real Module System mods (ROADMAP M1).
//!
//! Reads every mod's Python sources as text (nothing is run) and counts, per mod and in total:
//! which kit operations are used and how often, which `ti_*` trigger kinds appear, which
//! engine-called scripts (atlas hooks) are defined, and how many scripts/presentations the mod
//! defines that Native does not have. Only names known to the atlas are counted, so nothing is
//! guessed. Output is statistics only (no mod code): `ops.json`, `ti.json`, `summary.json`.
//!
//! The scan is textual: an operation use is `(name,` or `(neg|name,` after comments are removed.
//! Stale copies inside a mod folder are counted too; the numbers are a usage profile, not a
//! compiled truth.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde::Serialize;

#[derive(Default, Serialize)]
struct ModSummary {
    py_files: usize,
    lines: usize,
    op_uses: usize,
    distinct_ops: usize,
    ti_kinds: usize,
    hook_scripts: Vec<String>,
    scripts_defined: usize,
    scripts_not_in_native: usize,
    presentations_defined: usize,
    presentations_not_in_native: usize,
}

#[derive(Serialize)]
struct Usage {
    name: String,
    uses: usize,
    mods: usize,
    per_mod: BTreeMap<String, usize>,
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut dir = None;
    let mut atlas = None;
    let mut out = PathBuf::from("kit-meta/corpus");
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--atlas" => match it.next() {
                Some(path) => atlas = Some(PathBuf::from(path)),
                None => return crate::fail("--atlas needs a path"),
            },
            "--out" => match it.next() {
                Some(path) => out = PathBuf::from(path),
                None => return crate::fail("--out needs a folder"),
            },
            other if dir.is_none() && !other.starts_with("--") => dir = Some(PathBuf::from(other)),
            other => return crate::fail(&format!("corpus: unexpected argument {other}")),
        }
    }
    let Some(dir) = dir else {
        return crate::fail("usage: uw corpus <mods folder> [--atlas <atlas.json>] [--out <dir>]");
    };
    let path = match crate::atlas_path(atlas) {
        Ok(path) => path,
        Err(message) => return crate::fail(&message),
    };
    let atlas = match uw_atlas::Atlas::from_path(&path) {
        Ok(atlas) => atlas,
        Err(err) => return crate::fail(&format!("{}: {err}", path.display())),
    };
    match run(&dir, &atlas, &out) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(err) => crate::fail(&err),
    }
}

/// The scan itself, separate from argument handling so tests can call it on a fixture.
pub fn run(dir: &Path, atlas: &uw_atlas::Atlas, out: &Path) -> Result<String, String> {
    let op_names: BTreeSet<&str> = atlas.ops.iter().map(|op| op.name.as_str()).collect();
    let ti_names: BTreeSet<&str> =
        atlas.constants.iter().map(|c| c.name.as_str()).filter(|n| n.starts_with("ti_")).collect();
    let hooks: BTreeSet<&str> = atlas.hooks.iter().map(String::as_str).collect();
    let native_names = |table: &str| -> BTreeSet<String> {
        atlas
            .tables
            .get(table)
            .map(|t| t.entries.iter().filter_map(|e| e.name.clone()).collect())
            .unwrap_or_default()
    };
    let native_scripts = native_names("scripts");
    let native_presentations = native_names("presentations");

    let mut mods: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    mods.sort();
    if mods.is_empty() {
        return Err(format!("{}: no mod folders", dir.display()));
    }

    let mut op_use: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut ti_use: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut summary: BTreeMap<String, ModSummary> = BTreeMap::new();

    for mod_dir in &mods {
        let mod_name = mod_dir.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let mut files = Vec::new();
        collect_py(mod_dir, &mut files);
        files.sort();
        let mut s = ModSummary::default();
        let mut ops_here: BTreeSet<String> = BTreeSet::new();
        let mut ti_here: BTreeSet<String> = BTreeSet::new();
        let mut hooks_here: BTreeSet<String> = BTreeSet::new();
        let mut scripts: BTreeSet<String> = BTreeSet::new();
        let mut presentations: BTreeSet<String> = BTreeSet::new();
        for file in &files {
            let Ok(bytes) = std::fs::read(file) else { continue };
            let text = String::from_utf8_lossy(&bytes);
            s.py_files += 1;
            let stem = file.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            let is_scripts = stem.contains("scripts");
            let is_presentations = stem.contains("presentations");
            for raw in text.lines() {
                s.lines += 1;
                let line = strip_comment(raw);
                for name in op_calls(line) {
                    if op_names.contains(name) {
                        *op_use.entry(name.to_owned()).or_default().entry(mod_name.clone()).or_default() += 1;
                        ops_here.insert(name.to_owned());
                        s.op_uses += 1;
                    }
                }
                for name in idents(line).filter(|w| w.starts_with("ti_")) {
                    if ti_names.contains(name) {
                        *ti_use.entry(name.to_owned()).or_default().entry(mod_name.clone()).or_default() += 1;
                        ti_here.insert(name.to_owned());
                    }
                }
                if let Some(id) = entry_id(line) {
                    if is_scripts {
                        if hooks.contains(id) {
                            hooks_here.insert(id.to_owned());
                        }
                        scripts.insert(id.to_owned());
                    } else if is_presentations {
                        presentations.insert(id.to_owned());
                    }
                }
            }
        }
        s.distinct_ops = ops_here.len();
        s.ti_kinds = ti_here.len();
        s.hook_scripts = hooks_here.into_iter().collect();
        s.scripts_defined = scripts.len();
        s.scripts_not_in_native = scripts.iter().filter(|n| !native_scripts.contains(*n)).count();
        s.presentations_defined = presentations.len();
        s.presentations_not_in_native =
            presentations.iter().filter(|n| !native_presentations.contains(*n)).count();
        summary.insert(mod_name, s);
    }

    let usage = |names: &BTreeSet<&str>, table: &BTreeMap<String, BTreeMap<String, usize>>| -> Vec<Usage> {
        let mut list: Vec<Usage> = names
            .iter()
            .map(|n| {
                let per_mod = table.get(*n).cloned().unwrap_or_default();
                Usage { name: (*n).to_owned(), uses: per_mod.values().sum(), mods: per_mod.len(), per_mod }
            })
            .collect();
        list.sort_by(|a, b| b.mods.cmp(&a.mods).then(b.uses.cmp(&a.uses)).then(a.name.cmp(&b.name)));
        list
    };
    let ops = usage(&op_names, &op_use);
    let tis = usage(&ti_names, &ti_use);

    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    write_json(out, "summary.json", &summary)?;
    write_json(out, "ops.json", &ops)?;
    write_json(out, "ti.json", &tis)?;

    let used = ops.iter().filter(|u| u.uses > 0).count();
    let in_half = ops.iter().filter(|u| u.mods * 2 >= mods.len()).count();
    Ok(format!(
        "corpus: {} mods, {} kit operations: {} used at least once, {} used by at least half of the mods, {} never; {} ti_* kinds used -> {}",
        mods.len(),
        ops.len(),
        used,
        in_half,
        ops.len() - used,
        tis.iter().filter(|u| u.uses > 0).count(),
        out.display()
    ))
}

fn write_json<T: Serialize>(out: &Path, name: &str, value: &T) -> Result<(), String> {
    let path = out.join(name);
    let text = serde_json::to_string_pretty(value).expect("corpus statistics serialize");
    std::fs::write(&path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

/// Python sources that USE operations: kit headers, process/compile scripts and generated id
/// files only define names, so they are skipped.
fn collect_py(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_py(&path, out);
            continue;
        }
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        let skip = ["header_", "process_", "id_", "compile"].iter().any(|p| name.starts_with(p));
        if name.ends_with(".py") && !skip {
            out.push(path);
        }
    }
}

/// The line without its `#` comment (a `#` inside a string literal is kept).
fn strip_comment(line: &str) -> &str {
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for (i, c) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match (quote, c) {
            (Some(_), '\\') => escaped = true,
            (Some(q), c) if c == q => quote = None,
            (None, '"') | (None, '\'') => quote = Some(c),
            (None, '#') => return &line[..i],
            _ => {}
        }
    }
    line
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn idents(line: &str) -> impl Iterator<Item = &str> {
    line.split(|c: char| !is_ident(c)).filter(|w| !w.is_empty())
}

/// Every `(name,` / `(neg|this_or_next|name,` in the line: the operation of a statement tuple.
fn op_calls(line: &str) -> Vec<&str> {
    let mut found = Vec::new();
    for (i, _) in line.match_indices('(') {
        let mut rest = &line[i + 1..];
        loop {
            rest = rest.trim_start();
            let end = rest.find(|c: char| !is_ident(c)).unwrap_or(rest.len());
            if end == 0 {
                break;
            }
            let word = &rest[..end];
            let after = rest[end..].trim_start();
            if after.starts_with('|') {
                rest = &after[1..];
                continue;
            }
            if after.starts_with(',') || after.starts_with(')') {
                found.push(word);
            }
            break;
        }
    }
    found
}

/// The id of a definition line such as `("my_script",` (scripts, presentations).
fn entry_id(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix('(')?.trim_start();
    let rest = rest.strip_prefix('"').or_else(|| rest.strip_prefix('\''))?;
    let end = rest.find(|c: char| c == '"' || c == '\'')?;
    let id = &rest[..end];
    let tail = rest[end + 1..].trim_start();
    (tail.starts_with(',') && !id.is_empty() && id.chars().all(is_ident)).then_some(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_operations_with_and_without_prefixes() {
        let line = r#"  (neg|troop_slot_eq, ":t", 5, 1), (this_or_next|eq, 1, 2), (val_add, ":x", 1),"#;
        assert_eq!(op_calls(line), vec!["troop_slot_eq", "eq", "val_add"]);
    }

    #[test]
    fn comments_are_dropped_but_hash_in_string_is_kept() {
        assert_eq!(strip_comment(r#"(display_message, "@#1 win"), # (val_add, x, 1)"#), r#"(display_message, "@#1 win"), "#);
    }

    /// Two fixture mods against the golden atlas: counts per op and per mod, Native vs new
    /// scripts, the engine hook, and that header files and comments are not counted.
    #[test]
    fn run_counts_two_mods() {
        let atlas = uw_atlas::Atlas::from_str(include_str!("../tests/fixtures/atlas_golden.json")).expect("fixture atlas");
        let root = std::env::temp_dir().join(format!("uw_corpus_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (name, body) in [
            ("a", "scripts = [\n (\"game_start\", [\n  (display_message, \"@hi\"),\n  (neg|party_is_active, 1),\n ]),\n (\"a_own\", [ (display_message, \"@x\"), ]),\n]\n# (troop_add_gold, 1, 2),\n"),
            ("b", "scripts = [\n (\"b_own\", [ (display_message, \"@y\"), (assign, \":x\", 1), ]),\n]\n"),
        ] {
            let dir = root.join("mods").join(name);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("module_scripts.py"), body).unwrap();
            std::fs::write(dir.join("header_operations.py"), "x = (display_message, 1)\n").unwrap();
        }
        let out = root.join("out");
        run(&root.join("mods"), &atlas, &out).expect("corpus runs");
        let ops: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.join("ops.json")).unwrap()).unwrap();
        let get = |n: &str| ops.as_array().unwrap().iter().find(|o| o["name"] == n).cloned().unwrap();
        assert_eq!(get("display_message")["uses"], 3, "header file must not count");
        assert_eq!(get("display_message")["mods"], 2);
        assert_eq!(get("party_is_active")["per_mod"]["a"], 1, "neg| prefix is seen");
        assert_eq!(get("troop_add_gold")["uses"], 0, "commented-out code must not count");
        assert_eq!(ops[0]["name"], "display_message", "most widely used first");
        let summary: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.join("summary.json")).unwrap()).unwrap();
        assert_eq!(summary["a"]["scripts_defined"], 2);
        assert_eq!(summary["a"]["scripts_not_in_native"], 1, "game_start is Native");
        assert_eq!(summary["a"]["hook_scripts"][0], "game_start");
        assert_eq!(summary["b"]["scripts_not_in_native"], 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn definition_ids() {
        assert_eq!(entry_id(r#"  ("game_start", ["#), Some("game_start"));
        assert_eq!(entry_id(r#"  (troop_get_slot, ":a","#), None);
        assert_eq!(entry_id(r#"  ("has space", ["#), None);
    }
}
