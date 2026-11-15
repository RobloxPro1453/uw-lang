//! T-065: the dialog, trigger, simple-trigger, script and string records keep printing as UW record
//! text that parses back identical, and the one field the kit's vocabulary can type — the dialog
//! speaker — prints with the kit's flag names instead of a bare packed number.
//!
//! The real-atlas tests skip themselves when `target/uw-staging/atlas.json` is absent, like
//! `records_t041.rs`; the schema checks run either way, so a broken `.toml` never slips by.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value as Json;
use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, roundtrip_table, Schema, TableReport};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

fn schema_path(table: &str) -> String {
    format!("{}/../../kit-meta/records/{table}.toml", env!("CARGO_MANIFEST_DIR"))
}

/// Loads the real atlas and the table's schema; `None` when the atlas is absent.
fn load(table: &str) -> Option<(Atlas, Schema)> {
    let schema = Schema::from_path(schema_path(table))
        .unwrap_or_else(|e| panic!("{table} schema loads: {e}"));
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return None;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    Some((atlas, schema))
}

/// Round-trips every record of `table`, prints the coverage line and requires all records exact.
fn roundtrip(atlas: &Atlas, schema: &Schema) -> TableReport {
    let report = roundtrip_table(atlas, schema);
    eprintln!(
        "{}: {} records, exact {}, typed {}/{} fields, raw-rest {} values",
        schema.table, report.records, report.exact, report.typed_fields, report.fields, report.raw_rest
    );
    assert!(report.records > 0, "{}: no records in the atlas", schema.table);
    assert_eq!(
        report.exact, report.records,
        "{}: {} of {} records exact; first failures: {:?}",
        schema.table, report.exact, report.records, report.first_failures
    );
    assert_eq!(report.failed, 0, "{}: first failures: {:?}", schema.table, report.first_failures);
    report
}

/// Field 0 of a record value (a `{"list": …}` record or a tuple), when it is a number.
fn number_of_first_field(value: &Json) -> Option<u64> {
    match value {
        Json::Object(map) => map.get("list")?.as_array()?.first()?.as_u64(),
        Json::Array(items) => items.first()?.as_u64(),
        _ => None,
    }
}

/// The ten speaker constants of `header_dialogs.py:15-25`, plus one dialog line per shape they make.
const SPEAKER_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "readable-t065", "ops": [], "hooks": [], "ranges": [],
  "constants": [
    {"name":"anyone","value":4095,"module":"header_dialogs","line":15},
    {"name":"repeat_for_factions","value":4096,"module":"header_dialogs","line":16},
    {"name":"repeat_for_parties","value":8192,"module":"header_dialogs","line":17},
    {"name":"repeat_for_troops","value":12288,"module":"header_dialogs","line":18},
    {"name":"repeat_for_100","value":16384,"module":"header_dialogs","line":19},
    {"name":"repeat_for_1000","value":20480,"module":"header_dialogs","line":20},
    {"name":"plyr","value":65536,"module":"header_dialogs","line":22},
    {"name":"party_tpl","value":131072,"module":"header_dialogs","line":23},
    {"name":"auto_proceed","value":262144,"module":"header_dialogs","line":24},
    {"name":"multi_line","value":524288,"module":"header_dialogs","line":25}
  ],
  "tables": {
    "dialogs": {"module":"module_dialogs","var":"dialogs","id_file":null,"id_prefix":null,"entries":[
      {"index":0,"name":null,"id":null,"dup":null,"line":47,
       "value":{"list":[4095,"start",{"list":[]},"a","close_window",{"list":[]}]}},
      {"index":1,"name":null,"id":null,"dup":null,"line":273,
       "value":{"list":[69631,"start",{"list":[]},"b","close_window",{"list":[]}]}},
      {"index":2,"name":null,"id":null,"dup":null,"line":308,
       "value":{"list":[81919,"start",{"list":[]},"c","close_window",{"list":[]}]}},
      {"index":3,"name":null,"id":null,"dup":null,"line":null,
       "value":{"list":[65620,"start",{"list":[]},"d","close_window",{"list":[]}]}},
      {"index":4,"name":null,"id":null,"dup":null,"line":null,
       "value":{"list":[131077,"start",{"list":[]},"e","close_window",{"list":[]}]}}
    ]}
  }
}"#;

#[test]
fn dialogs_speaker_decodes_with_the_kit_constants() {
    let schema = Schema::from_path(schema_path("dialogs")).expect("dialogs schema loads");
    let atlas = Atlas::from_str(SPEAKER_ATLAS).expect("speaker atlas loads");
    let report = roundtrip(&atlas, &schema);
    assert_eq!((report.records, report.exact, report.failed), (5, 5, 0), "{:?}", report.first_failures);
    // The speaker is the one typed field; the six others print raw in every record.
    assert_eq!((report.typed_fields, report.fields, report.raw_rest), (1, 7, 25));
    let partner = |index: usize| {
        let entry = &atlas.tables["dialogs"].entries[index];
        let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
        text.lines()
            .find(|line| line.starts_with("  dialogue_partner "))
            .unwrap_or_else(|| panic!("no dialogue_partner line in:\n{text}"))
            .to_string()
    };
    // `anyone` (module_dialogs.py:47), `anyone|plyr` (:273), `anyone|plyr|repeat_for_troops` (:308).
    assert_eq!(partner(0), "  dialogue_partner [anyone]");
    assert_eq!(partner(1), "  dialogue_partner [anyone, plyr]");
    assert_eq!(partner(2), "  dialogue_partner [anyone, repeat_for_troops, plyr]");
    // A troop id (`trp_x|plyr`, e.g. module_dialogs.py:811) or a party-template id (`party_tpl|pt_…`,
    // line 1455; the index stands in as 5 here) has no readable type inside `flags:`, so it stays a
    // `rest` remainder while the flags around it are named.
    assert_eq!(partner(3), "  dialogue_partner [plyr, rest 84]");
    assert_eq!(partner(4), "  dialogue_partner [party_tpl, rest 5]");
}

