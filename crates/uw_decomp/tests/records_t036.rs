//! Real-atlas round-trip of the T-036 record schemas: scenes, skins, postfx_params
//! (`docs/tasks/T-036.md`). Every record must print as UW record text and parse back identical.
//!
//! The full atlas made by `py/atlas_dump.py` is not in git; the tests skip themselves when it is
//! missing (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md).

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{table} schema loads: {e}"))
}

/// Loads the table's schema and, when the atlas is on this machine, requires every record to be
/// exact: nothing missing, nothing dropped, no value changed.
fn roundtrip(table: &str, kind: &str, id_prefix: &str, records: usize) {
    let schema = schema(table);
    assert_eq!(
        (schema.table.as_str(), schema.kind.as_str(), schema.id_prefix.as_str()),
        (table, kind, id_prefix)
    );
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!(report.records, records, "`{table}` record count");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}

#[test]
fn real_atlas_scenes_round_trip() {
    roundtrip("scenes", "scene", "scn_", 554);
}

#[test]
fn real_atlas_skins_round_trip() {
    roundtrip("skins", "skin", "", 2);
}

#[test]
fn real_atlas_postfx_params_round_trip() {
    roundtrip("postfx_params", "postfx_param", "pfx_", 9);
}
