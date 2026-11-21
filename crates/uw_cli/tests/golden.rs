//! Golden tests for the wrapper files `uw build` writes: every folder under `tests/golden/` holds
//! an `input.uw`, an optional `debug` marker file (the emit every-menu-ids mode of `uw build
//! --debug`) and an `expected/` folder with one `<module>.py` per generated wrapper — or a single
//! `error.txt` when the mod must fail the check.
//!
//! The test parses, checks and emits only: no kit, no Python, no game. `uw_runtime.py` is the same
//! for every mod and is skipped here.
//!
//! After a deliberate change to the generated text, `UW_BLESS=1 cargo test -p uw_cli --test golden`
//! rewrites the expected files from the current output. Blessing is not a way to make a test pass:
//! always read the diff it wrote.

use std::fs;
use std::path::{Path, PathBuf};

use uw_diag::SourceFile;

const ATLAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");
const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
/// UW's own word library at the repository root (T-079): a case with a `lib` marker file gets it.
const LIB: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../lib");
/// The record schemas (`kit-meta/records`): `troop … like …` copies a troop through troops.toml
/// (T-092), `town … like …` a party through parties.toml (T-099) and `override faction …` a faction
/// through factions.toml (T-119).
const SCHEMAS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records");

/// The generated file kept for every run; comparing it would only copy `py/uw_runtime.py`.
const RUNTIME: &str = "uw_runtime.py";

/// A CRLF checkout must not change the goldens.
fn norm(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn cases() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(GOLDEN)
        .expect("tests/golden exists")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    dirs.sort();
    assert!(!dirs.is_empty(), "no golden cases in {GOLDEN}");
    dirs
}

/// The parsed real `lib/*.uw` files, in file order: UW's own words, before the mod's source.
fn lib_sources() -> Vec<uw_check::Source> {
    let mut paths: Vec<PathBuf> = fs::read_dir(LIB)
        .unwrap_or_else(|e| panic!("{LIB}: {e}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "uw"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let file = SourceFile::new(name, text);
            let tree = uw_syntax::parse(&file.text).unwrap_or_else(|d| panic!("{}", d.render(&file)));
            uw_check::Source { file, tree }
        })
        .collect()
}

/// Parses, checks and emits `input.uw` (with UW's own words when `with_lib`): the generated
/// wrappers by name, or the rendered errors.
fn run(input: &str, debug: bool, with_lib: bool) -> Result<Vec<(String, String)>, String> {
    let mut atlas = uw_atlas::Atlas::from_path(ATLAS).expect("the golden atlas fixture loads");
    // The game names `uw build` installs (P5b): the mod's `names.uw`, or the derived Native ones.
    atlas.set_game_names(&uw_decomp::names::pairs(&uw_decomp::names::derive(&atlas)));
    let file = SourceFile::new("input.uw", input.to_owned());
    let tree = match uw_syntax::parse(&file.text) {
        Ok(tree) => tree,
        Err(d) => return Err(d.render(&file)),
    };
    let mut sources = if with_lib { lib_sources() } else { Vec::new() };
    sources.push(uw_check::Source { file: file.clone(), tree });
    let schemas: Vec<uw_decomp::records::Schema> = ["troops.toml", "parties.toml", "factions.toml", "items.toml", "skills.toml"]
        .iter()
        .map(|name| {
            uw_decomp::records::Schema::from_path(Path::new(SCHEMAS).join(name))
                .unwrap_or_else(|e| panic!("the {name} schema loads: {e}"))
        })
        .collect();
    let checked = uw_check::check_with_schemas(&atlas, &sources, "golden", &schemas).map_err(|errors| {
        errors.iter().map(|e| e.diag.render(&sources[e.file].file)).collect::<Vec<_>>().join("\n")
    })?;
    Ok(uw_emit::emit(&checked, debug)
        .files
        .into_iter()
        .filter(|f| f.name != RUNTIME)
        .map(|f| (f.name, norm(&f.text)))
        .collect())
}

/// A small unified-ish diff: the lines that differ, expected (`-`) above actual (`+`).
fn diff(label: &str, expected: &str, actual: &str) -> String {
    let old: Vec<&str> = expected.trim_end_matches('\n').lines().collect();
    let new: Vec<&str> = actual.trim_end_matches('\n').lines().collect();
    let mut out = format!("{label} differs from expected:\n");
    let mut shown = 0;
    for i in 0..old.len().max(new.len()) {
        if old.get(i) == new.get(i) {
            continue;
        }
        shown += 1;
        if shown > 20 {
            out.push_str("  ... further lines differ\n");
            break;
        }
        out.push_str(&format!("  line {}\n", i + 1));
        if let Some(line) = old.get(i) {
            out.push_str(&format!("  - {line}\n"));
        }
        if let Some(line) = new.get(i) {
            out.push_str(&format!("  + {line}\n"));
        }
    }
    out
}

