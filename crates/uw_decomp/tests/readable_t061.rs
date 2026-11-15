//! T-061: the `parties` and `party_templates` fields the kit explains print readably with the
//! T-058 types and still round-trip the real atlas exactly. The atlas is machine-local
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md); the real-atlas tests skip themselves
//! when it is absent.

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Field, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const SCHEMA_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

fn schema(table: &str) -> Schema {
    let path = format!("{SCHEMA_DIR}/{table}.toml");
    Schema::from_path(&path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn field<'a>(schema: &'a Schema, name: &str) -> &'a Field {
    schema
        .fields
        .iter()
        .find(|field| field.name == name)
        .unwrap_or_else(|| panic!("no field `{name}`"))
}

/// The printed record text of the atlas entry `id`.
fn printed(atlas: &Atlas, schema: &Schema, id: &str) -> String {
    let (_, entry) = atlas.entry_by_id(id).unwrap_or_else(|| panic!("no entry `{id}`"));
    print_with_blocks(atlas, schema, entry, &[], &mut |_| unreachable!("no blocks"))
}

fn part_layout(field: &Field) -> Vec<(&str, u32, u32, Option<&str>)> {
    field.parts.iter().map(|part| (part.name.as_str(), part.shift, part.width, part.ty.as_deref())).collect()
}

#[test]
fn schemas_type_the_fields_the_kit_explains() {
    let parties = schema("parties");
    let flags = field(&parties, "party_flags");
    assert_eq!(flags.ty, "bits");
    // bits 8-26 are the pf_* flags (header_parties.py:14-32), bits 0-7 the icon index
    // (header_parties.py:13); a shifted `flags` part cannot match absolute constant values.
    assert_eq!(
        part_layout(flags),
        [
            ("flags", 0, 27, Some("flags:pf_")),
            ("carry_goods", 48, 8, None),
            ("carry_gold", 56, 8, None),
        ]
    );
    assert!(flags.parts.iter().all(|part| part.source.starts_with("header_parties.py:")));

    assert_eq!(field(&parties, "party_template").ty, "ref:party_templates");
    assert_eq!(field(&parties, "faction").ty, "ref:factions");

    // courage (bits 0-3), aggressiveness (bits 4-7) and banditness (bit 8) are packed numbers;
    // courage_4..15 repeat in header_troops.py, so a flags enum cannot name them uniquely.
    let personality = field(&parties, "personality");
    assert_eq!(personality.ty, "bits");
    assert_eq!(
        part_layout(personality),
        [("courage", 0, 4, None), ("aggressiveness", 4, 4, None), ("banditness", 8, 1, None)]
    );
    assert!(personality.parts.iter().all(|part| part.source.starts_with("header_parties.py:")));

    let ai_behavior = field(&parties, "ai_behavior");
    assert_eq!(ai_behavior.ty, "flags:ai_bhvr_");
    assert_eq!(ai_behavior.zero.as_deref(), Some("ai_bhvr_hold"));
    assert_eq!(ai_behavior.enums.len(), 1);
    assert_eq!(
        (ai_behavior.enums[0].mask.as_integer(), ai_behavior.enums[0].prefix.as_str()),
        (Some(0xFF), "ai_bhvr_")
    );

    for name in ["id", "name", "menu", "ai_target", "initial_coordinates", "stacks", "direction"] {
        assert_eq!(field(&parties, name).ty, "raw", "`{name}` stays raw");
    }
    assert!(parties.fields.iter().all(|field| field.source.starts_with("module_parties.py:")));
    assert_eq!(parties.fields.iter().filter(|field| field.ty != "raw").count(), 5);

    let templates = schema("party_templates");
    let flags = field(&templates, "party_flags");
    assert_eq!(flags.ty, "bits");
    assert_eq!(
        part_layout(flags),
        [
            ("flags", 0, 27, Some("flags:pf_")),
            ("carry_goods", 48, 8, None),
            ("carry_gold", 56, 8, None),
        ]
    );
    assert_eq!(field(&templates, "faction").ty, "ref:factions");
    let personality = field(&templates, "personality");
    assert_eq!(personality.ty, "bits");
    assert_eq!(
        part_layout(personality),
        [("courage", 0, 4, None), ("aggressiveness", 4, 4, None), ("banditness", 8, 1, None)]
    );
    for name in ["id", "name", "menu", "stacks"] {
        assert_eq!(field(&templates, name).ty, "raw", "`{name}` stays raw");
    }
    assert!(templates.fields.iter().all(|field| field.source.starts_with("module_party_templates.py:")));
    assert_eq!(templates.fields.iter().filter(|field| field.ty != "raw").count(), 3);
}

/// A tiny atlas with the kit's real bit layouts, so the typed party fields are checked even on a
/// machine without `target/uw-staging/atlas.json`.
const TYPED_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "parties-t061", "ops": [], "hooks": [], "ranges": [],
  "constants": [
    {"name":"pf_is_static","value":1024,"module":"header_parties","line":16},
    {"name":"pf_label_large","value":8192,"module":"header_parties","line":20},
    {"name":"pf_always_visible","value":16384,"module":"header_parties","line":22},
    {"name":"pf_limit_members","value":1048576,"module":"header_parties","line":27},
    {"name":"pf_show_faction","value":4194304,"module":"header_parties","line":29},
    {"name":"ai_bhvr_hold","value":0,"module":"header_parties","line":52}
  ],
  "tables": {
    "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_","entries":[
      {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,"value":[0]},
      {"index":1,"name":"commoners","id":"fac_commoners","dup":null,"line":2,"value":[1]},
      {"index":3,"name":"neutral","id":"fac_neutral","dup":null,"line":4,"value":[3]}
    ]},
    "party_templates": {"module":"module_party_templates","var":"party_templates","id_file":"ID_party_templates","id_prefix":"pt_","entries":[
      {"index":0,"name":"none","id":"pt_none","dup":null,"line":1,"value":["none","none",2,0,1,7,{"list":[]}]},
      {"index":5,"name":"cattle_herd","id":"pt_cattle_herd","dup":null,"line":6,
       "value":["cattle_herd","Cattle Herd",2814749767106598,0,3,7,{"list":[]}]}
    ]},
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","entries":[
      {"index":0,"name":"main_party","id":"p_main_party","dup":null,"line":1,
       "value":["main_party","Main Party",1048576,0,0,3,0,0,0,[17,52.5],{"list":[]}]},
      {"index":6,"name":"town_6","id":"p_town_6","dup":null,"line":7,
       "value":["town_6","Praven",4219916,0,0,3,0,0,0,[-96,26.4],{"list":[]},155]}
    ]}
  }
}"#;

