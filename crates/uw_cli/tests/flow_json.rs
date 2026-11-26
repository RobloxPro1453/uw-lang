//! `uw graph --json`: the uw-flow/1 file graph_viewer reads.

use serde_json::Value;
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_flow.json");

fn export(name: &str, extra: &[&str]) -> (bool, Value, String) {
    let out_path = std::env::temp_dir().join(format!("uw_flow_test_{}_{name}.json", std::process::id()));
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .args(["graph", "--atlas", FIXTURE, "--json"])
        .arg(&out_path)
        .args(extra)
        .output()
        .expect("uw runs");
    let stderr = String::from_utf8(out.stderr).unwrap();
    if !out.status.success() {
        return (false, Value::Null, stderr);
    }
    let text = std::fs::read_to_string(&out_path).expect("file written");
    let _ = std::fs::remove_file(&out_path);
    (true, serde_json::from_str(&text).expect("valid JSON"), stderr)
}

fn node<'a>(doc: &'a Value, id: &str) -> &'a Value {
    doc["nodes"].as_array().unwrap().iter().find(|n| n["id"] == id).unwrap_or_else(|| panic!("no node {id}"))
}

#[test]
fn every_field_the_schema_requires_is_present() {
    let (ok, doc, err) = export("fields", &[]);
    assert!(ok, "{err}");
    assert_eq!(doc["format"], "uw-flow/1");
    assert_eq!(doc["kit_hash"], "flow");
    for n in doc["nodes"].as_array().unwrap() {
        for key in ["id", "kind", "index", "text", "source", "on_enter_goes_to", "options", "stub"] {
            assert!(n.get(key).is_some(), "node {} lacks {key}", n["id"]);
        }
        for o in n["options"].as_array().unwrap() {
            for key in ["index", "id", "text", "conditional", "goes_to"] {
                assert!(o.get(key).is_some(), "option lacks {key}");
            }
        }
    }
    for e in doc["edges"].as_array().unwrap() {
        for key in ["from", "to", "op", "via_option", "option_index"] {
            assert!(e.get(key).is_some(), "edge lacks {key}");
        }
    }
}

#[test]
fn options_keep_order_and_repeated_ids() {
    let (_, doc, _) = export("options", &[]);
    let start = node(&doc, "menu_start");
    assert_eq!(start["text"], "Start here^{s10}");
    assert_eq!(start["source"], "module_game_menus.py:10");
    let opts = start["options"].as_array().unwrap();
    let got: Vec<(i64, &str, bool)> = opts
        .iter()
        .map(|o| (o["index"].as_i64().unwrap(), o["id"].as_str().unwrap(), o["conditional"].as_bool().unwrap()))
        .collect();
    assert_eq!(got, [(0, "mno_go", false), (1, "mno_cheat", true), (2, "mno_go", false)]);
    // the cheat button runs a script, then opens the town
    assert_eq!(opts[1]["goes_to"][0]["op"], "call_script");
    assert_eq!(opts[1]["goes_to"][0]["to"], "script_give");
    assert_eq!(opts[1]["goes_to"][1]["to"], "menu_town");
    // a malformed option (fewer than 4 fields) is skipped, not fatal
    assert_eq!(node(&doc, "menu_town")["options"].as_array().unwrap().len(), 1);
}

#[test]
fn every_arrow_ends_on_a_node_and_matches_goes_to() {
    let (_, doc, _) = export("edges", &[]);
    let ids: Vec<&str> = doc["nodes"].as_array().unwrap().iter().map(|n| n["id"].as_str().unwrap()).collect();
    let mut seen = std::collections::HashSet::new();
    assert!(ids.iter().all(|id| seen.insert(*id)), "node ids are unique");
    let edges = doc["edges"].as_array().unwrap();
    for e in edges {
        assert!(ids.contains(&e["from"].as_str().unwrap()) && ids.contains(&e["to"].as_str().unwrap()), "{e}");
    }
    // opening code of town goes to the world map; screens and scripts are stubs
    let town = node(&doc, "menu_town");
    assert_eq!(town["on_enter_goes_to"][0]["to"], "screen:map");
    let opening: Vec<&Value> = edges.iter().filter(|e| e["from"] == "menu_town" && e["via_option"].is_null()).collect();
    assert_eq!(opening.len(), 1);
    assert!(opening[0]["option_index"].is_null());
    let map = node(&doc, "screen:map");
    assert_eq!((map["kind"].as_str(), map["stub"].as_bool(), map["index"].as_i64()), (Some("screen"), Some(true), Some(-1)));
    assert_eq!(node(&doc, "script_give")["kind"], "script");
    // the second "go" button's arrow names option 2, not option 0
    assert!(edges.iter().any(|e| e["via_option"] == "mno_go" && e["option_index"] == 2));
    // an unknown menu name (mnu_nowhere) gives no arrow rather than a guessed one
    assert!(edges.iter().all(|e| e["to"] != "menu_nowhere"));
    assert_eq!(edges.len(), 6);
}

#[test]
fn buttons_say_what_they_do_and_when_they_show() {
    let (_, doc, _) = export("does", &[]);
    let start = node(&doc, "menu_start");
    let cheat = &start["options"][1];
    // shown only when (eq, 1, 0) holds
    assert_eq!(cheat["condition"][0]["op"], "eq");
    assert_eq!(cheat["condition"][0]["args"], serde_json::json!(["1", "0"]));
    assert_eq!(cheat["condition"][0]["not"], false);
    // does: run the script, then open the town, in order
    let does: Vec<&str> = cheat["does"].as_array().unwrap().iter().map(|s| s["op"].as_str().unwrap()).collect();
    assert_eq!(does, ["call_script", "jump_to_menu"]);
    assert_eq!(cheat["does"][0]["args"][0], "script_give");
    // a register written in the kit's encoded form is shown by its name
    assert_eq!(cheat["does"][0]["args"][1], "reg3");
    // a button without a condition has an empty list, never a missing field
    assert_eq!(start["options"][0]["condition"], serde_json::json!([]));
    let town = node(&doc, "menu_town");
    assert_eq!(town["on_enter_does"][0]["op"], "change_screen_map");
    for n in doc["nodes"].as_array().unwrap() {
        for key in ["summary_tr", "detail_tr", "confidence"] {
            assert!(n[key].is_string(), "{} lacks {key}", n["id"]);
        }
        assert!(n["on_enter_does"].is_array());
    }
}

#[test]
fn roots_default_to_the_game_starts_and_accept_a_bare_menu_name() {
    let (_, doc, _) = export("roots", &[]);
    let roots: Vec<&str> = doc["roots"].as_array().unwrap().iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert_eq!(roots, ["menu_start", "menu_start_phase_2"]);
    let (ok, doc, err) = export("bare", &["town"]);
    assert!(ok, "{err}");
    assert_eq!(doc["roots"][0]["id"], "menu_town");
    assert_eq!(doc["roots"].as_array().unwrap().len(), 1);
    // a script that a button runs is on the flow (as a stub node), so it may be a start too
    let (ok, doc, err) = export("script", &["script_give"]);
    assert!(ok, "{err}");
    assert_eq!(doc["roots"][0]["id"], "script_give");
    let (ok, _, err) = export("unknown", &["nothing_here"]);
    assert!(!ok);
    assert!(err.contains("no single thing"), "{err}");
}
