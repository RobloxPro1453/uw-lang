//! T-060: the item records print their kit-stated fields readably with the T-058 types and parse
//! back to the identical atlas value (`target/uw-staging/atlas.json`, made by `py/atlas_dump.py`).
//! The real-atlas test skips itself on a machine without that atlas, like the other real-atlas
//! tests; a tiny in-test atlas checks the same printing without the big file.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/items.toml");

fn schema() -> Schema {
    Schema::from_path(SCHEMA).expect("items schema loads")
}

/// One record of a tiny atlas: the kit's item value shape with only the constants the schema names.
fn tiny_atlas() -> Atlas {
    let item_flags = 0x2u64 | 0x400000 | 0x800000;
    // weight(1.5) | swing_damage(16, blunt=2): no other part is set, so the struct is short.
    let item_stats = (6u64 << 24) | (16u64 << 50) | (2u64 << 58);
    let modifier_bits = 0x200000u64 | 0x2000000;
    let json = serde_json::json!({
        "schema": 1, "kit_hash": "t060", "ops": [], "hooks": [], "ranges": [],
        "constants": [
            {"name": "itp_type_one_handed_wpn", "value": 2, "module": "header_items", "line": 14},
            {"name": "itp_primary", "value": 4194304, "module": "header_items", "line": 63},
            {"name": "itp_secondary", "value": 8388608, "module": "header_items", "line": 64},
            {"name": "itp_attach_armature", "value": 3840, "module": "header_items", "line": 42},
            {"name": "itp_attachment_mask", "value": 3840, "module": "header_items", "line": 43},
            {"name": "itcf_thrust_onehanded", "value": 1, "module": "header_items", "line": 279},
            {"name": "itcf_shoot_mask", "value": 1044480, "module": "header_items", "line": 304},
            {"name": "itcf_carry_mask", "value": 34091302912i64, "module": "header_items", "line": 314},
            {"name": "itcf_reload_mask", "value": 1030792151040i64, "module": "header_items", "line": 346},
            {"name": "imodbit_tattered", "value": 2097152, "module": "header_item_modifiers", "line": 67},
            {"name": "imodbit_thick", "value": 33554432, "module": "header_item_modifiers", "line": 71}
        ],
        "tables": {
            "factions": {"module": "module_factions", "var": "factions", "id_file": "ID_factions",
                "id_prefix": "fac_", "entries": [
                {"index": 16, "name": "kingdom_2", "id": "fac_kingdom_2", "dup": null, "line": 17, "value": [0]},
                {"index": 17, "name": "kingdom_3", "id": "fac_kingdom_3", "dup": null, "line": 18, "value": [0]}
            ]},
            "items": {"module": "module_items", "var": "items", "id_file": "ID_items",
                "id_prefix": "itm_", "entries": [
                {"index": 0, "name": "no_item", "id": "itm_no_item", "dup": null, "line": 52,
                 "value": {"list": [
                    "no_item", "INVALID ITEM", {"list": [["invalid_item", 0]]},
                    item_flags, 1, 3, item_stats, modifier_bits,
                    {"list": []}, {"list": [16, 17]}
                 ]}}
            ]}
        }
    });
    Atlas::from_str(&json.to_string()).expect("tiny items atlas loads")
}

