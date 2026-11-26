//! Operation blocks of the `triggers` and `simple_triggers` tables.
//!
//! Kit layout (header comment of `module_triggers.py` / `module_simple_triggers.py`):
//! triggers are `(check interval, delay, rearm, conditions, consequences)` and simple triggers are
//! `(interval, operations)`, so the blocks sit at paths `[3]`, `[4]` and `[1]`.

use crate::{items, parse_ops, Block};
use uw_atlas::Atlas;

/// Per table, the fields that hold an operation block: `(path, part of the label)`.
const FIELDS: [(&str, &[(usize, &str)]); 2] = [
    ("triggers", &[(3, "condition"), (4, "consequence")]),
    ("simple_triggers", &[(1, "ops")]),
];

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    scan(atlas).0
}

/// Fields that should hold an operation block but that [`parse_ops`] rejects.
pub fn unreadable(atlas: &Atlas) -> usize {
    scan(atlas).1
}

/// Reads all blocks of [`FIELDS`] and counts the fields that are not operation blocks.
fn scan(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut blocks = Vec::new();
    let mut unreadable = 0;
    for (table_name, fields) in FIELDS {
        let Some(table) = atlas.tables.get(table_name) else { continue };
        for entry in &table.entries {
            let head = entry.name.clone().unwrap_or_else(|| entry.index.to_string());
            for (path, part) in fields {
                // A short tuple has no such field at all; only a field that is there but is not
                // an operation block is counted as unreadable.
                let Some(field) = items(&entry.value).and_then(|v| v.get(*path)) else { continue };
                match parse_ops(field) {
                    Some(ops) => blocks.push(Block {
                        table: table_name.to_string(),
                        index: entry.index,
                        path: vec![*path],
                        label: format!("{table_name}/{head}/{part}"),
                        ops,
                    }),
                    None => unreadable += 1,
                }
            }
        }
    }
    (blocks, unreadable)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two entries of each table, plus a `scripts` table that must be left alone.
    const INLINE: &str = r#"{
      "schema": 1, "kit_hash": "inline",
      "ops": [
        {"name":"party_is_active","code":132,"line":94,"signature":"(party_is_active,<party_id>),","lhs":false,"can_fail":true,"global_lhs":false},
        {"name":"display_message","code":1106,"line":482,"signature":"(display_message,<string_id>,[hex_colour_code]),","lhs":false,"can_fail":false,"global_lhs":false}
      ],
      "hooks": [],
      "ranges": [],
      "tables": {
        "triggers": {"module":"module_triggers","var":"triggers","id_file":null,"id_prefix":null,
          "entries": [
            {"index":0,"name":null,"id":null,"dup":null,"line":33,
              "value":[0.1, 0.0, 0.1, {"list":[[132, "p_main_party"]]}, {"list":[[1106, "str_tutorial_map1"]]}]},
            {"index":1,"name":null,"id":null,"dup":null,"line":36,
              "value":[24.0, 0, 168.0, {"list":[]}, {"list":[4, [1106, "str_x", ":a"]]}]}
          ]},
        "simple_triggers": {"module":"module_simple_triggers","var":"simple_triggers","id_file":null,"id_prefix":null,
          "entries": [
            {"index":0,"name":null,"id":null,"dup":null,"line":50,
              "value":[24, {"list":[[132, "p_main_party"]]}]},
            {"index":1,"name":null,"id":null,"dup":null,"line":58,
              "value":[1.0, {"list":[]}]}
          ]},
        "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
          "entries": [{"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,
              "value":[[1106, "str_from_scripts"]]}]}
      }
    }"#;

    fn parse(text: &str) -> Atlas {
        Atlas::from_str(text).expect("inline atlas loads")
    }

    #[test]
    fn blocks_are_the_trigger_and_simple_trigger_fields() {
        let atlas = parse(INLINE);
        let blocks = blocks(&atlas);
        let got: Vec<(&str, usize, Vec<usize>, &str, usize)> = blocks
            .iter()
            .map(|b| (b.table.as_str(), b.index, b.path.clone(), b.label.as_str(), b.ops.len()))
            .collect();
        assert_eq!(
            got,
            vec![
                ("triggers", 0, vec![3], "triggers/0/condition", 1),
                ("triggers", 0, vec![4], "triggers/0/consequence", 1),
                ("triggers", 1, vec![3], "triggers/1/condition", 0),
                ("triggers", 1, vec![4], "triggers/1/consequence", 2),
                ("simple_triggers", 0, vec![1], "simple_triggers/0/ops", 1),
                ("simple_triggers", 1, vec![1], "simple_triggers/1/ops", 0),
            ]
        );
        // The operations themselves come through whole, operands included.
        assert_eq!(
            blocks[1].ops,
            vec![crate::RawOp {
                code: 1106,
                args: vec![crate::RawArg::Str("str_tutorial_map1".into())],
            }]
        );
        assert_eq!(blocks[3].ops[1].code, 1106);
        assert_eq!(blocks[3].ops[1].args, vec![crate::RawArg::Str("str_x".into()), crate::RawArg::Str(":a".into())]);
        assert_eq!(unreadable(&atlas), 0);
    }

    #[test]
    fn unreadable_counts_fields_that_are_not_blocks() {
        // The consequence of trigger 0 and the operations of simple trigger 0 hold a float
        // operand, so `parse_ops` rejects them; those fields yield no block but are counted.
        let text = INLINE
            .replace(r#"{"list":[[1106, "str_tutorial_map1"]]}"#, r#"{"list":[[1106, 1.5]]}"#)
            .replace(r#"{"list":[[132, "p_main_party"]]}]}"#, r#"{"list":[[132, 1.5]]}]}"#);
        let atlas = parse(&text);
        let blocks = blocks(&atlas);
        let labels: Vec<&str> = blocks.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(
            labels,
            vec![
                "triggers/0/condition",
                "triggers/1/condition",
                "triggers/1/consequence",
                "simple_triggers/1/ops",
            ]
        );
        assert_eq!(unreadable(&atlas), 2);
    }

    #[test]
    fn blocks_of_the_real_atlas_are_all_readable() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let blocks = blocks(&atlas);
        let unreadable = unreadable(&atlas);
        eprintln!("triggers/simple_triggers blocks: {}, unreadable: {unreadable}", blocks.len());
        assert!(!blocks.is_empty(), "no blocks read from the real atlas");
        assert_eq!(unreadable, 0, "unreadable fields in the real atlas");
    }
}
