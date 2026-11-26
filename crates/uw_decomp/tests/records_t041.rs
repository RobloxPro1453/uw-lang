//! T-041: real-atlas round-trip of the dialog, trigger and simple-trigger record schemas.
//!
//! Loads the full atlas made by `py/atlas_dump.py` when it exists on this machine
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md) and skips itself otherwise, like
//! `crates/uw_atlas/tests/real_atlas.rs`.

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

/// Round-trips every record of `table` and requires all of them exact and none failed.
fn roundtrip(table: &str) {
    // The schema is checked even when the atlas is absent, so a broken `.toml` never slips by.
    let schema_path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    let schema = Schema::from_path(&schema_path)
        .unwrap_or_else(|e| panic!("{table} schema loads: {e}"));
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "{table}: {} records, {} exact, {} failed",
        report.records, report.exact, report.failed
    );
    assert!(report.records > 0, "{table}: no records in the atlas");
    assert_eq!(
        report.exact, report.records,
        "{table}: {} of {} records exact; first failures: {:?}",
        report.exact, report.records, report.first_failures
    );
    assert_eq!(report.failed, 0, "{table}: first failures: {:?}", report.first_failures);
}

#[test]
fn real_atlas_dialogs_round_trip() {
    roundtrip("dialogs");
}

#[test]
fn real_atlas_triggers_round_trip() {
    roundtrip("triggers");
}

#[test]
fn real_atlas_simple_triggers_round_trip() {
    roundtrip("simple_triggers");
}
