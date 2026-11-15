//! T-039: the animations table must print and parse back exactly on the real atlas
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md). Skips itself when the atlas is absent.

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/animations.toml");

#[test]
fn real_atlas_animations_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = Schema::from_path(SCHEMA).expect("animations schema loads");
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!(report.records, 670);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(report.exact, report.records);
}
