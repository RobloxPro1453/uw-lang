//! T-122: `kit-meta/hardwired.toml` lists, per table, the records and positions the engine itself
//! depends on, so a from-scratch mod (`base = "empty"`) knows what it must keep. Every record carries
//! one kit evidence line; this test keeps the file honest against the kit and against the atlas dump.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use uw_atlas::Atlas;

const TOML: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/hardwired.toml");
const KIT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../Module_system 1.171");
const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");

/// Table -> the kit file that numbers its records (the kit writes ID = list index).
const ID_FILES: &[(&str, &str)] = &[
    ("animations", "ID_animations.py"),
    ("factions", "ID_factions.py"),
    ("items", "ID_items.py"),
    ("menus", "ID_menus.py"),
    ("mission_templates", "ID_mission_templates.py"),
    ("parties", "ID_parties.py"),
    ("party_templates", "ID_party_templates.py"),
    ("skills", "ID_skills.py"),
    ("strings", "ID_strings.py"),
    ("troops", "ID_troops.py"),
];

#[derive(Deserialize)]
struct Hardwired {
    record: Vec<Record>,
}

#[derive(Deserialize)]
struct Record {
    table: String,
    id: String,
    position: usize,
    reason: String,
    evidence: String,
    certainty: String,
}

fn records() -> Vec<Record> {
    let text = fs::read_to_string(TOML).unwrap_or_else(|e| panic!("{TOML}: {e}"));
    let file: Hardwired = toml::from_str(&text).unwrap_or_else(|e| panic!("{TOML}: {e}"));
    file.record
}

/// `evidence` split into the kit file name and the 1-based line number.
fn evidence_parts(record: &Record) -> (&str, usize) {
    let (file, line) = record
        .evidence
        .rsplit_once(':')
        .unwrap_or_else(|| panic!("{}: evidence {:?} is not file:line", record.id, record.evidence));
    let line = line
        .parse()
        .unwrap_or_else(|_| panic!("{}: evidence {:?} has no line number", record.id, record.evidence));
    (file, line)
}

/// Every record points at a real kit line, and its id sits at that position in the kit's ID file.
#[test]
fn every_record_has_kit_evidence() {
    let records = records();
    assert!(!records.is_empty(), "kit-meta/hardwired.toml has no [[record]]");
    let mut seen = HashSet::new();
    for record in &records {
        assert!(!record.reason.trim().is_empty(), "{}: empty reason", record.id);
        assert!(
            matches!(record.certainty.as_str(), "comment" | "inferred"),
            "{}: certainty {:?} is neither comment nor inferred",
            record.id,
            record.certainty
        );
        assert!(
            seen.insert((record.table.clone(), record.position)),
            "{}: {} position {} is claimed twice",
            record.id,
            record.table,
            record.position
        );
        assert!(
            ID_FILES.iter().any(|(table, _)| table == &record.table),
            "{}: unknown table {}",
            record.id,
            record.table
        );
    }

    let kit = Path::new(KIT);
    if !kit.is_dir() {
        println!("note: no kit at {KIT}, skipping the kit line checks");
        return;
    }
    for record in &records {
        let (file, line) = evidence_parts(record);
        // The kit files are cp1254 (see module_strings.py:1), so read the bytes and decode leniently;
        // only line numbering and comments matter here.
        let bytes = fs::read(kit.join(file))
            .unwrap_or_else(|e| panic!("{}: evidence file {file}: {e}", record.id));
        let text = String::from_utf8_lossy(&bytes).into_owned();
        let lines: Vec<&str> = text.lines().collect();
        assert!(
            line >= 1 && line <= lines.len(),
            "{}: evidence {} is outside {file} ({} lines)",
            record.id,
            record.evidence,
            lines.len()
        );
        if record.certainty == "comment" {
            assert!(
                lines[line - 1].trim_start().starts_with('#'),
                "{}: evidence {} is not a kit comment: {:?}",
                record.id,
                record.evidence,
                lines[line - 1]
            );
        }
        let id_file = ID_FILES.iter().find(|(table, _)| table == &record.table).unwrap().1;
        let ids = fs::read(kit.join(id_file)).unwrap_or_else(|e| panic!("{id_file}: {e}"));
        let ids = String::from_utf8_lossy(&ids).into_owned();
        let want = format!("{} = {}", record.id, record.position);
        assert!(
            ids.lines().any(|line| line.trim() == want),
            "{id_file} does not define `{want}` ({} cites {})",
            record.id,
            record.evidence
        );
    }
}

/// Each id resolves in the atlas dump, at the position the kit numbers it.
#[test]
fn ids_resolve_in_the_atlas() {
    let records = records();
    assert!(!records.is_empty(), "kit-meta/hardwired.toml has no [[record]]");
    if !Path::new(ATLAS).exists() {
        println!("note: no atlas at {ATLAS}, skipping the id checks");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).unwrap_or_else(|e| panic!("{ATLAS}: {e}"));
    for record in &records {
        let (table_name, _) = atlas
            .entry_by_id(&record.id)
            .unwrap_or_else(|| panic!("{} does not resolve in the atlas", record.id));
        assert_eq!(table_name, record.table, "{} resolves in {table_name}", record.id);
        let table = atlas.tables.get(&record.table).unwrap();
        let entry = table
            .entries
            .iter()
            .find(|entry| entry.index == record.position)
            .unwrap_or_else(|| panic!("{}: {} has no entry at {}", record.id, record.table, record.position));
        match &entry.id {
            Some(id) => assert_eq!(
                id, &record.id,
                "{}: {} position {} holds {id}",
                record.id, record.table, record.position
            ),
            // The kit repeats this name, so the id belongs to the later occurrence; the position is
            // still this record, so it is checked through the table's id prefix and the name.
            None => {
                assert!(entry.dup.is_some(), "{}: entry at {} has no id and no dup", record.id, record.position);
                let prefix = table.id_prefix.as_deref().unwrap_or("");
                let name = entry
                    .name
                    .as_deref()
                    .unwrap_or_else(|| panic!("{}: no name at {}", record.id, record.position));
                assert_eq!(
                    format!("{prefix}{name}"),
                    record.id,
                    "{}: {} position {} holds name {name:?}",
                    record.id,
                    record.table,
                    record.position
                );
            }
        }
    }
}
