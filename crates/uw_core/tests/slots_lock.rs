//! D1 §2): object fields keep their slot numbers between builds through
//! `slots.lock`, so a saved game never reads a value through the wrong field.

use uw_atlas::Atlas;
use uw_core::lower::{lower, lower_locked, CoreData};
use uw_core::slots::SlotLock;
use uw_core::{parse, Lowered};

/// The kit has agent slots up to 5, so the first free agent slot is 6. The ops are the ones a field's
/// start value needs (written when the agent spawns).
fn atlas(top_agent_slot: i64) -> Atlas {
    let text = format!(
        r#"{{"schema": 1, "ops": [
              {{"name": "assign", "code": 1, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true}},
              {{"name": "val_clamp", "code": 2, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true}},
              {{"name": "agent_set_slot", "code": 3, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false}}
            ], "hooks": ["game_start"], "ranges": [],
            "constants": [
              {{"name": "slot_agent_health", "value": 3, "module": "module_constants.py", "line": null}},
              {{"name": "slot_agent_top", "value": {top_agent_slot}, "module": "module_constants.py", "line": null}}
            ],
            "tables": {{}}}}"#
    );
    Atlas::from_str(&text).expect("inline atlas")
}

fn data() -> CoreData {
    CoreData::builtin().expect("committed properties.toml")
}

fn build(source: &str, atlas: &Atlas, lock: &SlotLock) -> Lowered {
    let file = parse("fields", "features/fields.uw", source).unwrap_or_else(|e| panic!("parse: {e:?}"));
    lower_locked(&[file], atlas, &data(), lock).unwrap_or_else(|e| panic!("lower: {e:?}"))
}

fn slot(lowered: &Lowered, name: &str) -> i64 {
    lowered.slot_lock.get("agent", name).unwrap_or_else(|| panic!("agent.{name} not in lock"))
}

#[test]
fn first_build_numbers_above_the_kit_and_records_them() {
    let first = build("var agent.stamina = 100 in 0..100\nvar agent.rage = 0\n", &atlas(5), &SlotLock::new());
    assert_eq!(slot(&first, "stamina"), 6);
    assert_eq!(slot(&first, "rage"), 7);
    assert!(first.warnings.is_empty(), "{:?}", first.warnings);
    // The plain `lower` gives the same numbers (it starts from an empty lock).
    let file = parse("fields", "features/fields.uw", "var agent.stamina = 100 in 0..100\nvar agent.rage = 0\n").unwrap();
    let plain = lower(&[file], &atlas(5), &data()).unwrap();
    assert_eq!(plain.slot_lock, first.slot_lock);
}

#[test]
fn reordering_and_adding_fields_never_moves_an_old_one() {
    let first = build("var agent.stamina = 100\nvar agent.rage = 0\n", &atlas(5), &SlotLock::new());
    // The lock goes through its file text, as it would between two builds.
    let lock = SlotLock::parse(&first.slot_lock.to_text()).expect("lock text parses");
    let second = build("var agent.focus = 1\nvar agent.rage = 0\nvar agent.stamina = 100\n", &atlas(5), &lock);
    assert_eq!(slot(&second, "stamina"), 6, "old field kept its slot");
    assert_eq!(slot(&second, "rage"), 7, "old field kept its slot");
    assert_eq!(slot(&second, "focus"), 8, "a new field goes above every locked one, even when declared first");
}

#[test]
fn a_removed_field_keeps_its_number_reserved_and_warns() {
    let first = build("var agent.stamina = 100\nvar agent.rage = 0\n", &atlas(5), &SlotLock::new());
    let second = build("var agent.stamina = 100\nvar agent.focus = 1\n", &atlas(5), &first.slot_lock);
    assert_eq!(slot(&second, "focus"), 8, "rage's slot 7 must not be given to another field");
    assert_eq!(slot(&second, "rage"), 7, "the removed field stays in the lock");
    assert_eq!(second.warnings.len(), 1, "{:?}", second.warnings);
    assert!(second.warnings[0].message.contains("agent.rage"), "{}", second.warnings[0].message);
}

#[test]
fn a_kit_that_now_uses_a_locked_slot_is_an_error() {
    let first = build("var agent.stamina = 100\n", &atlas(5), &SlotLock::new());
    let file = parse("fields", "features/fields.uw", "var agent.stamina = 100\n").unwrap();
    let errors = lower_locked(&[file], &atlas(6), &data(), &first.slot_lock).expect_err("slot 6 is the kit's now");
    assert!(errors.iter().any(|e| e.message.contains("kit now uses")), "{errors:?}");
}
