//! T-080: party records read like the game: the map icon by name (`icon_town`), `menu none` and
//! `ai_target none`, and every parties/party_templates record still round-trips the real atlas
//! exactly. The atlas is machine-local (`target/uw-staging/atlas.json`); these tests skip
//! themselves when it is absent.

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

fn schema(table: &str) -> Schema {
    let path = format!("{SCHEMA_DIR}/{table}.toml");
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// The printed record text of the atlas entry `id`.
fn printed(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let (_, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no entry `{id}`"));
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

#[test]
fn praven_reads_menu_and_target_as_none_and_the_icon_by_name() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = schema("parties");
    let town = printed(&atlas, &schema, "p_town_6");
    assert!(town.contains("  menu none\n"), "{town}");
    assert!(town.contains("  ai_target none\n"), "{town}");
    assert!(town.contains("party_flags {icon icon_town,"), "{town}");
    assert!(!town.contains("rest 12"), "the icon is no longer flag rest bits:\n{town}");
}

#[test]
fn party_templates_read_the_same_icon_and_menu_none() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = schema("party_templates");
    let herd = printed(&atlas, &schema, "pt_cattle_herd");
    assert!(herd.contains("party_flags {icon icon_cattle,"), "{herd}");
    assert!(herd.contains("  menu none\n"), "{herd}");
}

#[test]
fn every_party_record_reads_back_exact() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    for table in ["parties", "party_templates"] {
        let schema = schema(table);
        let report = roundtrip_table(&atlas, &schema);
        assert!(report.records > 0, "{table}: no records");
        assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
        assert_eq!(report.exact, report.records, "{table}: {:?}", report.first_failures);
    }
}
