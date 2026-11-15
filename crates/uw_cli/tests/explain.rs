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
    assert!(out.starts_with("menu_start  ·  \"Hi\"
"), "{out}");
    assert!(out.contains("what:     menu (menus #0)"), "{out}");
    assert!(out.contains("written in code as \"mnu_start\""), "{out}");
    assert!(out.contains("defined:  module_game_menus.py:10"), "{out}");
    assert!(out.contains("used by:  3 places (3 mentions)"), "{out}");
    // Groups, biggest first: scripts (2) before dialogs (1).
    let scripts = out.find("scripts (2)").expect("scripts group");
    let dialogs = out.find("dialogs (1)").expect("dialogs group");
    assert!(scripts < dialogs, "{out}");
    assert!(out.contains("jump_to_menu"), "{out}");
}

#[test]
fn leads_to_lists_what_a_thing_mentions() {
    // The other direction: script_game_start jumps to a menu and calls a helper.
    let (ok, out, _) = uw(&["script_game_start"]);
    assert!(ok, "{out}");
    let leads = &out[out.find("leads to:").expect("leads to line")..];
    assert!(leads.starts_with("leads to: 3 places"), "{out}");
    assert!(leads.contains("menu_camp#1"), "{out}");
    assert!(leads.contains("module_game_menus.py:20"), "{out}");
    assert!(leads.contains("script_helper"), "{out}");
    // A menu that mentions nothing has no "leads to" part.
    let (ok, out, _) = uw(&["menu_camp"]);
    assert!(ok);
    assert!(!out.contains("leads to"), "{out}");
}

#[test]
fn shadowed_duplicate_is_flagged_unreachable() {
    let (ok, out, _) = uw(&["menu_camp"]);
    assert!(ok);
    assert!(out.contains("unreachable from code: \"mnu_camp\" means the first entry with this name (#1)"), "{out}");
    let (ok, out, _) = uw(&["menu_camp#1"]);
    assert!(ok);
    assert!(out.contains("written in code as \"mnu_camp\""), "{out}");
    assert!(out.contains("used by:  1 place (1 mentions)"), "{out}");
}

#[test]
fn explains_globals_hooks_and_ops() {
    let (ok, out, _) = uw(&["$g_x"]);
    assert!(ok && out.contains("what:     global variable"), "{out}");
    assert!(out.contains("used by:  2 places (3 mentions)"), "{out}");
    assert!(out.contains("assign, eq"), "{out}");
    let (ok, out, _) = uw(&["game_start"]);
    assert!(ok && out.contains("engine hook"), "{out}");
    assert!(out.contains("used by:  nothing in Native"), "{out}");
    let (ok, out, _) = uw(&["try_end"]);
    assert!(ok && out.contains("what:     operation (code 3)"), "{out}");
}

#[test]
fn in_game_name_and_partial_names_work() {
    // "B" is the in-game text of exactly one menu.
    let (ok, out, _) = uw(&["b"]);
    assert!(ok && out.starts_with("menu_camp  ·  \"B\""), "{out}");
    // A part of a name lists what matches, best first.
    let (ok, out, _) = uw(&["cam"]);
    assert!(ok, "{out}");
    assert!(out.starts_with("'cam' is not an exact name. Things that match (2):"), "{out}");
    assert!(out.contains("menu_camp#1"), "{out}");
}

#[test]
fn unknown_thing_fails_with_suggestion() {
    let (ok, _, err) = uw(&["mnu_strat"]);
    assert!(!ok);
    assert!(err.contains("'mnu_strat' is not in Native, and no name contains it; did you mean"), "{err}");
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
    assert!(short.contains("… 383 more"), "{short}");
    let all = uw_real(&["--all", "trp_player"]).unwrap();
    assert!(!all.contains(" more
"));
    assert!(all.lines().count() > short.lines().count());
    // Source check (module_dialogs.py): (eq, "$g_next_menu", "mnu_castle_taken").
    let menu = uw_real(&["mnu_castle_taken"]).unwrap();
    assert!(menu.contains("dlga_award_fief_to_vassal_2:close_window"), "{menu}");
    // In-game name and word search on the real data.
    let praven = uw_real(&["Praven"]).unwrap();
    assert!(praven.starts_with("p_town_6  ·  \"Praven\""), "{praven}");
    let merchant = uw_real(&["merchant_quest"]).unwrap();
    assert!(merchant.contains("Things that match"), "{merchant}");
}