#[test]
fn items_schema_follows_the_kit_layout_comment() {
    let schema = schema();
    assert_eq!(
        (schema.table.as_str(), schema.kind.as_str(), schema.id_prefix.as_str(), schema.list),
        ("items", "item", "itm_", true)
    );
    let names: Vec<&str> = schema.fields.iter().map(|field| field.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "id",
            "name",
            "meshes",
            "item_flags",
            "item_capabilities",
            "value",
            "item_stats",
            "modifier_bits",
            "triggers",
            "sold_in"
        ]
    );
    let ty = |name: &str| {
        schema.fields.iter().find(|field| field.name == name).map(|field| field.ty.as_str())
    };
    assert_eq!(ty("item_flags"), Some("flags:itp_"));
    assert_eq!(ty("item_capabilities"), Some("flags:itcf_"));
    assert_eq!(ty("item_stats"), Some("bits"));
    assert_eq!(ty("modifier_bits"), Some("flags:imodbit_"));
    assert_eq!(ty("sold_in"), Some("list:ref:factions"));
    for raw in ["id", "name", "meshes", "value", "triggers"] {
        assert_eq!(ty(raw), Some("raw"), "`{raw}` has no kit meaning");
    }
    assert!(schema.fields.iter().all(|field| field.source.starts_with("module_items.py:")));
    assert!(schema.fields[8].optional && schema.fields[9].optional);

    let stats = schema.fields.iter().find(|field| field.name == "item_stats").expect("stats");
    let parts: Vec<(&str, u32, u32)> =
        stats.parts.iter().map(|part| (part.name.as_str(), part.shift, part.width)).collect();
    assert_eq!(
        parts,
        [
            ("head_armor", 0, 8),
            ("body_armor", 8, 8),
            ("leg_armor", 16, 8),
            ("weight", 24, 8),
            ("difficulty", 32, 8),
            ("hit_points", 40, 16),
            ("swing_damage", 50, 8),
            ("swing_damage_type", 58, 2),
            ("thrust_damage", 60, 8),
            ("thrust_damage_type", 68, 2),
            ("weapon_length", 70, 10),
            ("speed_rating", 80, 8),
            ("shoot_speed", 90, 10),
            ("max_ammo", 100, 8),
            ("abundance", 110, 8)
        ]
    );
    assert!(stats.parts.iter().all(|part| part.source.starts_with("header_items.py:")));
}

#[test]
fn typed_item_fields_print_readably_and_read_back() {
    let atlas = tiny_atlas();
    let schema = schema();
    let entry = &atlas.tables["items"].entries[0];
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!((report.records, report.exact, report.failed), (1, 1, 0), "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields, report.raw_rest), (5, 10, 5));
    let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
    assert!(
        text.contains("  item_flags [itp_type_one_handed_wpn, itp_primary, itp_secondary]\n"),
        "{text}"
    );
    assert!(text.contains("  item_capabilities [itcf_thrust_onehanded]\n"), "{text}");
    assert!(text.contains("weight 6, difficulty 0, hit_points 16384"), "{text}");
    assert!(text.contains("swing_damage 16, swing_damage_type 2"), "{text}");
    assert!(text.contains("  modifier_bits [imodbit_tattered, imodbit_thick]\n"), "{text}");
    assert!(text.contains("  sold_in [fac_kingdom_2, fac_kingdom_3]\n"), "{text}");
}

#[test]
fn real_atlas_items_round_trip() {
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = schema();
    let report = roundtrip_table(&atlas, &schema);
    // ID_items.py has one id per item (itm_ccoop_new_items_end = 619).
    assert_eq!(report.records, 620, "items record count");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields), (5, 10));

    let text = |id: &str| {
        let (table, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no `{id}`"));
        assert_eq!(table, "items");
        print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"))
    };
    let no_item = text("itm_no_item");
    assert!(
        no_item.contains("  item_flags [itp_type_one_handed_wpn, itp_primary, itp_secondary]\n"),
        "{no_item}"
    );
    assert!(no_item.contains("weight 6, difficulty 0, hit_points 16384"), "{no_item}");
    assert!(no_item.contains("  modifier_bits []\n"), "{no_item}");

    let boots = text("itm_practice_boots");
    assert!(
        boots.contains("  item_flags [itp_type_foot_armor, itp_attach_armature, itp_civilian]\n"),
        "{boots}"
    );
    assert!(boots.contains("leg_armor 10"), "{boots}");

    let horse = text("itm_steppe_horse");
    assert!(horse.contains("  sold_in [fac_kingdom_2, fac_kingdom_3]\n"), "{horse}");
    assert!(horse.contains("hit_points 120"), "{horse}");

    let end = text("itm_ccoop_new_items_end");
    assert!(end.contains("  item_flags []\n"), "{end}");
    assert!(end.contains("  item_capabilities []\n"), "{end}");
    assert!(end.contains("  item_stats {"), "{end}");
    assert!(end.contains("  modifier_bits []\n"), "{end}");
}
