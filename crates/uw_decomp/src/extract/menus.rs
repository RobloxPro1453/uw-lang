//! Operation blocks of the `menus` table(s). Filled in by its own task (docs/tasks/T-02x).

use crate::{items, parse_ops, Block, RawOp};
use serde_json::Value as Json;
use uw_atlas::Atlas;

/// The atlas table this module reads.
const TABLE: &str = "menus";
/// Menu layout (module_game_menus.py:20): id, flags, text, mesh, opening code, options.
const MENU_ON_ENTER: usize = 4;
const MENU_OPTIONS: usize = 5;
/// Option layout (module_game_menus.py:28): id, condition block, text, consequence block.
const OPTION_CONDITION: usize = 1;
const OPTION_CONSEQUENCE: usize = 3;

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    scan(atlas).0
}

/// How many fields of this table held something block-like that [`parse_ops`] could not read.
/// A missing or `null` field is not a block and is not counted.
pub fn unreadable(atlas: &Atlas) -> usize {
    scan(atlas).1
}

/// The blocks and the unreadable count, in one pass over the table's entries.
fn scan(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut out = Vec::new();
    let mut bad = 0;
    let Some(table) = atlas.tables.get(TABLE) else {
        return (out, bad);
    };
    for entry in &table.entries {
        let Some(fields) = entry.value.as_array() else {
            continue;
        };
        let who = entry.name.clone().unwrap_or_else(|| entry.index.to_string());
        if let Some(ops) = field(fields, MENU_ON_ENTER, &mut bad) {
            out.push(Block {
                table: TABLE.to_owned(),
                index: entry.index,
                path: vec![MENU_ON_ENTER],
                label: format!("{TABLE}/{who}/enter"),
                ops,
            });
        }
        let Some(options) = fields.get(MENU_OPTIONS).and_then(items) else {
            continue;
        };
        for (i, option) in options.iter().enumerate() {
            let Some(parts) = option.as_array() else {
                continue;
            };
            let id = match parts.first() {
                Some(Json::String(s)) => s.clone(),
                Some(other) => other.to_string(),
                None => continue,
            };
            let kinds = [(OPTION_CONDITION, "condition"), (OPTION_CONSEQUENCE, "consequence")];
            for (at, part) in kinds {
                let Some(ops) = field(parts, at, &mut bad) else {
                    continue;
                };
                out.push(Block {
                    table: TABLE.to_owned(),
                    index: entry.index,
                    path: vec![MENU_OPTIONS, i, at],
                    label: format!("{TABLE}/{who}/option {id}/{part}"),
                    ops,
                });
            }
        }
    }
    (out, bad)
}

/// Reads `parts[at]` as an operation block. A missing or `null` field is not a block; anything
/// else [`parse_ops`] rejects is counted in `bad`.
fn field(parts: &[Json], at: usize, bad: &mut usize) -> Option<Vec<RawOp>> {
    let value = parts.get(at).filter(|v| !v.is_null())?;
    match parse_ops(value) {
        Some(ops) => Some(ops),
        None => {
            *bad += 1;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two menus: named with an enter block plus two options, unnamed (label falls back to the
    /// index) with a plain-array enter block and one option.
    const MINI: &str = r#"{
      "schema": 1, "kit_hash": "menus",
      "ops": [
        {"name":"change_screen_map","code":2052,"line":1,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
        {"name":"jump_to_menu","code":2060,"line":2,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}
      ],
      "hooks": [], "ranges": [], "constants": [],
      "tables": {
        "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu",
          "entries": [
            {"index":0,"name":"start","id":"menu_start","dup":null,"line":10,
             "value":["start",0,"Start","none",{"list":[[2052],[1106,"@hi"]]},{"list":[
               ["go",{"list":[[31,1,0]]},"Go",{"list":[[2060,"mnu_town"]]}],
               ["back",{"list":[]},"Back",{"list":[]}]
             ]}]},
            {"index":3,"name":null,"id":null,"dup":null,"line":null,
             "value":["last",0,"","none",[],{"list":[
               ["leave",{"list":[[4]]},"Leave",{"list":[[2052]]}]
             ]}]}
          ]}
      }
    }"#;

    /// One menu whose enter block cannot be read, plus a `null` field, short options and a
    /// non-array option.
    const BROKEN: &str = r#"{
      "schema": 1, "kit_hash": "menus",
      "ops": [], "hooks": [], "ranges": [], "constants": [],
      "tables": {
        "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu",
          "entries": [
            {"index":0,"name":"broken","id":"menu_broken","dup":null,"line":10,
             "value":["broken",0,"","none",{"list":[[31,1.5]]},{"list":[
               ["a",{"list":[[31,1]]},"A",{"list":[[4]]}],
               ["short"],
               "not an option"
             ]}]},
            {"index":1,"name":"empty","id":"menu_empty","dup":null,"line":20,
             "value":[null,null,null,null,null,null]}
          ]}
      }
    }"#;

    fn ops_of(block: &Block) -> Vec<u64> {
        block.ops.iter().map(|op| op.code).collect()
    }

    #[test]
    fn blocks_paths_labels_and_op_counts() {
        let atlas = Atlas::from_str(MINI).unwrap();
        let found = blocks(&atlas);
        let seen: Vec<(&str, Vec<usize>, Vec<u64>)> =
            found.iter().map(|b| (b.label.as_str(), b.path.clone(), ops_of(b))).collect();
        assert_eq!(
            seen,
            vec![
                ("menus/start/enter", vec![4], vec![2052, 1106]),
                ("menus/start/option go/condition", vec![5, 0, 1], vec![31]),
                ("menus/start/option go/consequence", vec![5, 0, 3], vec![2060]),
                ("menus/start/option back/condition", vec![5, 1, 1], vec![]),
                ("menus/start/option back/consequence", vec![5, 1, 3], vec![]),
                ("menus/3/enter", vec![4], vec![]),
                ("menus/3/option leave/condition", vec![5, 0, 1], vec![4]),
                ("menus/3/option leave/consequence", vec![5, 0, 3], vec![2052]),
            ]
        );
        // The table and the entry index travel with the block; operands are kept as they are.
        assert_eq!(found[0].table, "menus");
        assert_eq!(found[0].index, 0);
        assert_eq!(found[5].index, 3);
        assert_eq!(found[0].ops[1].args, vec![crate::RawArg::Str("@hi".into())]);
        assert_eq!(found[2].ops[0].args, vec![crate::RawArg::Str("mnu_town".into())]);
        assert_eq!(unreadable(&atlas), 0);
    }

    #[test]
    fn unreadable_counts_only_broken_block_fields() {
        let atlas = Atlas::from_str(BROKEN).unwrap();
        let found = blocks(&atlas);
        let labels: Vec<&str> = found.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(labels, vec!["menus/broken/option a/condition", "menus/broken/option a/consequence"]);
        // The float inside the enter block is the only field that should be a block but is not.
        assert_eq!(unreadable(&atlas), 1);
    }

    #[test]
    fn real_atlas_menus_are_all_readable() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let found = blocks(&atlas);
        let bad = unreadable(&atlas);
        eprintln!("menus: {} blocks, {bad} unreadable", found.len());
        assert!(!found.is_empty(), "menus must hold operation blocks");
        assert_eq!(bad, 0, "every menu block field must be readable");
    }
}
