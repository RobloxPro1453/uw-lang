//! T-059: the troop records print their kit-typed fields readably (troop flags, scene site/entry,
//! item refs, weapon proficiencies, skills) and still parse back to the identical atlas value.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/troops.toml");

/// Loads the real atlas and the troops schema; `None` when the atlas is absent.
fn load() -> Option<(Atlas, Schema)> {
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return None;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = Schema::from_path(SCHEMA).expect("troops schema loads");
    Some((atlas, schema))
}

/// The printed record text of one troop, looked up by id (duplicate troop names have no id).
fn troop_text(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let entry = atlas.tables["troops"]
        .entries
        .iter()
        .find(|entry| entry.id.as_deref() == Some(id))
        .unwrap_or_else(|| panic!("no troop {id}"));
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

#[test]
fn real_atlas_troops_print_typed_fields() {
    let Some((atlas, schema)) = load() else { return };
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    // troop_flags, scene, faction, inventory, attributes, weapon_proficiencies, skills.
    assert_eq!((report.typed_fields, report.fields), (7, 14));

    // trp_player (module_troops.py:244): wp(15) fills one_handed..throwing with 15, no scene, no
    // inventory, no skills.
    let player = troop_text(&atlas, &schema, "trp_player");
    assert!(player.contains("  troop_flags [tf_hero, tf_unmoveable_in_party_window]\n"), "{player}");
    assert!(player.contains("  scene {site 0, entry 0}\n"), "{player}");
    assert!(player.contains("  inventory []\n"), "{player}");
    assert!(player.contains("  weapon_proficiencies {one_handed 15, two_handed 15, "), "{player}");
    assert!(player.contains("  skills {trade 0, leadership 0, "), "{player}");

    // trp_npc3_4 (Ymira, module_troops.py:2562): a woman hero carrying a guard helmet.
    let ymira = troop_text(&atlas, &schema, "trp_npc3_4");
    assert!(
        ymira.contains("  troop_flags [tf_female, tf_hero, tf_unmoveable_in_party_window]\n"),
        "{ymira}"
    );
    assert!(ymira.contains("  inventory [itm_guard_helmet, "), "{ymira}");

    // trp_tournament_master (module_troops.py:805): `scn_zendar_center|entry(1)`, and ID_scenes.py
    // line 52 makes scn_zendar_center scene 51.
    let master = troop_text(&atlas, &schema, "trp_tournament_master");
    assert!(master.contains("  scene {site 51, entry 1}\n"), "{master}");
}

/// A tiny atlas with only the tables and constants the troops schema names, so the typed fields
/// are also checked where the real atlas is absent: flags must pick the troop constants even
/// though header_presentations.py gives tf_hero's bits the name tf_center_justify, and the packed
/// scene / proficiency / skill bits must land on their documented positions.
fn tiny_atlas() -> Atlas {
    let flags = 16u64 | 0x1000_0000; // tf_hero | tf_unmoveable_in_party_window
    let scene = 51u64 | (1 << 16); // scn_zendar_center | entry(1)
    let attributes = 5u64 | (6 << 8) | (4 << 16) | (5 << 24) | (3 << 32);
    let wp = 15u64 | (24 << 40) | (5 << 60); // one_handed 15, crossbow 24, firearm 5
    // knows_ironflesh_3 = 3 << 144 (header_skills.py:414).
    let skills = "66902235595591869424607154817945084517941248";
    let json = format!(
        r#"{{
      "schema": 1, "kit_hash": "t059", "ops": [], "hooks": [], "ranges": [],
      "constants": [
        {{"name":"tf_left_align","value":4,"module":"header_presentations","line":9}},
        {{"name":"tf_center_justify","value":16,"module":"header_presentations","line":11}},
        {{"name":"tf_vertical_align_center","value":4096,"module":"header_presentations","line":13}},
        {{"name":"tf_single_line","value":32768,"module":"header_presentations","line":15}},
        {{"name":"tf_male","value":0,"module":"header_troops","line":10}},
        {{"name":"tf_female","value":1,"module":"header_troops","line":11}},
        {{"name":"tf_undead","value":2,"module":"header_troops","line":12}},
        {{"name":"tf_hero","value":16,"module":"header_troops","line":14}},
        {{"name":"tf_mounted","value":1024,"module":"header_troops","line":19}},
        {{"name":"tf_guarantee_boots","value":1048576,"module":"header_troops","line":23}},
        {{"name":"tf_unmoveable_in_party_window","value":268435456,"module":"header_troops","line":31}}
      ],
      "tables": {{
        "factions": {{"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_","entries":[
          {{"index":0,"name":"commoners","id":"fac_commoners","dup":null,"line":1,"value":[0]}}
        ]}},
        "items": {{"module":"module_items","var":"items","id_file":"ID_items","id_prefix":"itm_","entries":[
          {{"index":0,"name":"guard_helmet","id":"itm_guard_helmet","dup":null,"line":1,"value":[0]}}
        ]}},
        "troops": {{"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_","entries":[
          {{"index":0,"name":"player","id":"trp_player","dup":null,"line":244,
           "value":{{"list":["player","Player","Player",{flags},0,0,0,{{"list":[]}},{attributes},{wp},{skills},0]}}}},
          {{"index":1,"name":"npc3_4","id":"trp_npc3_4","dup":null,"line":2562,
           "value":{{"list":["npc3_4","Ymira","Ymira",{female_flags},{scene},0,0,{{"list":[0]}},{attributes},0,0,1,2,0,1,0]}}}},
          {{"index":2,"name":"farmer","id":"trp_farmer","dup":null,"line":322,
           "value":{{"list":["farmer","Farmer","Farmers",0,0,0,0,{{"list":[]}},0,0,0,0]}}}},
          {{"index":3,"name":"left_align","id":"trp_left_align","dup":null,"line":1,
           "value":{{"list":["left_align","Left","Left",4,0,0,0,{{"list":[]}},0,0,0,0]}}}}
        ]}}
      }}
    }}"#,
        flags = flags,
        female_flags = 1u64 | 16 | 0x1000_0000,
        scene = scene,
        attributes = attributes,
        wp = wp,
        skills = skills,
    );
    Atlas::from_str(&json).expect("tiny atlas loads")
}

