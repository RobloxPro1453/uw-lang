//! The engine's words come from the kit: the core
//! language reads `kit-meta/easy/ops.json`; the hand-made `properties.toml` holds only mechanism, never a
//! kit name with another meaning and never a threshold.

use uw_core::lower::kit::{kit_ops, kit_properties};
use uw_core::lower::CoreData;
use uw_core::{Arg, Op};

mod common;
use common::lower_ok;

fn ops_of(source: &str) -> Vec<Op> {
    let lowered = lower_ok(&[("a", source)]);
    let mut ops: Vec<Op> = lowered.scripts.iter().flat_map(|(_, ops)| ops.clone()).collect();
    ops.extend(lowered.script_appends.iter().flat_map(|(_, ops)| ops.clone()));
    ops
}

fn names(ops: &[Op]) -> Vec<&str> {
    ops.iter().map(|op| op.name.as_str()).collect()
}

fn find<'a>(ops: &'a [Op], name: &str) -> &'a Op {
    ops.iter().find(|op| op.name == name).unwrap_or_else(|| panic!("no '{name}' in {:?}", names(ops)))
}

#[test]
fn the_kit_gives_hundreds_of_words() {
    let props = kit_properties(&kit_ops());
    assert!(props.len() > 250, "only {} kit words", props.len());
    let get = |path: &str| props.iter().find(|p| p.path == path).unwrap_or_else(|| panic!("no kit word '{path}'"));
    assert!(get("agent.speed").vector, "agent.speed is the agent's velocity (a position register)");
    assert_eq!(get("agent.speed_modifier").set[0].op, "agent_set_speed_modifier");
    assert_eq!(get("agent.is_alive").cond[0].op, "agent_is_alive");
    let hp = get("agent.hit_points");
    assert_eq!(hp.get[0].op, "store_agent_hit_points", "read from the line under the assignment");
    assert_eq!(hp.set[0].op, "agent_set_hit_points");
}

#[test]
fn a_hand_made_entry_never_reuses_a_kit_name_or_holds_a_number() {
    let kit: Vec<String> = kit_properties(&kit_ops()).into_iter().map(|p| p.path).collect();
    for prop in CoreData::hand_made().expect("properties.toml parses").properties {
        assert!(!kit.contains(&prop.path), "'{}' is a kit word: use the kit's entry, not a hand-made one", prop.path);
        for step in prop.get.iter().chain(&prop.set).chain(&prop.cond) {
            for arg in &step.args {
                assert!(
                    arg.parse::<i64>().is_err(),
                    "'{}' holds the number {arg}: a threshold or design number is the modder's code",
                    prop.path
                );
            }
        }
    }
}

#[test]
fn a_vector_as_a_number_is_its_length_in_metres() {
    let ops = ops_of("while player.agent.speed > 0.5 {\n  player.agent.speed_modifier = 150%\n}\n");
    let speed = find(&ops, "agent_get_speed");
    assert!(matches!(&speed.args[0], Arg::Name(reg) if reg.starts_with("pos")), "{speed:?}");
    let distance = find(&ops, "get_distance_between_positions");
    assert_eq!(distance.args[1], speed.args[0], "the length is measured from the agent's velocity");
    let gt = find(&ops, "gt");
    assert_eq!(gt.args[1], Arg::Int(500), "0.5 m/s in thousandths");
    let set = find(&ops, "agent_set_speed_modifier");
    assert_eq!(set.args[1], Arg::Int(150), "150% is the number 150 the engine wants");
}

#[test]
fn the_owner_sketch_speed_not_zero_compiles() {
    let ops = ops_of("while player.agent.speed != 0 {\n  player.agent.speed_modifier = 150%\n}\n");
    assert!(names(&ops).contains(&"agent_get_speed"));
}

#[test]
fn a_component_is_read_in_metres() {
    let ops = ops_of("var agent.forward = 0\nwhile in mission {\n  player.agent.forward = player.agent.speed.y\n}\n");
    let names = names(&ops);
    let at = names.iter().position(|n| *n == "position_get_y").expect("position_get_y");
    assert_eq!(names[at - 1], "set_fixed_point_multiplier");
    assert_eq!(ops[at - 1].args, vec![Arg::Int(1000)], "metres in thousandths, like every number");
}

#[test]
fn a_kit_value_is_scaled_and_an_object_is_not() {
    let ops = ops_of("var agent.hp = 0\nwhile in mission {\n  player.agent.hp = player.agent.hit_points\n}\n");
    let names = names(&ops);
    let read = names.iter().position(|n| *n == "store_agent_hit_points").expect("store_agent_hit_points");
    let agent = &ops[read].args[1];
    assert!(matches!(agent, Arg::Local(_)));
    let fetch = ops[..read].iter().rev().find(|op| op.name == "get_player_agent_no").expect("the player's agent");
    assert_eq!(&fetch.args[0], agent, "the object goes in as the engine's id, unscaled");
    assert_eq!(ops[read + 1].name, "val_mul");
    assert_eq!(ops[read + 1].args[1], Arg::Int(1000), "whole hit points → the language's thousandths");
}

#[test]
fn a_kit_condition_is_a_bare_condition() {
    let ops = ops_of("while player.agent.is_alive {\n  player.agent.speed_modifier = 100%\n}\n");
    let alive = find(&ops, "agent_is_alive");
    assert!(matches!(&alive.args[0], Arg::Local(_)));
    assert!(!names(&ops).contains(&"neq"), "a condition, not a value compared with 0");
}

#[test]
fn alex_sprint_is_written_with_kit_words_only() {
    let stamina = "var agent.stamina = 100 in 0..100\nwhile in mission {\n  player.agent.stamina += 15 per second\n}\n";
    let sprint = std::fs::read_to_string(format!("{}/../../examples/alex/features/sprint.uw", env!("CARGO_MANIFEST_DIR")))
        .expect("the Alex sprint example");
    let lowered = lower_ok(&[("sprint", sprint.as_str()), ("stamina", stamina)]);
    let ops: Vec<Op> = lowered.scripts.iter().flat_map(|(_, ops)| ops.clone()).collect();
    let names = names(&ops);
    let key = names.iter().position(|n| *n == "key_is_down").expect("Alt is checked");
    let speed = names.iter().position(|n| *n == "agent_get_speed").expect("movement is read from the kit's agent.speed");
    let set = names.iter().position(|n| *n == "agent_set_speed_modifier").expect("speed modifier is set");
    assert!(key < speed && speed < set, "Alt and movement gate the sprint: {names:?}");
}
