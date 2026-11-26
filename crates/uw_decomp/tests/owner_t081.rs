//! T-081: `uw new` names the faction a town starts with, from the `game_start` script's
//! `call_script script_give_center_to_faction_aux <party> <faction>` calls. The real-atlas test
//! skips when the atlas is absent (like project_t044).

use std::path::{Path, PathBuf};

use uw_atlas::Atlas;
use uw_decomp::project::layout;
use uw_decomp::project::{export, verify};
use uw_decomp::records::{parse, print, Schema};

/// One faction, two towns and a `game_start` that gives only `p_town_6` to `fac_kingdom_1`.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t081",
  "ops": [{"name":"call_script","code":1,"line":14,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}],
  "hooks": ["game_start"], "ranges": [],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
      "entries": [
        {"index":0,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":1,
          "value":["kingdom_1","Kingdom of Swadia"]}
      ]},
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
      "entries": [
        {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":82,
          "value":["town_6","Praven"]},
        {"index":1,"name":"town_1","id":"p_town_1","dup":null,"line":77,
          "value":["town_1","Uxkhal"]}
      ]},
    "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_",
      "entries": [
        {"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,
          "value":["game_start",{"list":[[1,"script_give_center_to_faction_aux","p_town_6","fac_kingdom_1"]]}]}
      ]}
  }
}"#;

const TINY_SCHEMA: &str = r#"table = "parties"
kind = "party"
id_prefix = "p_"

[[fields]]
name = "id"
ty = "raw"
source = "module_parties.py:10"

[[fields]]
name = "name"
ty = "raw"
source = "module_parties.py:12"
"#;

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-owner-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Exports `atlas` with `schema` into `<root>/hikayem` and returns that folder.
fn exported(atlas: &Atlas, schema: &Schema, root: &Path) -> PathBuf {
    let out = root.join("hikayem");
    export(atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry)).expect("export");
    out
}

/// The `parties/<town>.uw` text of one record.
fn party_file(atlas: &Atlas, schema: &Schema, out: &Path, id: &str) -> String {
    let (_, entry) = atlas.entry_by_id(id).expect("party in the atlas");
    let relative = layout::path_for(atlas, schema, entry);
    std::fs::read_to_string(out.join(&relative)).expect("record file")
}

#[test]
fn tiny_atlas_marks_the_owner_and_reads_back_exact() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let schema = Schema::from_toml(TINY_SCHEMA).expect("tiny schema");
    let root = scratch("tiny");
    let out = exported(&atlas, &schema, &root);

    let praven = party_file(&atlas, &schema, &out, "p_town_6");
    assert!(
        praven.contains("owner fac_kingdom_1"),
        "no owner field in:\n{praven}"
    );
    assert!(
        praven.contains("// who gets it at game start (game_start script); change it here"),
        "the owner line does not say what it means:\n{praven}"
    );
    let uxkhal = party_file(&atlas, &schema, &out, "p_town_1");
    assert!(!uxkhal.contains("owner "), "a town without an owner call got a line:\n{uxkhal}");

    let report = verify(&atlas, &[schema], &out, &mut |schema, text| parse(schema, text)).expect("verify");
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(report.exact, report.records);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn game_names_replace_the_kit_id_in_the_owner_line() {
    let mut atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    atlas.set_game_names(&[("swadia".into(), "fac_kingdom_1".into())]);
    let schema = Schema::from_toml(TINY_SCHEMA).expect("tiny schema");
    let root = scratch("names");
    let out = exported(&atlas, &schema, &root);

    let praven = party_file(&atlas, &schema, &out, "p_town_6");
    assert!(
        praven.contains("owner swadia"),
        "no game-name owner field in:\n{praven}"
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn real_atlas_praven_owner_and_verify() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let mut atlas = Atlas::from_path(path).expect("full atlas loads");
    let names = uw_decomp::names::derive(&atlas);
    atlas.set_game_names(&uw_decomp::names::pairs(&names));
    let dir = format!("{}/../../kit-meta/records", env!("CARGO_MANIFEST_DIR"));
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("schema folder")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .collect();
    paths.sort();
    let schemas: Vec<Schema> = paths
        .iter()
        .map(|path| Schema::from_path(path).expect("schema loads"))
        .collect();
    let parties = schemas.iter().find(|schema| schema.table == "parties").expect("parties schema");

    let root = scratch("native");
    let out = root.join("hikayem");
    export(&atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");

    let praven = party_file(&atlas, parties, &out, "p_town_6");
    assert!(praven.contains("owner swadia"), "no owner line in the Praven file:\n{praven}");

    let report = verify(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("verify");
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(report.exact, report.records);

    std::fs::remove_dir_all(&root).ok();
}