#[test]
fn tiny_atlas_troops_print_and_read_back_the_typed_fields() {
    let atlas = tiny_atlas();
    let schema = Schema::from_path(SCHEMA).expect("troops schema loads");
    let report = roundtrip_table(&atlas, &schema);
    assert_eq!((report.records, report.exact, report.failed), (4, 4, 0), "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields), (7, 14));

    let player = troop_text(&atlas, &schema, "trp_player");
    // tf_center_justify (header_presentations.py) shares tf_hero's bits and must not print here.
    assert!(player.contains("  troop_flags [tf_hero, tf_unmoveable_in_party_window]\n"), "{player}");
    assert!(player.contains("  faction fac_commoners\n"), "{player}");
    assert!(player.contains("  inventory []\n"), "{player}");
    assert!(player.contains("  attributes {str 5, agi 6, int 4, cha 5, level 3}\n"), "{player}");
    assert!(
        player.contains(
            "  weapon_proficiencies {one_handed 15, two_handed 0, polearm 0, archery 0, crossbow 24, throwing 0, firearm 5}\n"
        ),
        "{player}"
    );
    assert!(player.contains("  skills {trade 0, leadership 0, "), "{player}");
    assert!(player.contains(", ironflesh 3, reserved_14 0, "), "{player}");
    assert!(player.contains(", reserved_18 0}"), "{player}");

    let ymira = troop_text(&atlas, &schema, "trp_npc3_4");
    assert!(
        ymira.contains("  troop_flags [tf_female, tf_hero, tf_unmoveable_in_party_window]\n"),
        "{ymira}"
    );
    assert!(ymira.contains("  scene {site 51, entry 1}\n"), "{ymira}");
    assert!(ymira.contains("  inventory [itm_guard_helmet]\n"), "{ymira}");
    // Past the layout comment the two upgrade targets have no kit meaning: they print as field_15
    // and field_16 and read back unchanged.
    assert!(ymira.contains("  field_15 1\n  field_16 0\n"), "{ymira}");

    let farmer = troop_text(&atlas, &schema, "trp_farmer");
    assert!(farmer.contains("  troop_flags [tf_male]\n"), "{farmer}");

    // A presentation-only bit is not a troop flag: it stays a `rest`.
    let left = troop_text(&atlas, &schema, "trp_left_align");
    assert!(left.contains("  troop_flags [rest 4]\n"), "{left}");
}
