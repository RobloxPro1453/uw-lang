//! Runs the real `uw` binary on a small atlas fixture with a record schema folder.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_records.json");

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-roundtrip-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

/// Writes a `factions` schema with exactly these fields (every one required).
fn write_schema(dir: &Path, fields: &[&str]) {
    std::fs::create_dir_all(dir).expect("schema folder");
    let mut text = String::from("table = \"factions\"\nkind = \"faction\"\nid_prefix = \"fac_\"\n");
    for name in fields {
        text.push_str(&format!(
            "\n[[fields]]\nname = \"{name}\"\nty = \"raw\"\nsource = \"module_factions.py:7\"\n"
        ));
    }
    std::fs::write(dir.join("factions.toml"), text).expect("schema file");
}

/// The words of one summary line, with padding and the record separators dropped.
fn words<'a>(text: &'a str, table: &str) -> Vec<&'a str> {
    text.lines()
        .find(|line| line.starts_with(table))
        .unwrap_or_else(|| panic!("no `{table}` line in:\n{text}"))
        .split_whitespace()
        .collect()
}

#[test]
fn measures_records_of_every_table_with_a_schema() {
    let root = scratch("ok");
    let schemas = root.join("schemas");
    write_schema(&schemas, &["signed", "ratio", "title", "members"]);
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("roundtrip")
        .args(["--atlas", FIXTURE])
        .arg("--schemas")
        .arg(&schemas)
        .arg("--out")
        .arg(root.join("out"))
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(out.status.success(), "{stdout}\n{stderr}");

    // The records block after the ops summary: the schema table with its counts, ...
    let records = stdout.split("\nrecords\n").nth(1).expect("a records block");
    assert_eq!(words(records, "factions"), ["factions", "2", "records,", "exact", "2,", "failed", "0"]);
    // ... the tables without a schema, ...
    assert!(
        records.lines().any(|line| line.starts_with("parties") && line.contains("no schema")),
        "{records}"
    );
    // ... and its own TOTAL line.
    assert_eq!(words(records, "TOTAL"), ["TOTAL", "2", "records,", "exact", "2,", "failed", "0"]);

    // Every record's text is written, one file per schema table.
    let text = std::fs::read_to_string(root.join("out/records/factions.uw")).expect("records file");
    assert!(text.contains("faction no_faction {"), "{text}");
    assert!(text.contains("  signed -7"), "{text}");
    assert!(text.contains("  members [1, 2]"), "{text}");
    assert!(text.contains("faction forest_bandits {"), "{text}");
    assert!(!root.join("out/records/parties.uw").exists());

    std::fs::remove_dir_all(&root).ok();
}

/// The real Native atlas, when this machine has one (docs/DEVELOPMENT.md > Atlas).
const REAL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
/// The record schemas of the real kit.
const REAL_SCHEMAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

#[test]
fn real_atlas_menus_and_scripts_records_show_their_blocks() {
    if !Path::new(REAL).exists() {
        eprintln!("skipped: no atlas at {REAL}");
        return;
    }
    let root = scratch("real");
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("roundtrip")
        .args(["--atlas", REAL])
        .arg("--schemas")
        .arg(REAL_SCHEMAS)
        .arg("--out")
        .arg(root.join("out"))
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(out.status.success(), "{stdout}\n{stderr}");

    // A block field printed raw would change the record value and fail those counts.
    let ops = stdout.split("\nrecords\n").next().expect("an ops summary");
    assert_eq!(
        &words(ops, "TOTAL")[..5],
        &["TOTAL", "9113", "blocks", "structured", "9113"],
        "{ops}"
    );
    let records = stdout.split("\nrecords\n").nth(1).expect("a records block");
    assert_eq!(words(records, "menus"), ["menus", "217", "records,", "exact", "217,", "failed", "0"]);
    assert_eq!(words(records, "scripts"), ["scripts", "610", "records,", "exact", "610,", "failed", "0"]);
    assert_eq!(words(records, "TOTAL"), ["TOTAL", "12815", "records,", "exact", "12815,", "failed", "0"]);

    // The written text shows the decompiler's code, not the raw numbers.
    let menus = std::fs::read_to_string(root.join("out/records/menus.uw")).expect("menus records");
    assert!(menus.contains("ops {"), "{menus}");
    let scripts = std::fs::read_to_string(root.join("out/records/scripts.uw")).expect("scripts records");
    assert!(scripts.contains("ops {"), "{scripts}");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_record_that_does_not_round_trip_fails_the_run() {
    let root = scratch("failed");
    let schemas = root.join("schemas");
    // One field past what the fixture's records have: both records print but cannot be read back.
    write_schema(&schemas, &["signed", "ratio", "title", "members", "colour"]);
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("roundtrip")
        .args(["--atlas", FIXTURE])
        .arg("--schemas")
        .arg(&schemas)
        .arg("--out")
        .arg(root.join("out"))
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    assert!(!out.status.success(), "{stdout}");
    let records = stdout.split("\nrecords\n").nth(1).expect("a records block");
    assert_eq!(words(records, "factions"), ["factions", "2", "records,", "exact", "0,", "failed", "2"]);
    assert_eq!(words(records, "TOTAL"), ["TOTAL", "2", "records,", "exact", "0,", "failed", "2"]);
    assert!(records.contains("record failures:"), "{records}");
    assert!(records.contains("missing required field `colour`"), "{records}");

    std::fs::remove_dir_all(&root).ok();
}
