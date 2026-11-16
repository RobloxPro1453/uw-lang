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
            "troop_image",
            "upgrade_1",
            "upgrade_2"
        ]
    );
    let ty = |name: &str| {
        schema.fields.iter().find(|field| field.name == name).map(|field| field.ty.as_str())
    };
    assert_eq!(ty("faction"), Some("ref:factions"));
    assert_eq!(ty("attributes"), Some("bits"));
    assert_eq!(ty("upgrade_1"), Some("ref:troops"));
    assert_eq!(ty("upgrade_2"), Some("ref:troops"));
    // Since P5 (T-059) more troop fields are typed; every field still cites a kit file.
    assert!(schema.fields.iter().all(|field| field.source.contains(".py:")));
    let attributes = schema.fields.iter().find(|field| field.name == "attributes").expect("attributes");
    let parts: Vec<(&str, u32, u32)> =
        attributes.parts.iter().map(|part| (part.name.as_str(), part.shift, part.width)).collect();
    assert_eq!(
        parts,
        [
            ("strength", 0, 8),
            ("agility", 8, 8),
            ("intelligence", 16, 8),
            ("charisma", 24, 8),
            ("level", 32, 8)
        ]
    );
    assert!(attributes.parts.iter().all(|part| part.source.starts_with("header_troops.py:")));
    assert!(schema.fields[12].optional && schema.fields[13].optional);
    assert!(schema.fields[14].optional && schema.fields[15].optional);
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
    // The pilot fields print readably, not as bare indices / packed integers.
    let text = uw_decomp::records::print_with_blocks(
        &atlas,
        &schema,
        &atlas.tables["troops"].entries[0],
        &[],
        &mut |_| unreachable!("no blocks"),
    );
    assert!(text.contains("  faction fac_"), "{text}");
    assert!(text.contains("  attributes {strength "), "{text}");
}
