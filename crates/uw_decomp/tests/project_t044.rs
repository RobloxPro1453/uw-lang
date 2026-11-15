//! T-044: `uw new <dir> --from native` writes every record as UW record text and `uw new --check`
//! reads the folder back; the real-atlas test skips when the atlas is absent (like records_t042).

use std::path::{Path, PathBuf};

use uw_atlas::Atlas;
use uw_decomp::project::layout;
use uw_decomp::project::{export, verify};
use uw_decomp::records::{parse, print, Schema};

/// Two factions, three raw fields each: enough to show grouping, comments and the read-back.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t044", "ops": [], "hooks": [], "ranges": [],
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
    let dir = std::env::temp_dir().join(format!("uw-new-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn tiny() -> (Atlas, Schema) {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let schema = Schema::from_toml(TINY_SCHEMA).expect("tiny schema");
    (atlas, schema)
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

/// Copies every file of `from` into `to` (the project folders have subfolders).
fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("copy folder");
    for entry in std::fs::read_dir(from).expect("read export") {
        let entry = entry.expect("dir entry");
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).expect("copy file");
        }
    }
}

#[test]
fn export_writes_every_record_and_verify_reads_them_back() {
    let root = scratch("ok");
    let (atlas, schema) = tiny();
    let out = root.join("hikayem");
    let summary = export(&atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry))
        .expect("export writes the folder");
    assert_eq!(summary.records, 2);
    // As many files as distinct layout paths (factions: one file per record since T-045).
    let paths: std::collections::BTreeSet<String> = atlas.tables["factions"]
        .entries
        .iter()
        .map(|e| layout::path_for(&atlas, &schema, e))
        .collect();
    assert_eq!(summary.files, paths.len());

    // The record file sits where layout::path_for says; its first line names the table and the count.
    let relative = layout::path_for(&atlas, &schema, &atlas.tables["factions"].entries[0]);
    let text = std::fs::read_to_string(out.join(&relative)).expect("record file");
    let first = text.lines().next().expect("a header line");
    assert!(first.starts_with("// factions") && first.contains("2 records"), "{first}");
    // The comment line above a record carries the display name (the record's `name` field).
    assert!(text.contains("// No Faction"), "{text}");
    assert!(text.contains("faction no_faction {"), "{text}");
    assert!(text.contains("faction forest_bandits {"), "{text}");
    assert!(!text.contains('\r'), "the export writes \\n only");

    // uw.toml and the Turkish README.
    let toml = std::fs::read_to_string(out.join("uw.toml")).expect("uw.toml");
    assert!(toml.contains("base = \"native-export\""), "{toml}");
    assert!(toml.contains("kit_hash = \"t044\""), "{toml}");
    let readme = std::fs::read_to_string(out.join("README.md")).expect("README.md");
    assert!(readme.contains("uw new --check"), "{readme}");
    assert!(readme.lines().count() <= 30, "the README stays short:\n{readme}");

    // Every record comes back identical to the atlas.
    let report = verify(&atlas, &[schema.clone()], &out, &mut |schema, text| parse(schema, text))
        .expect("verify reads the folder");
    assert_eq!(report.records, 2);
    assert_eq!(report.exact, 2);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_non_empty_folder_is_refused() {
    let root = scratch("nonempty");
    let out = root.join("hikayem");
    std::fs::create_dir_all(&out).expect("existing folder");
    std::fs::write(out.join("keep.txt"), "my own work").expect("existing file");
    let (atlas, schema) = tiny();

    let error = export(&atlas, &[schema], &out, &mut |schema, entry| print(schema, entry))
        .expect_err("a non-empty folder must not be overwritten");
    assert!(error.contains("not empty"), "{error}");
    assert_eq!(std::fs::read_to_string(out.join("keep.txt")).expect("kept file"), "my own work");
    assert_eq!(uw_files(&out), Vec::<PathBuf>::new(), "nothing was written");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn verify_reports_a_record_that_no_longer_matches_the_atlas() {
    let root = scratch("changed");
    let (atlas, schema) = tiny();
    let out = root.join("hikayem");
    export(&atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry)).expect("export");
    let damaged = root.join("damaged");
    copy_tree(&out, &damaged);

    // One field's value edited: the record still parses, but it is not the atlas value any more.
    let file = uw_files(&damaged)
        .into_iter()
        .find(|path| std::fs::read_to_string(path).unwrap_or_default().contains("faction no_faction"))
        .expect("the record file");
    let text = std::fs::read_to_string(&file).expect("record file");
    let edited = text.replace("  flags 0", "  flags 9");
    assert_ne!(edited, text, "{text}");
    std::fs::write(&file, edited).expect("write the edit");

    let report = verify(&atlas, &[schema.clone()], &damaged, &mut |schema, text| parse(schema, text))
        .expect("verify reads the folder");
    assert_eq!(report.records, 2);
    assert_eq!(report.exact, 1);
    assert_eq!(report.failed, 1, "{:?}", report.first_failures);
    assert!(report.first_failures[0].contains("no_faction"), "{:?}", report.first_failures);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn verify_reports_a_missing_record() {
    let root = scratch("missing");
    let (atlas, schema) = tiny();
    let out = root.join("hikayem");
    export(&atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry)).expect("export");
    let damaged = root.join("damaged");
    copy_tree(&out, &damaged);

    let entry = &atlas.tables["factions"].entries[0];
    let file = uw_files(&damaged)
        .into_iter()
        .find(|path| std::fs::read_to_string(path).unwrap_or_default().contains("faction no_faction"))
        .expect("the record file");
    let text = std::fs::read_to_string(&file).expect("record file");
    let gone = text.replace(print(&schema, entry).trim_end(), "");
    assert_ne!(gone, text, "{text}");
    std::fs::write(&file, gone).expect("write the removal");

    let report = verify(&atlas, &[schema.clone()], &damaged, &mut |schema, text| parse(schema, text))
        .expect("verify reads the folder");
    assert_eq!(report.records, 2);
    assert_eq!(report.exact, 1);
    assert_eq!(report.failed, 1, "{:?}", report.first_failures);
    assert!(report.first_failures[0].contains("missing"), "{:?}", report.first_failures);

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn a_record_comment_carries_the_turkish_description() {
    // The kit's own menus schema, and one menu whose short Turkish description exists.
    let schema = Schema::from_path(format!("{}/../../kit-meta/records/menus.toml", env!("CARGO_MANIFEST_DIR")))
        .expect("menus schema loads");
    let atlas = Atlas::from_str(
        r#"{"schema": 1, "kit_hash": "t044", "ops": [], "hooks": [], "ranges": [],
            "tables": {"menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus",
              "id_prefix":"menu_", "entries": [
                {"index":0,"name":"arena_duel_conclusion","id":"menu_arena_duel_conclusion","dup":null,
                 "line":1,"value":["arena_duel_conclusion",0,"str","none",[],[]]}]}}}"#,
    )
    .expect("tiny atlas");
    let root = scratch("description");
    let out = root.join("hikayem");
    export(&atlas, &[schema.clone()], &out, &mut |schema, entry| print(schema, entry)).expect("export");
    let relative = layout::path_for(&atlas, &schema, &atlas.tables["menus"].entries[0]);
    let text = std::fs::read_to_string(out.join(&relative)).expect("record file");
    let prefix = "// arena_duel_conclusion — ";
    let comment = text
        .lines()
        .find(|line| line.starts_with("// arena_duel_conclusion"))
        .unwrap_or_else(|| panic!("no comment line in:\n{text}"));
    assert!(comment.starts_with(prefix) && comment.len() > prefix.len(), "{comment}");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn real_atlas_export_and_verify() {
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
    let report = verify(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("verify");
    eprintln!("{} files, {} records, exact {}, failed {}", summary.files, summary.records, report.exact, report.failed);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(report.records, 12815);
    assert_eq!(report.exact, report.records);

    std::fs::remove_dir_all(&root).ok();
}
