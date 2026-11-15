//! Runs the real `uw` binary against a small atlas fixture.

use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_refs.json");

fn uw(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("explain")
        .args(["--atlas", FIXTURE])
        .args(args)
        .output()
        .expect("uw runs");
    (
        out.status.success(),
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
    )
}

#[test]
fn explains_entry_by_code_reference() {
    let (ok, out, _) = uw(&["mnu_start"]);
    assert!(ok);
    assert!(out.starts_with("menu_start  (menus #0, name \"start\")"), "{out}");
    assert!(out.contains("in code: mnu_start"), "{out}");
    assert!(out.contains("defined: module_game_menus.py:10"), "{out}");
    assert!(out.contains("used:    3 times in 3 entries (1 dialogs, 2 scripts)"), "{out}");
    assert!(out.contains("jump_to_menu"), "{out}");
}

#[test]
fn shadowed_duplicate_is_flagged_unreachable() {
    let (ok, out, _) = uw(&["menu_camp"]);
    assert!(ok);
    assert!(out.contains("unreachable, mnu_camp names the first entry with this name (#1)"), "{out}");
    let (ok, out, _) = uw(&["menu_camp#1"]);
    assert!(ok);
    assert!(out.contains("in code: mnu_camp\n"), "{out}");
    assert!(out.contains("1 times in 1 entries (1 scripts)"), "{out}");
}

#[test]
fn explains_globals_hooks_and_ops() {
    let (ok, out, _) = uw(&["$g_x"]);
    assert!(ok && out.contains("(global variable)"), "{out}");
    assert!(out.contains("used:    3 times in 2 entries (2 scripts)"), "{out}");
    assert!(out.contains("assign, eq"), "{out}");
    let (ok, out, _) = uw(&["game_start"]);
    assert!(ok && out.contains("engine hook"), "{out}");
    assert!(out.contains("used:    nowhere"), "{out}");
    let (ok, out, _) = uw(&["try_end"]);
    assert!(ok && out.starts_with("try_end  (operation, code 3)"), "{out}");
}

#[test]
fn unknown_thing_fails_with_suggestion() {
    let (ok, _, err) = uw(&["mnu_strat"]);
    assert!(!ok);
    assert!(err.contains("'mnu_strat' is not defined in Native; did you mean 'mnu_start'?"), "{err}");
}

/// The real Native atlas, when this machine has one (docs/DEVELOPMENT.md > Atlas).
const REAL: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn uw_real(args: &[&str]) -> Option<String> {
    if !std::path::Path::new(REAL).exists() {
        eprintln!("skipped: no atlas at {REAL}");
        return None;
    }
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .args(["explain", "--atlas", REAL])
        .args(args)
        .output()
        .expect("uw runs");
    assert!(out.status.success());
    Some(String::from_utf8(out.stdout).unwrap())
}

#[test]
fn real_atlas_short_form_caps_and_all_lists_everything() {
    let Some(short) = uw_real(&["trp_player"]) else { return };
    assert!(short.contains("more (uw explain --all)"), "{short}");
    let all = uw_real(&["--all", "trp_player"]).unwrap();
    assert!(!all.contains("more (uw explain --all)"));
    assert!(all.lines().count() > short.lines().count());
    // Source check (module_dialogs.py): (eq, "$g_next_menu", "mnu_castle_taken").
    let menu = uw_real(&["mnu_castle_taken"]).unwrap();
    assert!(menu.contains("dialogs #5"), "{menu}");
}
