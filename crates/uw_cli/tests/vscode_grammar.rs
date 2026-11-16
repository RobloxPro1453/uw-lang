//! T-098: the VS Code extension under `editors/vscode/` must be valid JSON, and its TextMate
//! grammar must colour every keyword `uw_syntax` reads (`crates/uw_syntax/src/lib.rs`). Node-free:
//! the files are only read.

use std::path::PathBuf;

use serde_json::Value;

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../editors/vscode");

/// The keywords of `crates/uw_syntax/src/lib.rs`: top-level items, then statements / menu members.
const KEYWORDS: [&str; 20] = [
    "mod", "override", "in", "new", "troop", "on", "fn", "save", "every", "if", "else", "for",
    "require", "attempt", "option", "when", "after", "text", "change", "remove",
];

fn load(rel: &str) -> Value {
    let path = PathBuf::from(DIR).join(rel);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{} is not valid JSON: {e}", path.display()))
}

/// Every word of every regular expression (`match` / `begin` / `end`) in the grammar.
fn regex_words(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, v) in map {
                if matches!(key.as_str(), "match" | "begin" | "end") {
                    if let Value::String(re) = v {
                        out.extend(re.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).map(str::to_owned));
                    }
                }
                regex_words(v, out);
            }
        }
        Value::Array(items) => items.iter().for_each(|v| regex_words(v, out)),
        _ => {}
    }
}

/// The one test the spec asks for: the grammar parses and every keyword is in it.
#[test]
fn grammar_parses_and_colours_every_keyword() {
    let grammar = load("syntaxes/uw.tmLanguage.json");
    assert_eq!(grammar["scopeName"], "source.uw", "the grammar's scope name");
    let mut words = Vec::new();
    regex_words(&grammar, &mut words);
    let missing: Vec<&str> = KEYWORDS.iter().copied().filter(|k| !words.iter().any(|w| w.as_str() == *k)).collect();
    assert!(missing.is_empty(), "syntaxes/uw.tmLanguage.json does not colour {missing:?}");
}

#[test]
fn manifest_declares_the_uw_language() {
    let manifest = load("package.json");
    assert_eq!(manifest["name"], "uw-lang");
    let language = &manifest["contributes"]["languages"][0];
    assert_eq!(language["id"], "uw");
    assert_eq!(language["extensions"][0], ".uw");
    let grammar = &manifest["contributes"]["grammars"][0];
    assert_eq!(grammar["language"], "uw");
    assert_eq!(grammar["scopeName"], load("syntaxes/uw.tmLanguage.json")["scopeName"]);
    for rel in [language["configuration"].as_str().expect("language configuration"), grammar["path"].as_str().expect("grammar")] {
        let path = PathBuf::from(DIR).join(rel);
        assert!(path.is_file(), "{} is missing", path.display());
        load(rel);
    }
}

/// T-101: the extension runs `uw check --json` on open and save, so the manifest must name its entry
/// point, activate for the `uw` language and default `uw.path` to `uw`.
#[test]
fn manifest_runs_the_check_extension_for_uw_files() {
    let manifest = load("package.json");
    assert_eq!(manifest["main"], "./extension.js");
    let events: Vec<&str> =
        manifest["activationEvents"].as_array().expect("activationEvents").iter().map(|e| e.as_str().expect("activation event")).collect();
    assert!(events.contains(&"onLanguage:uw"), "activationEvents has no onLanguage:uw: {events:?}");
    assert_eq!(manifest["contributes"]["configuration"]["properties"]["uw.path"]["default"], "uw");
    let entry = PathBuf::from(DIR).join(manifest["main"].as_str().expect("main"));
    assert!(entry.is_file(), "{} is missing", entry.display());
}

#[test]
fn language_configuration_toggles_comments_and_matches_brackets() {
    let config = load("language-configuration.json");
    assert_eq!(config["comments"]["lineComment"], "//");
    let brackets: Vec<(String, String)> = config["brackets"]
        .as_array()
        .expect("brackets")
        .iter()
        .map(|b| (b[0].as_str().expect("bracket").to_owned(), b[1].as_str().expect("bracket").to_owned()))
        .collect();
    for (open, close) in [("{", "}"), ("(", ")")] {
        assert!(
            brackets.iter().any(|(o, c)| o.as_str() == open && c.as_str() == close),
            "the bracket pair {open}{close} is missing"
        );
    }
}
