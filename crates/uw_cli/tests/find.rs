//! Runs the real `uw` binary: `uw find "<text>"` lists every project text that contains a name
//! (T-105): whole word by default, `--part` inside a word, `--json` for the editor, and the
//! translations under `languages/`.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// A comment that mentions the old name is not a text; the record name field is.
const PARTY_FILE: &str = "// the old name Praven stays in this comment only
party praven {
  id \"town_1\"
  name \"Praven\"
  x 0
}
";
const MENU_FILE: &str = "// the start menu
menu mnu_start {
  id \"start_phase_2\"
  text \"Join a caravan to Praven, in the Kingdom of Swadia\"
}
";
const TROOP_FILE: &str = "troop merchant_town_1 {
  id \"merchant_town_1\"
  name \"Merchant of Praven\"
}
";
const TR_PARTIES: &str = "p_town_1|Praven Şehri
p_town_2|Pravenia
";

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-find-cli-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

/// A project folder with the three record files and, when asked, the Turkish translations.
fn project(name: &str, languages: bool) -> PathBuf {
    let root = scratch(name);
    let project = root.join("hikayem");
    std::fs::create_dir_all(project.join("parties").join("towns")).expect("record folder");
    std::fs::create_dir_all(project.join("menus")).expect("menu folder");
    std::fs::create_dir_all(project.join("troops")).expect("troop folder");
    std::fs::write(project.join("parties").join("towns").join("town_1.uw"), PARTY_FILE).expect("own file");
    std::fs::write(project.join("menus").join("mnu_start.uw"), MENU_FILE).expect("menu file");
    std::fs::write(project.join("troops").join("soldiers.uw"), TROOP_FILE).expect("troop file");
    if languages {
        std::fs::create_dir_all(project.join("languages").join("tr")).expect("language folder");
        std::fs::write(project.join("languages").join("tr").join("parties.csv"), TR_PARTIES).expect("tr");
    }
    project
}

/// Runs `uw` with this folder as the working folder and no stdin.
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

#[test]
fn find_lists_every_text_with_the_whole_word_and_no_longer_one() {
    let project = project("words", true);
    let arg = project.to_str().expect("utf-8 path");
    let (ok, text) = uw_in(None, &["find", "praven", arg]);
    assert!(ok, "{text}");
    // The record's own name, with its kind and id.
    assert!(text.contains("parties/towns/town_1.uw:4"), "{text}");
    assert!(text.contains("party town_1"), "{text}");
    assert!(text.contains("\"Praven\""), "{text}");
    // A menu text, with its field.
    assert!(text.contains("menus/mnu_start.uw:4"), "{text}");
    assert!(text.contains("menu start_phase_2"), "{text}");
    assert!(text.contains("Join a caravan to Praven, in the Kingdom of Swadia"), "{text}");
    // A troop text.
    assert!(text.contains("troops/soldiers.uw:3"), "{text}");
    assert!(text.contains("troop merchant_town_1"), "{text}");
    assert!(text.contains("Merchant of Praven"), "{text}");
    // The translation, but not "Pravenia": it is not the whole word.
    assert!(text.contains("languages/tr/parties.csv:1"), "{text}");
    assert!(text.contains("\"Praven Şehri\""), "{text}");
    assert!(!text.contains("parties.csv:2"), "{text}");
    assert!(!text.contains("Pravenia"), "{text}");
    // A comment mentioning the name is not a project text.
    assert!(!text.contains("comment only"), "{text}");
    assert!(text.contains("3 + 1 yer bulundu."), "{text}");
}

#[test]
fn part_also_finds_the_name_inside_a_longer_word() {
    let project = project("part", true);
    let arg = project.to_str().expect("utf-8 path");
    let (ok, text) = uw_in(None, &["find", "Praven", arg, "--part"]);
    assert!(ok, "{text}");
    assert!(text.contains("languages/tr/parties.csv:2"), "{text}");
    assert!(text.contains("\"Pravenia\""), "{text}");
    assert!(text.contains("3 + 2 yer bulundu."), "{text}");
}

#[test]
fn json_prints_one_object_per_hit_and_no_summary() {
    let project = project("json", true);
    let arg = project.to_str().expect("utf-8 path");
    let (ok, text) = uw_in(None, &["find", "Praven", arg, "--json"]);
    assert!(ok, "{text}");
    assert!(!text.contains("yer bulundu"), "{text}");
    let rows: Vec<serde_json::Value> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).unwrap_or_else(|e| panic!("{line} is not JSON: {e}")))
        .collect();
    assert_eq!(rows.len(), 4, "{text}");
    let own = rows.iter().find(|row| row["record"] == "party town_1").expect("own name row");
    assert_eq!(own["file"], "parties/towns/town_1.uw");
    assert_eq!(own["line"], 4);
    assert_eq!(own["field"], "name");
    assert_eq!(own["text"], "Praven");
    let menu = rows.iter().find(|row| row["record"] == "menu start_phase_2").expect("menu row");
    assert_eq!(menu["field"], "text");
    let tr = rows.iter().find(|row| row["record"] == "p_town_1").expect("translation row");
    assert_eq!(tr["file"], "languages/tr/parties.csv");
    assert_eq!(tr["line"], 1);
    assert_eq!(tr["field"], "text");
    assert_eq!(tr["text"], "Praven Şehri");
}

#[test]
fn without_languages_it_searches_this_folder_and_counts_only_uw_texts() {
    let project = project("nolangs", false);
    let (ok, text) = uw_in(Some(&project), &["find", "Praven"]);
    assert!(ok, "{text}");
    assert!(text.contains("3 yer bulundu."), "{text}");
    assert!(!text.contains("languages/"), "{text}");
}
