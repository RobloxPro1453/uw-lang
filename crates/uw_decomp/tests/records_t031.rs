//! Real-atlas round-trip of the troop records (T-031): every troop prints as UW record text and
//! parses back to the identical atlas value.

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/troops.toml");

#[test]
fn troops_schema_follows_the_kit_layout_comment() {
    let schema = Schema::from_path(SCHEMA).expect("troops schema loads");
    assert_eq!(
        (schema.table.as_str(), schema.kind.as_str(), schema.id_prefix.as_str()),
        ("troops", "troop", "trp_")
    );
    let names: Vec<&str> = schema.fields.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "id",
            "name",
            "plural_name",
            "troop_flags",
            "scene",
            "reserved",
            "faction",
            "inventory",
            "attributes",
            "weapon_proficiencies",
            "skills",
            "face_code",
            "face_code_2",
            "troop_image"
        ]
    );
    assert!(schema.fields.iter().all(|f| f.ty == "raw" && f.source.starts_with("module_troops.py:")));
    assert!(schema.fields[12].optional && schema.fields[13].optional);
}

#[test]
fn real_atlas_troops_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = Schema::from_path(SCHEMA).expect("troops schema loads");
    let report = roundtrip_table(&atlas, &schema);
    // ID_troops.py has one id per troop in list order (ids 0..1071).
    assert_eq!(report.records, 1072);
    assert_eq!(report.exact, report.records);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}
