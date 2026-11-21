//! Where each record goes in a `uw new` project (P3a). Towns, castles and villages get their own
//! folder, kings/lords/ladies/companions/pretenders too, rich records get one file each and the
//! small lookup tables one file per table (docs/arsiv/meclis-yon1-yon2/YON2-KARAR.md §3.2: the file name is the
//! Native id, not the display name).

use serde_json::Value as Json;
use uw_atlas::{Atlas, Entry};

use crate::items;
use crate::records::Schema;

/// Tables whose records each get their own `<table>/<id>.uw` file.
const PER_RECORD: [&str; 11] = [
    "factions",
    "items",
    "party_templates",
    "quests",
    "scenes",
    "scene_props",
    "menus",
    "scripts",
    "mission_templates",
    "presentations",
    "map_icons",
];

/// Party sub-folders, by the kit's range prefix: `towns_begin`..`towns_end`
/// (`module_constants.py:1552`).
const PARTY_FOLDERS: [(&str, &str); 3] =
    [("towns", "towns"), ("castles", "castles"), ("villages", "villages")];

/// Troop sub-folders, by range (`module_constants.py:1375-1390`). The kit's ranges do not overlap
/// (`kings_end = lords_begin`), so the order only decides a record that sits on two bounds.
const TROOP_FOLDERS: [(&str, &str); 5] = [
    ("companions", "companions"),
    ("kings", "kings"),
    ("lords", "lords"),
    ("pretenders", "pretenders"),
    ("ladies", "kingdom_ladies"),
];

/// Relative path (forward slashes, ends in `.uw`) of the file that holds this record. Many records may
/// share one path; the exporter puts them in one file in atlas order.
///
/// - `parties`: `parties/towns/…`, `parties/castles/…`, `parties/villages/…`, else `parties/…`;
/// - `troops`: `troops/<companions|kings|lords|pretenders|ladies>/…` by range, else `troops/…`;
/// - one file per record: `factions`, `items`, `party_templates`, `quests`, `scenes`,
///   `scene_props`, `menus`, `scripts`, `mission_templates`, `presentations`, `map_icons`;
/// - `dialogs`: one file per starting dialog state (record field 2);
/// - every other table (`strings`, `sounds`, `meshes`, `animations`, `music`, `skills`, `skins`,
///   `postfx_params`, `particle_systems`, `info_pages`, `tableau_materials`, `triggers`,
///   `simple_triggers`): one file, `<table>.uw`.
pub fn path_for(atlas: &Atlas, schema: &Schema, entry: &Entry) -> String {
    match schema.table.as_str() {
        "parties" => ranged(atlas, &PARTY_FOLDERS, schema, entry),
        "troops" => ranged(atlas, &TROOP_FOLDERS, schema, entry),
        "dialogs" => match dialog_state(entry) {
            Some(state) => format!("dialogs/{state}.uw"),
            // Field 2 is not a string in the atlas: keep the record in a file of its own rather
            // than in a folder name that is not an id.
            None => format!("dialogs/{}.uw", file_id(schema, entry)),
        },
        table if PER_RECORD.contains(&table) => format!("{table}/{}.uw", file_id(schema, entry)),
        table => format!("{table}.uw"),
    }
}

/// `<table>/<folder>/<id>.uw` for the first sub-folder whose range holds the entry, else
/// `<table>/<id>.uw`. A range the atlas does not have is skipped, so those records stay in the
/// table's own folder.
fn ranged(atlas: &Atlas, folders: &[(&str, &str)], schema: &Schema, entry: &Entry) -> String {
    let id = file_id(schema, entry);
    match folders.iter().find(|(_, prefix)| in_range(atlas, prefix, entry.index)) {
        Some((folder, _)) => format!("{}/{folder}/{id}.uw", schema.table),
        None => format!("{}/{id}.uw", schema.table),
    }
}

