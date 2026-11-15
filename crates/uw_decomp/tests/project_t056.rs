//! T-056: `uw new` writes `uw.lock` (the record order) and `project::load` reads a project folder
//! back into whole tables in that order. The real-atlas test skips when the atlas is absent.

use std::path::{Path, PathBuf};

use serde_json::json;
use uw_atlas::Atlas;
use uw_decomp::project::export;
use uw_decomp::project::load::load;
use uw_decomp::records::{parse, print, record_difference, Schema};

/// Two factions, three raw fields each: enough to show the lock order and the read-back.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t056", "ops": [], "hooks": [], "ranges": [],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
      "entries": [
        {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,
          "value":["no_faction","No Faction",0]},
        {"index":1,"name":"forest_bandits","id":"fac_forest_bandits","dup":null,"line":2,
          "value":["forest_bandits","Forest Bandits",1]}
      ]}
  }
}"#;

const TINY_SCHEMA: &str = r#"table = "factions"
kind = "faction"
id_prefix = "fac_"

[[fields]]
name = "id"
ty = "raw"
source = "module_factions.py:5"

[[fields]]
name = "name"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "flags"
ty = "raw"
source = "module_factions.py:8"
"#;

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-load-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn tiny() -> (Atlas, Schema) {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let schema = Schema::from_toml(TINY_SCHEMA).expect("tiny schema");
    (atlas, schema)
}

/// Exports the tiny atlas into `<root>/hikayem` and returns the folder.
fn exported(root: &Path) -> (PathBuf, Atlas, Schema) {
    let (atlas, schema) = tiny();
    let out = root.join("hikayem");
    export(&atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry)).expect("export");
    (out, atlas, schema)
}

/// Every `.uw` file of a folder, in path order.
fn uw_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return files };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(uw_files(&path));
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("uw") {
            files.push(path);
        }
    }
    files.sort();
    files
}

#[test]
fn export_writes_the_lock_and_load_reads_the_atlas_order() {
    let root = scratch("order");
    let (out, atlas, schema) = exported(&root);

    // uw.lock: the kit hash and the ids of every table, in atlas order.
    let lock = std::fs::read_to_string(out.join("uw.lock")).expect("uw.lock");
    assert!(lock.contains("kit_hash = \"t056\""), "{lock}");
    assert!(lock.contains("factions = [\"no_faction\", \"forest_bandits\"]"), "{lock}");

    let values = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect("load reads the folder");
    let factions = &values["factions"];
    assert_eq!(factions.len(), 2);
    let entries = &atlas.tables["factions"].entries;
    assert_eq!(factions[0], entries[0].value);
    assert_eq!(factions[1], entries[1].value);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_moved_file_still_loads_in_lock_order() {
    let root = scratch("moved");
    let (out, atlas, schema) = exported(&root);
    // Move the first record out of its folder; the lock, not the layout, keeps its position.
    std::fs::create_dir_all(out.join("zzz")).expect("new folder");
    let from = out.join("factions/no_faction.uw");
    std::fs::rename(&from, out.join("zzz/tasindi.uw")).expect("move the record");

    let values = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect("load reads the moved folder");
    let entries = &atlas.tables["factions"].entries;
    assert_eq!(values["factions"], vec![entries[0].value.clone(), entries[1].value.clone()]);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_deleted_record_is_an_error() {
    let root = scratch("deleted");
    let (out, atlas, schema) = exported(&root);
    std::fs::remove_file(out.join("factions/forest_bandits.uw")).expect("delete the record");

    let errors = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect_err("a missing record is an error");
    assert_eq!(
        errors,
        vec!["factions forest_bandits: kayıt silinmiş; silme henüz yok (P3b v1), kaydı geri koy".to_owned()]
    );

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_duplicated_record_is_an_error_naming_both_files() {
    let root = scratch("duplicate");
    let (out, atlas, schema) = exported(&root);
    std::fs::copy(out.join("factions/no_faction.uw"), out.join("factions/kopya.uw")).expect("copy the record");

    let errors = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect_err("a duplicated record is an error");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("factions no_faction: iki kayıt aynı id:"), "{}", errors[0]);
    assert!(errors[0].contains("no_faction.uw"), "{}", errors[0]);
    assert!(errors[0].contains("kopya.uw"), "{}", errors[0]);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_new_record_is_appended_last() {
    let root = scratch("new");
    let (out, atlas, schema) = exported(&root);
    let value = json!(["yeni_faction", "Yeni Faction", 9]);
    let entry = uw_atlas::Entry {
        index: 2,
        name: Some("yeni_faction".into()),
        id: Some("fac_yeni_faction".into()),
        dup: None,
        line: None,
        value: value.clone(),
    };
    std::fs::write(out.join("factions/yeni_faction.uw"), print(&schema, &entry)).expect("new record");

    let values = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect("load appends the new record");
    let entries = &atlas.tables["factions"].entries;
    assert_eq!(values["factions"].len(), 3);
    assert_eq!(values["factions"][0], entries[0].value);
    assert_eq!(values["factions"][1], entries[1].value);
    assert_eq!(values["factions"][2], value);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_wrong_kit_hash_or_a_missing_lock_is_an_error() {
    let root = scratch("lock");
    let (out, atlas, schema) = exported(&root);
    let lock_path = out.join("uw.lock");
    let lock = std::fs::read_to_string(&lock_path).expect("uw.lock");
    std::fs::write(&lock_path, lock.replace("kit_hash = \"t056\"", "kit_hash = \"t999\"")).expect("edit lock");

    let errors = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect_err("another kit hash is an error");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("kit_hash t999"), "{}", errors[0]);
    assert!(errors[0].contains("sessiz yeniden sıralama yok"), "{}", errors[0]);

    std::fs::remove_file(&lock_path).expect("remove the lock");
    let errors = load(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect_err("an unreadable lock is an error");
    assert!(errors[0].contains("uw.lock"), "{}", errors[0]);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn real_atlas_export_and_load_keep_every_record_in_order() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
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
    let summary = export(&atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");
    let values = load(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("load");

    let mut checked = 0usize;
    let mut bad = Vec::new();
    for schema in &schemas {
        let Some(table) = atlas.tables.get(&schema.table) else { continue };
        let loaded = &values[&schema.table];
        if loaded.len() != table.entries.len() {
            bad.push(format!("{}: {} values, {} atlas entries", schema.table, loaded.len(), table.entries.len()));
            continue;
        }
        for (entry, value) in table.entries.iter().zip(loaded) {
            checked += 1;
            if let Some(difference) = record_difference(&entry.value, value, &[]) {
                bad.push(format!("{}[{}]: {difference}", schema.table, entry.index));
            }
        }
    }
    eprintln!("export {} files, {} records; load {} records; checked {checked}", summary.files, summary.records, values.values().map(Vec::len).sum::<usize>());
    assert!(bad.is_empty(), "{:?}", &bad[..bad.len().min(5)]);
    assert_eq!(checked, 12815);

    std::fs::remove_dir_all(&root).ok();
}
