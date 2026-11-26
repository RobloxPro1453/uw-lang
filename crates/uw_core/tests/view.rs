//! D8a: Native operations written back as lines of the core language, and read back to the same operation.

use uw_core::view::Viewer;
use uw_core::{Arg, Op};

mod common;
use common::{atlas, lower_ok};

fn op(name: &str, args: Vec<Arg>) -> Op {
    Op { name: name.to_owned(), args, neg: false, this_or_next: false }
}

fn name(text: &str) -> Arg {
    Arg::Name(text.to_owned())
}

/// The line, lowered again inside `game.start`: the operations it gives.
fn back(line: &str) -> Vec<Op> {
    let lowered = lower_ok(&[("main", &format!("on game.start {{\n  {line}\n}}\n"))]);
    lowered.script_appends.iter().flat_map(|(_, ops)| ops.clone()).collect()
}

#[test]
fn an_action_on_a_kit_thing_reads_back_to_the_same_operation() {
    let viewer = Viewer::new();
    let native = op("jump_to_scene", vec![name("scn_town_6_center"), Arg::Int(0)]);
    let line = viewer.line(&atlas(), &native).expect("jump_to_scene is a kit action");
    assert_eq!(line, "scene.town_6_center.jump(0)");
    assert_eq!(back(&line), vec![native]);
}

#[test]
fn constants_and_numbers_read_back_unchanged() {
    let viewer = Viewer::new();
    let native = op("troop_raise_attribute", vec![name("trp_player"), name("ca_strength"), Arg::Int(3)]);
    let line = viewer.line(&atlas(), &native).expect("a kit action with a constant");
    assert_eq!(line, "troop.player.raise_attribute(ca.strength, 3)");
    assert_eq!(back(&line), vec![native]);
}

#[test]
fn a_setter_and_a_game_action_read_back() {
    let viewer = Viewer::new();
    let set = op("set_jump_mission", vec![name("mt_town_center")]);
    let line = viewer.line(&atlas(), &set).expect("a setter of a thing");
    assert_eq!(line, "mission.jump = mission_template.town_center");
    assert_eq!(back(&line), vec![set]);
    let screen = op("change_screen_mission", Vec::new());
    assert_eq!(viewer.line(&atlas(), &screen).as_deref(), Some("screen.mission()"));
}

#[test]
fn what_the_language_cannot_write_yet_is_left_to_the_kit_form() {
    let viewer = Viewer::new();
    let atlas = atlas();
    // a Native global, a negated condition, an operation no kit word writes
    assert_eq!(viewer.line(&atlas, &op("jump_to_menu", vec![Arg::Global("$g_next_menu".into())])), None);
    let mut negated = op("change_screen_map", Vec::new());
    negated.neg = true;
    assert_eq!(viewer.line(&atlas, &negated), None);
    assert_eq!(viewer.line(&atlas, &op("try_begin", Vec::new())), None);
}

#[test]
fn a_try_block_reads_as_if_else() {
    let viewer = Viewer::new();
    let mut ton = op("eq", vec![Arg::Local(":town".into()), name("p_town_6")]);
    ton.this_or_next = true;
    let mut neq = op("eq", vec![Arg::Local(":x".into()), Arg::Int(0)]);
    neq.neg = true;
    let ops = vec![
        op("try_begin", vec![]),
        ton,
        op("eq", vec![Arg::Local(":town".into()), name("p_main_party")]),
        neq,
        op("change_screen_map", vec![]),
        op("else_try", vec![]),
        op("change_screen_mission", vec![]),
        op("try_end", vec![]),
    ];
    let lines: Vec<String> = viewer
        .block(&atlas(), &ops)
        .iter()
        .map(|line| format!("{}{}", "  ".repeat(line.depth), line.text))
        .collect();
    assert_eq!(
        lines,
        vec![
            "if (town == party.town_6 or town == party.main_party) and x != 0 {",
            "  screen.map()",
            "} else {",
            "  screen.mission()",
            "}",
        ]
    );
}

#[test]
fn a_try_with_no_condition_reads_in_place_and_unknown_ops_stay_visible() {
    let viewer = Viewer::new();
    let ops = vec![
        op("try_begin", vec![]),
        op("change_screen_map", vec![]),
        op("try_end", vec![]),
        op("assign", vec![Arg::Global("$g_x".into()), Arg::Int(1)]),
    ];
    let lines = viewer.block(&atlas(), &ops);
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert_eq!((lines[0].text.as_str(), lines[0].written), ("screen.map()", true));
    assert_eq!((lines[1].text.as_str(), lines[1].written), ("(assign, $g_x, 1)", false));
}
