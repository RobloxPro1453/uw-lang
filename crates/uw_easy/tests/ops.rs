//! T-130: the easy vocabulary of D3 (rules R1–R12, N1, R13, R14) on a small fixture atlas and on
//! the real kit. The real part uses `target/uw-staging/atlas.json`; when this lane cannot dump the
//! atlas it reads the same facts out of the kit header (`target/ref/kit/header_operations.py`), and
//! skips with a note when neither is there.

use std::path::PathBuf;

use uw_atlas::Atlas;
use uw_easy::ops::{Kind, Record, Report};

const FIXTURE: &str = r#"{
 "schema": 1,
 "kit_hash": "fixture",
 "ops": [
  {"name": "agent_get_position", "code": 1, "line": 770, "signature": "(agent_get_position,<position_no>,<agent_id>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "agent_set_speed_modifier", "code": 2, "line": 1128, "signature": "(agent_set_speed_modifier, <agent_id>, <value>),", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "key_is_down", "code": 3, "line": 70, "signature": "(key_is_down, <key_id>),", "lhs": false, "can_fail": true, "global_lhs": false},
  {"name": "store_agent_hit_points", "code": 4, "line": 782, "signature": "stores agent hit points in reg0", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "faction_get_color", "code": 5, "line": 546, "signature": "(faction_get_color, <color>, <faction_id>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "party_get_slot", "code": 6, "line": 900, "signature": "(party_get_slot,<destination>,<party_id>,<slot_no>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "party_set_slot", "code": 7, "line": 901, "signature": "(party_set_slot,<party_id>,<slot_no>,<value>),", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "ge", "code": 30, "line": 40, "signature": "greater than or equal to -- (ge,<value>,<value>),", "lhs": false, "can_fail": true, "global_lhs": false},
  {"name": "try_begin", "code": 22, "line": 17, "signature": "(try_begin),", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "str_store_troop_name", "code": 8, "line": 480, "signature": "(str_store_troop_name,<string_no>,<troop_id>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "display_message", "code": 9, "line": 105, "signature": "(display_message, <string_id>, [<value>]),", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "add_xp_to_troop", "code": 10, "line": 620, "signature": "(add_xp_to_troop,<value>,<troop_id>),", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "troop_remove_items", "code": 11, "line": 622, "signature": "puts cost of items to reg0", "lhs": false, "can_fail": false, "global_lhs": false},
  {"name": "scene_prop_get_slot", "code": 12, "line": 910, "signature": "(scene_prop_get_slot,<destination>,<scene_prop_id>,<slot_no>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "prop_instance_get_slot", "code": 13, "line": 911, "signature": "(prop_instance_get_slot,<destination>,<prop_instance_id>,<slot_no>),", "lhs": true, "can_fail": false, "global_lhs": false},
  {"name": "frobnicate_thing", "code": 14, "line": 999, "signature": "does something the rules do not know", "lhs": false, "can_fail": false, "global_lhs": false}
 ],
 "hooks": [],
 "ranges": [],
 "tables": {}
}"#;

fn fixture() -> Atlas {
    Atlas::from_str(FIXTURE).expect("the fixture atlas is valid")
}

fn find<'a>(report: &'a Report, native: &str) -> &'a Record {
    match report.records.iter().find(|record| record.native == native) {
        Some(record) => record,
        None => panic!("{native} got no name; it is manual: {:?}", report.manual.iter().map(|m| &m.native).collect::<Vec<_>>()),
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The real kit: the atlas when this lane has one, else the kit header it is dumped from.
fn kit_atlas() -> Option<Atlas> {
    let atlas = root().join("target/uw-staging/atlas.json");
    if atlas.is_file() {
        if let Ok(atlas) = Atlas::from_path(&atlas) {
            return Some(atlas);
        }
    }
    let header = root().join("target/ref/kit/header_operations.py");
    if header.is_file() {
        if let Ok(atlas) = uw_easy::ops::atlas_from_header(&header) {
            return Some(atlas);
        }
    }
    None
}

#[test]
fn fixture_names_follow_the_d3_rules() {
    let report = Report::generate(&fixture());
    let record = find(&report, "agent_get_position");
    assert_eq!(record.easy, "agent.position");
    assert_eq!(record.kind, Kind::PropertyGet);
    assert_eq!(record.rule, "R1b");
    assert_eq!(record.subject, "agent");
    assert_eq!(record.params, ["position_no", "agent_id"]);
    assert_eq!(record.line, Some(770));

    let record = find(&report, "agent_set_speed_modifier");
    assert_eq!(record.easy, "agent.speed_modifier");
    assert_eq!(record.kind, Kind::PropertySet);
    assert_eq!(record.rule, "R2");
    assert_eq!(record.params, ["agent_id", "value"]);

    let record = find(&report, "key_is_down");
    assert_eq!((record.easy.as_str(), record.kind), ("key.is_down", Kind::Condition));

    assert_eq!(find(&report, "store_agent_hit_points").easy, "agent.hit_points");
    assert_eq!(find(&report, "store_agent_hit_points").rule, "R1c");

    let color = find(&report, "faction_get_color");
    assert_eq!((color.easy.as_str(), color.rule.as_str()), ("faction.color", "R13"));

    assert_eq!(find(&report, "party_get_slot").easy, "party.slot");
    assert_eq!(find(&report, "party_set_slot").easy, "party.slot");
    assert_eq!(find(&report, "party_get_slot").rule, "R5");

    assert_eq!(find(&report, "ge").easy, "value.ge");
    assert_eq!(find(&report, "try_begin").easy, "flow.try_begin");
    assert_eq!(find(&report, "str_store_troop_name").easy, "text.store_troop_name");
    assert_eq!(find(&report, "display_message").kind, Kind::Method);
    assert_eq!(find(&report, "add_xp_to_troop").easy, "troop.add_xp");

    // R14: "writes reg0" makes it a function.
    let record = find(&report, "troop_remove_items");
    assert_eq!((record.easy.as_str(), record.kind, record.rule.as_str()), ("troop.remove_items", Kind::Function, "R14"));

    // One name per concept: two props of the same slot name clash, the dropped one becomes a TODO.
    assert_eq!(report.clashes.len(), 1);
    assert_eq!(report.clashes[0].easy, "prop.slot");
    assert_eq!(report.clashes[0].kept, "scene_prop_get_slot");
    assert_eq!(report.clashes[0].dropped, ["prop_instance_get_slot"]);
    let manual = report.manual.iter().find(|entry| entry.native == "prop_instance_get_slot").expect("the loser is manual");
    assert!(manual.note.contains("name clash"), "{}", manual.note);
    assert!(report.manual.iter().any(|entry| entry.native == "frobnicate_thing" && entry.note.is_empty()));
}

#[test]
fn fixture_output_is_written() {
    let report = Report::generate(&fixture());
    let dir = std::env::temp_dir().join(format!("uw-easy-{}", std::process::id()));
    report.write(&dir).expect("write the fixture vocabulary");
    let ops = std::fs::read_to_string(dir.join("ops.json")).expect("ops.json");
    let json: serde_json::Value = serde_json::from_str(&ops).expect("ops.json is JSON");
    assert_eq!(json["auto"], report.auto());
    assert_eq!(json["ops"].as_array().expect("one record per op").len(), report.auto());
    let manual = std::fs::read_to_string(dir.join("manual.toml")).expect("manual.toml");
    assert!(manual.contains("native = \"frobnicate_thing\""), "{manual}");
    assert!(manual.contains("easy = \"\""), "a TODO has no invented name");
    std::fs::remove_dir_all(&dir).expect("clean up");
}

/// D0: a name filled in by hand in manual.toml survives `uw easy` and reaches ops.json as rule M.
#[test]
fn hand_names_reach_ops_json() {
    let report = Report::generate(&fixture());
    let dir = std::env::temp_dir().join(format!("uw-easy-hand-{}", std::process::id()));
    report.write(&dir).expect("first write");
    let manual = std::fs::read_to_string(dir.join("manual.toml")).expect("manual.toml");
    let filled = manual.replacen(
        "easy = \"\"\nnative = \"frobnicate_thing\"",
        "easy = \"thing.frobnicate\"\nnative = \"frobnicate_thing\"",
        1,
    );
    assert_ne!(filled, manual, "the fixture TODO is where the test expects it");
    std::fs::write(dir.join("manual.toml"), &filled).expect("fill one name");
    report.write(&dir).expect("second write keeps it");
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("ops.json")).expect("ops.json")).expect("JSON");
    let hand = json["ops"].as_array().unwrap().iter().find(|op| op["native"] == "frobnicate_thing").expect("named op");
    assert_eq!(hand["easy"], "thing.frobnicate");
    assert_eq!(hand["rule"], "M");
    assert_eq!(hand["subject"], "thing");
    assert_eq!(json["hand"], 1);
    assert_eq!(json["manual"].as_u64().unwrap() as usize, report.manual_count() - 1);
    assert!(std::fs::read_to_string(dir.join("manual.toml")).unwrap().contains("easy = \"thing.frobnicate\""));
    std::fs::remove_dir_all(&dir).expect("clean up");
}

/// One name per concept: the committed vocabulary never gives one easy name of one kind to two operations.
#[test]
fn committed_names_are_unique_per_kind() {
    let text = std::fs::read_to_string(root().join("kit-meta/easy/ops.json")).expect("committed ops.json");
    let json: serde_json::Value = serde_json::from_str(&text).expect("JSON");
    let mut seen = std::collections::BTreeMap::new();
    for op in json["ops"].as_array().expect("ops") {
        let key = (op["easy"].as_str().unwrap().to_owned(), op["kind"].as_str().unwrap().to_owned());
        if let Some(other) = seen.insert(key.clone(), op["native"].as_str().unwrap().to_owned()) {
            panic!("`{}` ({}) names both {other} and {}", key.0, key.1, op["native"]);
        }
    }
}

#[test]
fn real_kit_vocabulary() {
    let Some(atlas) = kit_atlas() else {
        eprintln!("skipped: no target/uw-staging/atlas.json and no target/ref/kit/header_operations.py");
        return;
    };
    let report = Report::generate(&atlas);
    assert!(report.auto() >= 900, "only {} operations named automatically", report.auto());
    let mut names = std::collections::BTreeSet::new();
    for record in &report.records {
        assert!(names.insert((record.easy.clone(), record.kind)), "duplicate easy name {} ({})", record.easy, record.kind.as_str());
    }
    assert_eq!(find(&report, "agent_get_position").easy, "agent.position");
    assert_eq!(find(&report, "agent_set_speed_modifier").easy, "agent.speed_modifier");
    assert_eq!(find(&report, "key_is_down").kind, Kind::Condition);
    eprintln!(
        "easy: {} auto, {} manual, {} name clashes",
        report.auto(),
        report.manual_count(),
        report.clashes.len()
    );
}

#[test]
fn committed_vocabulary_is_current() {
    let Some(atlas) = kit_atlas() else {
        eprintln!("skipped: no atlas and no kit header");
        return;
    };
    let report = Report::generate(&atlas);
    let committed_dir = root().join("kit-meta/easy");
    let committed = std::fs::read_to_string(committed_dir.join("ops.json")).unwrap_or_default();
    let committed_manual = std::fs::read_to_string(committed_dir.join("manual.toml")).unwrap_or_default();
    // Regenerate away from the worktree, so the check never changes the files it compares.
    let dir = std::env::temp_dir().join(format!("uw-easy-current-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    std::fs::write(dir.join("manual.toml"), &committed_manual).expect("seed manual.toml");
    report.write(&dir).expect("generate the vocabulary");
    let fresh = std::fs::read_to_string(dir.join("ops.json")).expect("ops.json");
    let fresh_manual = std::fs::read_to_string(dir.join("manual.toml")).expect("manual.toml");
    std::fs::remove_dir_all(&dir).expect("clean up");
    assert_eq!(committed, fresh, "kit-meta/easy/ops.json is stale; regenerate it with `uw easy`");
    assert_eq!(committed_manual, fresh_manual, "kit-meta/easy/manual.toml is stale; regenerate it with `uw easy`");
}
