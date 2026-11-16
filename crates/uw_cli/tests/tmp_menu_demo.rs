//! T-083: `examples/menu_demo` still parses, checks and emits with UW's own words (`lib/core.uw`)
//! after its option was rewritten to the easy-language `player.*` form. The golden cases cover the
//! same words in a script; this one keeps the example itself compiling. No kit and no Python here,
//! so this is `uw build`'s check and emit half only.

use std::fs;
use std::path::PathBuf;

use uw_diag::SourceFile;

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");
const LIB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../lib");
const DEMO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/menu_demo/menu_demo.uw");

/// Parses a file with its own name as origin; a parse error is the test's failure.
fn source(path: &std::path::Path) -> uw_check::Source {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    let file = SourceFile::new(name, text);
    let tree = uw_syntax::parse(&file.text).unwrap_or_else(|d| panic!("{}", d.render(&file)));
    uw_check::Source { file, tree }
}

#[test]
fn the_menu_demo_example_checks_against_the_fixture_atlas() {
    let mut atlas = uw_atlas::Atlas::from_path(ATLAS).expect("the fixture atlas loads");
    atlas.set_game_names(&uw_decomp::names::pairs(&uw_decomp::names::derive(&atlas)));
    let mut paths: Vec<PathBuf> = fs::read_dir(LIB)
        .unwrap_or_else(|e| panic!("{LIB}: {e}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "uw"))
        .collect();
    paths.sort();
    let mut sources: Vec<uw_check::Source> = paths.iter().map(|path| source(path)).collect();
    sources.push(source(std::path::Path::new(DEMO)));
    let checked = uw_check::check(&atlas, &sources, "menu_demo").unwrap_or_else(|errors| {
        let text: Vec<String> =
            errors.iter().map(|e| e.diag.render(&sources[e.file].file)).collect();
        panic!("{}", text.join("\n"));
    });
    let files = uw_emit::emit(&checked, false).files;
    let menus = files.iter().find(|f| f.name == "module_game_menus.py").expect("module_game_menus.py");
    // The new option body, lowered through the lib's object words.
    for expected in [
        "(troop_add_gold, \"trp_player\", 1000)",
        "(troop_remove_gold, \"trp_player\", 50)",
        "(call_script, \"script_change_troop_renown\", \"trp_player\", 10)",
        "(call_script, \"script_change_player_honor\", 1)",
        "(ge, \":_uw_get",
        "(display_message, \"@UW: player.gold, renown and honor work.\")",
        "(party_relocate_near_party, \"p_main_party\", \"p_town_6\", 1)",
        "change_screen_map,",
    ] {
        assert!(menus.text.contains(expected), "missing {expected} in:\n{}", menus.text);
    }
}
