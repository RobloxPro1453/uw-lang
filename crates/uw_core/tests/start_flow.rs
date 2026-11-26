//! D4b: the start flow written with the kit's own words (ROADMAP D4b, CORE-v0 §4 "Kit things").
//! A kit thing by its kind and kit name (`troop.player`, `party.town_6`), the kit's actions
//! (`scene.town_6_center.jump(0)`, `screen.mission()`) and `on menu.<name>.enter`; nothing here is a word
//! the compiler made up for Alex.

use uw_core::{Arg, Op};

mod common;
use common::{lower_err, lower_ok};

fn name(text: &str) -> Arg {
    Arg::Name(text.to_owned())
}

fn find<'a>(ops: &'a [Op], op: &str) -> &'a Op {
    ops.iter().find(|o| o.name == op).unwrap_or_else(|| panic!("no '{op}' in {ops:?}"))
}

fn start_ops(source: &str) -> Vec<Op> {
    let lowered = lower_ok(&[("main", source)]);
    lowered.script_appends.iter().flat_map(|(_, ops)| ops.clone()).collect()
}

#[test]
fn a_kit_thing_is_named_by_its_kind_and_kit_name() {
    let ops = start_ops("on game.start {\n  troop.player.name = \"Alex\"\n}\n");
    let store = find(&ops, "str_store_string");
    assert_eq!(store.args[1], Arg::Text("@Alex".to_owned()));
    let set = find(&ops, "troop_set_name");
    assert_eq!(set.args, vec![name("trp_player"), store.args[0].clone()], "the name goes in through the register");
}

#[test]
fn a_kit_thing_takes_only_the_words_of_its_own_kind() {
    // `troop.player` ends in `player`, but `player.face_keys` is a multiplayer player's word, not a troop's.
    let ops = start_ops("on game.start {\n  troop.player.face_keys = \"0x01\"\n}\n");
    assert_eq!(find(&ops, "troop_set_face_keys").args[0], name("trp_player"));
    assert!(ops.iter().all(|op| op.name != "player_set_face_keys"), "{ops:?}");
}

#[test]
fn a_kit_action_takes_ids_as_they_are_and_numbers_in_whole_units() {
    let ops = start_ops(
        "on game.start {\n  party.main_party.relocate_near_party(party.town_6, 1)\n  scene.town_6_center.jump(0)\n  screen.mission()\n}\n",
    );
    assert_eq!(
        find(&ops, "party_relocate_near_party").args,
        vec![name("p_main_party"), name("p_town_6"), Arg::Int(1)],
        "the radius 1 is the engine's 1, not 1000"
    );
    assert_eq!(find(&ops, "jump_to_scene").args, vec![name("scn_town_6_center"), Arg::Int(0)]);
    assert!(find(&ops, "change_screen_mission").args.is_empty());
}

#[test]
fn a_setter_of_a_thing_takes_its_id() {
    let ops = start_ops("on game.start {\n  mission.jump = mission_template.town_center\n}\n");
    assert_eq!(find(&ops, "set_jump_mission").args, vec![name("mt_town_center")], "an id is not divided by 1000");
}

#[test]
fn a_menu_enter_block_is_appended_to_the_kit_menu() {
    let source = "var walked_in = 0\non menu.start_game_0.enter {\n  if walked_in == 0 {\n    walked_in = 1\n    screen.mission()\n  } else {\n    screen.map()\n  }\n}\n";
    let lowered = lower_ok(&[("main", source)]);
    assert_eq!(lowered.menu_appends.len(), 1, "{:?}", lowered.menu_appends);
    let (menu, ops) = &lowered.menu_appends[0];
    assert_eq!(menu, "start_game_0", "the menu's kit name, without its tag");
    let names: Vec<&str> = ops.iter().map(|op| op.name.as_str()).collect();
    assert_eq!(names.first(), Some(&"try_begin"), "{names:?}");
    assert!(names.contains(&"change_screen_mission") && names.contains(&"change_screen_map"), "{names:?}");
}

#[test]
fn a_thing_the_kit_does_not_have_is_an_error() {
    let message = lower_err(&[("main", "on game.start {\n  troop.nobody.name = \"x\"\n}\n")]);
    assert!(message.contains("troop.nobody"), "{message}");
    let message = lower_err(&[("main", "on menu.no_such_menu.enter {\n  screen.map()\n}\n")]);
    assert_eq!(message, "the kit has no menu 'no_such_menu'");
}

#[test]
fn an_action_is_not_a_value() {
    let message = lower_err(&[("main", "var x = 0\non game.start {\n  x = screen.map()\n}\n")]);
    assert!(message.contains("is an action"), "{message}");
}

#[test]
fn a_kit_constant_is_written_by_its_easy_name() {
    // `troop_raise_attribute (troop_id, attribute_id, value)`: its own `value` parameter is the call's.
    let ops = start_ops("on game.start {\n  troop.player.raise_attribute(ca.strength, 3)\n}\n");
    assert_eq!(find(&ops, "troop_raise_attribute").args, vec![name("trp_player"), name("ca_strength"), Arg::Int(3)]);
}

#[test]
fn a_menu_is_opened_with_the_kits_jump() {
    let lowered = lower_ok(&[("main", "on menu.start_game_0.enter {\n  menu.start_phase_2.jump()\n}\n")]);
    let (_, ops) = &lowered.menu_appends[0];
    assert_eq!(find(ops, "jump_to_menu").args, vec![name("mnu_start_phase_2")]);
}
