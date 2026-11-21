//! T-120: `uw coverage` — the cells of the Module System inventory a golden test proves, the tags
//! that have no proof behind them, and the cell totals read from the atlas (never typed in here).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_coverage.json");

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-coverage-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir
}

/// The gold list of the test: a header line and the given cells.
fn gold_list(dir: &Path, cells: &[&str]) -> PathBuf {
    let path = dir.join("gold.txt");
    let mut text = String::from("# the owner's in-game proofs\n");
    for cell in cells {
        text.push_str(cell);
        text.push('\n');
    }
    std::fs::write(&path, text).expect("gold list");
    path
}

/// The gold list of the test: a header line and nothing else.
fn empty_gold(dir: &Path) -> PathBuf {
    gold_list(dir, &[])
}

fn coverage(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("coverage")
        .args(args)
        .args(["--atlas", FIXTURE])
        .output()
        .expect("uw runs");
    (out.status.success(), String::from_utf8(out.stdout).expect("utf-8 stdout"))
}

/// One golden test folder: its `input.uw` and the one file its `expected/` holds.
fn golden(case: &Path, name: &str, input: &str, expected: &str) {
    let dir = case.join(name);
    std::fs::create_dir_all(dir.join("expected")).expect("expected folder");
    std::fs::write(dir.join("input.uw"), input).expect("input.uw");
    std::fs::write(dir.join("expected").join(format!("{expected}.py")), "# generated\n").expect("expected file");
}

#[test]
fn a_tag_without_its_table_in_the_expected_output_does_not_count() {
    let dir = scratch("tables");
    let case = dir.join("golden");
    // Proven: the tag says a menu is created and the expected output is module_game_menus.py.
    golden(&case, "good_menu", "mod coverage\n\n// covers: menus.new\nnew menu start2 {\n    text \"@Hi\"\n}\n", "module_game_menus");
    // A lie: the tag says the menu is overridden, the expected output is another module.
    golden(&case, "bad_menu", "mod coverage\n\n// covers: menus.override\n", "module_scripts");

    // Gold: a cell the owner lists counts only when a golden test proves it too.
    let gold = gold_list(&dir, &["menus.new", "menus.override"]);
    let (ok, out) = coverage(&["--json", "--golden", &case.display().to_string(), "--gold", &gold.display().to_string()]);
    assert!(ok, "coverage must succeed:\n{out}");
    let json: serde_json::Value = serde_json::from_str(out.trim()).expect("one JSON object");

    assert_eq!(json["tables"]["filled"], 1, "{json}");
    assert_eq!(json["hooks"]["filled"], 0, "{json}");
    assert_eq!(json["ops"]["filled"], 0, "{json}");
    assert_eq!(json["tables"]["gold"], 1, "menus.override has no proof, so only menus.new is gold: {json}");
    assert_eq!(json["gold_cells"], serde_json::json!(["menus.new"]), "{json}");
    let unproven = json["tags_without_proof"].as_array().expect("tags_without_proof");
    assert_eq!(unproven.len(), 1, "{json}");
    assert_eq!(unproven[0]["test"], "bad_menu");
    assert_eq!(unproven[0]["cell"], "menus.override");

    let empty: Vec<&str> = json["empty"].as_array().expect("empty").iter().filter_map(|cell| cell.as_str()).collect();
    assert!(empty.contains(&"menus.like"), "{json}");
    assert!(!empty.iter().any(|cell| cell.starts_with("hook.")), "the table grid alone without --all: {json}");

    // The plain output names the same things.
    let (ok, text) = coverage(&["--golden", &case.display().to_string(), "--gold", &gold.display().to_string()]);
    assert!(ok, "coverage must succeed:\n{text}");
    assert!(text.contains("tag without proof: 1"), "{text}");
    assert!(text.contains("bad_menu: menus.override"), "{text}");

    // `--all` adds the empty hook and op cells to the list.
    let (ok, out) = coverage(&[
        "--json",
        "--all",
        "--golden",
        &case.display().to_string(),
        "--gold",
        &gold.display().to_string(),
    ]);
    assert!(ok, "coverage must succeed:\n{out}");
    let json: serde_json::Value = serde_json::from_str(out.trim()).expect("one JSON object");
    let empty: Vec<&str> = json["empty"].as_array().expect("empty").iter().filter_map(|cell| cell.as_str()).collect();
    assert!(empty.contains(&"hook.ti_on_game_start"), "{json}");
    assert!(empty.contains(&"op.assign"), "{json}");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_cell_totals_are_the_counts_of_the_atlas() {
    let text = std::fs::read_to_string(FIXTURE).expect("the coverage atlas fixture");
    let atlas: serde_json::Value = serde_json::from_str(&text).expect("the fixture is JSON");
    let tables = atlas["tables"].as_object().expect("tables").len() * 5;
    let hooks = atlas["constants"]
        .as_array()
        .expect("constants")
        .iter()
        .filter_map(|constant| constant["name"].as_str())
        .filter(|name| name.starts_with("ti_") && *name != "ti_once" && *name != "ti_never")
        .collect::<BTreeSet<&str>>()
        .len();
    let ops = atlas["ops"]
        .as_array()
        .expect("ops")
        .iter()
        .filter_map(|op| op["name"].as_str())
        .collect::<BTreeSet<&str>>()
        .len();
    assert!(tables > 0 && hooks > 0 && ops > 0, "the fixture must hold every grid");

    let dir = scratch("totals");
    let case = dir.join("golden");
    std::fs::create_dir_all(&case).expect("golden folder");
    let gold = empty_gold(&dir);
    let (ok, out) = coverage(&["--json", "--golden", &case.display().to_string(), "--gold", &gold.display().to_string()]);
    assert!(ok, "coverage must succeed:\n{out}");
    let json: serde_json::Value = serde_json::from_str(out.trim()).expect("one JSON object");

    assert_eq!(json["tables"]["total"], tables, "{json}");
    assert_eq!(json["hooks"]["total"], hooks, "{json}");
    assert_eq!(json["ops"]["total"], ops, "{json}");

    std::fs::remove_dir_all(&dir).ok();
}
