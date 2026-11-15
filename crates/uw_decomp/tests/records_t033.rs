//! T-033: the `parties` and `party_templates` schemas print every record of the real atlas as UW
//! record text and parse it back identical. The atlas is machine-local
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md); the round-trip tests skip themselves
//! when it is absent.

use uw_atlas::Atlas;
use uw_decomp::records::{roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

fn schema(table: &str) -> Schema {
    let path = format!("{SCHEMA_DIR}/{table}.toml");
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn field_names(schema: &Schema) -> Vec<&str> {
    schema.fields.iter().map(|f| f.name.as_str()).collect()
}

/// Holds even when no atlas is on the machine, so the field order cannot silently drift.
#[test]
fn schemas_follow_the_kit_layout_comments() {
    let parties = schema("parties");
    assert_eq!(
        (parties.table.as_str(), parties.kind.as_str(), parties.id_prefix.as_str()),
        ("parties", "party", "p_")
    );
    assert_eq!(
        field_names(&parties),
        [
            "id",
            "name",
            "party_flags",
            "menu",
            "party_template",
            "faction",
            "personality",
            "ai_behavior",
            "ai_target",
            "initial_coordinates",
            "stacks",
            "direction",
        ]
    );
    assert!(parties.fields.iter().all(|f| f.ty == "raw" && f.source.starts_with("module_parties.py:")));
    // 12) Party direction in degrees [optional].
    let optional: Vec<&str> =
        parties.fields.iter().filter(|f| f.optional).map(|f| f.name.as_str()).collect();
    assert_eq!(optional, ["direction"]);

    let templates = schema("party_templates");
    assert_eq!(
        (templates.table.as_str(), templates.kind.as_str(), templates.id_prefix.as_str()),
        ("party_templates", "party_template", "pt_")
    );
    assert_eq!(
        field_names(&templates),
        ["id", "name", "party_flags", "menu", "faction", "personality", "stacks"]
    );
    assert!(templates.fields.iter().all(|f| f.ty == "raw"
        && !f.optional
        && f.source.starts_with("module_party_templates.py:")));
}

#[test]
fn real_atlas_parties_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema("parties"));
    eprintln!("parties: {} records, {} exact, {} failed", report.records, report.exact, report.failed);
    assert_eq!(report.records, 240, "the atlas has 240 parties");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}

#[test]
fn real_atlas_party_templates_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let report = roundtrip_table(&atlas, &schema("party_templates"));
    eprintln!(
        "party_templates: {} records, {} exact, {} failed",
        report.records, report.exact, report.failed
    );
    assert_eq!(report.records, 63, "the atlas has 63 party_templates");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
}
