//! Real-atlas round-trip checks for the T-040 record schemas (strings, menus).
//!
//! Each test skips itself when the atlas is not staged in this worktree (the lane has no kit build);
//! the schema still loads first, so a broken `kit-meta/records/<table>.toml` fails here too.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(path).unwrap_or_else(|e| panic!("{table} schema loads: {e}"))
}

fn atlas() -> Option<Atlas> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return None;
    }
    Some(Atlas::from_path(path).expect("full atlas loads"))
}

#[test]
fn real_atlas_strings_round_trip() {
    let schema = schema("strings");
    let Some(atlas) = atlas() else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("strings: {} records, {} exact, {} failed", report.records, report.exact, report.failed);
    assert!(report.records > 0, "the atlas holds no string records");
    assert_eq!(report.exact, report.records, "first failures: {:?}", report.first_failures);
    assert_eq!(report.failed, 0, "first failures: {:?}", report.first_failures);
}

#[test]
fn real_atlas_menus_round_trip() {
    let schema = schema("menus");
    let Some(atlas) = atlas() else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("menus: {} records, {} exact, {} failed", report.records, report.exact, report.failed);
    assert!(report.records > 0, "the atlas holds no menu records");
    assert_eq!(report.exact, report.records, "first failures: {:?}", report.first_failures);
    assert_eq!(report.failed, 0, "first failures: {:?}", report.first_failures);
}