#[test]
fn typed_fields_print_readably_and_read_back_exactly() {
    let atlas = Atlas::from_str(TYPED_ATLAS).expect("tiny atlas loads");

    let parties = schema("parties");
    let text = printed(&atlas, &parties, "p_main_party");
    assert_eq!(
        text,
        concat!(
            "party main_party {\n",
            "  id main_party\n",
            "  name \"Main Party\"\n",
            "  party_flags {flags [pf_limit_members], carry_goods 0, carry_gold 0}\n",
            "  menu 0\n",
            "  party_template pt_none\n",
            "  faction fac_neutral\n",
            "  personality {courage 0, aggressiveness 0, banditness 0}\n",
            "  ai_behavior [ai_bhvr_hold]\n",
            "  ai_target 0\n",
            "  initial_coordinates (17, 52.5)\n",
            "  stacks []\n",
            "}\n",
        )
    );
    let town = printed(&atlas, &parties, "p_town_6");
    assert!(
        town.contains(
            "  party_flags {flags [pf_is_static, pf_label_large, pf_always_visible, \
             pf_show_faction, rest 12], carry_goods 0, carry_gold 0}\n"
        ),
        "{town}"
    );
    let report = roundtrip_table(&atlas, &parties);
    assert_eq!((report.records, report.exact, report.failed), (2, 2, 0), "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields, report.raw_rest), (5, 12, 13));

    let templates = schema("party_templates");
    let text = printed(&atlas, &templates, "pt_cattle_herd");
    assert_eq!(
        text,
        concat!(
            "party_template cattle_herd {\n",
            "  id cattle_herd\n",
            "  name \"Cattle Herd\"\n",
            "  party_flags {flags [rest 38], carry_goods 10, carry_gold 0}\n",
            "  menu 0\n",
            "  faction fac_neutral\n",
            "  personality {courage 7, aggressiveness 0, banditness 0}\n",
            "  stacks []\n",
            "}\n",
        )
    );
    let report = roundtrip_table(&atlas, &templates);
    assert_eq!((report.records, report.exact, report.failed), (2, 2, 0), "{:?}", report.first_failures);
    assert_eq!((report.typed_fields, report.fields, report.raw_rest), (3, 7, 8));
}

#[test]
fn real_atlas_parties_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = schema("parties");
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "parties: typed {}/{} fields, raw-rest {} values, {} records, {} exact",
        report.typed_fields, report.fields, report.raw_rest, report.records, report.exact
    );
    assert_eq!(report.records, 240, "the atlas has 240 parties");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(
        report.raw_rest, 1639,
        "parties: typed {}/{} fields, raw-rest {}",
        report.typed_fields, report.fields, report.raw_rest
    );
    let main = printed(&atlas, &schema, "p_main_party");
    for line in [
        "  party_flags {flags [pf_limit_members], carry_goods 0, carry_gold 0}\n",
        "  party_template pt_none\n",
        "  faction fac_player_faction\n",
        "  personality {courage 0, aggressiveness 0, banditness 0}\n",
        "  ai_behavior [ai_bhvr_hold]\n",
    ] {
        assert!(main.contains(line), "{line:?} missing from:\n{main}");
    }
    let town = printed(&atlas, &schema, "p_town_6");
    for line in [
        "  party_flags {flags [pf_is_static, pf_label_large, pf_always_visible, \
         pf_show_faction, rest 12], carry_goods 0, carry_gold 0}\n",
        "  faction fac_neutral\n",
    ] {
        assert!(town.contains(line), "{line:?} missing from:\n{town}");
    }
}

#[test]
fn real_atlas_party_templates_round_trip() {
    if !std::path::Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = schema("party_templates");
    let report = roundtrip_table(&atlas, &schema);
    eprintln!(
        "party_templates: typed {}/{} fields, raw-rest {} values, {} records, {} exact",
        report.typed_fields, report.fields, report.raw_rest, report.records, report.exact
    );
    assert_eq!(report.records, 63, "the atlas has 63 party_templates");
    assert_eq!(report.exact, report.records, "{:?}", report.first_failures);
    assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    assert_eq!(
        report.raw_rest, 252,
        "party_templates: typed {}/{} fields, raw-rest {}",
        report.typed_fields, report.fields, report.raw_rest
    );
    let herd = printed(&atlas, &schema, "pt_cattle_herd");
    for line in [
        "  party_flags {flags [rest 38], carry_goods 10, carry_gold 0}\n",
        "  faction fac_neutral\n",
        "  personality {courage 7, aggressiveness 0, banditness 0}\n",
    ] {
        assert!(herd.contains(line), "{line:?} missing from:\n{herd}");
    }
}
