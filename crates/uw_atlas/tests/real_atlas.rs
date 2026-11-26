//! Loads the full atlas made by `py/atlas_dump.py` when it exists on this machine
//! (`target/uw-staging/atlas.json`, see docs/DEVELOPMENT.md). Skips itself otherwise.

use uw_atlas::Atlas;

#[test]
fn full_native_atlas_loads_and_resolves() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !std::path::Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    assert_eq!(atlas.tables.len(), 27);
    assert_eq!(atlas.hooks.len(), 49);
    let (table, praven) = atlas.entry_by_id("p_town_6").expect("p_town_6");
    assert_eq!((table, praven.index, praven.line), ("parties", 26, Some(82)));
    assert_eq!(atlas.entry_by_id("script_game_start").expect("game_start").1.index, 0);
    // Every shadowed duplicate is reachable as id#n.
    for t in atlas.tables.values() {
        for e in t.entries.iter().filter(|e| e.dup.is_some() && e.id.is_none()) {
            let prefix = t.id_prefix.as_deref().expect("named table");
            let key = format!("{prefix}{}#{}", e.name.as_deref().unwrap(), e.dup.unwrap());
            let (_, found) = atlas.entry_by_id(&key).unwrap_or_else(|| panic!("{key} resolves"));
            assert_eq!(found.index, e.index, "{key}");
        }
    }
}

/// kit-meta/hooks.toml (py/hooks_meta.py) must list exactly the atlas' hooks, in kit order.
#[test]
fn hooks_toml_matches_atlas_hooks() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !std::path::Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let atlas = Atlas::from_path(path).expect("full atlas loads");
    let toml = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/hooks.toml"))
        .expect("kit-meta/hooks.toml");
    let names: Vec<&str> = toml
        .lines()
        .filter_map(|l| l.strip_prefix('[')?.strip_suffix(']'))
        .collect();
    assert_eq!(names, atlas.hooks.iter().map(String::as_str).collect::<Vec<_>>());
}
