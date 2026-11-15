//! `uw explain <thing>`: where something is defined in Native and where operations use it.
//!
//! Short by default (owner's wish: long output tires); `--all` lists every use. Output is
//! English like every other message of the tool.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use uw_atlas::{Atlas, Entry, Reference};

/// How many using entries the short form shows.
const SHORT_LIMIT: usize = 10;
/// How many characters of an entry's value the short form shows.
const VALUE_PREVIEW: usize = 90;

/// Explains `query` against `atlas`. `Err` carries the message for an unknown thing.
pub fn explain(atlas: &Atlas, query: &str, all: bool) -> Result<String, String> {
    let query = query.trim();
    if let Some(global) = query.strip_prefix('$') {
        if !global.is_empty() {
            let refs = atlas.references(|text| text == query);
            if !refs.is_empty() {
                let mut out = format!("{query}  (global variable)\n");
                write_uses(&mut out, atlas, &refs, all);
                return Ok(out);
            }
        }
    }
    if let Some((table, entry)) = atlas.entry_by_id(query).or_else(|| atlas.resolve_ref(query)) {
        return Ok(explain_entry(atlas, table, entry, all));
    }
    if atlas.is_hook(query) {
        if let Some((table, entry)) = atlas.entry_by_id(&format!("script_{query}")) {
            return Ok(explain_entry(atlas, table, entry, all));
        }
    }
    if let Some(op) = atlas.op(query) {
        let mut out = format!("{}  (operation, code {})\n", op.name, op.code);
        if let Some(line) = op.line {
            let _ = writeln!(out, "  defined: header_operations.py:{line}");
        }
        if !op.signature.is_empty() {
            let _ = writeln!(out, "  usage:   {}", op.signature);
        }
        let mut traits = Vec::new();
        if op.can_fail {
            traits.push("condition (can fail)");
        }
        if op.lhs {
            traits.push("writes its first operand");
        }
        if op.global_lhs {
            traits.push("writes a global");
        }
        if !traits.is_empty() {
            let _ = writeln!(out, "  kind:    {}", traits.join(", "));
        }
        return Ok(out);
    }
    if let Some(range) = atlas.ranges.iter().find(|range| range.name == query) {
        let mut out = format!("{}  (constant) = {}\n", range.name, range.value);
        if let Some(line) = range.line {
            let _ = writeln!(out, "  defined: module_constants.py:{line}");
        }
        return Ok(out);
    }
    Err(unknown(atlas, query))
}

fn unknown(atlas: &Atlas, query: &str) -> String {
    let refs: Vec<String> = atlas
        .tables
        .values()
        .filter_map(|table| {
            let tag = table.ref_tag.as_deref()?;
            Some(table.entries.iter().filter_map(move |entry| {
                entry.name.as_deref().map(|name| format!("{tag}_{}", name.to_lowercase()))
            }))
        })
        .flatten()
        .collect();
    let candidates = atlas
        .all_ids()
        .chain(refs.iter().map(String::as_str))
        .chain(atlas.ops.iter().map(|op| op.name.as_str()))
        .chain(atlas.ranges.iter().map(|range| range.name.as_str()));
    let mut message = format!("'{query}' is not defined in Native");
    if let Some(candidate) = uw_diag::suggest(query, candidates) {
        let _ = write!(message, "; did you mean '{candidate}'?");
    }
    message
}

