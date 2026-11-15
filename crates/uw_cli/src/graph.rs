//! What leads where: the menus / scripts / dialogs one Native thing leads to, as a tree.

use std::collections::HashSet;

use crate::model::{self, Model, Target};

/// One line of the graph tree: where it sits, what it is and how its parent reaches it.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TreeLine {
    pub depth: usize,
    pub target: Target,
    pub label: String,
    pub hint: Option<String>,
    /// Operation of the statement that mentions this line under its parent.
    pub op: Option<String>,
    /// True when the target was already shown higher up, so it is not expanded again.
    pub repeat: bool,
}

/// Depth-first tree of the entries `root` leads to, down to `max_depth` (0 = the root only).
#[allow(dead_code)]
pub fn tree(model: &Model, root: &Target, max_depth: usize) -> Vec<TreeLine> {
    let mut lines = Vec::new();
    let mut shown = HashSet::new();
    shown.insert(root.clone());
    lines.push(line(model, 0, root, None, false));
    if max_depth > 0 {
        expand(model, root, 1, max_depth, &mut shown, &mut lines);
    }
    lines
}

/// Draws the tree, one line each: two spaces per depth, then the label, the operation, the
/// in-game name/text and `↺` for a repeat.
#[allow(dead_code)]
pub fn render_text(lines: &[TreeLine]) -> String {
    let mut out = String::new();
    for line in lines {
        for _ in 0..line.depth {
            out.push_str("  ");
        }
        out.push_str(&line.label);
        if let Some(op) = &line.op {
            out.push_str(&format!("  ({op})"));
        }
        if let Some(hint) = &line.hint {
            out.push_str(&format!("  \"{}\"", model::short(hint, 50)));
        }
        if line.repeat {
            out.push_str("  ↺");
        }
        out.push('\n');
    }
    out
}

fn line(model: &Model, depth: usize, target: &Target, op: Option<&str>, repeat: bool) -> TreeLine {
    let hint = match target {
        Target::Entry { table, index } => {
            model::entry_hint(table, &model.atlas.tables[table].entries[*index])
        }
        Target::Global(_) | Target::Op(_) | Target::Const(_) => None,
    };
    TreeLine { depth, target: target.clone(), label: model.label(target), hint, op: op.map(str::to_owned), repeat }
}

/// Adds the entry children of `source` at `depth` and, under each new one, its own children.
fn expand(
    model: &Model,
    source: &Target,
    depth: usize,
    max_depth: usize,
    shown: &mut HashSet<Target>,
    lines: &mut Vec<TreeLine>,
) {
    // Each child once per parent, with the operation of its first mention.
    let mut seen_children = HashSet::new();
    for (child, op) in model.outgoing(source) {
        if !matches!(child, Target::Entry { .. }) || !seen_children.insert(child) {
            continue;
        }
        let repeat = !shown.insert(child.clone());
        lines.push(line(model, depth, child, op.as_deref(), repeat));
        if !repeat && depth < max_depth {
            expand(model, child, depth + 1, max_depth, shown, lines);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_model() -> Model {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_refs.json");
        Model::new(uw_atlas::Atlas::from_path(path).expect("fixture"))
    }

    fn script(index: usize) -> Target {
        Target::Entry { table: "scripts".into(), index }
    }

    #[test]
    fn game_start_leads_to_its_entries_with_ops() {
        let model = fixture_model();
        let lines = tree(&model, &script(0), 3);
        let got: Vec<(usize, &str, Option<&str>, bool)> =
            lines.iter().map(|l| (l.depth, l.label.as_str(), l.op.as_deref(), l.repeat)).collect();
        assert_eq!(
            got,
            vec![
                (0, "script_game_start", None, false),
                (1, "menu_camp#1", Some("jump_to_menu"), false),
                (1, "menu_start", Some("eq"), false),
                (1, "script_helper", Some("call_script"), false),
                // helper reaches menu_start again: marked, not expanded.
                (2, "menu_start", Some("assign"), true),
            ]
        );
        assert!(lines[1..].iter().all(|l| matches!(l.target, Target::Entry { .. })));
    }

    #[test]
    fn max_depth_zero_is_only_the_root() {
        let model = fixture_model();
        let lines = tree(&model, &script(0), 0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].depth, 0);
        assert_eq!(lines[0].label, "script_game_start");
        assert_eq!(lines[0].target, script(0));
        assert_eq!(lines[0].op, None);
        assert!(!lines[0].repeat);
    }

    const CALLS_ATLAS: &str = r#"{
      "schema": 1,
      "ops": [{"name":"call_script","code":1,"line":14,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}],
      "hooks": [],
      "ranges": [],
      "tables": {
        "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
          "entries": [
            {"index":0,"name":"a","id":"script_a","dup":null,"line":10,"value":["a",{"list":[[1,"script_b"]]}]},
            {"index":1,"name":"b","id":"script_b","dup":null,"line":20,"value":["b",{"list":[[1,"script_a"]]}]}
          ]}
      }
    }"#;

    #[test]
    fn scripts_calling_each_other_terminate_and_mark_the_repeat() {
        let model = Model::new(uw_atlas::Atlas::from_str(CALLS_ATLAS).expect("inline atlas"));
        let lines = tree(&model, &script(0), 10);
        let got: Vec<(usize, &str, Option<&str>, bool)> =
            lines.iter().map(|l| (l.depth, l.label.as_str(), l.op.as_deref(), l.repeat)).collect();
        assert_eq!(
            got,
            vec![
                (0, "script_a", None, false),
                (1, "script_b", Some("call_script"), false),
                (2, "script_a", Some("call_script"), true),
            ]
        );
    }

    #[test]
    fn render_text_draws_depth_ops_hints_and_repeat() {
        let lines = vec![
            TreeLine {
                depth: 0,
                target: Target::Op("call_script".into()),
                label: "call_script".into(),
                hint: None,
                op: None,
                repeat: false,
            },
            TreeLine {
                depth: 1,
                target: Target::Entry { table: "menus".into(), index: 0 },
                label: "menu_start".into(),
                hint: Some("Hi".into()),
                op: Some("jump_to_menu".into()),
                repeat: false,
            },
            TreeLine {
                depth: 2,
                target: Target::Entry { table: "menus".into(), index: 1 },
                label: "menu_camp#1".into(),
                hint: Some("x".repeat(60)),
                op: None,
                repeat: true,
            },
        ];
        assert_eq!(
            render_text(&lines),
            format!(
                "call_script\n  menu_start  (jump_to_menu)  \"Hi\"\n    menu_camp#1  \"{}…\"  ↺\n",
                "x".repeat(49)
            )
        );
    }
}
