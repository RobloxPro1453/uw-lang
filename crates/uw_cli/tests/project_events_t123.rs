//! T-123: a project folder made by `uw new` (it has `uw.lock`) builds the new language in its
//! `events/*.uw` files — `dialog`, `every` … — through the same uw_check path as patch mode, and a
//! project without items builds exactly as before. The test drives the real `uw` binary on a tiny
//! atlas, a copy of the three record schemas and a fake kit, so the overlay can be read after the
//! (deliberately missing) Python 2.7 step.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// One troop (so `dialog` has a partner), and the two tables the patches append to.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t123", "ops": [], "hooks": [], "ranges": [],
  "constants": [{"name":"tf_male","value":0,"module":"header_troops","line":25}],
  "tables": {
    "troops": {"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_","ref_tag":"trp",
      "entries": [
        {"index":0,"name":"swadian_recruit","id":"trp_swadian_recruit","dup":null,"line":1,
          "value":["swadian_recruit","Swadian Recruit","Swadian Recruits",0,0,0,0,{"list":[]},0,0,0,"face_1"]}
      ]},
    "dialogs": {"module":"module_dialogs","var":"dialogs","id_file":"ID_dialogs","id_prefix":"dlg_","ref_tag":"dlg",
      "entries": []},
    "simple_triggers": {"module":"module_simple_triggers","var":"simple_triggers","id_file":"ID_simple_triggers",
      "id_prefix":"st_","ref_tag":"st","entries": []}
  }
}"#;

/// An item file a user writes in the project folder: one dialog and one daily trigger.
const ITEMS: &str = r#"// T-123: the new language in a project folder.
dialog swadian_recruit first_meeting {
    recruit: "T123 hello" -> end
}

every day {
}
"#;

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-t123-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

/// The record schemas `uw` finds through `kit-meta/records` in the current folder's parents.
fn copy_schemas(root: &Path) {
    let from = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/records");
    let to = root.join("kit-meta").join("records");
    std::fs::create_dir_all(&to).expect("schema folder");
    for file in ["troops.toml", "dialogs.toml", "simple_triggers.toml"] {
        std::fs::copy(from.join(file), to.join(file)).unwrap_or_else(|e| panic!("{}: {e}", from.join(file).display()));
    }
}

/// A kit the build can find and stage; its one step cannot run, which is where the build stops.
fn fake_kit(root: &Path) {
    let kit = root.join("Module_system 1.171");
    std::fs::create_dir_all(&kit).expect("kit folder");
    std::fs::write(kit.join("build_module.bat"), "python.exe process_init.py\r\n").expect("batch file");
    for module in ["module_troops", "module_dialogs", "module_simple_triggers"] {
        std::fs::write(kit.join(format!("{module}.py")), "# native\n").expect("kit module");
    }
}

fn uw(cwd: &Path, atlas: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .args(args)
        .current_dir(cwd)
        .env("UW_ATLAS", atlas)
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    (out.status.success(), format!("{stdout}\n{stderr}"))
}

/// The overlay the build wrote, by file name.
fn overlay(root: &Path, project: &str) -> BTreeMap<String, String> {
    let dir = root.join("target").join("uw-build").join(project).join("overlay");
    let mut files = BTreeMap::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())).flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let text = std::fs::read_to_string(entry.path()).expect("overlay file");
        files.insert(name, text);
    }
    files
}

#[test]
fn a_project_builds_its_events_and_a_project_without_them_is_unchanged() {
    let root = scratch("events");
    copy_schemas(&root);
    fake_kit(&root);
    let atlas = root.join("atlas.json");
    std::fs::write(&atlas, TINY_ATLAS).expect("tiny atlas");
    let project = root.join("t123proj");
    let atlas_arg = atlas.to_str().expect("utf-8 path");
    let project_arg = project.to_str().expect("utf-8 path");

    let (ok, text) = uw(&root, &atlas, &["new", project_arg, "--from", "native", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(project.join("uw.lock").is_file(), "a project folder has uw.lock");

    // Before this change a project without items wrote exactly the tables: no runtime helper, no
    // patch section. That is the output the change must keep.
    let (_, text) = uw(&root, &atlas, &["build", project_arg, "--no-install"]);
    assert!(text.contains("modules  3 written"), "{text}");
    let before = overlay(&root, "t123proj");
    assert!(!before.contains_key("uw_runtime.py"), "a project without items needs no runtime: {before:?}");
    let empty_table = "# -*- coding: latin-1 -*-\n# written by uw (project build, P3b); do not edit\ndialogs = [\n]\n";
    let expected = format!("from native_module_dialogs import *\n{empty_table}");
    assert_eq!(before.get("module_dialogs.py").map(String::as_str), Some(expected.as_str()));
    assert!(before["module_simple_triggers.py"].ends_with("simple_triggers = [\n]\n"), "{before:?}");
    assert!(before["module_troops.py"].contains("swadian_recruit"), "{before:?}");

    // The same project with one dialog and one trigger: both must reach the overlay.
    std::fs::create_dir_all(project.join("events")).expect("events folder");
    std::fs::write(project.join("events").join("main.uw"), ITEMS).expect("events file");
    let (_, text) = uw(&root, &atlas, &["build", project_arg, "--no-install"]);
    let after = overlay(&root, "t123proj");
    let dialogs = after.get("module_dialogs.py").expect("module_dialogs.py written");
    let dialogs = dialogs.as_str();
    assert!(dialogs.contains("T123 hello"), "the dialog line is in module_dialogs.py:\n{dialogs}\n{text}");
    assert!(dialogs.contains("_uw.insert_before_state(dialogs"), "the dialog is a patch:\n{dialogs}");
    assert!(dialogs.starts_with(&before["module_dialogs.py"]), "the whole-table part is unchanged:\n{dialogs}");
    let triggers = after.get("module_simple_triggers.py").expect("module_simple_triggers.py written");
    let triggers = triggers.as_str();
    assert!(triggers.contains("_uw.append_entry(simple_triggers"), "the `every` is a new trigger:\n{triggers}");
    assert!(triggers.contains("24,\n    [],"), "`every day` is a 24 hour interval:\n{triggers}");
    assert!(triggers.starts_with(&before["module_simple_triggers.py"]), "the whole-table part is unchanged:\n{triggers}");
    assert!(after.contains_key("uw_runtime.py"), "the patches import uw_runtime.py: {after:?}");

    let _ = std::fs::remove_dir_all(&root);
}
