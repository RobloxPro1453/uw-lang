//! T-112: the smart-audit rows whose fix fits the record language — troop upgrade targets as troop
//! refs (T1), a scene site as a scene ref (T3) and the item `sold_in` field (I8). Every fixed row
//! reads back exactly: the tiny atlas checks it without the big file, the real-atlas test skips
//! itself when `target/uw-staging/atlas.json` is absent.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn schema(table: &str) -> Schema {
    let path = format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"));
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{table} schema loads: {e}"))
}

/// The printed record text of `id`, the full atlas id (`trp_swadian_recruit`).
fn text_of(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let (table, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no `{id}` in the atlas"));
    assert_eq!(table, schema.table, "`{id}` is in the `{}` table", schema.table);
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no block paths"))
}

/// A tiny atlas with the tables and names these two schemas need: a recruit upgraded to troop 1
/// (with the padding 0 second target), a scene 51 with entry 52, and an item sold in two factions.
fn tiny_atlas() -> Atlas {
    let scene = 51u64 | (52 << 16);
    let json = serde_json::json!({
        "schema": 1, "kit_hash": "t112", "ops": [], "hooks": [], "ranges": [],
        "constants": [
            {"name": "tf_male", "value": 0, "module": "header_troops", "line": 10}
        ],
        "tables": {
            "factions": {"module": "module_factions", "var": "factions", "id_file": "ID_factions",
                "id_prefix": "fac_", "entries": [
                {"index": 0, "name": "commoners", "id": "fac_commoners", "dup": null, "line": 1, "value": [0]},
                {"index": 16, "name": "kingdom_2", "id": "fac_kingdom_2", "dup": null, "line": 17, "value": [0]},
                {"index": 17, "name": "kingdom_3", "id": "fac_kingdom_3", "dup": null, "line": 18, "value": [0]}
            ]},
            "scenes": {"module": "module_scenes", "var": "scenes", "id_file": "ID_scenes",
                "id_prefix": "scn_", "entries": [
                {"index": 51, "name": "zendar_center", "id": "scn_zendar_center", "dup": null, "line": 52, "value": []}
            ]},
            "troops": {"module": "module_troops", "var": "troops", "id_file": "ID_troops",
                "id_prefix": "trp_", "entries": [
                {"index": 0, "name": "recruit", "id": "trp_recruit", "dup": null, "line": 1,
                 "value": {"list": ["recruit", "Recruit", "Recruits", 0, scene, 0, 0,
                    {"list": []}, 0, 0, 0, 0, 0, 0, 1, 0]}},
                {"index": 1, "name": "militia", "id": "trp_militia", "dup": null, "line": 2,
                 "value": {"list": ["militia", "Militia", "Militia", 0, 0, 0, 0,
                    {"list": []}, 0, 0, 0, 0]}}
            ]},
            "items": {"module": "module_items", "var": "items", "id_file": "ID_items",
                "id_prefix": "itm_", "entries": [
                {"index": 0, "name": "sword", "id": "itm_sword", "dup": null, "line": 1,
                 "value": {"list": ["sword", "Sword", {"list": []}, 0, 0, 1, 0, 0,
                    {"list": []}, {"list": [16, 17]}]}}
            ]}
        }
    });
    Atlas::from_str(&json.to_string()).expect("tiny t112 atlas loads")
}

#[test]
fn tiny_atlas_applied_rows_print_names_and_read_back() {
    let mut atlas = tiny_atlas();
    // The `uw new` situation: the project's names layer is installed, so refs print game names.
    atlas.set_game_names(&[
        ("militia".to_owned(), "trp_militia".to_owned()),
        ("swadia".to_owned(), "fac_kingdom_2".to_owned()),
        ("rhodoks".to_owned(), "fac_kingdom_3".to_owned()),
    ]);
    let troops = schema("troops");
    let items = schema("items");
    let report = roundtrip_table(&atlas, &troops);
    assert_eq!((report.records, report.exact, report.failed), (2, 2, 0), "{:?}", report.first_failures);
    let report = roundtrip_table(&atlas, &items);
    assert_eq!((report.records, report.exact, report.failed), (1, 1, 0), "{:?}", report.first_failures);

    // T1: the upgrade target is a troop ref; the padding 0 is not a troop and prints `none`.
    let recruit = text_of(&atlas, &troops, "trp_recruit");
    assert!(recruit.contains("  upgrade_1 militia\n"), "{recruit}");
    assert!(recruit.contains("  upgrade_2 none\n"), "{recruit}");
    // A record whose upgrade fields are absent keeps printing none of them.
    let militia = text_of(&atlas, &troops, "trp_militia");
    assert!(!militia.contains("upgrade_"), "{militia}");

    // T3: the scene site is a scene ref; `no_scene = 0` prints `none`.
    assert!(recruit.contains("  scene {site scn_zendar_center, entry 52}\n"), "{recruit}");

    // I8: the merchandise list reads `sold_in`, not `factions`.
    let sword = text_of(&atlas, &items, "itm_sword");
    assert!(sword.contains("  sold_in [swadia, rhodoks]\n"), "{sword}");
    assert!(!sword.contains("  factions "), "{sword}");
}

#[test]
fn real_atlas_applied_rows_print_names_and_read_back() {
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let troops = schema("troops");
    let items = schema("items");
    for (table, schema) in [("troops", &troops), ("items", &items)] {
        let report = roundtrip_table(&atlas, schema);
        assert_eq!(report.exact, report.records, "{table}: {:?}", report.first_failures);
        assert_eq!(report.failed, 0, "{table}: {:?}", report.first_failures);
    }

    // T1: swadian_recruit is upgraded to swadian_militia (module_troops.py:2638); the second target
    // is the padding 0 (header_troops.py:267), not a troop.
    let recruit = text_of(&atlas, &troops, "trp_swadian_recruit");
    assert!(recruit.contains("  upgrade_1 trp_swadian_militia\n"), "{recruit}");
    assert!(recruit.contains("  upgrade_2 none\n"), "{recruit}");

    // T3: 0 is no_scene (module_troops.py:133); town_1_arena_master sites scn_town_1_arena
    // (module_troops.py:1440, ID_scenes.py:143).
    let player = text_of(&atlas, &troops, "trp_player");
    assert!(player.contains("  scene {site none, entry 0}\n"), "{player}");
    let master = text_of(&atlas, &troops, "trp_town_1_arena_master");
    assert!(master.contains("  scene {site scn_town_1_arena, entry 52}\n"), "{master}");

    // I8: charger is sold in fac_kingdom_1 and fac_kingdom_5 (module_items.py:260).
    let charger = text_of(&atlas, &items, "itm_charger");
    assert!(charger.contains("  sold_in [fac_kingdom_1, fac_kingdom_5]\n"), "{charger}");
}
