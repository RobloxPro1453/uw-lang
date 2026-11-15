//! `uw explain <thing>`: what something in Native is, where it is defined and who uses it.
//!
//! Short by default (owner: long output tires); `--all` lists every use and the raw value. When
//! the name is not exact, the closest matches are listed instead (part of a name works).

use std::fmt::Write as _;

use crate::model::{short, Detail, Model};

/// Uses shown per group in the short form.
const SHORT_SITES: usize = 5;
/// Groups shown in the short form.
const SHORT_GROUPS: usize = 4;
/// Matches listed when the name is not exact.
const MATCHES: usize = 12;

/// Explains `query`; a partial name lists what matches. `Err` only when nothing matches.
pub fn explain(model: &Model, query: &str, all: bool) -> Result<String, String> {
    match model.resolve(query) {
        Some(target) => Ok(render(&model.describe(&target), all)),
        None => matches(model, query),
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

/// Plain-text form of a detail (also the CLI test surface).
pub fn render(detail: &Detail, all: bool) -> String {
    let mut out = String::new();
    match &detail.hint {
        Some(hint) => {
            let _ = writeln!(out, "{}  ·  \"{}\"", detail.label, short(hint, 70));
        }
        None => {
            let _ = writeln!(out, "{}", detail.label);
        }
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
    for group in detail.uses.iter().take(groups) {
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
    if groups < detail.uses.len() {
        let rest: Vec<String> = detail.uses[groups..]
            .iter()
            .map(|g| format!("{} {}", g.sites.len(), g.table))
            .collect();
        let _ = writeln!(out, "\n  … and {}", rest.join(", "));
    }
    if !all && (groups < detail.uses.len() || detail.uses.iter().any(|g| g.sites.len() > SHORT_SITES)) {
        out.push_str("\n  (uw explain --all for everything, uw browse to click through)\n");
    }
    out
}
