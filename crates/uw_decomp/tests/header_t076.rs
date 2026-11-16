//! T-076: a record header prints the project game name the modder knows (`faction swadia`) and reads
//! back as the bare kit id (`kingdom_1`); without names installed the header stays the id.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::names;
use uw_decomp::records::{parse_with_blocks, print_with_blocks, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/factions.toml");

/// Loads the real atlas and the factions schema; `None` when the atlas is absent.
fn load() -> Option<(Atlas, Schema)> {
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return None;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = Schema::from_path(SCHEMA).expect("factions schema loads");
    Some((atlas, schema))
}

/// Faction `fac_kingdom_1` as the kit gets it: no operation blocks, so `compile` never runs.
#[test]
fn real_atlas_faction_kingdom_1_prints_its_game_name() {
    let Some((mut atlas, schema)) = load() else { return };
    let index = atlas.tables["factions"]
        .entries
        .iter()
        .position(|entry| entry.id.as_deref() == Some("fac_kingdom_1"))
        .expect("no faction fac_kingdom_1 in the atlas");

    // No names installed: the header is the kit id.
    let bare = print_with_blocks(&atlas, &schema, &atlas.tables["factions"].entries[index], &[], &mut |_| {
        unreachable!("no blocks")
    });
    assert!(bare.starts_with("faction kingdom_1 {"), "{bare}");

    // With `uw new`'s names installed the header is the game name, and it reads back as the id.
    atlas.set_game_names(&names::pairs(&names::derive(&atlas)));
    let entry = &atlas.tables["factions"].entries[index];
    let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
    assert!(text.starts_with("faction swadia {"), "{text}");
    let (id, value) =
        parse_with_blocks(&atlas, &schema, &text, &mut |_| unreachable!("no blocks")).expect("game-name text parses");
    assert_eq!(id, "kingdom_1");
    assert_eq!(value, entry.value);
}

/// One faction and one party in a `fac_` / `p` tagged atlas: the same two directions without the
/// real atlas, and a game name of another table is not stripped.
const TINY: &str = r#"{
  "schema": 1, "kit_hash": "t076", "ops": [], "hooks": [], "ranges": [],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_","ref_tag":"fac",
      "entries": [
        {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,
          "value":["no_faction","No Faction",0,10,[],[]]},
        {"index":1,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":2,
          "value":["kingdom_1","Kingdom of Swadia",0,10,[],[]]}
      ]},
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","ref_tag":"p",
      "entries": [
        {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":3,"value":["town_6","Praven",0]}
      ]}
  }
}"#;

#[test]
fn header_prints_the_game_name_and_reads_back_the_kit_id() {
    let schema = Schema::from_path(SCHEMA).expect("factions schema loads");
    let mut atlas = Atlas::from_str(TINY).expect("tiny atlas loads");
    atlas.set_game_names(&[
        ("swadia".to_string(), "fac_kingdom_1".to_string()),
        ("praven".to_string(), "p_town_6".to_string()),
    ]);

    let entry = &atlas.tables["factions"].entries[1];
    let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
    assert!(text.starts_with("faction swadia {"), "{text}");
    let (id, value) =
        parse_with_blocks(&atlas, &schema, &text, &mut |_| unreachable!("no blocks")).expect("parses");
    assert_eq!(id, "kingdom_1");
    assert_eq!(value, entry.value);

    // A game name whose reference belongs to another table (`praven` -> `p_town_6`) is left alone,
    // so the check still says the record is not in the atlas.
    let other = text.replace("swadia", "praven");
    let (id, _) = parse_with_blocks(&atlas, &schema, &other, &mut |_| unreachable!("no blocks")).expect("parses");
    assert_eq!(id, "praven");

    // No names installed: the header is the kit id.
    let plain = Atlas::from_str(TINY).expect("tiny atlas loads");
    let text = print_with_blocks(&plain, &schema, &plain.tables["factions"].entries[1], &[], &mut |_| {
        unreachable!("no blocks")
    });
    assert!(text.starts_with("faction kingdom_1 {"), "{text}");
}
