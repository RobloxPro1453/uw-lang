//! troop operations for the start dry run (see families/mod.rs for the rules).
//!
//! A troop record keeps everything the kit packs into it: the flag word (type/gender, `tf_hero`,
//! header_troops.py:10-31), `attributes` (str/agi/int/cha plus `level`, header_troops.py:90-213) and
//! `skills` (`knows_X_n`, 4 bits per skill, header_skills.py:9-50, process_troops.py:44-62). Reads go
//! through the record cells, so a write made earlier in the run wins over the record's start value.
//!
//! What a record cannot hold is never invented. A [`Val`] is one number, so a change to the name or
//! to the item list is written as `unknown`, and an operation whose result the engine computes
//! (gold, equipping, attribute clamping) is counted as unmodelled with its reason in the name.
//! `troop_set_age` writes an `age` cell no record field backs (`module_troops.py` has no age field):
//! the trace keeps Native's value, the record printer never shows it.

use serde_json::Value as Json;

use crate::{Cell, Flow, Machine, Val};

/// `troop_type_mask`: gender/type is the low nibble of `troop_flags` (header_troops.py:10-13).
const TYPE_MASK: i64 = 0x0f;
/// `tf_hero` (header_troops.py:14).
const HERO_BIT: i64 = 0x10;
/// `ca_strength`..`ca_charisma` = 0..3, one byte each in `attributes` (header_troops.py:34-37, :90-205).
const ATTRIBUTE_COUNT: i64 = 4;
const ATTRIBUTE_SHIFT: i64 = 8;
const ATTRIBUTE_MASK: i64 = 0xff;
/// `level(N)` sits in the top byte of `attributes` (header_troops.py:207-213).
const LEVEL_SHIFT: i64 = 32;
/// `knows_X_n` is `n << (4 * skill index)`; the kit has 42 skill indices (header_skills.py:9-50, :474).
const SKILL_COUNT: i64 = 42;
const SKILL_SHIFT: i64 = 4;
const SKILL_MASK: i64 = 0x0f;

