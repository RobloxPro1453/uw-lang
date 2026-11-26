//! D3 §4): the HUD as ONE hidden presentation, `ui.bar` / `ui.text`, and
//! `on key.pressed` / `on key.released` in the mission dispatcher.

mod common;

use common::{lower_err, lower_ok};
use uw_core::lower::time::{report, MISSION_FRAME};
use uw_core::lower::ui::HUD;
use uw_core::{Arg, Lowered, Op, Presentation};

fn hud(lowered: &Lowered) -> &Presentation {
    assert_eq!(lowered.presentations.len(), 1, "one HUD presentation: {:?}", lowered.presentations);
    let p = &lowered.presentations[0];
    assert_eq!(p.name, HUD);
    p
}

fn frame(lowered: &Lowered) -> &[Op] {
    &lowered.scripts.iter().find(|(name, _)| name == MISSION_FRAME).expect("mission frame").1
}

fn has(ops: &[Op], name: &str, args: &[Arg]) -> bool {
    ops.iter().any(|op| op.name == name && op.args == args)
}

fn global(name: &str) -> Arg {
    Arg::Global(name.to_owned())
}

fn pos1() -> Arg {
    Arg::Name("pos1".to_owned())
}

/// `position_set_x pos1 x`, `position_set_y pos1 y`, then `setter overlay pos1`.
fn placed(ops: &[Op], setter: &str, overlay: &Arg, x: i64, y: i64) -> bool {
    ops.windows(3).any(|w| {
        w[0].name == "position_set_x"
            && w[0].args == [pos1(), Arg::Int(x)]
            && w[1].name == "position_set_y"
            && w[1].args == [pos1(), Arg::Int(y)]
            && w[2].name == setter
            && w[2].args == [overlay.clone(), pos1()]
    })
}

fn alex_stamina() -> String {
    std::fs::read_to_string(format!("{}/tests/alex/features/stamina.uw", env!("CARGO_MANIFEST_DIR")))
        .expect("the Alex stamina feature")
}

#[test]
fn alex_stamina_bar_is_one_hud_presentation() {
    let source = alex_stamina();
    let lowered = lower_ok(&[("stamina", &source)]);
    let p = hud(&lowered);
    assert_eq!(p.flags, vec!["prsntf_read_only", "prsntf_manual_end_only"]);
    assert_eq!(p.on_load[0].name, "set_fixed_point_multiplier");
    assert_eq!(p.on_load.last().unwrap().name, "presentation_set_duration");
    let (bg, fill, label) = (
        global("$uw_hud_stamina_bar_bg"),
        global("$uw_hud_stamina_bar_fill"),
        global("$uw_hud_stamina_bar_label"),
    );
    let plane = Arg::Name("mesh_white_plane".to_owned());
    assert!(has(&p.on_load, "create_mesh_overlay", &[bg.clone(), plane.clone()]));
    assert!(has(&p.on_load, "create_mesh_overlay", &[fill.clone(), plane]));
    // size = (220, 14) screen units × 50 for the white plane; bottom_left = 20 from both edges.
    assert!(placed(&p.on_load, "overlay_set_size", &bg, 11_000, 700));
    assert!(placed(&p.on_load, "overlay_set_position", &bg, 20, 20));
    assert!(placed(&p.on_load, "overlay_set_position", &fill, 20, 20));
    assert!(has(&p.on_load, "overlay_set_color", &[fill.clone(), Arg::Int(0x3FA34D)]));
    assert!(has(&p.on_load, "create_text_overlay", &[label.clone(), Arg::Text("@Stamina".to_owned())]));
    assert!(placed(&p.on_load, "overlay_set_position", &label, 20, 38), "label 4 units above the bar");

    // Every frame: width = stamina × 11000 ÷ 100000 (max 100 in thousandths), clamped, then resized.
    let run = &p.on_run;
    assert!(run.iter().any(|op| op.name == "agent_get_slot"), "reads the field: {run:?}");
    let mul = run.iter().position(|op| op.name == "store_mul" && op.args[2] == Arg::Int(11_000)).expect("× width");
    let width = run[mul].args[0].clone();
    assert!(has(run, "val_div", &[width.clone(), Arg::Int(100_000)]));
    assert!(has(run, "val_clamp", &[width.clone(), Arg::Int(0), Arg::Int(11_001)]));
    let resize = run.iter().position(|op| op.name == "overlay_set_size" && op.args[0] == fill).expect("fill resized");
    assert_eq!(run[resize - 2].args, vec![pos1(), width], "the new width goes into pos1");

    // The dispatcher starts the HUD whenever it is not up (also after a full-screen window wiped it).
    let frame = frame(&lowered);
    let guard = frame.iter().position(|op| op.name == "is_presentation_active").expect("guard");
    assert!(frame[guard].neg);
    assert_eq!(frame[guard].args, vec![Arg::Name(format!("prsnt_{HUD}"))]);
    assert_eq!(frame[guard + 1].name, "start_presentation");
    // Frame + mission start + the agent-spawn trigger for `var agent.stamina = 100`; the HUD has none.
    assert_eq!(lowered.mission_triggers.len(), 3, "no trigger of its own");
    let spawn = lowered
        .mission_triggers
        .iter()
        .find(|t| matches!(t.when, uw_core::TriggerWhen::Event(ref e) if e == "ti_on_agent_spawn"))
        .expect("agent-spawn trigger");
    assert_eq!(spawn.body[0].name, "store_trigger_param_1");
    let set = spawn.body.iter().find(|op| op.name == "agent_set_slot").expect("start value written");
    assert_eq!(set.args[0], Arg::Local(":uw_agent".into()));
    assert!(spawn.body.iter().any(|op| op.name == "assign" && op.args[1] == Arg::Int(100_000)), "100 in thousandths");
    assert!(report(&lowered).contains("keep prsnt_uw_hud up (1 parts"), "{}", report(&lowered));
}

