//! T-103: a project's `owner` line is a center's game-start owner. `uw new` writes it from the
//! `game_start` script, the loader reads it back, and a changed owner rewrites exactly that one
//! give-center argument. The real-atlas case lives in `owner_t081.rs`.

use std::path::{Path, PathBuf};

use serde_json::json;
use uw_atlas::Atlas;
use uw_decomp::project::load::load_with_owners;
use uw_decomp::project::{apply_owners, export, OwnerChange};
use uw_decomp::records::{parse, print, Schema};

/// Two factions and two towns; `game_start` gives both towns to `fac_kingdom_1`.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t103",
  "ops": [{"name":"call_script","code":1,"line":14,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}],
  "hooks": ["game_start"], "ranges": [],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
      "entries": [
        {"index":0,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":1,
          "value":["kingdom_1","Kingdom of Swadia"]},
        {"index":1,"name":"kingdom_2","id":"fac_kingdom_2","dup":null,"line":2,
          "value":["kingdom_2","Kingdom of Vaegirs"]}
      ]},
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
      "entries": [
        {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":82,
          "value":["town_6","Praven"]},
        {"index":1,"name":"town_1","id":"p_town_1","dup":null,"line":77,
          "value":["town_1","Uxkhal"]},
        {"index":2,"name":"town_9","id":"p_town_9","dup":null,"line":90,
          "value":["town_9","Tulga"]}
      ]},
    "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_",
      "entries": [
        {"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,
          "value":["game_start",{"list":[
            [1,"script_give_center_to_faction_aux","p_town_6","fac_kingdom_1"],
            [1,"script_give_center_to_faction_aux","p_town_1","fac_kingdom_1"]
          ]}]}
      ]}
  }
}"#;

const PARTIES_SCHEMA: &str = r#"table = "parties"
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

const SCRIPTS_SCHEMA: &str = r#"table = "scripts"
kind = "script"
id_prefix = "script_"

[[fields]]
name = "name"
ty = "raw"
source = "module_scripts.py:22"

[[fields]]
name = "ops"
ty = "raw"
source = "module_scripts.py:22"
"#;

fn schemas() -> Vec<Schema> {
    vec![
        Schema::from_toml(PARTIES_SCHEMA).expect("parties schema"),
        Schema::from_toml(SCRIPTS_SCHEMA).expect("scripts schema"),
    ]
}

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-owner-t103-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Exports the tiny atlas into `<root>/hikayem` and returns that folder with its schemas.
fn exported(atlas: &Atlas, root: &Path) -> (PathBuf, Vec<Schema>) {
    let schemas = schemas();
    let out = root.join("hikayem");
    export(atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");
    (out, schemas)
}

/// The file holding `p_town_6`.
fn owner_file(out: &Path) -> PathBuf {
    out.join("parties/town_6.uw")
}

#[test]
fn owner_accepts_game_names_and_kit_ids() {
    for (i, written) in ["vaegirs", "kingdom_2", "fac_kingdom_2"].iter().enumerate() {
        let mut atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
        atlas.set_game_names(&[
            ("swadia".into(), "fac_kingdom_1".into()),
            ("vaegirs".into(), "fac_kingdom_2".into()),
        ]);
        let root = scratch(&format!("names-{i}"));
        let (out, schemas) = exported(&atlas, &root);
        let path = owner_file(&out);
        let text = std::fs::read_to_string(&path).expect("party file");
        assert!(text.contains("owner swadia"), "{text}");
        std::fs::write(&path, text.replace("owner swadia", &format!("owner {written}"))).expect("write");

        let (_, owners) =
            load_with_owners(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("load");
        assert_eq!(owners.get("p_town_6").map(String::as_str), Some("fac_kingdom_2"), "owner {written}");

        std::fs::remove_dir_all(&root).ok();
    }
}

#[test]
fn changed_owner_rewrites_exactly_one_game_start_argument() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let root = scratch("changed");
    let (out, schemas) = exported(&atlas, &root);
    let path = owner_file(&out);
    let text = std::fs::read_to_string(&path).expect("party file");
    assert!(text.contains("owner fac_kingdom_1"), "{text}");
    std::fs::write(&path, text.replace("owner fac_kingdom_1", "owner fac_kingdom_2")).expect("write");

    let (mut records, owners) =
        load_with_owners(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("load");
    assert_eq!(owners.get("p_town_6").map(String::as_str), Some("fac_kingdom_2"));
    assert_eq!(owners.get("p_town_1").map(String::as_str), Some("fac_kingdom_1"));

    let changes = apply_owners(&atlas, &mut records, &owners).expect("apply owners");
    assert_eq!(
        changes,
        vec![OwnerChange { party: "p_town_6".into(), from: "fac_kingdom_1".into(), to: "fac_kingdom_2".into() }]
    );
    let statements = &records["scripts"][0][1]["list"];
    assert_eq!(statements[0], json!([1, "script_give_center_to_faction_aux", "p_town_6", "fac_kingdom_2"]));
    assert_eq!(statements[1], json!([1, "script_give_center_to_faction_aux", "p_town_1", "fac_kingdom_1"]));

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn owner_on_a_center_without_a_give_call_is_an_error() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let root = scratch("no-call");
    let (out, schemas) = exported(&atlas, &root);
    let path = out.join("parties/town_9.uw");
    let text = std::fs::read_to_string(&path).expect("party file");
    let header = "party town_9 {\n";
    assert!(text.contains(header), "{text}");
    std::fs::write(&path, text.replace(header, &format!("{header}  owner fac_kingdom_2\n"))).expect("write");

    let (mut records, owners) =
        load_with_owners(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("load");
    assert_eq!(owners.get("p_town_9").map(String::as_str), Some("fac_kingdom_2"));
    let errors = apply_owners(&atlas, &mut records, &owners).expect_err("a center with no call is an error");
    assert!(
        errors.iter().any(|error| error.contains("p_town_9") && error.contains("çağrı yok")),
        "{errors:?}"
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn unknown_owner_names_the_file_and_line() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let root = scratch("unknown");
    let (out, schemas) = exported(&atlas, &root);
    let path = owner_file(&out);
    let text = std::fs::read_to_string(&path).expect("party file");
    let changed = text.replace("owner fac_kingdom_1", "owner nowhere");
    let line = changed.lines().position(|line| line.contains("owner nowhere")).expect("owner line") + 1;
    std::fs::write(&path, &changed).expect("write");

    let errors = load_with_owners(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text))
        .expect_err("an unknown owner is an error");
    assert!(
        errors.iter().any(|error| error.contains("nowhere")
            && error.contains(&format!("line {line}"))
            && error.contains("tanınmıyor")),
        "{errors:?}"
    );

    std::fs::remove_dir_all(&root).ok();
}
