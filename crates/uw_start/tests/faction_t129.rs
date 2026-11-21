//! Faction relations, quests and time on a tiny hand-made atlas (factions schema from
//! kit-meta/records/factions.toml, ops by name like tests/flow.rs).

use std::path::PathBuf;

use uw_start::{Cell, Flow, Machine};

fn op(name: &str, code: u64, lhs: bool, can_fail: bool) -> String {
    format!(
        r#"{{"name":"{name}","code":{code},"line":1,"signature":"","lhs":{lhs},"can_fail":{can_fail},"global_lhs":false}}"#
    )
}

fn atlas(scripts: &[(&str, &str)]) -> uw_atlas::Atlas {
    let ops = [
        op("assign", 2133, true, false),
        op("check_quest_active", 200, false, true),
        op("faction_set_color", 1276, false, false),
        op("set_quest_progression", 1285, false, false),
        op("set_relation", 1270, false, false),
        op("start_quest", 1280, false, false),
        op("store_current_hours", 2270, true, false),
        op("store_relation", 2190, true, false),
    ]
    .join(",");
    let entries: Vec<String> = scripts
        .iter()
        .enumerate()
        .map(|(i, (name, body))| {
            format!(
                r#"{{"index":{i},"name":"{name}","id":"script_{name}","dup":null,"line":null,"value":["{name}",{{"list":{body}}}]}}"#
            )
        })
        .collect();
    let text = format!(
        r#"{{"schema":1,"kit_hash":"t","ops":[{ops}],"hooks":[],"ranges":[],
        "constants":[],
        "tables":{{
          "scripts":{{"module":"module_scripts","var":"scripts","id_file":null,"id_prefix":"script_","ref_tag":"script","entries":[{}]}},
          "factions":{{"module":"module_factions","var":"factions","id_file":null,"id_prefix":"fac_","ref_tag":"fac","entries":[
            {{"index":0,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":null,
              "value":["kingdom_1","Kingdom of Swadia",0,0.9,[["outlaws",-0.05]],[],11989572]}},
            {{"index":1,"name":"outlaws","id":"fac_outlaws","dup":null,"line":null,
              "value":["outlaws","Outlaws",0,0.5,[],[],8947848]}},
            {{"index":2,"name":"player_faction","id":"fac_player_faction","dup":null,"line":null,
              "value":["player_faction","Player",0,0.1,[],[],11989572]}}]}},
          "quests":{{"module":"module_quests","var":"quests","id_file":null,"id_prefix":"qst_","ref_tag":"qst","entries":[
            {{"index":0,"name":"deliver_message","id":"qst_deliver_message","dup":null,"line":null,"value":["deliver_message",0]}}]}}
        }}}}"#,
        entries.join(",")
    );
    uw_atlas::Atlas::from_str(&text).expect("atlas")
}

/// The real factions schema, so `initial_field_text` prints the record like `uw new` does.
fn schemas() -> Vec<uw_decomp::records::Schema> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/records/factions.toml");
    vec![uw_decomp::records::Schema::from_path(&path).expect("factions schema")]
}

fn number(m: &Machine, global: &str) -> Option<i64> {
    m.globals.get(global).map(|v| v.n)
}

#[test]
fn store_relation_reads_the_record_list() {
    let a = atlas(&[(
        "start",
        r#"[[2190,"$relation","fac_kingdom_1","fac_outlaws"]]"#,
    )]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(number(&m, "relation"), Some(-5), "-0.05 on the engine's -100..100 scale");
    assert!(m.report.unknown_ops.get("store_relation").is_none(), "{:?}", m.report.unknown_ops);
}

#[test]
fn store_relation_falls_back_to_the_other_record_then_to_zero() {
    // outlaws lists nothing; kingdom_1's list names the pair (process_factions.py writes both ways).
    let a = atlas(&[(
        "start",
        r#"[[2190,"$reverse","fac_outlaws","fac_kingdom_1"],[2190,"$unknown_pair","fac_outlaws","fac_player_faction"]]"#,
    )]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(number(&m, "reverse"), Some(-5));
    assert_eq!(number(&m, "unknown_pair"), Some(0), "no list gives the pair: the matrix is 0");
    assert!(m.report.unknown_ops.get("store_relation").is_none(), "{:?}", m.report.unknown_ops);
}

#[test]
fn set_relation_then_store_relation_sees_it() {
    // set_relation writes one direction (header_operations.py:543 has no symmetry note).
    let a = atlas(&[(
        "start",
        r#"[[1270,"fac_kingdom_1","fac_outlaws",40],
            [2190,"$relation","fac_kingdom_1","fac_outlaws"],
            [2190,"$opposite","fac_outlaws","fac_kingdom_1"]]"#,
    )]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(number(&m, "relation"), Some(40));
    assert_eq!(number(&m, "opposite"), Some(-5), "the record's start value is untouched");
    let write = m.trace.first().expect("set_relation wrote a cell");
    assert_eq!(write.cell, Cell::field("factions", 0, "relation 1"));
    assert_eq!(write.value.n, 40);
    assert!(m.report.unknown_ops.get("set_relation").is_none(), "{:?}", m.report.unknown_ops);
    assert!(m.report.unknown_ops.get("store_relation").is_none(), "{:?}", m.report.unknown_ops);
}

#[test]
fn store_current_hours_is_zero_at_game_start() {
    let a = atlas(&[("start", r#"[[2270,"$hours"]]"#)]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(number(&m, "hours"), Some(0));
    assert!(m.report.unknown_ops.get("store_current_hours").is_none(), "{:?}", m.report.unknown_ops);
}

#[test]
fn quests_start_inactive_and_start_quest_makes_them_active() {
    let a = atlas(&[
        ("check", r#"[[200,"qst_deliver_message"]]"#),
        ("start_quest", r#"[[1280,"qst_deliver_message"]]"#),
    ]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("check", vec![]), Ok(Flow::Fail), "no quest is active at start");
    assert!(m.report.unknown_ops.get("check_quest_active").is_none(), "{:?}", m.report.unknown_ops);
    assert_eq!(m.run_script("start_quest", vec![]), Ok(Flow::Done));
    assert_eq!(
        m.cells.get(&Cell::field("quests", 0, "active")).map(|v| v.n),
        Some(1),
        "start_quest sets the quest active"
    );
    assert_eq!(m.run_script("check", vec![]), Ok(Flow::Done), "the check sees the started quest");
    assert!(m.report.unknown_ops.get("start_quest").is_none(), "{:?}", m.report.unknown_ops);
}

#[test]
fn quest_progression_and_faction_colour_are_recorded() {
    let a = atlas(&[(
        "start",
        r#"[[1285,"qst_deliver_message",3],[1276,"fac_outlaws",16711680]]"#,
    )]);
    let mut m = Machine::new(&a, schemas(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(m.cells.get(&Cell::field("quests", 0, "progression")).map(|v| v.n), Some(3));
    assert_eq!(
        m.cells.get(&Cell::field("factions", 1, "faction_color")).map(|v| v.n),
        Some(16711680)
    );
    assert!(m.report.unknown_ops.get("set_quest_progression").is_none(), "{:?}", m.report.unknown_ops);
    assert!(m.report.unknown_ops.get("faction_set_color").is_none(), "{:?}", m.report.unknown_ops);
}
