//! T-121: every non-error golden mod says which Module System cells it proves, one `// covers:` line
//! per cell at the end of its `input.uw`. A tag is `table.action` (`action` is new, like, override,
//! disable or hook), `hook.<name>` or `op.<name>`.
//!
//! The tags must sit after the code: generated lines carry `# input.uw:<line>`, so a line added at
//! the top would shift every origin comment in `expected/`.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
/// The record schemas: one `<table>.toml` per table the `table.action` tags may name.
const RECORDS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

fn cases() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(GOLDEN)
        .expect("tests/golden exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    dirs.sort();
    assert!(!dirs.is_empty(), "no golden cases in {GOLDEN}");
    dirs
}

/// The table names the kit defines, taken from `kit-meta/records/<table>.toml`.
fn tables() -> BTreeSet<String> {
    fs::read_dir(RECORDS)
        .unwrap_or_else(|e| panic!("{RECORDS}: {e}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect()
}

/// An op, hook or ti name: `_`/letters first, then `_`/letters/digits.
fn is_name(name: &str) -> bool {
    !name.is_empty()
        && name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Whether `cell` has one of the three shapes: `table.action`, `hook.<name>`, `op.<name>`.
fn valid_cell(cell: &str, tables: &BTreeSet<String>) -> bool {
    if let Some(name) = cell.strip_prefix("hook.") {
        return is_name(name);
    }
    if let Some(name) = cell.strip_prefix("op.") {
        return is_name(name);
    }
    let Some((table, action)) = cell.rsplit_once('.') else { return false };
    tables.contains(table) && matches!(action, "new" | "like" | "override" | "disable" | "hook")
}

#[test]
fn every_non_error_golden_case_tags_the_cells_it_proves() {
    let tables = tables();
    let mut failures = Vec::new();
    let mut tagged = 0;
    for dir in cases() {
        let case = dir.file_name().unwrap().to_string_lossy().into_owned();
        if dir.join("expected").join("error.txt").is_file() {
            continue;
        }
        let input = fs::read_to_string(dir.join("input.uw"))
            .unwrap_or_else(|e| panic!("{case}/input.uw: {e}"));
        let mut count = 0;
        for line in input.lines() {
            let Some(rest) = line.trim().strip_prefix("// covers:") else { continue };
            for cell in rest.split(',') {
                let cell = cell.trim();
                if !valid_cell(cell, &tables) {
                    failures.push(format!("{case}: `{cell}` is not a known cover cell"));
                } else {
                    count += 1;
                }
            }
        }
        if count == 0 {
            failures.push(format!("{case}: no `// covers:` line"));
        }
        tagged += 1;
    }
    assert!(tagged > 0, "no non-error golden cases in {GOLDEN}");
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
