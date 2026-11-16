//! Runs the real `uw` binary: `uw rename` finds a record's name and every text that mentions it
//! and rewrites them (T-094). The record schemas come from `kit-meta/records`.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// One town, `p_town_6` at index 0, with the tags and ranges the record text needs.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t094", "ops": [], "hooks": [], "constants": [],
  "ranges": [
    {"name": "towns_begin", "value": "p_town_6", "line": 1552},
    {"name": "towns_end", "value": 1, "line": 1556}
  ],
  "tables": {
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
      "ref_tag":"p",
      "entries": [
        {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":1,
          "value":["town_6","Praven",0]}
      ]}
  }
}"#;

/// The project's record, menu and translation files, with their numbers of lines.
const OWN_FILE: &str = "// Praven — the town of the north
party praven {
  id \"town_6\"
  name \"Praven\"
  x 0
}
";
const MENU_FILE: &str = "// Camp menu; the caravan to Praven stops here
menu mnu_camp {
  option \"Join a caravan to Praven.\" 0
  option \"Pravenfall is elsewhere\" 0
}
";
const TR_PARTIES: &str = "p_town_6|Praven Şehri
p_town_5|Uxkhal
";
const TR_MENUS: &str = "mnu_camp_option|Praven'e katıl
";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-rename-cli-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

/// A project folder with the three kinds of text plus `names.uw`, and the atlas next to it.
fn project(name: &str) -> (PathBuf, PathBuf) {
    let root = scratch(name);
    let project = root.join("hikayem");
    std::fs::create_dir_all(project.join("parties").join("towns")).expect("record folder");
    std::fs::create_dir_all(project.join("menus")).expect("menu folder");
    std::fs::create_dir_all(project.join("languages").join("tr")).expect("language folder");
    std::fs::write(project.join("names.uw"), "// names\npraven = p_town_6\n").expect("names");
    std::fs::write(project.join("parties").join("towns").join("town_6.uw"), OWN_FILE).expect("own file");
    std::fs::write(project.join("menus").join("mnu_camp.uw"), MENU_FILE).expect("menu file");
    std::fs::write(project.join("languages").join("tr").join("parties.csv"), TR_PARTIES).expect("tr");
    std::fs::write(project.join("languages").join("tr").join("menus.csv"), TR_MENUS).expect("tr menus");
    let atlas = root.join("atlas.json");
    std::fs::write(&atlas, TINY_ATLAS).expect("tiny atlas");
    (project, atlas)
}

/// Runs `uw` with this folder as the working folder and no stdin (so a prompt reads "no").
fn uw_in(dir: Option<&Path>, args: &[&str]) -> (bool, String) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_uw"));
    if let Some(dir) = dir {
        command.current_dir(dir);
    }
    let out = command
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    (out.status.success(), format!("{stdout}\n{stderr}"))
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).expect("file")
}

#[test]
fn rename_lists_and_then_rewrites_the_name_and_every_mention() {
    let (project, atlas) = project("all");
    let project_arg = project.to_str().expect("utf-8 path");
    let atlas_arg = atlas.to_str().expect("utf-8 path");
    let own = project.join("parties").join("towns").join("town_6.uw");
    let menu = project.join("menus").join("mnu_camp.uw");
    let tr_parties = project.join("languages").join("tr").join("parties.csv");
    let tr_menus = project.join("languages").join("tr").join("menus.csv");

    let (ok, text) = uw_in(None, &["rename", project_arg, "praven", "Adam Siken", "--list", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("own name"), "{text}");
    assert!(text.contains("parties/towns/town_6.uw:4  name \"Praven\""), "{text}");
    assert!(text.contains("other English texts"), "{text}");
    assert!(text.contains("menus/mnu_camp.uw:3  option \"Join a caravan to Praven.\" 0"), "{text}");
    assert!(text.contains("translations"), "{text}");
    assert!(text.contains("languages/tr/parties.csv:1  p_town_6|Praven Şehri"), "{text}");
    assert!(text.contains("languages/tr/menus.csv:1  mnu_camp_option|Praven'e katıl"), "{text}");
    assert!(text.contains("--list: nothing written"), "{text}");
    // Nothing is written by --list.
    assert_eq!(read(&own), OWN_FILE);
    assert_eq!(read(&tr_parties), TR_PARTIES);

    // Without --yes and no answer on stdin, nothing is written either.
    let (ok, text) = uw_in(None, &["rename", project_arg, "praven", "Adam Siken", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("[y/N]"), "{text}");
    assert_eq!(read(&own), OWN_FILE);

    let (ok, text) = uw_in(None, &["rename", project_arg, "praven", "Adam Siken", "--yes", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("wrote"), "{text}");
    // Only the old name changed: the comment, the header, the game name and `Pravenfall` stay.
    assert_eq!(
        read(&own),
        "// Praven — the town of the north
party praven {
  id \"town_6\"
  name \"Adam Siken\"
  x 0
}
"
    );
    assert_eq!(
        read(&menu),
        "// Camp menu; the caravan to Praven stops here
menu mnu_camp {
  option \"Join a caravan to Adam Siken.\" 0
  option \"Pravenfall is elsewhere\" 0
}
"
    );
    assert_eq!(read(&tr_parties), "p_town_6|Adam Siken Şehri\np_town_5|Uxkhal\n");
    assert_eq!(read(&tr_menus), "mnu_camp_option|Adam Siken'e katıl\n");
    assert_eq!(read(&project.join("names.uw")), "// names\npraven = p_town_6\n", "the game name stays");
    let _ = std::fs::remove_dir_all(project.parent().unwrap());
}

#[test]
fn rename_works_from_inside_the_project_folder() {
    let (project, atlas) = project("inside");
    let atlas_arg = atlas.to_str().expect("utf-8 path");
    let (ok, text) = uw_in(Some(&project), &["rename", "praven", "Adam Siken", "--list", "--atlas", atlas_arg]);
    assert!(ok, "{text}");
    assert!(text.contains("parties/towns/town_6.uw:4"), "{text}");
    let _ = std::fs::remove_dir_all(project.parent().unwrap());
}