#[test]
fn parts_of_many_features_share_the_presentation() {
    let lowered = lower_ok(&[
        ("a", "var hp = 50\nui.bar hp_bar {\n  value = hp\n  position = top_right\n  size = (100, 10)\n}\n"),
        ("b", "var shown = true\nui.text hello {\n  label = \"Hello\"\n  position = (400, 300)\n  visible = shown\n}\n"),
    ]);
    let p = hud(&lowered);
    // top_right: x = 1000 − 20 − 100, y = 750 − 20 − 10.
    assert!(placed(&p.on_load, "overlay_set_position", &global("$uw_hud_hp_bar_bg"), 880, 720));
    let text = global("$uw_hud_hello_text");
    assert!(placed(&p.on_load, "overlay_set_position", &text, 400, 300));
    let display = p.on_run.iter().find(|op| op.name == "overlay_set_display").expect("visible = shown");
    assert_eq!(display.args[0], text);
    // Max 0 cannot stop the frame: the division is guarded.
    assert!(p.on_run.iter().any(|op| op.name == "neq"));
}

#[test]
fn ui_mistakes_are_explained() {
    let msg = lower_err(&[("a", "ui.bar b {\n  value = 1\n  colour = #FFFFFF\n}\n")]);
    assert!(msg.contains("has no property `colour`"), "{msg}");
    let msg = lower_err(&[("a", "ui.bar b {\n  max = 1\n}\n")]);
    assert!(msg.contains("needs `value = …`"), "{msg}");
    let msg = lower_err(&[("a", "ui.bar b {\n  value = 1\n  position = middle\n}\n")]);
    assert!(msg.contains("`position` is an anchor"), "{msg}");
    let msg = lower_err(&[("a", "ui.button go {\n  label = \"Go\"\n}\n")]);
    assert!(msg.contains("`ui.button` is not lowered yet"), "{msg}");
    let msg = lower_err(&[("a", "ui.text t {\n  label = \"x\"\n}\n"), ("b", "ui.text t {\n  label = \"y\"\n}\n")]);
    assert!(msg.contains("a ui part named `t` already exists"), "{msg}");
    let msg = lower_err(&[("a", "ui.bar b {\n  value = 1\n  alpha = 150%\n}\n")]);
    assert!(msg.contains("from 0% to 100%"), "{msg}");
}

#[test]
fn keys_fire_on_the_edge_inside_the_frame() {
    let lowered = lower_ok(&[(
        "keys",
        "var n = 0\non key.pressed(q) {\n  n += 1\n}\non key.pressed(kick) when n < 3 {\n  n = 0\n}\n\
         on key.released(alt) {\n  n -= 1\n}\n",
    )]);
    let frame = frame(&lowered);
    assert!(has(frame, "key_clicked", &[Arg::Name("key_q".into())]), "a keyboard key");
    assert!(has(frame, "game_key_clicked", &[Arg::Name("gk_kick".into())]), "a game control the player can rebind");
    assert!(frame.iter().any(|op| op.name == "lt"), "`when` is a condition of the block");
    // Released: down this frame? remember it, then fire when it was down and is not any more.
    let down = frame.iter().position(|op| op.name == "key_is_down").expect("alt state");
    assert_eq!(frame[down].args, vec![Arg::Name("key_left_alt".into())], "`alt` is the `key.held` alias");
    let previous = global("$uw_keys_key0");
    let saved = frame.iter().position(|op| op.name == "assign" && op.args[0] == previous).expect("state saved");
    let fired = frame.iter().position(|op| op.name == "val_sub").expect("body");
    assert!(saved < fired, "the state is saved before the body runs");
    assert_eq!(lowered.mission_triggers.len(), 2, "keys add no trigger");
    let whats: Vec<&str> = lowered.schedule.iter().map(|s| s.what.as_str()).collect();
    assert_eq!(whats, vec!["on key.pressed(q)", "on key.pressed(kick)", "on key.released(alt)"]);

    let msg = lower_err(&[("a", "on key.pressed(nothing) {\n}\n")]);
    assert!(msg.contains("neither `key_nothing` nor `gk_nothing`"), "{msg}");
    let msg = lower_err(&[("a", "on key.pressed {\n}\n")]);
    assert!(msg.contains("needs the key"), "{msg}");
    let msg = lower_err(&[("a", "var n = 0\non key.pressed(q) {\n  n += 1 per second\n}\n")]);
    assert!(msg.contains("only makes sense in a continuous block"), "{msg}");
}
