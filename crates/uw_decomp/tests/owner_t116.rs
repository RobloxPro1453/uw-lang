//! T-116: a project's `lord` line is a center's game-start lord, next to its `owner`. `uw new`
//! writes it from the `game_start` `script_give_center_to_lord` calls, the loader reads it back, and
//! the build rewrites, adds or removes that one call without touching any other op.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as Json};
use uw_atlas::Atlas;
use uw_decomp::project::load::load_with_owners_names_and_lords;
use uw_decomp::project::{apply_lords, export, LordLine};
use uw_decomp::records::{parse, print, Schema};

/// Two factions, three towns and three lords; `game_start` gives both owned towns to `fac_kingdom_1`
/// and hands `p_town_6` to King Harlaus. `p_town_9` has neither an owner nor a lord call.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t116",
  "ops": [{"name":"call_script","code":1,"line":14,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}],
  "hooks": ["game_start"],
  "ranges": [
    {"name":"towns_begin","value":0,"line":1},
    {"name":"towns_end","value":3,"line":2}
  ],
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
    "troops": {"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_",
      "entries": [
        {"index":0,"name":"kingdom_1_lord","id":"trp_kingdom_1_lord","dup":null,"line":1,
          "value":["kingdom_1_lord","King Harlaus","Harlaus",0,0,0,0]},
        {"index":1,"name":"kingdom_2_lord","id":"trp_kingdom_2_lord","dup":null,"line":2,
          "value":["kingdom_2_lord","King Yaroglek","Yaroglek",0,0,0,1]},
        {"index":2,"name":"knight_1_1","id":"trp_knight_1_1","dup":null,"line":3,
          "value":["knight_1_1","Lord Deglan","Deglan",0,0,0,0]}
      ]},
    "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_",
      "entries": [
        {"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,
          "value":["game_start",{"list":[
            [1,"script_give_center_to_faction_aux","p_town_6","fac_kingdom_1"],
            [1,"script_give_center_to_faction_aux","p_town_1","fac_kingdom_1"],
            [1,"script_give_center_to_lord","p_town_6","trp_kingdom_1_lord",0],
            [1,"script_assign_lords_to_empty_centers"]
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

type Records = BTreeMap<String, Vec<Json>>;

fn schemas() -> Vec<Schema> {
    vec![
        Schema::from_toml(PARTIES_SCHEMA).expect("parties schema"),
        Schema::from_toml(SCRIPTS_SCHEMA).expect("scripts schema"),
    ]
}

/// The tiny atlas with the game names the project writes (`uw new` derives these from display
/// names; the test sets them directly).
fn atlas_with_names() -> Atlas {
    let mut atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    atlas.set_game_names(&[
        ("swadia".into(), "fac_kingdom_1".into()),
        ("vaegirs".into(), "fac_kingdom_2".into()),
        ("king_harlaus".into(), "trp_kingdom_1_lord".into()),
        ("yaroglek".into(), "trp_kingdom_2_lord".into()),
        ("lord_deglan".into(), "trp_knight_1_1".into()),
    ]);
    atlas
}

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-owner-t116-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Exports the atlas into `<root>/hikayem` and returns that folder with its schemas.
fn exported(atlas: &Atlas, root: &Path) -> (PathBuf, Vec<Schema>) {
    let schemas = schemas();
    let out = root.join("hikayem");
    export(atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");
    (out, schemas)
}

/// The file holding `town` (`town_6` -> `parties/towns/town_6.uw`).
fn town_file(out: &Path, town: &str) -> PathBuf {
    out.join("parties/towns").join(format!("{town}.uw"))
}

/// Loads the project into records, owners and lord lines.
fn load(atlas: &Atlas, schemas: &[Schema], out: &Path) -> (Records, BTreeMap<String, String>, Vec<LordLine>) {
    let (records, owners, _names, lords) =
        load_with_owners_names_and_lords(atlas, schemas, out, &mut |schema, text| parse(schema, text))
            .expect("load");
    (records, owners, lords)
}

/// The `game_start` statement list.
fn statements(records: &Records) -> &Vec<Json> {
    records["scripts"][0][1]["list"].as_array().expect("statement list")
}

#[test]
fn lord_is_printed_with_its_game_name_or_auto() {
    let atlas = atlas_with_names();
    let root = scratch("printed");
    let (out, schemas) = exported(&atlas, &root);

    let praven = std::fs::read_to_string(town_file(&out, "town_6")).expect("Praven file");
    assert!(praven.contains("lord king_harlaus"), "no lord line in:\n{praven}");
    let uxkhal = std::fs::read_to_string(town_file(&out, "town_1")).expect("Uxkhal file");
    assert!(uxkhal.contains("lord auto"), "no auto lord in:\n{uxkhal}");
    assert!(!praven.contains("lord auto"), "{praven}");

    let (records, _, lords) = load(&atlas, &schemas, &out);
    assert_eq!(statements(&records).len(), 4, "{records:?}");
    let praven_lord = lords.iter().find(|line| line.party == "p_town_6").expect("Praven lord line");
    assert_eq!(praven_lord.troop.as_deref(), Some("trp_kingdom_1_lord"));
    let uxkhal_lord = lords.iter().find(|line| line.party == "p_town_1").expect("Uxkhal lord line");
    assert_eq!(uxkhal_lord.troop, None);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn changed_lord_rewrites_exactly_one_statement() {
    let atlas = atlas_with_names();
    let root = scratch("changed");
    let (out, schemas) = exported(&atlas, &root);
    let path = town_file(&out, "town_6");
    let text = std::fs::read_to_string(&path).expect("Praven file");
    std::fs::write(&path, text.replace("lord king_harlaus", "lord lord_deglan")).expect("write");

    let (mut records, owners, lords) = load(&atlas, &schemas, &out);
    let before = statements(&records).clone();
    let notices = apply_lords(&atlas, &mut records, &owners, &lords).expect("apply lords");
    assert!(notices.is_empty(), "{notices:?}");

    let after = statements(&records);
    assert_eq!(after[2], json!([1, "script_give_center_to_lord", "p_town_6", "trp_knight_1_1", 0]));
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], before[1]);
    assert_eq!(after[3], before[3]);
    assert_eq!(after.len(), before.len());

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn auto_removes_the_lord_call() {
    let atlas = atlas_with_names();
    let root = scratch("auto");
    let (out, schemas) = exported(&atlas, &root);
    let path = town_file(&out, "town_6");
    let text = std::fs::read_to_string(&path).expect("Praven file");
    std::fs::write(&path, text.replace("lord king_harlaus", "lord auto")).expect("write");

    let (mut records, owners, lords) = load(&atlas, &schemas, &out);
    let before = statements(&records).clone();
    apply_lords(&atlas, &mut records, &owners, &lords).expect("apply lords");

    let after = statements(&records);
    assert_eq!(after.len(), before.len() - 1, "{after:?}");
    assert!(
        after.iter().all(|statement| statement.get(1).and_then(Json::as_str) != Some("script_give_center_to_lord")),
        "{after:?}"
    );
    assert_eq!(after[0], before[0]);
    assert_eq!(after[1], before[1]);
    assert_eq!(after[2], before[3]);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn owner_change_leaves_the_old_foreign_lord_as_auto_with_a_notice() {
    let atlas = atlas_with_names();
    let root = scratch("foreign-old");
    let (out, schemas) = exported(&atlas, &root);
    let path = town_file(&out, "town_6");
    let text = std::fs::read_to_string(&path).expect("Praven file");
    std::fs::write(&path, text.replace("owner swadia", "owner vaegirs")).expect("write");

    let (mut records, owners, lords) = load(&atlas, &schemas, &out);
    assert_eq!(owners.get("p_town_6").map(String::as_str), Some("fac_kingdom_2"));
    let notices = apply_lords(&atlas, &mut records, &owners, &lords).expect("apply lords");
    assert_eq!(notices.len(), 1, "{notices:?}");
    let notice = &notices[0];
    for part in ["Praven", "King Harlaus", "(Swadia)", "(Vaegirs)", "uymuyor", "Vaegir lordu", "Kendin seçmek için: lord"]
    {
        assert!(notice.contains(part), "`{part}` missing from: {notice}");
    }

    let after = statements(&records);
    assert!(
        !after.iter().any(|statement| *statement
            == json!([1, "script_give_center_to_lord", "p_town_6", "trp_kingdom_1_lord", 0])),
        "{after:?}"
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn chosen_foreign_lord_names_the_file_and_line() {
    let atlas = atlas_with_names();
    let root = scratch("foreign-chosen");
    let (out, schemas) = exported(&atlas, &root);
    let path = town_file(&out, "town_6");
    let text = std::fs::read_to_string(&path).expect("Praven file");
    let changed = text.replace("lord king_harlaus", "lord yaroglek");
    let line = changed.lines().position(|line| line.contains("lord yaroglek")).expect("lord line") + 1;
    std::fs::write(&path, &changed).expect("write");

    let (mut records, owners, lords) = load(&atlas, &schemas, &out);
    let errors = apply_lords(&atlas, &mut records, &owners, &lords).expect_err("a foreign lord is an error");
    assert!(
        errors.iter().any(|error| error.contains(&format!("line {line}"))
            && error.contains("uymuyor")
            && error.contains("King Yaroglek")),
        "{errors:?}"
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn chosen_lord_of_a_center_without_a_lord_call_gets_one_before_assign_lords() {
    let atlas = atlas_with_names();
    let root = scratch("added");
    let (out, schemas) = exported(&atlas, &root);
    let path = town_file(&out, "town_1");
    let text = std::fs::read_to_string(&path).expect("Uxkhal file");
    std::fs::write(&path, text.replace("lord auto", "lord lord_deglan")).expect("write");

    let (mut records, owners, lords) = load(&atlas, &schemas, &out);
    apply_lords(&atlas, &mut records, &owners, &lords).expect("apply lords");

    let after = statements(&records);
    assert_eq!(after.len(), 5, "{after:?}");
    assert_eq!(after[3], json!([1, "script_give_center_to_lord", "p_town_1", "trp_knight_1_1", 0]));
    assert_eq!(after[4], json!([1, "script_assign_lords_to_empty_centers"]));

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn real_atlas_praven_prints_king_harlaus() {
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

    let root = scratch("native");
    let out = root.join("hikayem");
    export(&atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");

    let praven = std::fs::read_to_string(town_file(&out, "town_6")).expect("Praven file");
    assert!(praven.contains("lord king_harlaus"), "no lord line in the Praven file:\n{praven}");

    std::fs::remove_dir_all(&root).ok();
}
