//! Operation blocks of the `other` tables: items, scene props and tableau materials.

use crate::{items, parse_ops, Block};
use uw_atlas::{Atlas, Entry};

/// Tables whose entry tuple holds a trigger list, and that field's index. `module_items.py`:
/// 9) optional triggers (index 8; 10) merchandise factions is not an operation block).
/// `module_scene_props.py`: 5) triggers (index 4).
const TRIGGER_TABLES: [(&str, usize); 2] = [("items", 8), ("scene_props", 4)];

/// `module_tableau_materials.py`: 10) operations block (index 9).
const TABLEAU_OPS: usize = 9;

/// Every operation block of these tables, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    scan(atlas).0
}

/// How many fields that should hold an operation block `parse_ops` could not read.
pub fn unreadable(atlas: &Atlas) -> usize {
    scan(atlas).1
}

fn scan(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut out = Vec::new();
    let mut bad = 0;
    for (table, field) in TRIGGER_TABLES {
        let Some(t) = atlas.tables.get(table) else { continue };
        for entry in &t.entries {
            let Some(value) = items(&entry.value) else { continue };
            let Some(triggers) = value.get(field).and_then(items) else { continue };
            for (pos, trigger) in triggers.iter().enumerate() {
                let parts = items(trigger);
                match parts.and_then(|p| p.get(1)).map(parse_ops) {
                    Some(Some(ops)) => {
                        let event = parts.and_then(|p| p.first()).and_then(|e| e.as_i64());
                        let part = match event {
                            Some(event) => event.to_string(),
                            None => format!("trigger {pos}"),
                        };
                        out.push(Block {
                            table: table.to_string(),
                            index: entry.index,
                            path: vec![field, pos, 1],
                            label: format!("{table}/{}/{part}", who(entry)),
                            ops,
                        });
                    }
                    _ => bad += 1,
                }
            }
        }
    }
    if let Some(t) = atlas.tables.get("tableau_materials") {
        for entry in &t.entries {
            let Some(value) = items(&entry.value) else { continue };
            let Some(ops_json) = value.get(TABLEAU_OPS) else { continue };
            match parse_ops(ops_json) {
                Some(ops) => out.push(Block {
                    table: "tableau_materials".to_string(),
                    index: entry.index,
                    path: vec![TABLEAU_OPS],
                    label: format!("tableau_materials/{}/operations", who(entry)),
                    ops,
                }),
                None => bad += 1,
            }
        }
    }
    (out, bad)
}

