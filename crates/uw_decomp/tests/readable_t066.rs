//! T-066: the readable fields of sounds, music, animations, particle_systems, skins and
//! postfx_params (`docs/tasks/T-066.md`). Each table must round-trip every record exactly on the
//! real atlas and its typed fields must print readably. Skips itself when the atlas is absent
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md).

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{table} schema loads: {e}"))
}

/// The full atlas, or `None` when this machine has none (the test then skips).
fn atlas() -> Option<Atlas> {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return None;
    }
    Some(Atlas::from_path(ATLAS).expect("full atlas loads"))
}

/// Every record of the table must print and parse back identical, and the coverage counters must
/// show the typed fields actually printed readably (no silent fallback to raw).
fn roundtrip(atlas: &Atlas, schema: &Schema, records: usize, typed: usize, fields: usize, raw_rest: usize) {
    let report = roundtrip_table(atlas, schema);
    assert_eq!(report.records, records, "`{}` record count", schema.table);
    assert_eq!(report.exact, report.records, "`{}`: {:?}", schema.table, report.first_failures);
    assert_eq!(report.failed, 0, "`{}`: {:?}", schema.table, report.first_failures);
    assert_eq!(
        (report.typed_fields, report.fields, report.raw_rest),
        (typed, fields, raw_rest),
        "`{}` coverage",
        schema.table
    );
}

/// Prints one record, found by atlas id or, for tables without ids (skins), by name.
fn printed(atlas: &Atlas, schema: &Schema, key: &str) -> String {
    let entry = atlas.tables[&schema.table]
        .entries
        .iter()
        .find(|entry| entry.id.as_deref() == Some(key) || entry.name.as_deref() == Some(key))
        .unwrap_or_else(|| panic!("`{}` has a record `{key}`", schema.table));
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

/// Prints the record at an atlas table index (skins have no id and no name in the atlas).
fn printed_index(atlas: &Atlas, schema: &Schema, index: usize) -> String {
    let entry = &atlas.tables[&schema.table].entries[index];
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

#[test]
fn real_atlas_sounds_readable() {
    let schema = schema("sounds");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 200, 1, 3, 400);
    // gong = sf_2d|sf_priority_9|sf_vol_7, click = sf_2d|sf_vol_3 (module_sounds.py:4,7).
    let gong = printed(&atlas, &schema, "snd_gong");
    assert!(gong.contains("  field_2 [sf_priority_9, sf_vol_7, sf_2d]"), "{gong}");
    let click = printed(&atlas, &schema, "snd_click");
    assert!(click.contains("  field_2 [sf_vol_3, sf_2d]"), "{click}");
}

#[test]
fn real_atlas_music_readable() {
    let schema = schema("music");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 74, 2, 4, 148);
    // arena_1 = mtf_sit_arena; ambushed_by_khergit = culture_3|sit_ambushed|sit_siege and
    // sit_fight|sit_multiplayer_fight|mtf_culture_all (module_music.py:34,41).
    let arena = printed(&atlas, &schema, "track_arena_1");
    assert!(arena.contains("  track_flags [mtf_sit_arena]"), "{arena}");
    let khergit = printed(&atlas, &schema, "track_ambushed_by_khergit");
    assert!(
        khergit.contains("  track_flags [mtf_culture_3, mtf_sit_ambushed, mtf_sit_siege]"),
        "{khergit}"
    );
    assert!(
        khergit.contains(
            "  continue_track_flags [mtf_culture_all, mtf_sit_fight, mtf_sit_multiplayer_fight]"
        ),
        "{khergit}"
    );
}

#[test]
fn real_atlas_animations_readable() {
    let schema = schema("animations");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 670, 2, 4, 1359);
    // ready_bow: acf_rot_vertical_bow|acf_anim_length(100) and
    // amf_priority_attack(10)|amf_use_weapon_speed|amf_keep|amf_client_owner_prediction|
    // amf_rider_rot_bow (module_animations.py:707).
    let bow = printed(&atlas, &schema, "anim_ready_bow");
    assert!(
        bow.contains("  animation_flags {flags [acf_rot_vertical_bow], animation_length 100}"),
        "{bow}"
    );
    assert!(
        bow.contains(
            "  animation_master_flags [amf_rider_rot_bow, amf_use_weapon_speed, amf_keep, amf_client_owner_prediction, rest 10]"
        ),
        "{bow}"
    );
    let stand = printed(&atlas, &schema, "anim_stand");
    assert!(stand.contains("  animation_flags {flags [], animation_length 0}"), "{stand}");
}

#[test]
fn real_atlas_particle_systems_readable() {
    let schema = schema("particle_systems");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 60, 1, 24, 1376);
    // game_rain = billboard_2d|global_emit_dir|always_emit (module_particle_systems.py:45).
    let rain = printed(&atlas, &schema, "psys_game_rain");
    assert!(
        rain.contains("  particle_system_flags [psf_always_emit, psf_global_emit_dir, psf_billboard_2d]"),
        "{rain}"
    );
    let snow = printed(&atlas, &schema, "psys_game_snow");
    assert!(
        snow.contains(
            "  particle_system_flags [psf_always_emit, psf_global_emit_dir, psf_billboard_3d, psf_randomize_rotation, psf_randomize_size]"
        ),
        "{snow}"
    );
}

#[test]
fn real_atlas_skins_readable() {
    let schema = schema("skins");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 2, 3, 18, 29);
    // man (module_skins.py:132): flags 0, blood particles psys_game_blood / psys_game_blood_2.
    let man = printed_index(&atlas, &schema, 0);
    assert!(man.contains("  skin_flags []"), "{man}");
    assert!(man.contains("  blood_particles_1 psys_game_blood"), "{man}");
    assert!(man.contains("  blood_particles_2 psys_game_blood_2"), "{man}");
    // woman (module_skins.py:166): skf_use_morph_key_10.
    let woman = printed_index(&atlas, &schema, 1);
    assert!(woman.contains("  skin_flags [skf_use_morph_key_10]"), "{woman}");
}

#[test]
fn real_atlas_postfx_params_readable() {
    let schema = schema("postfx_params");
    let Some(atlas) = atlas() else { return };
    roundtrip(&atlas, &schema, 9, 1, 6, 45);
    // Every kit record has flags 0 (module_postfx.py:19-31).
    let default = printed(&atlas, &schema, "pfx_default");
    assert!(default.contains("  flags []"), "{default}");
    let map = printed(&atlas, &schema, "pfx_map_params");
    assert!(map.contains("  flags []"), "{map}");
}
