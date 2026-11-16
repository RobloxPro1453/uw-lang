//! T-113: a record's `name_<code> "…"` lines are the project's own text for that record in one
//! language. The loader reads them before schema parsing (a schema has no such field), and
//! `uw new --check` (`verify`) must ignore them as it ignores `owner`.

use std::path::{Path, PathBuf};

use uw_atlas::Atlas;
use uw_decomp::project::load::load_with_owners_and_names;
use uw_decomp::project::{export, verify, LanguageName};
use uw_decomp::records::{parse, print, Schema};

/// Two towns, enough for a record with a name.
const TINY_ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t113", "ops": [], "hooks": [], "ranges": [],
  "tables": {
    "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_",
      "entries": [
        {"index":0,"name":"town_6","id":"p_town_6","dup":null,"line":82,
          "value":["town_6","Praven",0]},
        {"index":1,"name":"town_1","id":"p_town_1","dup":null,"line":77,
          "value":["town_1","Uxkhal",0]}
      ]}
  }
}"#;

const PARTIES_SCHEMA: &str = r#"table = "parties"
kind = "party"
id_prefix = "p_"

[[fields]]
name = "id"
ty = "raw"
source = "module_parties.py:10"

[[fields]]
name = "name"
ty = "raw"
source = "module_parties.py:12"

[[fields]]
name = "x"
ty = "raw"
source = "module_parties.py:13"
"#;

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-name-t113-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

/// Exports the tiny atlas into `<root>/hikayem` and returns that folder with its schemas.
fn exported(atlas: &Atlas, root: &Path) -> (PathBuf, Vec<Schema>) {
    let schemas = vec![Schema::from_toml(PARTIES_SCHEMA).expect("parties schema")];
    let out = root.join("hikayem");
    export(atlas, &schemas, &out, &mut |schema, entry| print(schema, entry)).expect("export");
    (out, schemas)
}

/// The party file with a name line added to `town_6`.
fn with_name_line(out: &Path, line: &str) -> PathBuf {
    let path = out.join("parties/town_6.uw");
    let text = std::fs::read_to_string(&path).expect("party file");
    let anchor = "  name \"Praven\"\n";
    assert!(text.contains(anchor), "{text}");
    std::fs::write(&path, text.replace(anchor, &format!("{anchor}{line}\n"))).expect("write");
    path
}

#[test]
fn name_lines_are_read_per_language_and_the_record_still_matches() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let root = scratch("read");
    let (out, schemas) = exported(&atlas, &root);
    with_name_line(&out, r#"  name_tr "Adam Siken""#);
    with_name_line(&out, r#"  name_de "Ein \"Zitat\" \\ und\nzwei""#);

    let (records, _owners, names) =
        load_with_owners_and_names(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("load");

    let expected = vec![
        LanguageName {
            table: "parties".into(),
            id: "p_town_6".into(),
            language: "de".into(),
            text: "Ein \"Zitat\" \\ und\nzwei".into(),
        },
        LanguageName {
            table: "parties".into(),
            id: "p_town_6".into(),
            language: "tr".into(),
            text: "Adam Siken".into(),
        },
    ];
    assert_eq!(names, expected);
    // The schema never saw the extra lines: the value is still the atlas value.
    let table = &atlas.tables["parties"];
    assert_eq!(records["parties"], table.entries.iter().map(|entry| entry.value.clone()).collect::<Vec<_>>());

    // `uw new --check` reads the same folder back and must ignore the name lines too.
    let summary = verify(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text)).expect("verify");
    assert_eq!((summary.records, summary.exact, summary.failed), (2, 2, 0), "{summary:?}");

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_name_line_without_a_string_is_reported_not_dropped() {
    let atlas = Atlas::from_str(TINY_ATLAS).expect("tiny atlas");
    let root = scratch("broken");
    let (out, schemas) = exported(&atlas, &root);
    with_name_line(&out, "  name_tr Adam");

    let errors = load_with_owners_and_names(&atlas, &schemas, &out, &mut |schema, text| parse(schema, text))
        .expect_err("a value that is not a quoted string stays a schema error");
    assert!(errors.iter().any(|error| error.contains("name_tr")), "{errors:?}");

    let _ = std::fs::remove_dir_all(&root);
}
