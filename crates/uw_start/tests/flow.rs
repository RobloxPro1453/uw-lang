//! Control-flow semantics of the start dry run on a tiny hand-made atlas (codes are arbitrary;
//! the machine dispatches on the names the atlas gives them).

use uw_start::{Cell, Flow, Machine};

fn op(name: &str, code: u64, lhs: bool, can_fail: bool) -> String {
    format!(
        r#"{{"name":"{name}","code":{code},"line":1,"signature":"","lhs":{lhs},"can_fail":{can_fail},"global_lhs":false}}"#
    )
}

fn atlas(scripts: &[(&str, &str)]) -> uw_atlas::Atlas {
    let ops = [
        op("call_script", 1, false, false),
        op("try_end", 3, false, false),
        op("try_begin", 4, false, false),
        op("else_try", 5, false, false),
        op("try_for_range", 6, true, false),
        op("store_script_param_1", 21, true, false),
        op("ge", 30, false, true),
        op("eq", 31, false, true),
        op("troop_set_slot", 500, false, false),
        op("troop_get_slot", 520, true, false),
        op("assign", 2133, true, false),
        op("val_add", 2105, true, false),
        op("store_random_in_range", 2136, true, false),
        op("mystery_op", 9999, true, false),
        op("neg", 2147483648, false, false),
        op("this_or_next", 1073741824, false, false),
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
          "troops":{{"module":"module_troops","var":"troops","id_file":null,"id_prefix":"trp_","ref_tag":"trp","entries":[
            {{"index":0,"name":"player","id":"trp_player","dup":null,"line":null,"value":["player"]}},
            {{"index":1,"name":"hero","id":"trp_hero","dup":null,"line":null,"value":["hero"]}}]}}
        }}}}"#,
        entries.join(",")
    );
    uw_atlas::Atlas::from_str(&text).expect("atlas")
}

fn slot(m: &Machine, troop: i64, slot: i64) -> Option<i64> {
    m.cells.get(&Cell::slot("troops", troop, slot)).map(|v| v.n)
}

#[test]
fn try_else_takes_the_first_branch_whose_conditions_hold() {
    // try_begin (eq 1 2) slot5=10 else_try (eq 1 1) slot5=20 else_try slot5=30 try_end
    let a = atlas(&[(
        "start",
        r#"[[4],[31,1,2],[500,"trp_hero",5,10],[5],[31,1,1],[500,"trp_hero",5,20],[5],[500,"trp_hero",5,30],[3]]"#,
    )]);
    let mut m = Machine::new(&a, Vec::new(), 7);
    assert_eq!(m.run_script("start", vec![]), Ok(Flow::Done));
    assert_eq!(slot(&m, 1, 5), Some(20));
    assert_eq!(m.trace.len(), 1, "the failed branch wrote nothing and the last branch never ran");
}

#[test]
fn neg_and_this_or_next_follow_the_kit_rules() {
    // (this_or_next|eq 1 2), (eq 3 3) -> holds; (neg|eq 1 1) -> fails the try.
    let a = atlas(&[(
        "start",
        r#"[[4],[1073741855,1,2],[31,3,3],[500,"trp_hero",1,1],[3],[4],[2147483679,1,1],[500,"trp_hero",2,1],[3]]"#,
    )]);
    let mut m = Machine::new(&a, Vec::new(), 7);
    m.run_script("start", vec![]).unwrap();
    assert_eq!(slot(&m, 1, 1), Some(1));
    assert_eq!(slot(&m, 1, 2), None);
}

#[test]
fn loops_calls_params_and_failing_scripts() {
    // for i in 0..3: call add(i) ; add: param -> slot 9 += param ; cf_no fails, so the try skips.
    let a = atlas(&[
        ("start", r#"[[6,":i",0,3],[1,"script_add",":i"],[3],[4],[1,"script_cf_no"],[500,"trp_hero",8,1],[3]]"#),
        ("add", r#"[[21,":p"],[520,":s","trp_hero",9],[2105,":s",":p"],[500,"trp_hero",9,":s"]]"#),
        ("cf_no", r#"[[31,0,1]]"#),
    ]);
    let mut m = Machine::new(&a, Vec::new(), 7);
    m.run_script("start", vec![]).unwrap();
    assert_eq!(slot(&m, 1, 9), Some(3), "0+1+2");
    assert_eq!(slot(&m, 1, 8), None, "a failing script fails the condition chain");
    // The slot-9 value read slot 9 itself: the trace knows the dependency.
    let last = m.trace.last().unwrap();
    assert!(last.value.deps.contains(&Cell::slot("troops", 1, 9)));
}

#[test]
fn unknown_operations_are_counted_and_poison_their_results() {
    let a = atlas(&[("start", r#"[[9999,":x",1],[500,"trp_hero",4,":x"]]"#)]);
    let mut m = Machine::new(&a, Vec::new(), 7);
    m.run_script("start", vec![]).unwrap();
    assert_eq!(m.report.unknown_ops.get("mystery_op"), Some(&1));
    assert!(m.trace[0].value.unknown);
}

#[test]
fn random_numbers_are_seeded_and_marked() {
    let a = atlas(&[("start", r#"[[2136,":r",0,100],[500,"trp_hero",3,":r"]]"#)]);
    let run = |seed| {
        let mut m = Machine::new(&a, Vec::new(), seed);
        m.run_script("start", vec![]).unwrap();
        m.trace[0].value.clone()
    };
    assert_eq!(run(5).n, run(5).n);
    assert!(run(5).random);
}
