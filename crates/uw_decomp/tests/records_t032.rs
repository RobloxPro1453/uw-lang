//! T-032: the items and skills schemas must print every record of the real atlas as UW record text
//! and parse it back identical (`target/uw-staging/atlas.json`, made by `py/atlas_dump.py`). Skips
//! itself on a machine without that atlas, like the other real-atlas tests.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

/// The full atlas; `None` (with a note) when this machine has none.
fn atlas() -> Option<Atlas> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return None;
    }
    Some(Atlas::from_path(path).expect("full atlas loads"))
}

/// One `kit-meta/records/<table>.toml`.
fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// Fails when a field is missing, an extra value is dropped or a value changes: both show up as
/// `exact < records` or `failed > 0`.
fn assert_all_exact(table: &str, atlas: &Atlas) {
    let report = roundtrip_table(atlas, &schema(table));
    assert!(report.records > 0, "no `{table}` records in the atlas: {report:?}");
    assert_eq!(
        report.exact, report.records,
        "{table}: {} of {} records exact: {:?}",
        report.exact, report.records, report.first_failures
    );
    assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
}

#[test]
fn real_atlas_items_round_trip() {
    let Some(atlas) = atlas() else { return };
    assert_all_exact("items", &atlas);
}

#[test]
fn real_atlas_skills_round_trip() {
    let Some(atlas) = atlas() else { return };
    assert_all_exact("skills", &atlas);
}
