//! Operation blocks of the `mission_templates` table(s).

use crate::{items, parse_ops, Block};
use uw_atlas::Atlas;

/// The atlas table this module reads.
const TABLE: &str = "mission_templates";

/// Field of a mission template as the kit writes it
/// (`(name, flags, type, description, [spawn records], [triggers])`) that holds the trigger list.
const TRIGGERS: usize = 5;

/// Fields of a trigger (`(interval, delay, rearm, [conditions], [consequences])`) that hold
/// operation blocks, with the label part naming them.
const OPERATION_FIELDS: [(usize, &str); 2] = [(3, "condition"), (4, "consequence")];

/// Every operation block of this table, in table order.
pub fn blocks(atlas: &Atlas) -> Vec<Block> {
    read(atlas).0
}

/// How many fields of this table should hold an operation block but did not read as one.
pub fn unreadable(atlas: &Atlas) -> usize {
    read(atlas).1
}

/// The blocks and their unreadable count in one walk.
fn read(atlas: &Atlas) -> (Vec<Block>, usize) {
    let mut blocks = Vec::new();
    let mut unreadable = 0;
    let Some(table) = atlas.tables.get(TABLE) else {
        return (blocks, unreadable);
    };
    for entry in &table.entries {
        let name = entry
            .name
            .clone()
            .unwrap_or_else(|| entry.index.to_string());
        let Some(triggers) = items(&entry.value).and_then(|template| template.get(TRIGGERS)).and_then(items)
        else {
            continue;
        };
        for (t, trigger) in triggers.iter().enumerate() {
            // A trigger may be a name reference to a Python variable (`common_battle_init_banner`);
            // that holds no block of its own.
            let Some(parts) = items(trigger) else {
                continue;
            };
            for (field, part) in OPERATION_FIELDS {
                let Some(ops) = parts.get(field).and_then(parse_ops) else {
                    unreadable += 1;
                    continue;
                };
                blocks.push(Block {
                    table: TABLE.to_string(),
                    index: entry.index,
                    path: vec![TRIGGERS, t, field],
                    label: format!("{TABLE}/{name}/trigger {t}/{part}"),
                    ops,
                });
            }
        }
    }
    (blocks, unreadable)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RawArg;

    /// Two mission templates: a name-referenced trigger and a float operand in a condition.
    const INLINE: &str = r#"{
      "schema": 1, "kit_hash": "inline",
      "ops": [{"name":"eq","code":31,"line":1,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}],
      "hooks": [], "ranges": [], "constants": [],
      "tables": {
        "mission_templates": {"module":"module_mission_templates","var":"mission_templates",
          "id_file":"ID_mission_templates","id_prefix":"mst_","ref_tag":"mt",
          "entries": [
            {"index":0,"name":"alley_fight","id":"mst_alley_fight","dup":null,"line":15097,
             "value":["alley_fight",4,2,"Alley fight",{"list":[[0,5,null,4,7,{"list":[]}]]},{"list":[
               [2,0,0,{"list":[[4],[31,"$x",3]]},{"list":[[1106,"str_hello"]]}],
               "common_battle_init_banner",
               [-19,0,0,{"list":[]},{"list":[[2052]]}]
             ]}]},
            {"index":3,"name":"town_default","id":"mst_town_default","dup":null,"line":1277,
             "value":["town_default",4,2,"Town default",{"list":[]},{"list":[
               [10,0,0,{"list":[[132,"p_main_party"]]},{"list":[[1623,"p_main_party","p_town_6",120],2052]}],
               [5,0,0,{"list":[[31,1.5]]},{"list":[]}]
             ]}]}
          ]}
      }
    }"#;

    fn inline_atlas() -> Atlas {
        Atlas::from_str(INLINE).expect("inline atlas parses")
    }

    #[test]
    fn blocks_have_exact_paths_labels_and_ops() {
        let atlas = inline_atlas();
        let blocks = blocks(&atlas);
        let seen: Vec<(usize, Vec<usize>, &str, usize)> = blocks
            .iter()
            .map(|b| (b.index, b.path.clone(), b.label.as_str(), b.ops.len()))
            .collect();
        let expected: Vec<(usize, Vec<usize>, &str, usize)> = vec![
            (0, vec![5, 0, 3], "mission_templates/alley_fight/trigger 0/condition", 2),
            (0, vec![5, 0, 4], "mission_templates/alley_fight/trigger 0/consequence", 1),
            (0, vec![5, 2, 3], "mission_templates/alley_fight/trigger 2/condition", 0),
            (0, vec![5, 2, 4], "mission_templates/alley_fight/trigger 2/consequence", 1),
            (3, vec![5, 0, 3], "mission_templates/town_default/trigger 0/condition", 1),
            (3, vec![5, 0, 4], "mission_templates/town_default/trigger 0/consequence", 2),
            (3, vec![5, 1, 4], "mission_templates/town_default/trigger 1/consequence", 0),
        ];
        let mut seen = seen;
        seen.sort();
        assert_eq!(seen, expected);
        assert!(blocks.iter().all(|b| b.table == "mission_templates"));
        // The first condition reads both statements: a bare code and a statement with operands.
        assert_eq!(blocks[0].ops[0].code, 4);
        assert_eq!(blocks[0].ops[1].code, 31);
        assert_eq!(
            blocks[0].ops[1].args,
            vec![RawArg::Str("$x".into()), RawArg::Int(3)]
        );
    }

    #[test]
    fn unparseable_field_is_counted_and_skipped() {
        let atlas = inline_atlas();
        assert_eq!(unreadable(&atlas), 1);
        assert!(!blocks(&atlas).iter().any(|b| b.index == 3 && b.path == vec![5, 1, 3]));
    }

    #[test]
    fn missing_table_holds_no_blocks() {
        let atlas = Atlas::from_str(
            r#"{"schema":1,"ops":[],"hooks":[],"ranges":[],"tables":{}}"#,
        )
        .unwrap();
        assert!(blocks(&atlas).is_empty());
        assert_eq!(unreadable(&atlas), 0);
    }

    #[test]
    fn real_atlas_mission_templates_are_all_readable() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let blocks = blocks(&atlas);
        let unreadable = unreadable(&atlas);
        println!("mission_templates: {} blocks, {unreadable} unreadable", blocks.len());
        assert!(!blocks.is_empty(), "no mission template blocks");
        assert_eq!(unreadable, 0);
    }
}
