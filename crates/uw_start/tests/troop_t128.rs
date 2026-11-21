//! Troop operations of the start dry run (T-128) on a tiny hand-made atlas whose `troops` table is
//! printed with the kit's real schema (`kit-meta/records/troops.toml`), so a record's start values
//! read exactly as `uw new` prints them (codes are arbitrary; the machine dispatches on names).

use std::path::PathBuf;

use uw_start::{Cell, Flow, Machine};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn schema() -> uw_decomp::records::Schema {
    uw_decomp::records::Schema::from_path(&root().join("kit-meta/records/troops.toml")).expect("troops schema")
}

/// Packed `attributes` of `trp_hero`: strength 5, agility 4, intelligence 3, charisma 2, level 12
/// (one byte each, the level in the top byte; header_troops.py:90-213).
const ATTRIBUTES: i64 = 5 | 4 << 8 | 3 << 16 | 2 << 24 | 12 << 32;
/// Packed `skills` of `trp_hero`: index 1 (leadership) 3 and index 10 levelled 7 (`n << (4 * index)`).
const SKILLS: i64 = 3 << 4 | 7 << 40;
/// `troop_flags` of `trp_hero`: `tf_female | tf_hero` (header_troops.py:10-14).
const HERO_FLAGS: i64 = 0x11;
/// The faction reference `trp_hero` starts with; deliberately not the one the script writes.
const START_FACTION: i64 = 6;

fn op(name: &str, code: u64, lhs: bool, can_fail: bool) -> String {
    format!(
        r#"{{"name":"{name}","code":{code},"line":1,"signature":"","lhs":{lhs},"can_fail":{can_fail},"global_lhs":false}}"#
    )
}

fn atlas(body: &str) -> uw_atlas::Atlas {
    let ops = [
        op("troop_get_type", 1506, true, false),
        op("troop_set_faction", 1550, false, false),
        // The kit names opcode 2173 both ways (header_operations.py:1224-1225); the tiny atlas gives
        // each name its own code so both paths run.
        op("store_troop_faction", 2173, true, false),
        op("store_faction_of_troop", 2174, true, false),
        op("store_attribute_level", 2172, true, false),
        op("store_character_level", 2171, true, false),
        op("store_skill_level", 2170, true, false),
        op("troop_set_age", 1555, false, false),
    ]
    .join(",");
    let text = format!(
        r#"{{"schema":1,"kit_hash":"t","ops":[{ops}],"hooks":[],"ranges":[],"constants":[],
        "tables":{{
          "scripts":{{"module":"module_scripts","var":"scripts","id_file":null,"id_prefix":"script_","ref_tag":"script","entries":[
            {{"index":0,"name":"start","id":"script_start","dup":null,"line":null,"value":["start",{{"list":{body}}}]}}]}},
          "troops":{{"module":"module_troops","var":"troops","id_file":null,"id_prefix":"trp_","ref_tag":"trp","entries":[
            {{"index":0,"name":"player","id":"trp_player","dup":null,"line":null,"value":["player","Player","Players",0,0,0,0,[],0,0,0,0]}},
            {{"index":1,"name":"hero","id":"trp_hero","dup":null,"line":null,"value":["hero","Hero","Heroes",{HERO_FLAGS},0,0,{START_FACTION},[],{ATTRIBUTES},0,{SKILLS},0]}}]}},
          "factions":{{"module":"module_factions","var":"factions","id_file":null,"id_prefix":"fac_","ref_tag":"fac","entries":[
            {{"index":0,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":null,"value":["kingdom_1","Kingdom 1"]}}]}}
        }}}}"#
    );
    uw_atlas::Atlas::from_str(&text).expect("atlas")
}

fn global(m: &Machine, name: &str) -> Option<i64> {
    m.globals.get(name).map(|v| v.n)
}

fn cell(m: &Machine, troop: i64, field: &str) -> Option<i64> {
    m.cells.get(&Cell::field("troops", troop, field)).map(|v| v.n)
}

#[test]
fn troop_faction_reads_the_record_and_sees_a_write() {
    // read the record's faction, set it to fac_kingdom_1 (index 0), read it again both ways.
    let a = atlas(
        r#"[[2173,"$before","trp_hero"],[1550,"trp_hero","fac_kingdom_1"],[2173,"$after","trp_hero"],[2174,"$alias","trp_hero"]]"#,
    );
    let mut m = Machine::new(&a, vec![schema()], 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(global(&m, "before"), Some(START_FACTION), "the record's own faction");
    assert_eq!(global(&m, "after"), Some(0), "the read sees the write, not the record");
    assert_eq!(global(&m, "alias"), Some(0), "store_faction_of_troop is the same operation");
    assert_eq!(cell(&m, 1, "faction"), Some(0));
    assert!(m.report.unknown_ops.is_empty(), "{:?}", m.report.unknown_ops);
}

#[test]
fn troop_type_comes_from_the_low_flag_bits() {
    // trp_hero carries tf_female|tf_hero (0x11) -> type 1; trp_player carries tf_male (0) -> 0.
    let a = atlas(r#"[[1506,"$hero","trp_hero"],[1506,"$player","trp_player"]]"#);
    let mut m = Machine::new(&a, vec![schema()], 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(global(&m, "hero"), Some(1));
    assert_eq!(global(&m, "player"), Some(0));
    assert!(m.report.unknown_ops.is_empty(), "{:?}", m.report.unknown_ops);
}

#[test]
fn attributes_skills_and_level_come_from_the_packed_fields() {
    let a = atlas(
        r#"[[2172,"$strength","trp_hero",0],[2172,"$agility","trp_hero",1],[2172,"$charisma","trp_hero",3],
            [2171,"$level","trp_hero"],[2170,"$leadership",1,"trp_hero"],[2170,"$high",10,"trp_hero"]]"#,
    );
    let mut m = Machine::new(&a, vec![schema()], 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(global(&m, "strength"), Some(5));
    assert_eq!(global(&m, "agility"), Some(4));
    assert_eq!(global(&m, "charisma"), Some(2));
    assert_eq!(global(&m, "level"), Some(12));
    assert_eq!(global(&m, "leadership"), Some(3));
    assert_eq!(global(&m, "high"), Some(7));
    assert!(m.report.unknown_ops.is_empty(), "{:?}", m.report.unknown_ops);
}

#[test]
fn troop_set_age_writes_the_age_cell() {
    // No troop record field holds the age: the trace keeps Native's value under `age`.
    let a = atlas(r#"[[1555,"trp_hero",37]]"#);
    let mut m = Machine::new(&a, vec![schema()], 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(cell(&m, 1, "age"), Some(37));
    assert!(m.report.unknown_ops.is_empty(), "{:?}", m.report.unknown_ops);
}
