//! T-035: every record of the map_icons, quests and info_pages tables prints as UW record text and
//! parses back identical on the real atlas (docs/meclis/YON2-KARAR.md §2).

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

/// Round-trips the whole table; skips when the staged atlas is not there.
fn check(table: &str, records: usize) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !std::path::Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    let schema_path = format!(
        "{}/../../kit-meta/records/{table}.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let schema = Schema::from_path(&schema_path).expect("schema loads");
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!(report.records, records, "{table}: atlas record count");
    assert_eq!(report.exact, report.records, "{table}: {:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
}

#[test]
fn real_atlas_map_icons_round_trip() {
    check("map_icons", 189);
}

#[test]
fn real_atlas_quests_round_trip() {
    check("quests", 91);
}

#[test]
fn real_atlas_info_pages_round_trip() {
    check("info_pages", 6);
}
