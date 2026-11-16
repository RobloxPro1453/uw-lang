//! T-075: record field and part names are full English words (`docs/tasks/T-075.md`). A troop must
//! print `strength 20`, never `str 20`, and no schema may bring an old short name back.

use std::path::Path;

use uw_atlas::Atlas;
use uw_decomp::records::{print_with_blocks, Schema};

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
const TROOPS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/troops.toml");
const SCHEMAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

/// The old UW-own short names T-075 replaced; no schema may use one again.
const OLD_NAMES: &[&str] = &[
    "str",
    "agi",
    "int",
    "cha",
    "min_pos",
    "max_pos",
    "anim_length",
    "num_particles_per_second",
    "emit_dir_randomness",
];

#[test]
fn real_atlas_troop_attributes_use_full_words() {
    if !Path::new(ATLAS).exists() {
        eprintln!("skipped: no atlas at {ATLAS}");
        return;
    }
    let atlas = Atlas::from_path(ATLAS).expect("full atlas loads");
    let schema = Schema::from_path(TROOPS).expect("troops schema loads");
    let entry = atlas.tables["troops"]
        .entries
        .iter()
        .find(|entry| entry.id.as_deref() == Some("trp_knight_1_1"))
        .expect("trp_knight_1_1 in the atlas");
    let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
    // module_troops.py:980 uses knight_attrib_5 = str_20|agi_20|int_15|cha_25|level(41) (:120).
    assert!(text.contains("strength 20"), "{text}");
    assert!(text.contains("agility 20"), "{text}");
    assert!(text.contains("intelligence 15"), "{text}");
    assert!(text.contains("charisma 25"), "{text}");
    assert!(!text.contains("str 20"), "{text}");
}

#[test]
fn no_schema_uses_an_old_short_name() {
    let mut checked = 0;
    for entry in std::fs::read_dir(SCHEMAS).expect("schema folder") {
        let path = entry.expect("schema entry").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }
        let schema = Schema::from_path(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        checked += 1;
        for field in &schema.fields {
            assert!(
                !OLD_NAMES.contains(&field.name.as_str()),
                "{}: field `{}` is an old short name",
                path.display(),
                field.name
            );
            for part in &field.parts {
                assert!(
                    !OLD_NAMES.contains(&part.name.as_str()),
                    "{}: part `{}` is an old short name",
                    path.display(),
                    part.name
                );
            }
        }
    }
    assert!(checked > 0, "no schemas found under {SCHEMAS}");
}