pub fn exec(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    let v = |m: &Machine, i: usize| args.get(i).map(|a| m.get(a)).unwrap_or_default();
    let dest = |i: usize| args.get(i).cloned().unwrap_or(Json::Null);
    match name {
        // ---- reads -----------------------------------------------------------------------
        // The type/gender is the low nibble of the flag word.
        "troop_get_type" => {
            let troop = v(m, 1);
            let flags = read_field(m, &troop, "troop_flags");
            m.set(&dest(0), Val::derived(flags.n & TYPE_MASK, &[&flags, &troop]));
            Some(Flow::Done)
        }
        "troop_is_hero" => {
            let troop = v(m, 0);
            let flags = read_field(m, &troop, "troop_flags");
            if troop.unknown || flags.unknown {
                m.unknown_op(&format!("{name} (unknown troop flags)"), false, args);
                return Some(Flow::Done);
            }
            Some(if flags.n & HERO_BIT != 0 { Flow::Done } else { Flow::Fail })
        }
        // The kit names opcode 2173 both `store_troop_faction` and `store_faction_of_troop`
        // (header_operations.py:1224-1225).
        "store_troop_faction" | "store_faction_of_troop" => {
            let troop = v(m, 1);
            let faction = read_field(m, &troop, "faction");
            m.set(&dest(0), faction);
            Some(Flow::Done)
        }
        // (destination, troop, attribute), unlike store_skill_level (header_operations.py:1222).
        "store_attribute_level" => {
            let (troop, attribute) = (v(m, 1), v(m, 2));
            if attribute.unknown || !(0..ATTRIBUTE_COUNT).contains(&attribute.n) {
                m.unknown_op(&format!("{name} (unknown attribute)"), true, args);
                return Some(Flow::Done);
            }
            let attributes = read_field(m, &troop, "attributes");
            let level = (attributes.n >> (ATTRIBUTE_SHIFT * attribute.n)) & ATTRIBUTE_MASK;
            m.set(&dest(0), Val::derived(level, &[&attributes, &troop, &attribute]));
            Some(Flow::Done)
        }
        // (destination, [troop]): what the engine reads for a missing troop is not documented, so
        // the level stays unknown instead of guessing a troop.
        "store_character_level" => {
            let Some(operand) = args.get(1) else {
                m.unknown_op(&format!("{name} (no troop operand)"), true, args);
                return Some(Flow::Done);
            };
            let troop = m.get(operand);
            let attributes = read_field(m, &troop, "attributes");
            let level = (attributes.n >> LEVEL_SHIFT) & ATTRIBUTE_MASK;
            m.set(&dest(0), Val::derived(level, &[&attributes, &troop]));
            Some(Flow::Done)
        }
        // (destination, skill, [troop]): the troop is last and optional (header_operations.py:1220).
        "store_skill_level" => {
            let skill = v(m, 1);
            let Some(operand) = args.get(2) else {
                m.unknown_op(&format!("{name} (no troop operand)"), true, args);
                return Some(Flow::Done);
            };
            if skill.unknown || !(0..SKILL_COUNT).contains(&skill.n) {
                m.unknown_op(&format!("{name} (unknown skill)"), true, args);
                return Some(Flow::Done);
            }
            let troop = m.get(operand);
            let skills = read_field(m, &troop, "skills");
            let level = (skills.n >> (SKILL_SHIFT * skill.n)) & SKILL_MASK;
            m.set(&dest(0), Val::derived(level, &[&skills, &troop, &skill]));
            Some(Flow::Done)
        }
        // 0 = first target, 1 = second, -1 when there is none (header_operations.py:635); the
        // record's padding target is 0 (`upgrade_1`/`upgrade_2` in kit-meta/records/troops.toml).
        "troop_get_upgrade_troop" => {
            let (troop, path) = (v(m, 1), v(m, 2));
            if path.unknown || !(0..2).contains(&path.n) {
                m.unknown_op(&format!("{name} (unknown upgrade path)"), true, args);
                return Some(Flow::Done);
            }
            let field = if path.n == 0 { "upgrade_1" } else { "upgrade_2" };
            let upgrade = read_field(m, &troop, field);
            let target = if upgrade.n == 0 { -1 } else { upgrade.n };
            m.set(&dest(0), Val::derived(target, &[&upgrade, &troop, &path]));
            Some(Flow::Done)
        }
        // ---- writes ----------------------------------------------------------------------
        "troop_set_faction" => {
            let (troop, faction) = (v(m, 0), v(m, 1));
            if cannot_write(m, name, &troop, args) {
                return Some(Flow::Done);
            }
            let value = Val::derived(faction.n, &[&faction, &troop]);
            m.write(Cell::field("troops", troop.n, "faction"), value);
            Some(Flow::Done)
        }
        // The record has no age field (module_troops.py:13-33): the trace keeps Native's value under
        // an `age` cell, so the start snapshot shows it.
        "troop_set_age" => {
            let (troop, age) = (v(m, 0), v(m, 1));
            if cannot_write(m, name, &troop, args) {
                return Some(Flow::Done);
            }
            let value = Val::derived(age.n, &[&age, &troop]);
            m.write(Cell::field("troops", troop.n, "age"), value);
            Some(Flow::Done)
        }
        // The name is record text and a `Val` holds one number, so the change stays unknown.
        "troop_set_name" => {
            let troop = v(m, 0);
            if cannot_write(m, name, &troop, args) {
                return Some(Flow::Done);
            }
            m.write(Cell::field("troops", troop.n, "name"), Val::unknown());
            Some(Flow::Done)
        }
        // The inventory is a list of items and merchandise is picked by the engine's random
        // generator: the trace records the change as unknown, never as a made-up list.
        "troop_clear_inventory" | "troop_add_item" | "troop_add_items" | "troop_remove_items"
        | "troop_add_merchandise" => {
            let troop = v(m, 0);
            if cannot_write(m, name, &troop, args) {
                return Some(Flow::Done);
            }
            m.write(Cell::field("troops", troop.n, "inventory"), Val::unknown());
            if name == "troop_remove_items" {
                // It puts the items' cost in reg0 (header_operations.py:622): the engine computes it.
                m.unknown_op(&format!("{name} (item cost in reg0)"), false, args);
                m.registers.insert(0, Val::unknown());
            }
            Some(Flow::Done)
        }
        // ---- explicitly unknown ----------------------------------------------------------
        "troop_equip_items" | "troop_has_item_equipped" => {
            // Which items the engine equips is engine state; a record keeps only the item list.
            m.unknown_op(&format!("{name} (equipment is engine state)"), false, args);
            Some(Flow::Done)
        }
        "troop_ensure_inventory_space" => {
            // Inventory capacity is engine state; the record keeps only the item list.
            m.unknown_op(&format!("{name} (inventory capacity is engine state)"), false, args);
            Some(Flow::Done)
        }
        "troop_raise_attribute" => {
            // Whether the engine clamps the raised attribute is undocumented
            //.
            m.unknown_op(&format!("{name} (engine clamp)"), false, args);
            Some(Flow::Done)
        }
        "store_troop_gold" => {
            // Gold is not a troop record field (header_operations.py:1204).
            m.unknown_op(&format!("{name} (gold is not a record field)"), true, args);
            Some(Flow::Done)
        }
        _ => None,
    }
}

/// Reads `field` of a troop record and keeps its dependencies (the way `<kind>_get_slot` does).
fn read_field(m: &mut Machine, troop: &Val, field: &str) -> Val {
    let read = m.read(Cell::field("troops", troop.n, field));
    Val::derived(read.n, &[&read, troop])
}

/// True when a writing operation's troop operand cannot name a record: it is counted as unmodelled
/// and the caller writes nothing (the `<kind>_set_slot (unknown target)` rule of lib.rs).
fn cannot_write(m: &mut Machine, name: &str, troop: &Val, args: &[Json]) -> bool {
    if !troop.unknown {
        return false;
    }
    m.unknown_op(&format!("{name} (unknown troop)"), false, args);
    true
}
