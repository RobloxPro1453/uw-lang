//! Operation blocks of the `scripts` table. Layout: `module_scripts.py:22`, an entry is
//! `(name, body)`, so the one operation block of an entry sits at field 1.

use crate::{items, parse_ops, Block};
use uw_atlas::{Atlas, Entry};

/// Table name in the atlas.
const TABLE: &str = "scripts";
/// Field of a scripts entry that holds its operation block (`module_scripts.py:22`, 2nd field).
const BODY_FIELD: usize = 1;

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    let Some(table) = atlas.tables.get(TABLE) else {
        return Vec::new();
    };
    table.entries.iter().filter_map(block_of).collect()
}

/// Entries of this table whose operation block could not be read (they emit no [`Block`]).
pub fn unreadable(atlas: &Atlas) -> usize {
    let Some(table) = atlas.tables.get(TABLE) else {
        return 0;
    };
    table.entries.iter().filter(|entry| block_of(entry).is_none()).count()
}

/// Reads the single block of an entry: `value` is `[name, body]`.
fn block_of(entry: &Entry) -> Option<Block> {
    let body = items(&entry.value)?.get(BODY_FIELD)?;
    let ops = parse_ops(body)?;
    let name = entry.name.clone().unwrap_or_else(|| entry.index.to_string());
    Some(Block {
        table: TABLE.to_string(),
        index: entry.index,
        path: vec![BODY_FIELD],
        label: format!("{TABLE}/{name}/body"),
        ops,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RawArg, RawOp};

    /// A tiny atlas: one script with statements, one with an empty body, one whose body is not
    /// an operation block (a float operand), and no other tables.
    fn atlas() -> Atlas {
        let text = r#"{
          "schema": 1, "kit_hash": "scripts",
          "ops": [],
          "hooks": [],
          "ranges": [],
          "tables": {
            "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
              "entries": [
                {"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,"value":["game_start",{"list":[
                  [2060,"mnu_camp"],
                  [2133,"$g_x",1],
                  3
                ]}]},
                {"index":7,"name":null,"id":null,"dup":null,"line":50,"value":["helper",{"list":[]}]},
                {"index":9,"name":"broken","id":"script_broken","dup":null,"line":60,"value":["broken",{"list":[[31,1.5]]}]}
              ]}
          }
        }"#;
        Atlas::from_str(text).expect("inline atlas loads")
    }

    #[test]
    fn blocks_read_the_body_field() {
        let atlas = atlas();
        let blocks = blocks(&atlas);
        assert_eq!(blocks.len(), 2, "a body that is not an operation block emits no block");

        let start = &blocks[0];
        assert_eq!(start.table, "scripts");
        assert_eq!(start.index, 0);
        assert_eq!(start.path, vec![1]);
        assert_eq!(start.label, "scripts/game_start/body");
        assert_eq!(start.ops.len(), 3);
        assert_eq!(start.ops[0], RawOp { code: 2060, args: vec![RawArg::Str("mnu_camp".into())] });
        assert_eq!(
            start.ops[1],
            RawOp { code: 2133, args: vec![RawArg::Str("$g_x".into()), RawArg::Int(1)] }
        );
        assert_eq!(start.ops[2], RawOp { code: 3, args: vec![] });

        let helper = &blocks[1];
        assert_eq!(helper.index, 7);
        assert_eq!(helper.path, vec![1]);
        assert_eq!(helper.label, "scripts/7/body", "an unnamed entry falls back to its index");
        assert!(helper.ops.is_empty(), "an empty body is a readable block with no ops");

        assert_eq!(unreadable(&atlas), 1, "only the broken body is counted");
    }

    #[test]
    fn full_atlas_scripts_are_all_readable() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let found = blocks(&atlas);
        let bad = unreadable(&atlas);
        println!("scripts: {} blocks, {} unreadable", found.len(), bad);
        assert!(!found.is_empty(), "the real atlas has script blocks");
        assert_eq!(bad, 0, "every script body must parse as an operation block");
    }
}
