//! T-104: `uw words --json` lists the project's game names, the `fn` words of `lib/` and `fns/`,
//! the object fields those fns define and the language keywords.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-words-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    dir.canonicalize().expect("canonical scratch folder")
}

fn words(dir: &Path) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("words")
        .arg(dir)
        .args(["--json", "--atlas", FIXTURE])
        .output()
        .expect("uw runs");
    (out.status.success(), String::from_utf8(out.stdout).expect("utf-8 stdout"))
}

/// The one JSON line with this label, if there is one.
fn find<'a>(out: &'a str, label: &str) -> Option<serde_json::Value> {
    out.lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|item| item["label"] == label)
}

#[test]
fn a_lib_word_a_project_fn_and_an_object_field_are_listed() {
    let dir = scratch("fns");
    std::fs::create_dir_all(dir.join("lib")).expect("lib folder");
    std::fs::create_dir_all(dir.join("fns")).expect("fns folder");
    std::fs::write(
        dir.join("lib/words.uw"),
        "fn shout(:text) {\n    display_message(:text)\n}\n\nfn hero.gold.get(:out) {\n    assign(:out, 0)\n}\n",
    )
    .expect("lib/words.uw");
    std::fs::write(dir.join("fns/mine.uw"), "fn greet() {\n    display_message(\"hi\")\n}\n").expect("fns/mine.uw");

    let (ok, out) = words(&dir);
    assert!(ok, "words must succeed:\n{out}");

    let shout = find(&out, "shout").expect("the lib fn `shout` is offered");
    assert_eq!(shout["kind"], "word");
    assert!(!shout["detail"].as_str().unwrap_or("").is_empty(), "{shout}");

    let greet = find(&out, "greet").expect("the project fn `greet` is offered");
    assert_eq!(greet["kind"], "word");

    let field = find(&out, "hero.gold").expect("the object field `hero.gold` is offered");
    assert_eq!(field["kind"], "field");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_game_name_and_a_keyword_are_listed() {
    let dir = scratch("names");
    std::fs::write(dir.join("input.uw"), "mod words\n").expect("input.uw");

    let (ok, out) = words(&dir);
    assert!(ok, "words must succeed:\n{out}");

    // `uw new`'s rule on the fixture: `p_town_6` shows "Praven" -> `praven`.
    let name = find(&out, "praven").expect("the game name `praven` is offered");
    assert_eq!(name["kind"], "name");
    assert!(!name["detail"].as_str().unwrap_or("").is_empty(), "{name}");

    let keyword = find(&out, "override").expect("the keyword `override` is offered");
    assert_eq!(keyword["kind"], "keyword");

    std::fs::remove_dir_all(&dir).ok();
}
