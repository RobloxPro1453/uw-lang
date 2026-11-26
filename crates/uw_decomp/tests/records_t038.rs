//! T-038: meshes, sounds and music round-trip on the real atlas (docs/tasks/T-038.md).
//! Each test skips when `target/uw-staging/atlas.json` is absent, like the factions test.

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

/// Prints and parses every record of `table` and checks it is exact.
fn check(table: &str, expected_records: usize) {
    // Load the schema before the atlas check so a broken TOML fails even when the atlas is
    // absent (other lanes load every `kit-meta/records/*.toml`).
    let schema_path =
        format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    let schema = Schema::from_path(&schema_path).expect("schema loads");
    let atlas_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !std::path::Path::new(atlas_path).exists() {
        eprintln!("skipped: no atlas at {atlas_path}");
        return;
    }
    let atlas = Atlas::from_path(atlas_path).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!(report.records, expected_records, "{table}");
    assert_eq!(report.exact, report.records, "{table}: {:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
}

#[test]
fn real_atlas_meshes_round_trip() {
    check("meshes", 555);
}

#[test]
fn real_atlas_sounds_round_trip() {
    check("sounds", 200);
}

#[test]
fn real_atlas_music_round_trip() {
    check("music", 74);
}