#[test]
fn dialogs_speaker_prints_with_the_kit_flag_names() {
    let schema = Schema::from_path(schema_path("dialogs")).expect("dialogs schema loads");
    // `header_dialogs.py:15-25`: the speaker packs `anyone` / `repeat_for_*` with the single-bit
    // flags `plyr`, `party_tpl`, `auto_proceed` and `multi_line`.
    assert_eq!(schema.fields[0].name, "dialogue_partner");
    assert_eq!(schema.fields[0].ty, "flags:plyr,party_tpl,auto_proceed,multi_line");
    assert_eq!(schema.fields[0].source, "module_dialogs.py:21");
    let enums: Vec<(&str, &str)> = schema.fields[0]
        .enums
        .iter()
        .map(|group| (group.mask.as_str().expect("mask is a kit constant name"), group.prefix.as_str()))
        .collect();
    assert_eq!(enums, [("anyone", "anyone"), ("repeat_for_troops", "repeat_for")]);
    assert!(schema.fields[0].zero.is_none() && schema.fields[0].parts.is_empty());
    // The rest of the record is a state name, a text id or an operation block: no readable type fits.
    assert!(schema.fields[1..].iter().all(|field| field.ty == "raw"));

    let Some((atlas, schema)) = load("dialogs") else { return };
    let report = roundtrip(&atlas, &schema);
    assert_eq!((report.typed_fields, report.fields), (1, 7));

    let mut partners: BTreeMap<u64, String> = BTreeMap::new();
    for entry in &atlas.tables["dialogs"].entries {
        let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
        let line = text
            .lines()
            .find(|line| line.starts_with("  dialogue_partner "))
            .unwrap_or_else(|| panic!("no dialogue_partner line in:\n{text}"))
            .to_string();
        // A typed field never prints as a bare number.
        assert!(
            line.starts_with("  dialogue_partner ["),
            "dialog {}: the speaker printed raw: {line}\n{text}",
            entry.index
        );
        if let Some(value) = number_of_first_field(&entry.value) {
            partners.insert(value, line);
        }
    }
    // Three speakers that exist in Native's module_dialogs.py, found by their bits:
    // `anyone` (line 47), `anyone|plyr` (line 273) and `anyone|plyr|repeat_for_troops` (line 308).
    assert_eq!(partners.get(&0xfff).map(String::as_str), Some("  dialogue_partner [anyone]"));
    assert_eq!(
        partners.get(&(0xfff | 0x10000)).map(String::as_str),
        Some("  dialogue_partner [anyone, plyr]")
    );
    assert_eq!(
        partners.get(&(0xfff | 0x3000 | 0x10000)).map(String::as_str),
        Some("  dialogue_partner [anyone, repeat_for_troops, plyr]")
    );
    // `party_tpl|pt_…` (line 1455): the party-template index is the `rest` remainder, the flag is named.
    assert!(
        partners.values().any(|line| line.contains("party_tpl")),
        "no party-template speaker printed with `party_tpl`"
    );
}

#[test]
fn triggers_records_are_exact() {
    let Some((atlas, schema)) = load("triggers") else { return };
    // The three intervals are game-time floats and the two blocks are operation blocks
    // (module_triggers.py:12-22): neither has a readable type, so every field stays raw.
    assert_eq!(schema.fields.len(), 5);
    assert!(schema.fields.iter().all(|field| field.ty == "raw"));
    let report = roundtrip(&atlas, &schema);
    assert_eq!(report.typed_fields, 0);
}

#[test]
fn simple_triggers_records_are_exact() {
    let Some((atlas, schema)) = load("simple_triggers") else { return };
    // A float interval and an operation block (module_simple_triggers.py:15-17): both stay raw.
    assert_eq!(schema.fields.len(), 2);
    assert!(schema.fields.iter().all(|field| field.ty == "raw"));
    let report = roundtrip(&atlas, &schema);
    assert_eq!(report.typed_fields, 0);
}

#[test]
fn scripts_records_are_exact() {
    let Some((atlas, schema)) = load("scripts") else { return };
    // A script name and an operation block (module_scripts.py:20-22): both stay raw.
    assert_eq!(schema.fields.len(), 2);
    assert!(schema.fields.iter().all(|field| field.ty == "raw"));
    let report = roundtrip(&atlas, &schema);
    assert_eq!(report.typed_fields, 0);
}

#[test]
fn strings_records_are_exact() {
    let Some((atlas, schema)) = load("strings") else { return };
    // Both positions are text (process_strings.py:14): a text id and its text, both stay raw.
    assert_eq!(schema.fields.len(), 2);
    assert!(schema.fields.iter().all(|field| field.ty == "raw"));
    let report = roundtrip(&atlas, &schema);
    assert_eq!(report.typed_fields, 0);
}
