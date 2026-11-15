//! Integration tests for the atlas loader, run against the real-data fixture.

use uw_atlas::Atlas;

fn fixture() -> Atlas {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/atlas_small.json"
    );
    Atlas::from_path(path).expect("fixture loads")
}

#[test]
fn loads_fixture() {
    let atlas = fixture();
    assert_eq!(atlas.schema, 1);
    assert_eq!(atlas.kit_hash.as_deref(), Some("fixture"));
    let tables: Vec<&str> = atlas.tables.keys().map(String::as_str).collect();
    assert_eq!(tables, ["dialogs", "items", "parties"]);
}

#[test]
fn entry_by_plain_id() {
    let atlas = fixture();
    let (table, entry) = atlas.entry_by_id("p_town_6").expect("p_town_6 exists");
    assert_eq!(table, "parties");
    assert_eq!(entry.index, 26);
    assert_eq!(entry.line, Some(82));
    assert_eq!(entry.name.as_deref(), Some("town_6"));
    assert_eq!(entry.value[1], serde_json::json!("Praven"));
}

#[test]
fn entry_by_dup_suffix() {
    let atlas = fixture();
    // The last occurrence keeps the identifier...
    let (_, last) = atlas
        .entry_by_id("itm_tutorial_spear")
        .expect("itm_tutorial_spear exists");
    assert_eq!(last.index, 130);
    assert_eq!(last.dup, Some(2));
    // ...the shadowed first occurrence only answers to `id#1`.
    let (table, first) = atlas
        .entry_by_id("itm_tutorial_spear#1")
        .expect("itm_tutorial_spear#1 exists");
    assert_eq!(table, "items");
    assert_eq!(first.index, 1);
    assert_eq!(first.id, None);
    let (_, second) = atlas
        .entry_by_id("itm_tutorial_spear#2")
        .expect("itm_tutorial_spear#2 exists");
    assert_eq!(second.index, 130);
    assert!(atlas.entry_by_id("itm_tutorial_spear#3").is_none());
    assert!(atlas.entry_by_id("itm_no_such_item").is_none());
    // Unnamed tables have no ids to look up.
    assert!(atlas.entry_by_id("").is_none());
}

#[test]
fn entries_named_lists_duplicates() {
    let atlas = fixture();
    let indexes: Vec<usize> = atlas
        .entries_named("items", "tutorial_spear")
        .map(|entry| entry.index)
        .collect();
    assert_eq!(indexes, [1, 130]);
    assert_eq!(atlas.entries_named("items", "no_such_name").count(), 0);
    assert_eq!(atlas.entries_named("no_such_table", "tutorial_spear").count(), 0);
}

#[test]
fn ops_can_fail() {
    let atlas = fixture();
    let eq = atlas.op("eq").expect("eq op exists");
    assert!(eq.can_fail);
    assert_eq!(eq.code, 31);
    assert!(!atlas.op("call_script").expect("call_script op").can_fail);
    let store_add = atlas.op("store_add").expect("store_add op");
    assert!(store_add.lhs);
    assert_eq!(store_add.code, 2120);
    assert!(atlas.op("no_such_op").is_none());
}

#[test]
fn hooks_are_bare_names() {
    let atlas = fixture();
    assert!(atlas.is_hook("game_start"));
    assert!(!atlas.is_hook("script_game_start"));
    assert!(!atlas.is_hook("not_a_hook"));
}

#[test]
fn all_ids_skips_unnamed_entries() {
    let atlas = fixture();
    let ids: Vec<&str> = atlas.all_ids().collect();
    assert!(ids.contains(&"p_town_6"));
    assert!(ids.contains(&"itm_tutorial_spear"));
    assert!(!ids.contains(&"itm_tutorial_spear#1"));
    // Tables in name order (dialogs, items, parties); items index 1 is the shadowed duplicate,
    // dialogs carry no ids.
    assert_eq!(
        ids,
        [
            "itm_no_item",
            "itm_tutorial_spear",
            "p_main_party",
            "p_temp_party",
            "p_town_6"
        ]
    );
}

#[test]
fn other_schema_is_rejected() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/atlas_small.json"
    );
    let mut text = std::fs::read_to_string(path).expect("fixture readable");
    text = text.replacen("\"schema\": 1", "\"schema\": 2", 1);
    let err = Atlas::from_str(&text).expect_err("schema 2 must be rejected");
    assert!(
        err.to_string().contains("schema"),
        "error mentions the schema: {err}"
    );
    // The unchanged text still loads, so the rejection is the schema check and not a typo.
    let original = std::fs::read_to_string(path).expect("fixture readable");
    assert!(Atlas::from_str(&original).is_ok());
}
