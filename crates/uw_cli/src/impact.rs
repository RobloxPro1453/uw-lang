//! Impact report: when a mod replaces a Native entry's code, which Native entries are no longer
//! reached from anywhere (they will never open now).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::model::{entry_hint, short, Model, Target};

/// Entries reachable before the cut but not after it, sorted: the engine's roots walked over the
/// outgoing edges between entries, once as it is and once ignoring the outgoing edges of every
/// entry in `cut`. A cut entry itself stays reachable when something reaches it.
#[allow(dead_code)]
pub fn unreachable_after(model: &Model, cut: &[Target]) -> Vec<Target> {
    let before = reachable(model, &[]);
    let after = reachable(model, cut);
    before.difference(&after).cloned().collect()
}

/// Plain-text report for `lost` (what [`unreachable_after`] returns): nothing when nothing is lost,
/// else a headline and one line per table, sorted by table name.
#[allow(dead_code)]
pub fn render(model: &Model, lost: &[Target]) -> String {
    if lost.is_empty() {
        return String::new();
    }
    let mut out =
        format!("impact: {} Native entries are no longer reached from anywhere:\n", lost.len());
    let mut by_table: BTreeMap<&str, Vec<&Target>> = BTreeMap::new();
    for target in lost {
        if let Target::Entry { table, .. } = target {
            by_table.entry(table.as_str()).or_default().push(target);
        }
    }
    for (table, targets) in by_table {
        let _ = write!(out, "  {table} ({}): ", targets.len());
        for (i, target) in targets.into_iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(&model.label(target));
            if let Target::Entry { table, index } = target {
                if let Some(hint) = entry_hint(table, &model.atlas.tables[table].entries[*index]) {
                    let _ = write!(out, " \"{}\"", short(&hint, 30));
                }
            }
        }
        out.push('\n');
    }
    out
}

/// Everything the engine's roots reach over entry-to-entry edges, not walking out of `cut` entries.
fn reachable(model: &Model, cut: &[Target]) -> BTreeSet<Target> {
    let mut seen = BTreeSet::new();
    let mut queue = model.engine_roots();
    while let Some(node) = queue.pop() {
        if !seen.insert(node.clone()) {
            continue;
        }
        if cut.contains(&node) {
            continue;
        }
        for (next, _op) in model.outgoing(&node) {
            if matches!(next, Target::Entry { .. }) && !seen.contains(next) {
                queue.push(next.clone());
            }
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> Model {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_impact.json");
        Model::new(uw_atlas::Atlas::from_path(path).expect("fixture"))
    }

    fn menu(index: usize) -> Target {
        Target::Entry { table: "menus".into(), index }
    }

    #[test]
    fn cutting_a_menu_hides_what_only_it_reached() {
        let model = model();
        // menus #0 start (a root) and the hook script reach menu a; a reaches b, b reaches c.
        assert_eq!(unreachable_after(&model, &[menu(1)]), vec![menu(2), menu(3)]);
    }

    #[test]
    fn cutting_a_later_menu_hides_only_what_follows_it() {
        let model = model();
        assert_eq!(unreachable_after(&model, &[menu(2)]), vec![menu(3)]);
    }

    #[test]
    fn nothing_cut_loses_nothing() {
        let model = model();
        assert!(unreachable_after(&model, &[]).is_empty());
        assert_eq!(render(&model, &[]), "");
    }

    #[test]
    fn render_lists_the_lost_entries_by_table() {
        let model = model();
        let lost = unreachable_after(&model, &[menu(1)]);
        assert_eq!(
            render(&model, &lost),
            "impact: 2 Native entries are no longer reached from anywhere:\n  menus (2): menu_b \"Hint b\", menu_c\n"
        );
    }
}