fn explain_entry(atlas: &Atlas, table: &str, entry: &Entry, all: bool) -> String {
    let t = &atlas.tables[table];
    let mut out = String::new();
    let _ = write!(out, "{}  ({table} #{}", entry_label(atlas, table, entry), entry.index);
    if let Some(name) = &entry.name {
        let _ = write!(out, ", name \"{name}\"");
    }
    out.push_str(")\n");
    if atlas.is_hook(entry.name.as_deref().unwrap_or("")) && table == "scripts" {
        out.push_str("  engine hook: the game calls this script itself\n");
    }
    if let Some(name) = &entry.name {
        if let Some(tag) = &t.ref_tag {
            let first = atlas.resolve_ref(&format!("{tag}_{name}"));
            let same = first.is_some_and(|(_, e)| e.index == entry.index);
            if same {
                let _ = writeln!(out, "  in code: {tag}_{}", name.to_lowercase());
            } else {
                let _ = writeln!(
                    out,
                    "  in code: unreachable, {tag}_{} names the first entry with this name (#{})",
                    name.to_lowercase(),
                    first.map_or(0, |(_, e)| e.index)
                );
            }
        }
    }
    match entry.line {
        Some(line) => {
            let _ = writeln!(out, "  defined: {}.py:{line}", t.module);
        }
        None => {
            let _ = writeln!(out, "  defined: {}.py (entry #{} of {})", t.module, entry.index, t.var);
        }
    }
    let _ = writeln!(out, "  value:   {}", preview(&entry.value));
    let refs = atlas.references(|text| {
        atlas
            .resolve_ref(text)
            .is_some_and(|(tbl, e)| tbl == table && e.index == entry.index)
    });
    write_uses(&mut out, atlas, &refs, all);
    out
}

/// `p_town_6`, `script_game_start`, `dialogs #5`.
fn entry_label(atlas: &Atlas, table: &str, entry: &Entry) -> String {
    if let Some(id) = &entry.id {
        return id.clone();
    }
    let t = &atlas.tables[table];
    match (&t.id_prefix, &entry.name, entry.dup) {
        (Some(prefix), Some(name), Some(dup)) => format!("{prefix}{}#{dup}", name.to_lowercase()),
        _ => format!("{table} #{}", entry.index),
    }
}

fn preview(value: &serde_json::Value) -> String {
    let text = value.to_string();
    if text.chars().count() <= VALUE_PREVIEW {
        return text;
    }
    let cut: String = text.chars().take(VALUE_PREVIEW).collect();
    format!("{cut}…")
}

fn write_uses(out: &mut String, atlas: &Atlas, refs: &[Reference], all: bool) {
    if refs.is_empty() {
        out.push_str("  used:    nowhere in Native operations\n");
        return;
    }
    // (table, index) -> op name -> count, in table order.
    let mut sites: BTreeMap<(&str, usize), BTreeMap<&str, usize>> = BTreeMap::new();
    for r in refs {
        let ops = sites.entry((r.table.as_str(), r.index)).or_default();
        *ops.entry(r.op.as_deref().unwrap_or("data")).or_default() += 1;
    }
    let mut per_table: BTreeMap<&str, usize> = BTreeMap::new();
    for (table, _) in sites.keys() {
        *per_table.entry(table).or_default() += 1;
    }
    let summary: Vec<String> = per_table.iter().map(|(t, n)| format!("{n} {t}")).collect();
    let _ = writeln!(
        out,
        "  used:    {} times in {} entries ({})",
        refs.len(),
        sites.len(),
        summary.join(", ")
    );
    let shown = if all { sites.len() } else { SHORT_LIMIT.min(sites.len()) };
    for ((table, index), ops) in sites.iter().take(shown) {
        let t = &atlas.tables[*table];
        let entry = &t.entries[*index];
        let place = match entry.line {
            Some(line) => format!("{}.py:{line}", t.module),
            None => format!("{}.py", t.module),
        };
        let ops: Vec<String> = ops
            .iter()
            .map(|(op, n)| if *n > 1 { format!("{op} x{n}") } else { (*op).to_string() })
            .collect();
        let _ = writeln!(
            out,
            "    {:<40} {:<30} {}",
            entry_label(atlas, table, entry),
            place,
            ops.join(", ")
        );
    }
    if shown < sites.len() {
        let _ = writeln!(out, "    … {} more (uw explain --all)", sites.len() - shown);
    }
}