/// True when `index` is inside the kit's `prefix_begin`..`prefix_end`. Half-open, like the engine's
/// range loops: the kit writes `towns_end = castles_begin` (`module_constants.py:1556`), and
/// `kingdom_ladies_end` points at the placeholder troop `trp_heroes_end` (`ID_troops.py:464`).
fn in_range(atlas: &Atlas, prefix: &str, index: usize) -> bool {
    match (range_index(atlas, &format!("{prefix}_begin")), range_index(atlas, &format!("{prefix}_end"))) {
        (Some(begin), Some(end)) => begin <= index && index < end,
        _ => false,
    }
}

/// The table index a range constant points at: a number is the index itself, a string is an entry
/// id (`towns_begin = "p_town_1"`) resolved against the atlas. `None` when the atlas has no such
/// range or its value does not resolve.
fn range_index(atlas: &Atlas, name: &str) -> Option<usize> {
    let value = &atlas.ranges.iter().find(|range| range.name == name)?.value;
    match value {
        Json::Number(number) => number.as_u64().map(|index| index as usize),
        Json::String(id) => atlas.entry_by_id(id).map(|(_, entry)| entry.index),
        _ => None,
    }
}

/// File name body: the record id without the schema prefix (`p_town_6` -> `town_6`), the record's
/// own name or index when the kit has no id for it. A repeated name keeps `<id>#<occurrence>`, the
/// key `Atlas::entry_by_id` already answers to, so the records of one duplicate group never
/// overwrite each other.
fn file_id(schema: &Schema, entry: &Entry) -> String {
    let id = match entry.id.as_deref().or(entry.name.as_deref()) {
        Some(id) if !id.is_empty() => id.strip_prefix(&schema.id_prefix).unwrap_or(id).to_owned(),
        _ => entry.index.to_string(),
    };
    windows_safe(&match entry.dup {
        Some(dup) => format!("{id}#{dup}"),
        None => id,
    })
}

/// `: = * ? " < > |` cannot appear in a Windows file name; the record text keeps the real id.
fn windows_safe(id: &str) -> String {
    id.chars()
        .map(|c| if ":=*?\"<>|".contains(c) { '_' } else { c })
        .collect()
}

