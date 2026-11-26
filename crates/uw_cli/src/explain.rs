//! `uw explain <thing>`: what something in Native is, where it is defined and who uses it.
//!
//! Short by default (owner: long output tires); `--all` lists every use and the raw value. When
//! the name is not exact, the closest matches are listed instead (part of a name works).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::model::{entry_hint, entry_label, short, Detail, Model, Target, UseGroup, UseSite};

/// Uses shown per group in the short form.
const SHORT_SITES: usize = 5;
/// Groups shown in the short form.
const SHORT_GROUPS: usize = 4;
/// Matches listed when the name is not exact.
const MATCHES: usize = 12;

/// Explains `query`; a partial name lists what matches. `Err` only when nothing matches.
pub fn explain(model: &Model, query: &str, all: bool) -> Result<String, String> {
    match model.resolve(query) {
        Some(target) => Ok(render(model, &target, &model.describe(&target), all)),
        None => matches(model, query),
    }
}

/// [`explain`] as one JSON object (`uw explain --json`, for the editor hovers): the name, what it is,
/// its Native id when it has one (`p_town_6`) and `text`, the plain output above.
pub fn explain_json(model: &Model, query: &str, all: bool) -> Result<String, String> {
    let text = explain(model, query, all)?;
    let (name, kind, native_id) = match model.resolve(query) {
        Some(target) => {
            let detail = model.describe(&target);
            (detail.label, detail.kind, native_id(model, &target))
        }
        // Not an exact name: `text` lists what matches, so there is no kind and no id.
        None => (query.trim().to_owned(), String::new(), None),
    };
    Ok(serde_json::json!({
        "name": name,
        "kind": kind,
        "native_id": native_id,
        "text": text,
    })
    .to_string())
}

/// The kit id a target is written as (`p_town_6`); a global, an operation and a constant have none.
fn native_id(model: &Model, target: &Target) -> Option<String> {
    match target {
        Target::Entry { table, index } => model.atlas.tables[table].entries[*index].id.clone(),
        Target::Global(_) | Target::Op(_) | Target::Const(_) => None,
    }
}

