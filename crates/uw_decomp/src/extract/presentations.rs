//! Operation blocks of the `presentations` table(s). Filled in by its own task (docs/tasks/T-02x).

use serde_json::Value as Json;
use uw_atlas::Atlas;

use crate::{items, parse_ops, Block};

/// The table this module reads: a presentation is `[id, flags, mesh, triggers]` and every trigger
/// is `[event, ops]` (`module_presentations.py` header), so a block sits at path `[3, <trigger>, 1]`.
const TABLE: &str = "presentations";

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    walk(atlas).0
}

/// Trigger bodies that should be a block but [`parse_ops`] rejects (a float operand, a nested
/// list, a body that is not a list at all).
pub fn unreadable(atlas: &Atlas) -> usize {
    walk(atlas).1
}

/// Reads the whole table once, returning its blocks and how many bodies could not be read.
fn walk(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut blocks = Vec::new();
    let mut unreadable = 0;
    let Some(table) = atlas.tables.get(TABLE) else { return (blocks, unreadable) };
    for entry in &table.entries {
        let Some(fields) = items(&entry.value) else { continue };
        let Some(triggers) = fields.get(3).and_then(items) else { continue };
        let name = entry.name.clone().unwrap_or_else(|| entry.index.to_string());
        for (pos, trigger) in triggers.iter().enumerate() {
            let Some(trigger) = items(trigger) else { continue };
            let Some(event) = trigger.first().and_then(Json::as_i64) else { continue };
            let Some(body) = trigger.get(1) else { continue };
            match parse_ops(body) {
                Some(ops) => blocks.push(Block {
                    table: TABLE.to_string(),
                    index: entry.index,
                    path: vec![3, pos, 1],
                    label: format!("{TABLE}/{name}/{event}"),
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
    use crate::{RawArg, RawOp};

    /// Two presentations: three trigger bodies, all readable.
    const INLINE: &str = r#"{
      "schema": 1, "kit_hash": "inline",
      "ops": [],
      "hooks": [],
      "ranges": [],
      "tables": {
        "presentations": {"module":"module_presentations","var":"presentations","id_file":"ID_presentations","id_prefix":"prsnt_","ref_tag":"prsnt",
          "entries": [
            {"index":3,"name":"banner_selection","id":"prsnt_banner_selection","dup":null,"line":8631,
              "value":["banner_selection",0,"mesh_load_window",{"list":[
                [-60,{"list":[[1974,1000],[1955,53],[1972,"$g"]]}],
                [-61,{"list":[[4]]}]
              ]}]},
            {"index":7,"name":"arena_training","id":"prsnt_arena_training","dup":null,"line":11627,
              "value":["arena_training",0,"mesh_arena_window",{"list":[
                [-64,{"list":[[31,"$k",3],39]}]
              ]}]}
          ]}
      }
    }"#;

    fn atlas(text: &str) -> Atlas {
        Atlas::from_str(text).expect("inline atlas loads")
    }

    #[test]
    fn blocks_reads_every_presentation_trigger() {
        let atlas = atlas(INLINE);
        let found = blocks(&atlas);
        let got: Vec<(usize, Vec<usize>, String, usize)> =
            found.iter().map(|b| (b.index, b.path.clone(), b.label.clone(), b.ops.len())).collect();
        assert_eq!(
            got,
            vec![
                (3, vec![3, 0, 1], "presentations/banner_selection/-60".to_string(), 3),
                (3, vec![3, 1, 1], "presentations/banner_selection/-61".to_string(), 1),
                (7, vec![3, 0, 1], "presentations/arena_training/-64".to_string(), 2),
            ]
        );
        assert!(found.iter().all(|b| b.table == "presentations"));
        assert_eq!(found[0].ops[0], RawOp { code: 1974, args: vec![RawArg::Int(1000)] });
        assert_eq!(found[0].ops[2], RawOp { code: 1972, args: vec![RawArg::Str("$g".into())] });
        assert_eq!(found[2].ops[1], RawOp { code: 39, args: vec![] });
        assert_eq!(unreadable(&atlas), 0);
    }

    #[test]
    fn unreadable_counts_broken_bodies_and_skips_them() {
        let text = r#"{
          "schema": 1, "kit_hash": "inline",
          "ops": [],
          "hooks": [],
          "ranges": [],
          "tables": {
            "presentations": {"module":"module_presentations","var":"presentations","id_file":"ID_presentations","id_prefix":"prsnt_","ref_tag":"prsnt",
              "entries": [
                {"index":0,"name":"good","id":"prsnt_good","dup":null,"line":1,
                  "value":["good",0,"mesh_x",{"list":[[-60,{"list":[[4]]}]]}]},
                {"index":1,"name":"bad","id":"prsnt_bad","dup":null,"line":2,
                  "value":["bad",0,"mesh_y",{"list":[[-61,{"list":[[31,1.5]]}],[-62,{"list":[[4]]}]]}]}
              ]}
          }
        }"#;
        let atlas = atlas(text);
        let found = blocks(&atlas);
        let labels: Vec<&str> = found.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(labels, vec!["presentations/good/-60", "presentations/bad/-62"]);
        assert_eq!(unreadable(&atlas), 1);
    }

    #[test]
    fn real_atlas_has_only_readable_presentation_triggers() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let found = blocks(&atlas);
        let bad = unreadable(&atlas);
        println!("presentations: {} blocks, {} unreadable", found.len(), bad);
        assert!(!found.is_empty(), "presentations has no operation blocks");
        assert_eq!(bad, 0, "{bad} presentation trigger bodies are unreadable");
    }
}
