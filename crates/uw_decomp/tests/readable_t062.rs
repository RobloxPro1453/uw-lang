//! T-062: the typed fields of factions, map_icons, quests and skills print readably on the real
//! atlas, and every record of all five tables (with info_pages) still parses back identical.
//!
//! Each test loads its schema first (so a broken `.toml` fails even without the atlas) and skips
//! itself when `target/uw-staging/atlas.json` is absent (docs/DEVELOPMENT.md).

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

fn load(table: &str) -> Option<(Atlas, Schema)> {
    let schema_path =
        format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    let schema = Schema::from_path(&schema_path)
        .unwrap_or_else(|e| panic!("{table} schema loads: {e}"));
    let atlas_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(atlas_path).exists() {
        eprintln!("skipped: no atlas at {atlas_path}");
        return None;
    }
    let atlas = Atlas::from_path(atlas_path).expect("full atlas loads");
    Some((atlas, schema))
}

/// Every record of `table` must be exact, and the schema must type exactly `typed_fields` fields
/// while `raw_rest` values still print raw (a silent fallback would raise that count).
fn roundtrip(atlas: &Atlas, schema: &Schema, records: usize, typed_fields: usize, raw_rest: usize) {
    let report = roundtrip_table(atlas, schema);
    assert_eq!(report.records, records, "{} record count", schema.table);
    assert_eq!(
        report.exact, report.records,
        "{}: first failures: {:?}",
        schema.table, report.first_failures
    );
    assert_eq!(report.failed, 0, "{}: first failures: {:?}", schema.table, report.first_failures);
    assert_eq!(
        (report.typed_fields, report.raw_rest),
        (typed_fields, raw_rest),
        "{} coverage",
        schema.table
    );
}

/// The printed text of the record whose atlas id is `id` (the id includes the schema prefix).
fn print_entry(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let entry = atlas.tables[schema.table.as_str()]
        .entries
        .iter()
        .find(|entry| entry.id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("no `{id}` in `{}`", schema.table));
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

#[test]
fn real_atlas_factions_print_typed_fields() {
    let Some((atlas, schema)) = load("factions") else { return };
    roundtrip(&atlas, &schema, 34, 1, 182);
    // max_player_rating(-30) = (100 - -30) << 8 = 33280 in module_factions.py:23.
    let text = print_entry(&atlas, &schema, "fac_outlaws");
    assert!(
        text.contains("  faction_flags {always_hide_label 0, max_player_rating 130}"),
        "{text}"
    );
    // ff_always_hide_label = 1 in module_factions.py:27.
    let text = print_entry(&atlas, &schema, "fac_innocents");
    assert!(
        text.contains("  faction_flags {always_hide_label 1, max_player_rating 0}"),
        "{text}"
    );
}

#[test]
fn real_atlas_map_icons_print_typed_fields() {
    let Some((atlas, schema)) = load("map_icons") else { return };
    roundtrip(&atlas, &schema, 189, 2, 615);
    // module_map_icons.py:36: mcn_no_shadow and the default sound 0 (snd_click).
    let text = print_entry(&atlas, &schema, "icon_town");
    assert!(text.contains("  map_icon_flags [mcn_no_shadow]"), "{text}");
    assert!(text.contains("  sound snd_click"), "{text}");
    // module_map_icons.py:24: no flags and snd_footstep_grass (17).
    let text = print_entry(&atlas, &schema, "icon_player");
    assert!(text.contains("  map_icon_flags []"), "{text}");
    assert!(text.contains("  sound snd_footstep_grass"), "{text}");
}

#[test]
fn real_atlas_quests_print_typed_fields() {
    let Some((atlas, schema)) = load("quests") else { return };
    roundtrip(&atlas, &schema, 91, 1, 273);
    // qf_random_quest = 2 (header_quests.py:11) on deliver_message.
    let text = print_entry(&atlas, &schema, "qst_deliver_message");
    assert!(text.contains("  quest_flags [qf_random_quest]"), "{text}");
    // No flag at all prints an empty list (there is no zero constant).
    let text = print_entry(&atlas, &schema, "qst_quests_end");
    assert!(text.contains("  quest_flags []"), "{text}");
}

#[test]
fn real_atlas_skills_print_typed_fields() {
    let Some((atlas, schema)) = load("skills") else { return };
    roundtrip(&atlas, &schema, 42, 1, 168);
    // sf_base_att_cha|sf_effects_party = 3|16 = 19 (module_skills.py:42).
    let text = print_entry(&atlas, &schema, "skl_trade");
    assert!(text.contains("  skill_flags [sf_base_att_cha, sf_effects_party]"), "{text}");
    // sf_base_att_cha|sf_inactive = 3|256 = 259 (module_skills.py:45).
    let text = print_entry(&atlas, &schema, "skl_reserved_1");
    assert!(text.contains("  skill_flags [sf_base_att_cha, sf_inactive]"), "{text}");
    // sf_base_att_int = 2 (module_skills.py:49).
    let text = print_entry(&atlas, &schema, "skl_persuasion");
    assert!(text.contains("  skill_flags [sf_base_att_int]"), "{text}");
}

#[test]
fn real_atlas_info_pages_stay_raw_but_exact() {
    // The kit states no type for any of the three positions: name and description are strings.
    let Some((atlas, schema)) = load("info_pages") else { return };
    roundtrip(&atlas, &schema, 6, 0, 18);
    let text = print_entry(&atlas, &schema, "ip_morale");
    assert!(text.contains("  name \"Morale\""), "{text}");
    assert!(text.contains("  field_3 \"Morale represents the ability"), "{text}");
}
