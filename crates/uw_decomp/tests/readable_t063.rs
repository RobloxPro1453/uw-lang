//! T-063: the readable fields of scenes, scene_props, meshes and tableau_materials. Every record
//! must still print as UW record text and parse back identical, and the fields the kit explains
//! (the flag sets) must print their names, not a bare number. The tests skip themselves when the
//! real atlas is absent (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md).

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

fn atlas_path() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json")
}

fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{table} schema loads: {e}"))
}

/// Loads the real atlas and the table's schema; `None` when the atlas is absent.
fn load(table: &str) -> Option<(Atlas, Schema)> {
    let path = atlas_path();
    let schema = schema(table);
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return None;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    Some((atlas, schema))
}

/// How many values the table's records hold in all.
fn total_values(atlas: &Atlas, table: &str) -> usize {
    atlas.tables[table]
        .entries
        .iter()
        .map(|entry| entry.value.as_array().map_or(1, Vec::len))
        .sum()
}

/// The printed record text of `id`, the full atlas id (`scn_random_scene`).
fn text_of(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let (table, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no `{id}` in the atlas"));
    assert_eq!(table, schema.table, "`{id}` is in the `{}` table", schema.table);
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no block paths"))
}

/// The schema side of the task: only the fields the kit explains are typed.
#[test]
fn the_four_schemas_type_what_the_kit_states() {
    let typed = |table: &str| -> Vec<(String, String)> {
        schema(table)
            .fields
            .into_iter()
            .filter(|field| field.ty != "raw")
            .map(|field| (field.name, field.ty))
            .collect()
    };
    assert_eq!(typed("scenes"), [("scene_flags".to_string(), "flags:sf_".to_string())]);
    assert_eq!(typed("scene_props"), [("scene_prop_flags".to_string(), "flags:sokf_".to_string())]);
    assert_eq!(typed("meshes"), [("mesh_flags".to_string(), "flags:render_order_plus_1".to_string())]);
    // The kit's header_tableau_materials.py declares no flag, so nothing there is typable.
    assert!(typed("tableau_materials").is_empty());
    // The low byte of a scene prop's flags is one type value, not a set of bits.
    let scene_props = schema("scene_props");
    let flags = scene_props.fields.iter().find(|field| field.name == "scene_prop_flags").expect("flags");
    assert_eq!(flags.enums.len(), 1);
    assert_eq!(flags.enums[0].prefix, "sokf_type_");
    let mask = format!("{:?}", flags.enums[0].mask);
    assert!(mask.contains("sokf_type_mask"), "the type byte is masked by sokf_type_mask: {mask}");
}

#[test]
fn real_atlas_scenes_round_trip_readable() {
    let Some((atlas, schema)) = load("scenes") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "scenes typed {}/{} fields, raw-rest {} values of {}",
        report.typed_fields,
        report.fields,
        report.raw_rest,
        total_values(&atlas, "scenes")
    );
    assert_eq!(report.records, 554);
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields), (1, 11));
    // `scene_flags` decodes for every record: a raw fallback would leave one more raw value.
    assert_eq!(report.raw_rest, total_values(&atlas, "scenes") - report.records);
    let text = text_of(&atlas, &schema, "scn_random_scene");
    assert!(
        text.contains("\n  scene_flags [sf_generate, sf_randomize, sf_auto_entry_points]\n"),
        "{text}"
    );
    let text = text_of(&atlas, &schema, "scn_novice_ground");
    assert!(text.contains("\n  scene_flags [sf_indoors]\n"), "{text}");
}

#[test]
fn real_atlas_scene_props_round_trip_readable() {
    let Some((atlas, schema)) = load("scene_props") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "scene_props typed {}/{} fields, raw-rest {} values of {}",
        report.typed_fields,
        report.fields,
        report.raw_rest,
        total_values(&atlas, "scene_props")
    );
    assert_eq!(report.records, 1046);
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields), (1, 5));
    assert_eq!(report.raw_rest, total_values(&atlas, "scene_props") - report.records);
    let text = text_of(&atlas, &schema, "spr_chest_a");
    assert!(text.contains("\n  scene_prop_flags [sokf_type_container]\n"), "{text}");
    let text = text_of(&atlas, &schema, "spr_barrier_20m");
    assert!(text.contains("\n  scene_prop_flags [sokf_type_barrier, sokf_invisible]\n"), "{text}");
    let text = text_of(&atlas, &schema, "spr_inventory");
    assert!(
        text.contains("\n  scene_prop_flags [sokf_type_container, sokf_place_at_origin]\n"),
        "{text}"
    );
    // The packed use time (header_scene_props.py:37,39-49) is not a flag bit: it prints as `rest`.
    let text = text_of(&atlas, &schema, "spr_door_destructible");
    assert!(
        text.contains(
            "\n  scene_prop_flags [sokf_show_hit_point_bar, sokf_destructible, sokf_moveable, rest 536870912]\n"
        ),
        "{text}"
    );
}

#[test]
fn real_atlas_meshes_round_trip_readable() {
    let Some((atlas, schema)) = load("meshes") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "meshes typed {}/{} fields, raw-rest {} values of {}",
        report.typed_fields,
        report.fields,
        report.raw_rest,
        total_values(&atlas, "meshes")
    );
    assert_eq!(report.records, 555);
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields), (1, 12));
    assert_eq!(report.raw_rest, total_values(&atlas, "meshes") - report.records);
    let text = text_of(&atlas, &schema, "mesh_checkbox_off");
    assert!(text.contains("\n  mesh_flags [render_order_plus_1]\n"), "{text}");
    let text = text_of(&atlas, &schema, "mesh_pic_bandits");
    assert!(text.contains("\n  mesh_flags []\n"), "{text}");
}

#[test]
fn real_atlas_tableau_materials_round_trip_readable() {
    let Some((atlas, schema)) = load("tableau_materials") else { return };
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "tableau_materials typed {}/{} fields, raw-rest {} values of {}",
        report.typed_fields,
        report.fields,
        report.raw_rest,
        total_values(&atlas, "tableau_materials")
    );
    assert_eq!(report.records, 55);
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    // Nothing in this table is typable: header_tableau_materials.py declares no flag at all.
    assert_eq!((report.typed_fields, report.fields), (0, 10));
    assert_eq!(report.raw_rest, total_values(&atlas, "tableau_materials"));
    let text = text_of(&atlas, &schema, "tableau_game_character_sheet");
    assert!(text.contains("\n  tableau_flags 0\n"), "{text}");
    assert!(text.contains("\n  width 1024\n"), "{text}");
}