fn who(entry: &Entry) -> String {
    entry.name.clone().unwrap_or_else(|| entry.index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RawArg, RawOp};

    const INLINE: &str = r#"{
      "schema": 1, "kit_hash": "inline",
      "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "items": {"module": "module_items", "var": "items", "id_file": "ID_items", "id_prefix": "itm_",
          "entries": [
            {"index": 0, "name": "plain", "id": "itm_plain", "dup": null, "line": 1,
             "value": ["plain", "Plain", {"list": [["mesh", 0]]}, 0, 0, 1, 0, 0]},
            {"index": 1, "name": "triggered", "id": "itm_triggered", "dup": null, "line": 2,
             "value": ["triggered", "Triggered", {"list": [["mesh", 0]]}, 0, 0, 1, 0, 0,
                       {"list": [[3, {"list": [4]}], [5, {"list": [[6, "$x"], [7, 2]]}]]},
                       {"list": ["fac_a"]}]}
          ]},
        "scene_props": {"module": "module_scene_props", "var": "scene_props", "id_file": "ID_scene_props",
          "id_prefix": "spr_",
          "entries": [
            {"index": 0, "name": "door", "id": "spr_door", "dup": null, "line": 1,
             "value": ["door", 0, "door_mesh", "bo_door", {"list": []}]},
            {"index": 1, "name": "gate", "id": "spr_gate", "dup": null, "line": 2,
             "value": ["gate", 1, "gate_mesh", "bo_gate",
                       {"list": [[2, {"list": [[10, "p_town_1"]]}]]}]}
          ]},
        "tableau_materials": {"module": "module_tableau_materials", "var": "tableaus",
          "id_file": "ID_tableau_materials", "id_prefix": "tableau_",
          "entries": [
            {"index": 0, "name": "profile", "id": "tab_profile", "dup": null, "line": 1,
             "value": ["profile", 0, "tableau_with_transparency", 1024, 1024, 0, 0, 266, 532,
                       {"list": [[20, ":troop_no", 1], 21]}]},
            {"index": 1, "name": null, "id": "tab_inventory", "dup": null, "line": 2,
             "value": ["inventory", 0, "tableau_with_transparency", 1024, 1024, 0, 0, 180, 270,
                       {"list": [22]}]}
          ]}
      }
    }"#;

    #[test]
    fn blocks_reads_trigger_and_tableau_fields() {
        let atlas = Atlas::from_str(INLINE).unwrap();
        let found = blocks(&atlas);
        let shape: Vec<(String, usize, Vec<usize>, String, usize)> = found
            .iter()
            .map(|b| (b.table.clone(), b.index, b.path.clone(), b.label.clone(), b.ops.len()))
            .collect();
        assert_eq!(
            shape,
            vec![
                ("items".into(), 1, vec![8, 0, 1], "items/triggered/3".into(), 1),
                ("items".into(), 1, vec![8, 1, 1], "items/triggered/5".into(), 2),
                ("scene_props".into(), 1, vec![4, 0, 1], "scene_props/gate/2".into(), 1),
                ("tableau_materials".into(), 0, vec![9], "tableau_materials/profile/operations".into(), 2),
                ("tableau_materials".into(), 1, vec![9], "tableau_materials/1/operations".into(), 1),
            ]
        );
        assert_eq!(found[0].ops, vec![RawOp { code: 4, args: vec![] }]);
        assert_eq!(
            found[1].ops,
            vec![
                RawOp { code: 6, args: vec![RawArg::Str("$x".into())] },
                RawOp { code: 7, args: vec![RawArg::Int(2)] },
            ]
        );
        assert_eq!(found[2].ops, vec![RawOp { code: 10, args: vec![RawArg::Str("p_town_1".into())] }]);
        assert_eq!(
            found[3].ops,
            vec![
                RawOp { code: 20, args: vec![RawArg::Str(":troop_no".into()), RawArg::Int(1)] },
                RawOp { code: 21, args: vec![] },
            ]
        );
        assert_eq!(unreadable(&atlas), 0);
    }

    const INLINE_BAD: &str = r#"{
      "schema": 1, "kit_hash": "inline-bad",
      "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "items": {"module": "module_items", "var": "items",
          "entries": [
            {"index": 0, "name": "bad", "id": "itm_bad", "dup": null, "line": 1,
             "value": ["bad", "Bad", {"list": []}, 0, 0, 1, 0, 0,
                       {"list": [[1, {"list": [[2, 1.5]]}]]}]}
          ]},
        "scene_props": {"module": "module_scene_props", "var": "scene_props",
          "entries": [
            {"index": 0, "name": "bad", "id": "spr_bad", "dup": null, "line": 1,
             "value": ["bad", 0, "mesh", "body", {"list": [[2, "not ops"]]}]}
          ]},
        "tableau_materials": {"module": "module_tableau_materials", "var": "tableaus",
          "entries": [
            {"index": 0, "name": "bad", "id": "tab_bad", "dup": null, "line": 1,
             "value": ["bad", 0, "material", 1, 1, 0, 0, 1, 1,
                       {"list": [[3, {"nested": true}]]}]}
          ]}
      }
    }"#;

    #[test]
    fn unreadable_fields_are_counted_and_emitted_nowhere() {
        let atlas = Atlas::from_str(INLINE_BAD).unwrap();
        assert!(blocks(&atlas).is_empty());
        assert_eq!(unreadable(&atlas), 3);
    }

    #[test]
    fn real_atlas_has_no_unreadable_blocks() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).is_file() {
            eprintln!("skipped: {path} missing");
            return;
        }
        let atlas = Atlas::from_path(path).unwrap();
        let found = blocks(&atlas);
        let bad = unreadable(&atlas);
        eprintln!("other blocks: {}, unreadable: {bad}", found.len());
        assert!(!found.is_empty());
        assert_eq!(bad, 0);
    }
}
