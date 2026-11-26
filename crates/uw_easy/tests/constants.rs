//! prefix rules for kit constants and slots.

use std::path::Path;

use serde_json::Value;
use uw_atlas::Atlas;
use uw_easy::{Kind, Prefixes};

fn prefix_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/easy/prefixes.toml")
}

fn fixture() -> (Atlas, Prefixes) {
    let atlas = Atlas::from_str(include_str!("fixtures/atlas_easy.json")).expect("fixture atlas");
    let prefixes = Prefixes::from_toml(include_str!("../../../kit-meta/easy/prefixes.toml"))
        .expect("prefix dictionary");
    (atlas, prefixes)
}

#[test]
fn fixture_prefixes_and_flags() {
    let (atlas, prefixes) = fixture();
    let constants = uw_easy::constants(&atlas, &prefixes);

    let find = |native: &str| {
        constants
            .iter()
            .find(|record| record.native == native)
            .unwrap_or_else(|| panic!("{native} has a record"))
    };

    let alt = find("key_left_alt");
    assert_eq!(alt.easy, "key.left_alt");
    assert_eq!(alt.family, "key");
    assert_eq!(alt.kind, Kind::Enum);
    assert_eq!(alt.line, Some(228));
    assert_eq!(alt.value, Value::from(56));

    // A member starting with a digit keeps the native word, so it stays readable.
    assert_eq!(find("key_1").easy, "key.key_1");

    let read_only = find("prsntf_read_only");
    assert_eq!(read_only.easy, "overlay.flag.read_only");
    assert_eq!(read_only.family, "overlay.flag");
    assert_eq!(read_only.kind, Kind::Flag);
    assert_eq!(find("prsntf_manual_end_only").kind, Kind::Flag);

    // tf_ means troop flags in header_troops but text flags in presentations.
    assert_eq!(find("tf_hero").easy, "troop.flag.hero");
    assert_eq!(find("tf_male").kind, Kind::Flag);
    assert_eq!(find("tf_left_align").easy, "text.flag.left_align");
    assert_eq!(find("tf_left_align").kind, Kind::Flag);

    // Masked item types are one enum family, not a flag set (D3 §3).
    assert_eq!(find("itp_type_horse").easy, "item.type.horse");
    assert_eq!(find("itp_type_horse").family, "item.type");
    assert_eq!(find("itp_type_horse").kind, Kind::Enum);
    assert_eq!(find("imodbit_plain").kind, Kind::Flag);
    assert_eq!(find("str_3").easy, "attributes.str_3");
    assert_eq!(find("str_3").kind, Kind::Enum);

    // slot_* belong to slots.json only.
    assert!(!constants.iter().any(|record| record.native.starts_with("slot_")));

    let slots = uw_easy::slots(&atlas);
    let lord = slots
        .iter()
        .find(|record| record.native == "slot_town_lord")
        .expect("slot_town_lord");
    assert_eq!(lord.easy, "town.lord");
    assert_eq!(lord.family, "town");
    assert_eq!(lord.value, Value::from(7));

    for slot in slots
        .iter()
        .filter(|record| record.native.starts_with("slot_party_template_"))
    {
        assert!(
            slot.easy.starts_with("party_template."),
            "{} -> {}",
            slot.native,
            slot.easy
        );
        assert!(!slot.easy.starts_with("party."), "{}", slot.native);
    }
    assert_eq!(
        slots
            .iter()
            .find(|record| record.native == "slot_party_template_lair_type")
            .unwrap()
            .easy,
        "party_template.lair_type"
    );
    assert_eq!(
        slots
            .iter()
            .find(|record| record.native == "slot_party_ignore_player_until")
            .unwrap()
            .easy,
        "party.ignore_player_until"
    );

    // Same family + same value: one concept, one easy name, the other an alias.
    let first = slots
        .iter()
        .find(|record| record.native == "slot_faction_quick_battle_tier_1_infantry")
        .unwrap();
    let second = slots
        .iter()
        .find(|record| record.native == "slot_faction_tier_1_troop")
        .unwrap();
    assert_eq!(first.easy, second.easy);
    assert!(first.alias_of.is_none() || second.alias_of.is_none());
    assert!(first.alias_of.is_some() || second.alias_of.is_some());
}