/// The starting dialog state (record field 2, `starting_dialog_state`) that groups dialog records
/// into one file.
fn dialog_state(entry: &Entry) -> Option<String> {
    match items(&entry.value)?.get(1)? {
        Json::String(state) if !state.is_empty() => Some(windows_safe(state)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::Path;

    use super::*;

    /// A tiny atlas with both range shapes the dump writes: string-valued (`towns_begin =
    /// "p_town_1"`) and numeric (`companions_begin = 0`). Index 0 and 5 of parties and index 5 of
    /// troops sit outside every range on purpose.
    const TINY: &str = r#"{
      "schema": 1, "kit_hash": "layout", "ops": [], "hooks": [], "constants": [],
      "ranges": [
        {"name": "towns_begin", "value": "p_town_1", "line": 1552},
        {"name": "towns_end", "value": "p_castle_1", "line": 1556},
        {"name": "castles_begin", "value": "p_castle_1", "line": 1553},
        {"name": "castles_end", "value": "p_village_1", "line": 1557},
        {"name": "villages_begin", "value": "p_village_1", "line": 1554},
        {"name": "villages_end", "value": "p_salt_mine", "line": 1558},
        {"name": "companions_begin", "value": 0, "line": 1389},
        {"name": "companions_end", "value": 1, "line": 1390},
        {"name": "kings_begin", "value": 1, "line": 1386},
        {"name": "kings_end", "value": 2, "line": 1387},
        {"name": "lords_begin", "value": 2, "line": 1383},
        {"name": "lords_end", "value": 3, "line": 1384},
        {"name": "pretenders_begin", "value": 3, "line": 1380},
        {"name": "pretenders_end", "value": 4, "line": 1381},
        {"name": "kingdom_ladies_begin", "value": 4, "line": 1375},
        {"name": "kingdom_ladies_end", "value": 5, "line": 1376}
      ],
      "tables": {
        "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","entries":[
          {"index":0,"name":"zendar","id":"p_zendar","dup":null,"line":1,"value":[]},
          {"index":1,"name":"town_1","id":"p_town_1","dup":null,"line":2,"value":[]},
          {"index":2,"name":"town_6","id":"p_town_6","dup":null,"line":3,"value":[]},
          {"index":3,"name":"castle_1","id":"p_castle_1","dup":null,"line":4,"value":[]},
          {"index":4,"name":"village_1","id":"p_village_1","dup":null,"line":5,"value":[]},
          {"index":5,"name":"salt_mine","id":"p_salt_mine","dup":null,"line":6,"value":[]}
        ]},
        "troops": {"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_","entries":[
          {"index":0,"name":"npc1","id":"trp_npc1","dup":null,"line":1,"value":{"list":[]}},
          {"index":1,"name":"kingdom_1_lord","id":"trp_kingdom_1_lord","dup":null,"line":2,"value":{"list":[]}},
          {"index":2,"name":"knight_1_1","id":"trp_knight_1_1","dup":null,"line":3,"value":{"list":[]}},
          {"index":3,"name":"kingdom_1_pretender","id":"trp_kingdom_1_pretender","dup":null,"line":4,"value":{"list":[]}},
          {"index":4,"name":"knight_1_1_wife","id":"trp_knight_1_1_wife","dup":null,"line":5,"value":{"list":[]}},
          {"index":5,"name":"heroes_end","id":"trp_heroes_end","dup":null,"line":6,"value":{"list":[]}}
        ]},
        "items": {"module":"module_items","var":"items","id_file":"ID_items","id_prefix":"itm_","entries":[
          {"index":0,"name":"practice_sword","id":null,"dup":1,"line":1,"value":[]},
          {"index":1,"name":"practice_sword","id":"itm_practice_sword","dup":2,"line":2,"value":[]}
        ]},
        "quests": {"module":"module_quests","var":"quests","id_file":"ID_quests","id_prefix":"qst_","entries":[
          {"index":0,"name":"kidnapped_girl","id":"qst_kidnapped:girl","dup":null,"line":1,"value":[]}
        ]},
        "dialogs": {"module":"module_dialogs","var":"dialogs","id_file":null,"id_prefix":null,"entries":[
          {"index":0,"name":null,"id":"dlga_start:close_window","dup":null,"line":1,"value":{"list":[1,"start"]}},
          {"index":1,"name":null,"id":"dlga_start:mayor_talk","dup":null,"line":2,"value":{"list":[1,"start"]}},
          {"index":2,"name":null,"id":"dlga_start:prisoner","dup":null,"line":3,"value":{"list":[1,"prisoner_quest"]}},
          {"index":3,"name":null,"id":"dlga_x_y","dup":null,"line":4,"value":{"list":[1]}}
        ]},
        "strings": {"module":"module_strings","var":"strings","id_file":"ID_strings","id_prefix":"str_","entries":[
          {"index":0,"name":"a","id":"str_a","dup":null,"line":1,"value":[]},
          {"index":1,"name":"b","id":"str_b","dup":null,"line":2,"value":[]}
        ]},
        "triggers": {"module":"module_triggers","var":"triggers","id_file":null,"id_prefix":null,"entries":[
          {"index":0,"name":null,"id":null,"dup":null,"line":1,"value":{"list":[]}}
        ]}
      }
    }"#;

    fn schema(table: &str, prefix: &str) -> Schema {
        Schema::from_toml(&format!(
            "table = \"{table}\"\nkind = \"record\"\nid_prefix = \"{prefix}\"\n\
             [[fields]]\nname = \"a\"\nty = \"raw\"\nsource = \"x:1\"\n"
        ))
        .expect("schema loads")
    }

    /// Path of the tiny atlas' entry `id`.
    fn path(atlas: &Atlas, table: &str, prefix: &str, id: &str) -> String {
        let (_, entry) = atlas.entry_by_id(id).expect("entry exists");
        path_for(atlas, &schema(table, prefix), entry)
    }

    #[test]
    fn towns_castles_and_villages_get_their_own_folder() {
        let atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
        assert_eq!(path(&atlas, "parties", "p_", "p_town_6"), "parties/towns/town_6.uw");
        assert_eq!(path(&atlas, "parties", "p_", "p_town_1"), "parties/towns/town_1.uw");
        assert_eq!(path(&atlas, "parties", "p_", "p_castle_1"), "parties/castles/castle_1.uw");
        assert_eq!(path(&atlas, "parties", "p_", "p_village_1"), "parties/villages/village_1.uw");
        // Outside every range: the table's own folder. `p_salt_mine` is `villages_end`, so the
        // range is half-open.
        assert_eq!(path(&atlas, "parties", "p_", "p_zendar"), "parties/zendar.uw");
        assert_eq!(path(&atlas, "parties", "p_", "p_salt_mine"), "parties/salt_mine.uw");
    }

    #[test]
    fn troop_ranges_group_kings_lords_ladies_companions_and_pretenders() {
        let atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
        assert_eq!(path(&atlas, "troops", "trp_", "trp_npc1"), "troops/companions/npc1.uw");
        assert_eq!(
            path(&atlas, "troops", "trp_", "trp_kingdom_1_lord"),
            "troops/kings/kingdom_1_lord.uw"
        );
        assert_eq!(path(&atlas, "troops", "trp_", "trp_knight_1_1"), "troops/lords/knight_1_1.uw");
        assert_eq!(
            path(&atlas, "troops", "trp_", "trp_kingdom_1_pretender"),
            "troops/pretenders/kingdom_1_pretender.uw"
        );
        assert_eq!(
            path(&atlas, "troops", "trp_", "trp_knight_1_1_wife"),
            "troops/ladies/knight_1_1_wife.uw"
        );
        // `trp_heroes_end` is `kingdom_ladies_end`, the first index outside the range.
        assert_eq!(path(&atlas, "troops", "trp_", "trp_heroes_end"), "troops/heroes_end.uw");
    }

    #[test]
    fn rich_records_get_one_file_and_lookup_tables_one_file_per_table() {
        let atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
        assert_eq!(path(&atlas, "items", "itm_", "itm_practice_sword"), "items/practice_sword#2.uw");
        // `:` is not allowed in a Windows file name; only the file name changes.
        assert_eq!(path(&atlas, "quests", "qst_", "qst_kidnapped:girl"), "quests/kidnapped_girl.uw");
        let (_, a) = atlas.entry_by_id("str_a").expect("entry exists");
        assert_eq!(path_for(&atlas, &schema("strings", "str_"), a), "strings.uw");
        let trigger = &atlas.tables["triggers"].entries[0];
        assert_eq!(path_for(&atlas, &schema("triggers", ""), trigger), "triggers.uw");
    }

    #[test]
    fn duplicate_names_do_not_share_a_path() {
        let atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
        let items = &atlas.tables["items"].entries;
        let schema = schema("items", "itm_");
        let first = path_for(&atlas, &schema, &items[0]);
        let second = path_for(&atlas, &schema, &items[1]);
        assert_eq!(first, "items/practice_sword#1.uw");
        assert_ne!(first, second);
    }

    #[test]
    fn dialogs_group_by_starting_state() {
        let atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
        let schema = schema("dialogs", "");
        let dialogs = &atlas.tables["dialogs"].entries;
        let start = path_for(&atlas, &schema, &dialogs[0]);
        assert_eq!(start, "dialogs/start.uw");
        // A group: records with the same state share one file on purpose.
        assert_eq!(path_for(&atlas, &schema, &dialogs[1]), start);
        assert_eq!(path_for(&atlas, &schema, &dialogs[2]), "dialogs/prisoner_quest.uw");
        // Field 2 missing: the record keeps a file of its own.
        assert_eq!(path_for(&atlas, &schema, &dialogs[3]), "dialogs/dlga_x_y.uw");
    }

    // ---------------------------------------------------------------- real atlas (not in git)

    const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    const RECORDS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

    fn real_atlas() -> Option<Atlas> {
        if !Path::new(ATLAS).exists() {
            eprintln!("skipped: no atlas at {ATLAS}");
            return None;
        }
        Some(Atlas::from_path(ATLAS).expect("full atlas loads"))
    }

    /// Every `kit-meta/records/<table>.toml`, by table name.
    fn real_schemas() -> Vec<(String, Schema)> {
        let mut schemas: Vec<(String, Schema)> = std::fs::read_dir(RECORDS)
            .expect("kit-meta/records readable")
            .map(|file| file.expect("dir entry").path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("toml"))
            .map(|path| {
                let schema = Schema::from_path(&path).expect("schema loads");
                (schema.table.clone(), schema)
            })
            .collect();
        schemas.sort_by(|a, b| a.0.cmp(&b.0));
        eprintln!("schemas: {}", schemas.len());
        assert!(!schemas.is_empty());
        schemas
    }

    fn check(atlas: &Atlas, schema: &Schema, id: &str, want: &str) {
        let (_, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no entry {id}"));
        assert_eq!(path_for(atlas, schema, entry), want, "{id}");
    }

    /// `p_town_6` -> `parties/towns/town_6.uw` on the real atlas, so the string-valued ranges
    /// (`towns_begin = "p_town_1"`) really resolve to table indices.
    #[test]
    fn real_atlas_parties_land_in_their_folder() {
        let Some(atlas) = real_atlas() else { return };
        let schema = Schema::from_path(format!("{RECORDS}/parties.toml")).expect("schema loads");
        check(&atlas, &schema, "p_town_6", "parties/towns/town_6.uw");
        check(&atlas, &schema, "p_castle_1", "parties/castles/castle_1.uw");
        check(&atlas, &schema, "p_village_1", "parties/villages/village_1.uw");
        check(&atlas, &schema, "p_zendar", "parties/zendar.uw");
        let schema = Schema::from_path(format!("{RECORDS}/troops.toml")).expect("schema loads");
        let (_, npc) = atlas.entry_by_id("trp_npc1").expect("trp_npc1");
        assert_eq!(path_for(&atlas, &schema, npc), "troops/companions/npc1.uw");
        let (_, knight) = atlas.entry_by_id("trp_knight_1_1").expect("trp_knight_1_1");
        assert_eq!(path_for(&atlas, &schema, knight), "troops/lords/knight_1_1.uw");
        let (_, lady) = atlas.entry_by_id("trp_knight_1_1_wife").expect("trp_knight_1_1_wife");
        assert_eq!(path_for(&atlas, &schema, lady), "troops/ladies/knight_1_1_wife.uw");
        // `trp_heroes_end` is the kit's `kingdom_ladies_end` placeholder: outside the range.
        let (_, end) = atlas.entry_by_id("trp_heroes_end").expect("trp_heroes_end");
        assert_eq!(path_for(&atlas, &schema, end), "troops/heroes_end.uw");
    }

    /// No two records of a table that gives every record its own file may share a path, and the
    /// report prints the file count per top folder.
    #[test]
    fn real_atlas_paths_are_unique_per_record() {
        let Some(atlas) = real_atlas() else { return };
        let grouped = |table: &str| {
            !PER_RECORD.contains(&table) && !matches!(table, "parties" | "troops")
        };
        let mut owners: BTreeMap<String, (String, usize)> = BTreeMap::new();
        let mut collisions: Vec<String> = Vec::new();
        let mut folders: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        let mut records = 0;
        for (table, schema) in real_schemas() {
            let Some(table_entries) = atlas.tables.get(&table) else {
                panic!("schema {table} has no table in the atlas");
            };
            for entry in &table_entries.entries {
                let path = path_for(&atlas, &schema, entry);
                let folder = path.split('/').next().unwrap_or_default().to_owned();
                folders.entry(folder).or_default().insert(path.clone());
                if grouped(&table) {
                    continue;
                }
                records += 1;
                match owners.insert(path.clone(), (table.clone(), entry.index)) {
                    Some((other_table, other_index)) => collisions.push(format!(
                        "{path}: {other_table}[{other_index}] and {table}[{}]",
                        entry.index
                    )),
                    None => {}
                }
            }
        }
        for (folder, files) in &folders {
            eprintln!("{folder}: {} files", files.len());
        }
        eprintln!("one-file-per-record paths: {records}");
        assert!(records > 0);
        assert!(collisions.is_empty(), "{} collisions: {collisions:#?}", collisions.len());
    }
}
