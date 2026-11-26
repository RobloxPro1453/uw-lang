//! T-064: menus, mission_templates and presentations print their kit-stated fields readably with
//! the T-058 types and parse back identical. The real-atlas tests skip when the atlas is absent,
//! like `records_t042.rs`.

use std::path::Path;

use uw_atlas::{Atlas, Entry};
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

fn atlas_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json")
}

fn schema_path(table: &str) -> String {
    format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"))
}

/// Loads the real atlas and the table's schema; `None` when the atlas is absent.
fn load(table: &str) -> Option<(Atlas, Schema)> {
    let path = atlas_path();
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return None;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    let schema = Schema::from_path(schema_path(table)).expect("schema loads");
    Some((atlas, schema))
}

/// One named entry of an atlas table.
fn find<'a>(atlas: &'a Atlas, table: &str, id: &str) -> &'a Entry {
    atlas.tables[table]
        .entries
        .iter()
        .find(|entry| entry.id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("table `{table}` has no entry `{id}`"))
}

/// The readable record text of one entry (no operation blocks).
fn text_of(atlas: &Atlas, schema: &Schema, table: &str, id: &str) -> String {
    print_with_blocks(atlas, schema, find(atlas, table, id), &[], &mut |_| {
        unreachable!("no block paths were given")
    })
}

/// Every record round-trips exactly; prints the coverage line for the report.
fn assert_all_exact(atlas: &Atlas, schema: &Schema, table: &str) {
    let report = roundtrip_table(atlas, schema);
    eprintln!(
        "{table}: {} records typed {}/{} fields, raw-rest {} values",
        report.records, report.typed_fields, report.fields, report.raw_rest
    );
    assert!(report.records > 0, "the atlas holds no `{table}` records");
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(report.exact, report.records);
}

/// The schema declarations the kit supports; this runs even when the real atlas is absent.
#[test]
fn schemas_declare_the_kit_stated_types() {
    let menus = Schema::from_path(schema_path("menus")).expect("menus schema loads");
    assert_eq!(menus.fields[1].name, "flags");
    assert_eq!(menus.fields[1].ty, "bits");
    let parts: Vec<(&str, Option<&str>, u32, u32)> = menus.fields[1]
        .parts
        .iter()
        .map(|part| (part.name.as_str(), part.ty.as_deref(), part.shift, part.width))
        .collect();
    assert_eq!(
        parts,
        [("menu_flags", Some("flags:mnf_"), 0, 32), ("text_color", None, 32, 32)]
    );
    for (i, field) in menus.fields.iter().enumerate() {
        if i != 1 {
            assert_eq!(field.ty, "raw", "menu field `{}`", field.name);
        }
    }

    let mission = Schema::from_path(schema_path("mission_templates")).expect("mission schema loads");
    assert_eq!(mission.fields[1].name, "mission_template_flags");
    assert_eq!(
        mission.fields[1].ty,
        "flags:mtf_arena_fight,mtf_team_fight,mtf_battle_mode,mtf_commit_casualties,\
         mtf_no_blood,mtf_synch_inventory"
    );
    for (i, field) in mission.fields.iter().enumerate() {
        if i != 1 {
            assert_eq!(field.ty, "raw", "mission-template field `{}`", field.name);
        }
    }

    let presentations = Schema::from_path(schema_path("presentations")).expect("presentation schema loads");
    assert_eq!(presentations.fields[1].name, "presentation_flags");
    assert_eq!(presentations.fields[1].ty, "flags:prsntf_");
    assert_eq!(presentations.fields[2].name, "background_mesh");
    assert_eq!(presentations.fields[2].ty, "ref:meshes");
    for (i, field) in presentations.fields.iter().enumerate() {
        if i != 1 && i != 2 {
            assert_eq!(field.ty, "raw", "presentation field `{}`", field.name);
        }
    }
}

#[test]
fn real_atlas_menus_round_trip() {
    let Some((atlas, schema)) = load("menus") else { return };
    assert_all_exact(&atlas, &schema, "menus");
    let start = text_of(&atlas, &schema, "menus", "menu_start_game_0");
    assert!(start.contains("menu_flags [mnf_disable_all_keys]"), "{start}");
    assert!(start.contains("text_color 4278190080"), "{start}");
    let zendar = text_of(&atlas, &schema, "menus", "menu_zendar");
    assert!(zendar.contains("menu_flags [mnf_auto_enter]"), "{zendar}");
    let camp = text_of(&atlas, &schema, "menus", "menu_camp");
    assert!(camp.contains("menu_flags [mnf_scale_picture]"), "{camp}");
}

#[test]
fn real_atlas_mission_templates_round_trip() {
    let Some((atlas, schema)) = load("mission_templates") else { return };
    assert_all_exact(&atlas, &schema, "mission_templates");
    let lead = text_of(&atlas, &schema, "mission_templates", "mst_lead_charge");
    assert!(
        lead.contains("mission_template_flags [mtf_battle_mode, mtf_synch_inventory]"),
        "{lead}"
    );
    let alley = text_of(&atlas, &schema, "mission_templates", "mst_alley_fight");
    assert!(alley.contains("mission_template_flags [mtf_battle_mode]"), "{alley}");
    let arena = text_of(&atlas, &schema, "mission_templates", "mst_arena_challenge_fight");
    assert!(arena.contains("mtf_commit_casualties"), "{arena}");
}

#[test]
fn real_atlas_presentations_round_trip() {
    let Some((atlas, schema)) = load("presentations") else { return };
    assert_all_exact(&atlas, &schema, "presentations");
    let credits = text_of(&atlas, &schema, "presentations", "prsnt_game_credits");
    assert!(credits.contains("presentation_flags [prsntf_read_only]"), "{credits}");
    assert!(credits.contains("background_mesh mesh_load_window"), "{credits}");
    let designer =
        text_of(&atlas, &schema, "presentations", "prsnt_game_custom_battle_designer");
    assert!(designer.contains("presentation_flags [prsntf_manual_end_only]"), "{designer}");
    assert!(designer.contains("background_mesh mesh_cb_ui_main"), "{designer}");
    let banner = text_of(&atlas, &schema, "presentations", "prsnt_banner_selection");
    assert!(banner.contains("presentation_flags []"), "{banner}");
}
