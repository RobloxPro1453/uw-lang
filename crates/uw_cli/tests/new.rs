//! Runs the real `uw` binary: `uw new <dir> --from native` writes a project folder and
//! `uw new --check <dir>` reads it back (T-044). The record schemas come from `kit-meta/records`.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Two factions with the fields of `kit-meta/records/factions.toml` (the last one optional).
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t044", "ops": [], "hooks": [], "ranges": [],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
      "entries": [
        {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,
          "value":["no_faction","No Faction",0,10,[],[]]},
        {"index":1,"name":"forest_bandits","id":"fac_forest_bandits","dup":null,"line":2,
          "value":["forest_bandits","Forest Bandits",1,5,[],[]]}
      ]}
  }
}"#;

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-new-cli-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

fn uw(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw")).args(args).output().expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    (out.status.success(), format!("{stdout}\n{stderr}"))
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
fn new_exports_a_folder_and_check_reads_it_back() {
    let root = scratch("ok");
    let atlas = root.join("atlas.json");
    std::fs::write(&atlas, TINY_ATLAS).expect("tiny atlas");
    let project = root.join("hikayem");
    let atlas_arg = atlas.to_str().expect("utf-8 path");
    let project_arg = project.to_str().expect("utf-8 path");

    let (ok, text) = uw(&["new", project_arg, "--from", "native", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("2 records"), "{text}");

    let toml = std::fs::read_to_string(project.join("uw.toml")).expect("uw.toml");
    assert!(toml.contains("base = \"native-export\""), "{toml}");
    assert!(toml.contains("kit_hash = \"t044\""), "{toml}");
    assert!(project.join("README.md").is_file(), "a README is written");
    assert!(!uw_files(&project).is_empty(), "the record files are written");

    let (ok, text) = uw(&["new", "--check", project_arg, "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("2 records, exact 2, failed 0"), "{text}");

    // An edited record makes the check fail and say which one.
    let file = uw_files(&project)
        .into_iter()
        .find(|path| std::fs::read_to_string(path).unwrap_or_default().contains("faction no_faction"))
        .expect("the record file");
    let records = std::fs::read_to_string(&file).expect("record file");
    std::fs::write(&file, records.replace("  coherence 10", "  coherence 11")).expect("edit");
    let (ok, text) = uw(&["new", "--check", project_arg, "--atlas", atlas_arg]);
    assert!(!ok, "{text}");
    assert!(text.contains("exact 1, failed 1"), "{text}");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn new_refuses_a_folder_that_is_not_empty() {
    let root = scratch("nonempty");
    let atlas = root.join("atlas.json");
    std::fs::write(&atlas, TINY_ATLAS).expect("tiny atlas");
    let project = root.join("hikayem");
    std::fs::create_dir_all(&project).expect("existing folder");
    std::fs::write(project.join("keep.txt"), "my own work").expect("existing file");
    let atlas_arg = atlas.to_str().expect("utf-8 path");
    let project_arg = project.to_str().expect("utf-8 path");

    let (ok, text) = uw(&["new", project_arg, "--from", "native", "--atlas", atlas_arg]);
    assert!(!ok, "{text}");
    assert!(text.contains("not empty"), "{text}");
    assert_eq!(std::fs::read_to_string(project.join("keep.txt")).expect("kept file"), "my own work");

    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn the_help_lists_the_new_commands() {
    let (ok, text) = uw(&[]);
    assert!(ok, "{text}");
    assert!(text.contains("uw new <dir> --from native"), "{text}");
    assert!(text.contains("uw new --check <dir>"), "{text}");
}