#[test]
fn committed_json_contains_easy_names() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/easy");
    let constants: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("constants.json")).expect("constants.json"))
            .expect("constants.json parses");
    let constants = constants.as_array().expect("constants.json is a list");
    let find = |native: &str| {
        constants
            .iter()
            .find(|record| record["native"].as_str() == Some(native))
            .unwrap_or_else(|| panic!("{native} in constants.json"))
    };
    assert_eq!(find("key_left_alt")["easy"].as_str(), Some("key.left_alt"));
    assert_eq!(
        find("prsntf_read_only")["easy"].as_str(),
        Some("overlay.flag.read_only")
    );
    assert_eq!(find("prsntf_read_only")["kind"].as_str(), Some("flag"));
    assert_eq!(find("itp_type_horse")["kind"].as_str(), Some("enum"));
    assert!(!constants
        .iter()
        .any(|record| record["native"].as_str().is_some_and(|n| n.starts_with("slot_"))));

    let slots: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("slots.json")).expect("slots.json"))
            .expect("slots.json parses");
    let slots = slots.as_array().expect("slots.json is a list");
    assert!(slots.len() >= 550, "only {} slots in slots.json", slots.len());
    let lord = slots
        .iter()
        .find(|record| record["native"].as_str() == Some("slot_town_lord"))
        .expect("slot_town_lord in slots.json");
    assert_eq!(lord["easy"].as_str(), Some("town.lord"));
    for slot in slots.iter().filter(|record| {
        record["native"]
            .as_str()
            .is_some_and(|native| native.starts_with("slot_party_template_"))
    }) {
        let easy = slot["easy"].as_str().expect("easy name");
        assert!(easy.starts_with("party_template."), "{easy}");
        assert!(!easy.starts_with("party."), "{easy}");
    }
}

#[test]
fn real_atlas_key_words_and_slots() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/uw-staging/atlas.json"
    );
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    let prefixes = Prefixes::from_path(prefix_path()).expect("prefix dictionary");

    for entry in prefixes.entries() {
        let (location, example) = entry
            .evidence
            .split_once(" (")
            .unwrap_or_else(|| panic!("bad evidence for {}: {}", entry.prefix, entry.evidence));
        let example = example
            .strip_suffix(')')
            .unwrap_or_else(|| panic!("bad evidence example: {}", entry.evidence));
        let (module, line) = location
            .split_once(".py:")
            .unwrap_or_else(|| panic!("bad evidence module: {}", entry.evidence));
        let line: u32 = line.parse().expect("evidence line number");
        let constant = atlas
            .constants
            .iter()
            .find(|constant| constant.name == example)
            .unwrap_or_else(|| panic!("{}: example {example} not in atlas", entry.prefix));
        assert_eq!(
            constant.module, module,
            "{}: evidence module for {example}",
            entry.prefix
        );
        assert_eq!(
            constant.line,
            Some(line),
            "{}: evidence line for {example}",
            entry.prefix
        );
        assert!(
            example.starts_with(&entry.prefix),
            "{}: example {example} does not start with the prefix",
            entry.prefix
        );
        if let Some(expected_module) = &entry.module {
            assert_eq!(
                &constant.module, expected_module,
                "{}: evidence example is in the rule's module",
                entry.prefix
            );
        }
    }

    let constants = uw_easy::constants(&atlas, &prefixes);
    let slots = uw_easy::slots(&atlas);

    let mut keys = 0;
    for constant in atlas.constants.iter().filter(|c| c.name.starts_with("key_")) {
        keys += 1;
        let record = constants
            .iter()
            .find(|record| record.native == constant.name)
            .unwrap_or_else(|| panic!("{} has a record", constant.name));
        assert!(
            record.easy.starts_with("key."),
            "{} -> {}",
            constant.name,
            record.easy
        );
        assert_eq!(record.family, "key");
    }
    assert!(keys > 100, "only {keys} key constants");

    let lord = slots
        .iter()
        .find(|record| record.native == "slot_town_lord")
        .expect("slot_town_lord");
    assert_eq!(lord.easy, "town.lord");

    for slot in slots
        .iter()
        .filter(|record| record.native.starts_with("slot_party_template_"))
    {
        assert!(
            slot.easy.starts_with("party_template."),
            "{} -> {}",
            slot.native,
            slot.easy
        );
        assert!(!slot.easy.starts_with("party."), "{}", slot.native);
    }

    assert!(
        slots.len() >= 550,
        "only {} slots named (D3 measured 572 definitions)",
        slots.len()
    );

    // The committed JSON must be exactly what the generator writes today.
    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../kit-meta/easy");
    let expected = serde_json::to_value(&constants).expect("constants serialize");
    let committed: Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("constants.json")).expect("constants.json"),
    )
    .expect("constants.json parses");
    assert_eq!(committed, expected, "constants.json is stale");
    let expected = serde_json::to_value(&slots).expect("slots serialize");
    let committed: Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("slots.json")).expect("slots.json"),
    )
    .expect("slots.json parses");
    assert_eq!(committed, expected, "slots.json is stale");
}
