//! T-042: every record of scripts, mission_templates, presentations and tableau_materials prints
//! as UW record text and parses back identical (round 1, raw). Tests skip when the real atlas is
//! absent, like `records.rs::real_atlas_factions_round_trip`.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

fn atlas_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json")
}

fn schema_path(table: &str) -> String {
    format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"))
}

/// Loads the real atlas and the table's schema; `None` when the atlas is absent.
fn load(table: &str) -> Option<(Atlas, Schema)> {
    let path = atlas_path();
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return None;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    let schema = Schema::from_path(schema_path(table)).expect("schema loads");
    Some((atlas, schema))
}

#[test]
fn real_atlas_scripts_round_trip() {
    let Some((atlas, schema)) = load("scripts") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("scripts: {} records", report.records);
    assert_eq!(report.exact, report.records);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}

#[test]
fn real_atlas_mission_templates_round_trip() {
    let Some((atlas, schema)) = load("mission_templates") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("mission_templates: {} records", report.records);
    assert_eq!(report.exact, report.records);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}

#[test]
fn real_atlas_presentations_round_trip() {
    let Some((atlas, schema)) = load("presentations") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("presentations: {} records", report.records);
    assert_eq!(report.exact, report.records);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}

#[test]
fn real_atlas_tableau_materials_round_trip() {
    let Some((atlas, schema)) = load("tableau_materials") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!("tableau_materials: {} records", report.records);
    assert_eq!(report.exact, report.records);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}
