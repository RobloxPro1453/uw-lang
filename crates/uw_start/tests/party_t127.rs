//! Party operations of the start dry run (T-127) on a tiny hand-made atlas (codes are arbitrary;
//! the machine dispatches on the names the atlas gives them) with the real
//! `kit-meta/records/parties.toml` and `party_templates.toml` schemas, so the record fields are read
//! the way the real run reads them.

use std::path::PathBuf;

use uw_start::{Cell, Flow, Machine};

/// The engine's `reg0` operand (`opmask_register|0`, header_common.py).
const REG0: i64 = 1 << 56;

fn op(name: &str, code: u64, lhs: bool, can_fail: bool) -> String {
    format!(
        r#"{{"name":"{name}","code":{code},"line":1,"signature":"","lhs":{lhs},"can_fail":{can_fail},"global_lhs":false}}"#
    )
}

fn schema(name: &str) -> uw_decomp::records::Schema {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/records").join(name);
    uw_decomp::records::Schema::from_path(&path).expect("schema")
}

fn machine(a: &uw_atlas::Atlas) -> Machine<'_> {
    Machine::new(a, vec![schema("parties.toml"), schema("party_templates.toml")], 7)
}

fn atlas(scripts: &[(&str, &str)]) -> uw_atlas::Atlas {
    let ops = [
        op("party_is_active", 132, false, true),
        op("party_set_slot", 501, false, false),
        op("spawn_around_party", 1100, false, false),
        op("party_get_num_companions", 1601, true, false),
        op("party_set_flags", 1603, false, false),
        op("party_get_template_id", 1609, true, false),
        op("party_add_members", 1610, false, false),
        op("party_set_faction", 1620, false, false),
        op("party_stack_get_troop_id", 1652, true, false),
        op("store_faction_of_party", 2204, true, false),
        op("store_distance_to_party_from_party", 2281, true, false),
        op("store_num_parties_of_template", 2310, true, false),
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
        "constants":[{{"name":"neg","value":2147483648,"module":"header_operations","line":1}},
                     {{"name":"this_or_next","value":1073741824,"module":"header_operations","line":1}}],
        "tables":{{
          "scripts":{{"module":"module_scripts","var":"scripts","id_file":null,"id_prefix":"script_","ref_tag":"script","entries":[{}]}},
          "parties":{{"module":"module_parties","var":"parties","id_file":null,"id_prefix":"p_","ref_tag":"p","entries":[
            {{"index":0,"name":"main_party","id":"p_main_party","dup":null,"line":null,
             "value":["main_party","Main Party",1024,0,"pt_none","fac_player_faction",0,0,0,[0,0],
                      {{"list":[["trp_hero",3,0],["trp_peasant",5,0],["trp_player",2,1]]}},0]}},
            {{"index":1,"name":"other","id":"p_other","dup":null,"line":null,
             "value":["other","Other",1024,0,"pt_none","fac_commoners",0,0,0,[3,4],{{"list":[]}},0]}},
            {{"index":2,"name":"moving","id":"p_moving","dup":null,"line":null,
             "value":["moving","Moving",0,0,"pt_bandits","fac_outlaws",0,0,0,[10,10],{{"list":[]}},0]}}]}},
          "party_templates":{{"module":"module_party_templates","var":"party_templates","id_file":null,"id_prefix":"pt_","ref_tag":"pt","entries":[
            {{"index":0,"name":"none","id":"pt_none","dup":null,"line":null,"value":["none","none",0,0,"fac_commoners",0,{{"list":[]}}]}},
            {{"index":1,"name":"bandits","id":"pt_bandits","dup":null,"line":null,"value":["bandits","Bandits",0,0,"fac_outlaws",0,{{"list":[]}}]}}]}},
          "factions":{{"module":"module_factions","var":"factions","id_file":null,"id_prefix":"fac_","ref_tag":"fac","entries":[
            {{"index":0,"name":"player_faction","id":"fac_player_faction","dup":null,"line":null,"value":["player_faction"]}},
            {{"index":1,"name":"commoners","id":"fac_commoners","dup":null,"line":null,"value":["commoners"]}},
            {{"index":2,"name":"outlaws","id":"fac_outlaws","dup":null,"line":null,"value":["outlaws"]}},
            {{"index":3,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":null,"value":["kingdom_1"]}}]}},
          "troops":{{"module":"module_troops","var":"troops","id_file":null,"id_prefix":"trp_","ref_tag":"trp","entries":[
            {{"index":0,"name":"player","id":"trp_player","dup":null,"line":null,"value":["player"]}},
            {{"index":1,"name":"hero","id":"trp_hero","dup":null,"line":null,"value":["hero"]}},
            {{"index":2,"name":"peasant","id":"trp_peasant","dup":null,"line":null,"value":["peasant"]}}]}}
        }}}}"#,
        entries.join(",")
    );
    uw_atlas::Atlas::from_str(&text).expect("atlas")
}

fn field(m: &Machine, party: i64, name: &str) -> Option<i64> {
    m.cells.get(&Cell::field("parties", party, name)).map(|v| v.n)
}

fn slot(m: &Machine, party: i64, slot: i64) -> Option<i64> {
    m.cells.get(&Cell::slot("parties", party, slot)).map(|v| v.n)
}

fn unmodelled(m: &Machine, op: &str) -> bool {
    m.report.unknown_ops.contains_key(op)
}

#[test]
fn party_set_faction_writes_the_field_and_store_faction_of_party_reads_it() {
    let a = atlas(&[(
        "start",
        r#"[[1620,"p_main_party","fac_kingdom_1"],[2204,":f","p_main_party"],[501,"p_main_party",5,":f"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(field(&m, 0, "faction"), Some(3), "fac_kingdom_1 is faction 3");
    assert_eq!(slot(&m, 0, 5), Some(3), "the read returned the faction that was written");
    assert!(!unmodelled(&m, "party_set_faction"));
    assert!(!unmodelled(&m, "store_faction_of_party"));
}

#[test]
fn a_record_party_reports_the_faction_its_record_has() {
    let a = atlas(&[("start", r#"[[2204,":f","p_other"],[501,"p_other",5,":f"]]"#)]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(slot(&m, 1, 5), Some(1), "p_other's record says fac_commoners");
    assert!(!unmodelled(&m, "store_faction_of_party"));
}

#[test]
fn spawn_around_party_adds_a_party_and_puts_its_id_into_reg0() {
    // spawn_around_party(p_main_party, pt_bandits) then party_set_slot(reg0, 6, 1) to see the id.
    let body = format!(r#"[[1100,"p_main_party","pt_bandits"],[501,{REG0},6,1]]"#);
    let a = atlas(&[("start", &body)]);
    let mut m = machine(&a);
    assert_eq!(m.party_count, 3, "three record parties before the run");
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(m.party_count, 4, "the spawned party is the fourth one");
    assert_eq!(m.registers.get(&0).map(|v| v.n), Some(3), "reg0 holds the new party's id");
    assert_eq!(field(&m, 3, "party_template"), Some(1), "its template is pt_bandits");
    assert_eq!(field(&m, 3, "faction"), Some(2), "and the template's faction, fac_outlaws");
    assert_eq!(slot(&m, 3, 6), Some(1), "the register held the id of the party that was written");
    assert!(!unmodelled(&m, "spawn_around_party"));
}

#[test]
fn distance_between_two_static_parties_comes_from_their_coordinates() {
    // p_main_party stands at (0, 0) and p_other at (3, 4): three-four-five.
    let a = atlas(&[(
        "start",
        r#"[[2281,":d","p_main_party","p_other"],[501,"p_main_party",7,":d"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(slot(&m, 0, 7), Some(5));
    assert!(!unmodelled(&m, "store_distance_to_party_from_party"));
}

#[test]
fn distance_to_a_party_that_moves_stays_unknown() {
    // p_moving has no pf_is_static bit: its position is wherever the AI took it, not (10, 10).
    let a = atlas(&[(
        "start",
        r#"[[2281,":d","p_main_party","p_moving"],[501,"p_main_party",7,":d"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    let written = m.cells.get(&Cell::slot("parties", 0, 7)).expect("the slot was written");
    assert!(written.unknown, "an unknown position gives an unknown distance");
    assert!(!unmodelled(&m, "store_distance_to_party_from_party"), "modelled, just not knowable");
}

#[test]
fn members_are_read_from_the_record_stack_list() {
    // p_main_party holds hero 3, peasant 5 and a prisoner stack (player 2).
    let a = atlas(&[(
        "start",
        r#"[[1601,":n","p_main_party"],[501,"p_main_party",9,":n"],
            [1652,":t","p_main_party",1],[501,"p_main_party",10,":t"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(slot(&m, 0, 9), Some(8), "3 + 5 companions, the prisoner left out");
    assert_eq!(slot(&m, 0, 10), Some(2), "stack 1 is the second companion stack, trp_peasant");
    assert!(!unmodelled(&m, "party_get_num_companions"));
    assert!(!unmodelled(&m, "party_stack_get_troop_id"));
}

#[test]
fn members_stop_being_known_once_an_operation_changes_them() {
    let a = atlas(&[(
        "start",
        r#"[[1610,"p_main_party","trp_hero",4],[1601,":n","p_main_party"],[501,"p_main_party",9,":n"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    let stacks = m.cells.get(&Cell::field("parties", 0, "stacks")).expect("stacks were marked");
    assert!(stacks.unknown, "the engine added the members, the record no longer holds them");
    let written = m.cells.get(&Cell::slot("parties", 0, 9)).expect("the slot was written");
    assert!(written.unknown, "so the companion count is unknown");
    assert!(!unmodelled(&m, "party_add_members"), "modelled, just not knowable");
}

#[test]
fn party_is_active_follows_the_parties_that_exist() {
    let a = atlas(&[
        ("start", r#"[[132,"p_main_party"],[1100,"p_main_party","pt_bandits"],[132,72057594037927936]]"#),
        ("past_the_end", r#"[[132,4]]"#),
    ]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done), "a record party and the spawned one are active");
    assert_eq!(m.run_script("past_the_end", vec![]), Ok(Flow::Fail), "party 4 does not exist");
    assert!(!unmodelled(&m, "party_is_active"));
}

#[test]
fn the_template_count_counts_the_parties_of_that_template() {
    // p_moving is the only record party with pt_bandits; the spawned one is a second.
    let a = atlas(&[(
        "start",
        r#"[[1100,"p_main_party","pt_bandits"],[2310,":c","pt_bandits"],[501,"p_main_party",8,":c"]]"#,
    )]);
    let mut m = machine(&a);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(slot(&m, 0, 8), Some(2));
    assert!(!unmodelled(&m, "store_num_parties_of_template"));
}
