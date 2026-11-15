//! Real-atlas record round-trip for the T-037 tables (scene_props, particle_systems). Skips itself
//! when `target/uw-staging/atlas.json` is absent (docs/DEVELOPMENT.md).

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn check(table: &str, records: usize) {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    let schema = Schema::from_path(&path).unwrap_or_else(|e| panic!("{table} schema: {e}"));
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema);
    println!(
        "{table}: {} records, {} exact, {} failed",
        report.records, report.exact, report.failed
    );
    assert_eq!(report.records, records, "{table}");
    assert_eq!(report.exact, report.records, "{table}: {:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
}

#[test]
fn real_atlas_scene_props_round_trip() {
    check("scene_props", 1046);
}

#[test]
fn real_atlas_particle_systems_round_trip() {
    check("particle_systems", 60);
}
