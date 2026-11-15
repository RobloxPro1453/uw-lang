//! Operation blocks of the `dialogs` table(s).
//!
//! One dialog line is `[speaker, state, condition, text, next state, consequence, optional voice]`
//! (`module_dialogs.py` header comment), so the operation blocks sit at paths `[2]` and `[5]`.

use crate::{items, parse_ops, Block};
use serde_json::Value as Json;
use uw_atlas::Atlas;

/// Atlas key of the table (`py/atlas_dump.py` table list).
const TABLE: &str = "dialogs";

/// Fields of a dialog line that hold an operation block, with the label part each one gets.
/// An empty list is a block with no ops: the field is there, it just has nothing to do.
const FIELDS: [(usize, &str); 2] = [(2, "condition"), (5, "consequence")];

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    walk(atlas).0
}

/// Number of fields that should be an operation block but [`parse_ops`] cannot read (a float
/// operand, a nested list, ...). A field the entry does not have is absent, not unreadable.
pub fn unreadable(atlas: &Atlas) -> usize {
    walk(atlas).1
}

/// Both answers in one pass: the blocks, and how many block fields could not be read.
fn walk(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut blocks = Vec::new();
    let mut unreadable = 0;
    let Some(table) = atlas.tables.get(TABLE) else {
        return (blocks, unreadable);
    };
    for entry in &table.entries {
        let Some(fields) = items(&entry.value) else {
            continue;
        };
        // Dialog lines have no name; the state is what tells two lines apart in reports.
        let state = fields.get(1).and_then(Json::as_str);
        for (path, part) in FIELDS {
            let Some(field) = fields.get(path) else {
                continue;
            };
            match parse_ops(field) {
                Some(ops) => blocks.push(Block {
                    table: TABLE.to_string(),
                    index: entry.index,
                    path: vec![path],
                    label: match state {
                        Some(state) => format!("{TABLE}/{} {state}/{part}", entry.index),
                        None => format!("{TABLE}/{}/{part}", entry.index),
                    },
                    ops,
                }),
                None => unreadable += 1,
            }
        }
    }
    (blocks, unreadable)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two dialog lines: the first has a condition and an empty consequence, the second is the
    /// other way round (`"list"` wrappers as the dump writes them).
    const ATLAS: &str = r#"{
      "schema": 1, "kit_hash": "dialogs", "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "dialogs": {"module": "module_dialogs", "var": "dialogs", "id_file": null, "id_prefix": null,
          "entries": [
            {"index": 0, "name": null, "id": null, "dup": null, "line": null, "value": {"list": [
              "anyone", "start", {"list": [[2200, "$g_talk_troop"], [2199, "$g_talk_agent"]]},
              "Hello", "close_window", {"list": []}
            ]}},
            {"index": 1, "name": null, "id": null, "dup": null, "line": null, "value": {"list": [
              "plyr", "lord_talk", {"list": []}, "Text", "tavernkeeper_talk",
              {"list": [[2060, "mnu_start"], 4]}
            ]}}
          ]}
      }
    }"#;

    #[test]
    fn blocks_have_the_dialog_paths_labels_and_ops() {
        let atlas = Atlas::from_str(ATLAS).expect("atlas loads");
        let blocks = blocks(&atlas);
        assert_eq!(unreadable(&atlas), 0);
        let seen: Vec<_> = blocks.iter().map(|b| (b.table.as_str(), b.index, b.path.clone(), b.label.as_str())).collect();
        assert_eq!(
            seen,
            vec![
                ("dialogs", 0, vec![2], "dialogs/0 start/condition"),
                ("dialogs", 0, vec![5], "dialogs/0 start/consequence"),
                ("dialogs", 1, vec![2], "dialogs/1 lord_talk/condition"),
                ("dialogs", 1, vec![5], "dialogs/1 lord_talk/consequence"),
            ]
        );
        let counts: Vec<usize> = blocks.iter().map(|b| b.ops.len()).collect();
        assert_eq!(counts, vec![2, 0, 0, 2]);
        // The consequence of the second line really is that line's field 6, not the condition.
        assert_eq!(blocks[3].ops[0].code, 2060);
        assert_eq!(blocks[0].ops[0].code, 2200);
        assert_eq!(blocks[3].ops[1], crate::RawOp { code: 4, args: vec![] });
    }

    #[test]
    fn an_unreadable_field_is_counted_and_emitted_nowhere() {
        // Field 2 holds a float operand, which parse_ops refuses; field 5 is readable.
        let text = r#"{
          "schema": 1, "kit_hash": "bad", "ops": [], "hooks": [], "ranges": [],
          "tables": {
            "dialogs": {"module": "module_dialogs", "var": "dialogs", "id_file": null, "id_prefix": null,
              "entries": [
                {"index": 7, "name": null, "id": null, "dup": null, "line": null, "value": {"list": [
                  "anyone", "start", {"list": [[31, 1, 1.5]]}, "Hi", "close_window", {"list": [[4]]}
                ]}}
              ]}
          }
        }"#;
        let atlas = Atlas::from_str(text).expect("atlas loads");
        assert_eq!(unreadable(&atlas), 1);
        let blocks = blocks(&atlas);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].path, vec![5]);
        assert_eq!(blocks[0].label, "dialogs/7 start/consequence");
        assert_eq!(blocks[0].ops.len(), 1);
    }

    /// Coverage proof on the real atlas (`py/atlas_dump.py` output, not in git). Skips itself when
    /// the atlas was never built, like `crates/uw_atlas/tests/real_atlas.rs`.
    #[test]
    fn real_atlas_dialogs_are_all_readable() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let blocks = blocks(&atlas);
        let unreadable = unreadable(&atlas);
        println!("dialogs: {} blocks, {unreadable} unreadable", blocks.len());
        assert!(!blocks.is_empty());
        assert_eq!(unreadable, 0);
    }
}