/// Not an exact name: the things that match (Ok), or an error when nothing does.
fn matches(model: &Model, query: &str) -> Result<String, String> {
    let hits = model.search(query, MATCHES);
    if hits.is_empty() {
        let mut message = format!("'{query}' is not in Native, and no name contains it");
        let candidates: Vec<String> = model.atlas.all_ids().map(str::to_owned).collect();
        if let Some(candidate) = uw_diag::suggest(query, candidates.iter().map(String::as_str)) {
            let _ = write!(message, "; did you mean '{candidate}'?");
        }
        return Err(message);
    }
    let total = model.count(query);
    let mut out = format!("'{query}' is not an exact name. Things that match ({total}):\n");
    let width = hits.iter().map(|h| h.label.chars().count()).max().unwrap_or(0).min(44);
    for hit in &hits {
        let _ = write!(out, "  {:<width$}  {:<16}", hit.label, hit.kind);
        if let Some(hint) = &hit.hint {
            let _ = write!(out, "  \"{}\"", short(hint, 50));
        }
        out.push('\n');
    }
    if total > hits.len() {
        let _ = writeln!(out, "  … {} more", total - hits.len());
    }
    out.push_str("Run `uw explain <name>` with one of these, or `uw browse` to look around.
");
    Ok(out)
}

/// Plain-text form of a detail (also the CLI test surface): where it is used, then what it leads
/// to (the entries it mentions).
pub fn render(model: &Model, target: &Target, detail: &Detail, all: bool) -> String {
    let mut out = used_by(detail, all);
    out.push_str(&leads_to(model, target, all));
    out
}

/// The "used by" part: the entries that mention `detail`, grouped by table.
fn used_by(detail: &Detail, all: bool) -> String {
    let mut out = String::new();
    match &detail.hint {
        Some(hint) => {
            let _ = writeln!(out, "{}  ·  \"{}\"", detail.label, short(hint, 70));
        }
        None => {
            let _ = writeln!(out, "{}", detail.label);
        }
    }
    if let Some(tr) = &detail.tr {
        let _ = writeln!(out, "  tr:       \"{}\"", short(tr, 70));
    }
    let _ = writeln!(out, "  what:     {}", detail.kind);
    for note in &detail.notes {
        let _ = writeln!(out, "  note:     {note}");
    }
    if let Some(defined) = &detail.defined {
        let _ = writeln!(out, "  defined:  {defined}");
    }
    if all {
        if let Some(value) = &detail.value {
            let _ = writeln!(out, "  value:    {value}");
        }
    }
    let sites: usize = detail.uses.iter().map(|g| g.sites.len()).sum();
    if sites == 0 {
        out.push_str("  used by:  nothing in Native (no operation mentions it)\n");
        return out;
    }
    let _ = writeln!(
        out,
        "  used by:  {sites} {} ({} mentions)",
        if sites == 1 { "place" } else { "places" },
        detail.total_mentions
    );
    let groups = if all { detail.uses.len() } else { SHORT_GROUPS.min(detail.uses.len()) };
    out.push_str(&groups_text(&detail.uses, all));
    if !all && (groups < detail.uses.len() || detail.uses.iter().any(|g| g.sites.len() > SHORT_SITES)) {
        out.push_str("\n  (uw explain --all for everything, uw browse to click through)\n");
    }
    out
}

/// Groups of places: a table and its rows (the form under `used by` and `leads to`).
fn groups_text(groups: &[UseGroup], all: bool) -> String {
    let mut out = String::new();
    let shown_groups = if all { groups.len() } else { SHORT_GROUPS.min(groups.len()) };
    for group in groups.iter().take(shown_groups) {
        let _ = writeln!(out, "\n  {} ({})", group.table, group.sites.len());
        let shown = if all { group.sites.len() } else { SHORT_SITES.min(group.sites.len()) };
        let width = group.sites.iter().take(shown).map(|s| s.label.chars().count()).max().unwrap_or(0).min(44);
        for site in group.sites.iter().take(shown) {
            let _ = write!(out, "    {:<width$}  {:<30}  {}", site.label, site.place, site.ops_text());
            if let Some(hint) = &site.hint {
                let _ = write!(out, "  \"{}\"", short(hint, 40));
            }
            out.push('\n');
        }
        if shown < group.sites.len() {
            let _ = writeln!(out, "    … {} more", group.sites.len() - shown);
        }
    }
    if shown_groups < groups.len() {
        let rest: Vec<String> =
            groups[shown_groups..].iter().map(|g| format!("{} {}", g.sites.len(), g.table)).collect();
        let _ = writeln!(out, "\n  … and {}", rest.join(", "));
    }
    out
}

/// The "leads to" part: the entries `target` mentions, grouped by table (the other direction).
/// Nothing when it mentions no entry: globals and operations are not places.
fn leads_to(model: &Model, target: &Target, all: bool) -> String {
    // table -> index -> op -> count, in the order the entry mentions them.
    let mut by_table: BTreeMap<&str, BTreeMap<usize, BTreeMap<&str, usize>>> = BTreeMap::new();
    for (mentioned, op) in model.outgoing(target) {
        let Target::Entry { table, index } = mentioned else { continue };
        *by_table
            .entry(table.as_str())
            .or_default()
            .entry(*index)
            .or_default()
            .entry(op.as_deref().unwrap_or("data"))
            .or_default() += 1;
    }
    let mut groups: Vec<UseGroup> = by_table
        .into_iter()
        .map(|(table, sites)| UseGroup {
            table: table.to_owned(),
            sites: sites
                .into_iter()
                .map(|(index, ops)| {
                    let t = &model.atlas.tables[table];
                    let entry = &t.entries[index];
                    UseSite {
                        target: Target::Entry { table: table.to_owned(), index },
                        label: entry_label(&model.atlas, table, entry),
                        hint: entry_hint(table, entry),
                        place: match entry.line {
                            Some(line) => format!("{}.py:{line}", t.module),
                            None => format!("{}.py", t.module),
                        },
                        ops: ops.into_iter().map(|(op, n)| (op.to_owned(), n)).collect(),
                    }
                })
                .collect(),
        })
        .collect();
    if groups.is_empty() {
        return String::new();
    }
    groups.sort_by(|a, b| b.sites.len().cmp(&a.sites.len()).then(a.table.cmp(&b.table)));
    let sites: usize = groups.iter().map(|g| g.sites.len()).sum();
    let mut out = format!("\n  leads to: {sites} {}\n", if sites == 1 { "place" } else { "places" });
    out.push_str(&groups_text(&groups, all));
    out
}