/// Compares `actual` with the expected file at `path`; with `bless` the file is rewritten instead.
fn compare(bless: bool, path: &Path, actual: &str) -> Result<(), String> {
    let label = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    if bless {
        let mut text = actual.to_owned();
        if !text.ends_with('\n') {
            text.push('\n');
        }
        return fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()));
    }
    let expected = fs::read_to_string(path)
        .map_err(|e| format!("{}: {e} (`UW_BLESS=1` writes it)", path.display()))?;
    let expected = norm(&expected);
    if expected.trim_end_matches('\n') == actual.trim_end_matches('\n') {
        Ok(())
    } else {
        Err(diff(&label, &expected, actual))
    }
}

#[test]
fn generated_wrappers_match_the_golden_files() {
    let bless = std::env::var("UW_BLESS").as_deref() == Ok("1");
    let mut failures = Vec::new();
    for dir in cases() {
        let case = dir.file_name().unwrap().to_string_lossy().into_owned();
        let input = norm(&fs::read_to_string(dir.join("input.uw")).expect("every case has an input.uw"));
        let debug = dir.join("debug").is_file();
        let with_lib = dir.join("lib").is_file();
        let expected = dir.join("expected");
        let error_case = expected.join("error.txt").is_file();
        match run(&input, debug, with_lib) {
            Ok(files) => {
                if error_case {
                    failures.push(format!("{case}: expected an error, but parse/check/emit succeeded"));
                    continue;
                }
                if files.is_empty() {
                    failures.push(format!("{case}: no wrapper files were generated"));
                    continue;
                }
                for (name, text) in files {
                    let path = expected.join(&name);
                    if bless {
                        let _ = fs::create_dir_all(&expected);
                    }
                    if let Err(e) = compare(bless, &path, &text) {
                        failures.push(format!("{case}: {e}"));
                    }
                }
            }
            Err(errors) => {
                if !error_case {
                    failures.push(format!("{case}: parse/check/emit failed:\n{errors}"));
                    continue;
                }
                if let Err(e) = compare(bless, &expected.join("error.txt"), &errors) {
                    failures.push(format!("{case}: {e}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// T-079: the real `lib/core.uw` parses and checks with no error, and every word inlines to its
/// Native operations. Calling all of them forces the check into every fn body.
#[test]
fn the_real_lib_parses_and_checks_against_the_fixture_atlas() {
    let input = "mod lib_words\n\non game.start {\n    give_gold(1000)\n    take_gold(250)\n    \
                 give_item(0)\n    take_item(0)\n    add_troops(trp_player, 10)\n    \
                 remove_troops(trp_player, 5)\n    add_renown(10)\n    add_honor(5)\n    \
                 move_near(praven)\n    show_map()\n    say(\"@Hello\")\n    \
                 player.gold += 1000\n    player.gold -= 50\n    player.renown += 10\n    \
                 player.honor += 1\n    if player.gold >= 100 {\n        say(\"@Still rich.\")\n    }\n    \
                 :level = player.level\n    :renown = player.renown\n    :honor = player.honor\n    \
                 player.give(0)\n    player.remove(0)\n    if player.has(0) {\n        say(\"@Has it.\")\n    }\n}\n";
    let files = run(input, false, true).expect("the real lib/core.uw parses and checks");
    let scripts = files.iter().find(|(name, _)| name == "module_scripts.py").expect("module_scripts.py");
    for expected in [
        "(troop_add_gold, \"trp_player\", 1000)",
        "(store_troop_gold, \":gold__fn1\", \"trp_player\")",
        "(ge, \":gold__fn1\", 250)",
        "(troop_add_item, \"trp_player\", 0)",
        "(troop_remove_item, \"trp_player\", 0)",
        "(party_add_members, \"p_main_party\", \"trp_player\", 10)",
        "(party_remove_members, \"p_main_party\", \"trp_player\", 5)",
        "(call_script, \"script_change_troop_renown\", \"trp_player\", 10)",
        "(call_script, \"script_change_player_honor\", 5)",
        "(party_relocate_near_party, \"p_main_party\", \"p_town_6\", 1)",
        "change_screen_map,",
        "(display_message, \"@Hello\")",
        "(troop_remove_gold, \"trp_player\", 50)",
        "(ge, \":gold__fn14\", 50)",
        "(troop_get_slot, \":_uw_get2\", \"trp_player\", slot_troop_renown)",
        "(assign, \":_uw_get3\", \"$player_honor\")",
        "(store_character_level, \":_uw_get1\", \"trp_player\")",
        "(player_has_item, 0)",
    ] {
        assert!(scripts.1.contains(expected), "missing {expected} in:\n{}", scripts.1);
    }
}
