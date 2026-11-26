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

/// A tiny atlas shaped like the real dump: ops with flag bits, ref tags that differ from the
/// ID prefixes (`mnu` vs `menu_`), a repeated name, and one script block.
fn refs_fixture() -> Atlas {
    let text = r#"{
      "schema": 1, "kit_hash": "refs",
      "ops": [
        {"name":"call_script","code":1,"line":14,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"end_try","code":3,"line":15,"signature":"deprecated, use try_end instead","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"try_end","code":3,"line":16,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"eq","code":31,"line":20,"signature":"","lhs":false,"can_fail":true,"global_lhs":false},
        {"name":"jump_to_menu","code":2060,"line":30,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"assign","code":2133,"line":40,"signature":"","lhs":true,"can_fail":false,"global_lhs":false},
        {"name":"neg","code":2147483648,"line":440,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"this_or_next","code":1073741824,"line":441,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}
      ],
      "hooks": ["game_start"],
      "ranges": [],
      "tables": {
        "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu",
          "entries": [
            {"index":0,"name":"start","id":"menu_start","dup":null,"line":10,"value":["start",0,"Hi",{"list":[]}]},
            {"index":1,"name":"camp","id":null,"dup":1,"line":20,"value":["camp",0,"A",{"list":[]}]},
            {"index":2,"name":"Camp","id":"menu_camp","dup":2,"line":30,"value":["Camp",0,"B",{"list":[]}]}
          ]},
        "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
          "entries": [
            {"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,"value":["game_start",{"list":[
              [2060,"mnu_camp"],
              [2133,"$g_x",1],
              [1073741855,"$g_x",[5,"mnu_start"]],
              [1,"script_helper"],
              3
            ]}]},
            {"index":1,"name":"helper","id":"script_helper","dup":null,"line":50,"value":["helper",{"list":[[2133,"$g_x","mnu_start"]]}]}
          ]},
        "dialogs": {"module":"module_dialogs","var":"dialogs","id_file":null,"id_prefix":null,"ref_tag":null,
          "entries": [{"index":0,"name":null,"id":null,"dup":null,"line":null,"value":{"list":["anyone","start",{"list":[]},"Hello","close_window",{"list":[[2060,"mnu_start"]]}]}}]}
      }
    }"#;
    Atlas::from_str(text).expect("refs fixture loads")
}

#[test]
fn resolve_ref_uses_operation_tags_and_first_match() {
    let atlas = refs_fixture();
    assert_eq!(atlas.table_for_tag("mnu"), Some("menus"));
    assert!(atlas.resolve_ref("menu_start").is_none(), "ID prefix is not an operation tag");
    let (table, start) = atlas.resolve_ref("mnu_start").expect("mnu_start");
    assert_eq!((table, start.index), ("menus", 0));
    // Repeated name: operations get the FIRST entry (find_object), ID_*.py the last.
    assert_eq!(atlas.resolve_ref("mnu_CAMP").expect("mnu_camp").1.index, 1);
    assert_eq!(atlas.entry_by_id("menu_camp").expect("menu_camp").1.index, 2);
    assert!(atlas.resolve_ref("mnu_nope").is_none());
    assert!(atlas.resolve_ref("_start").is_none());
    assert!(atlas.resolve_ref("nounderscore").is_none());
}

#[test]
fn op_for_code_strips_flags_and_skips_deprecated_aliases() {
    let atlas = refs_fixture();
    assert_eq!(atlas.op_for_code(3).map(|op| op.name.as_str()), Some("try_end"));
    assert_eq!(atlas.op_for_code(1073741855).map(|op| op.name.as_str()), Some("eq"));
    assert_eq!(atlas.op_for_code(2147483679).map(|op| op.name.as_str()), Some("eq"));
    assert!(atlas.op_for_code(99999).is_none());
}

#[test]
fn references_name_the_operation_and_entry() {
    let atlas = refs_fixture();
    let refs = atlas.references(|text| {
        atlas.resolve_ref(text).is_some_and(|(t, e)| t == "menus" && e.index == 0)
    });
    let got: Vec<(&str, usize, Option<&str>)> = refs
        .iter()
        .map(|r| (r.table.as_str(), r.index, r.op.as_deref()))
        .collect();
    assert_eq!(
        got,
        [
            ("dialogs", 0, Some("jump_to_menu")),
            // Nested tuple inside a statement keeps the statement's op.
            ("scripts", 0, Some("eq")),
            ("scripts", 1, Some("assign")),
        ]
    );
    let globals = atlas.references(|text| text == "$g_x");
    assert_eq!(globals.len(), 3);
    assert!(globals.iter().all(|r| r.table == "scripts"));
    // The menu entry's own name string is data, not an operation operand.
    let camp = atlas.references(|text| text == "camp");
    assert_eq!(camp, [uw_atlas::Reference { table: "menus".into(), index: 1, op: None }]);
}
