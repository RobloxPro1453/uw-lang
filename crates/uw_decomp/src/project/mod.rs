//! `uw new <dir> --from native` (P3a, docs/meclis/YON2-KARAR.md §3.2): writes every Native record as
//! UW record text into a project folder, and reads such a folder back to prove nothing was lost.
//! Export only: the folder is not compiled yet (that is P3b).

pub mod layout;
pub mod load;
pub mod pymodule;

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use serde_json::Value as Json;
use uw_atlas::{Atlas, Entry};

use crate::records::Schema;

/// Turkish plain-language descriptions (`kisa`), found like the atlas: this or a parent folder.
const DESCRIPTIONS_REL: &str = "kit-meta/aciklama";

/// At most this many failures are kept; enough to see what broke.
const FIRST_FAILURES: usize = 5;

/// Scripts whose `call_script` gives a center to a faction at game start.
const OWNER_SCRIPTS: [&str; 2] =
    ["script_give_center_to_faction_aux", "script_give_center_to_faction"];

/// The script that runs once when a new game starts.
const GAME_START: &str = "game_start";

/// What one export wrote.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExportSummary {
    pub files: usize,
    pub records: usize,
}

/// What one verify found when it read a project folder back.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VerifySummary {
    /// Records the atlas has (and the files should hold).
    pub records: usize,
    /// Records that came back identical to the atlas.
    pub exact: usize,
    pub failed: usize,
    /// The first few failures, with the file and the record id.
    pub first_failures: Vec<String>,
}

/// Writes every entry of every schema's table under `out`, at `layout::path_for`. Records that share a
/// path go into one file, in atlas order, separated by a blank line. `render` turns one entry into its
/// record text (records::print today; the block-aware printer after T-043).
///
/// A non-empty `out` is refused: an export never overwrites someone's folder.
pub fn export(
    atlas: &Atlas,
    schemas: &[Schema],
    out: &Path,
    render: &mut dyn FnMut(&Schema, &Entry) -> String,
) -> Result<ExportSummary, String> {
    if out.exists() {
        let empty = std::fs::read_dir(out)
            .map_err(|e| format!("{}: {e}", out.display()))?
            .next()
            .is_none();
        if !empty {
            return Err(format!("{} is not empty; refusing to overwrite it (choose a new folder)", out.display()));
        }
    }
    std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let descriptions = descriptions();
    let owners = owners(atlas);
    let files = group_by_path(atlas, schemas);
    let mut summary = ExportSummary::default();
    for (relative, records) in &files {
        let table = &records[0].0.table;
        let mut text = format!("// {table}: {} records\n", records.len());
        for (schema, entry) in records {
            text.push('\n');
            if let Some(line) = comment(schema, entry, &descriptions) {
                text.push_str(&line);
                text.push('\n');
            }
            let mut body = render(schema, entry).trim_end().to_owned();
            if schema.table == "parties" {
                if let Some(line) = owner_line(atlas, &owners, entry) {
                    // Inside the record, as its first field: the modder changes the owner here.
                    if let Some(index) = body.find('\n') {
                        body.insert_str(index + 1, &format!("  {line}\n"));
                    }
                }
            }
            text.push_str(&body);
            text.push('\n');
        }
        let path = out.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        summary.files += 1;
        summary.records += records.len();
    }
    write_metadata(atlas, schemas, out)?;
    Ok(summary)
}

/// Reads every `layout::path_for` file of `root` back and compares its records with the atlas by id.
/// `parse` turns one record text into `(id, atlas value)` (records::parse today).
pub fn verify(
    atlas: &Atlas,
    schemas: &[Schema],
    root: &Path,
    parse: &mut dyn FnMut(&Schema, &str) -> Result<(String, Json), String>,
) -> Result<VerifySummary, String> {
    verify_with(atlas, schemas, root, parse, &mut |_, entry, value| *value == entry.value)
}

/// [`verify`] with the record comparison given: `same(schema, entry, parsed value)`. `uw new --check`
/// compares operation blocks as operations (`records::record_difference`).
pub fn verify_with(
    atlas: &Atlas,
    schemas: &[Schema],
    root: &Path,
    parse: &mut dyn FnMut(&Schema, &str) -> Result<(String, Json), String>,
    same: &mut dyn FnMut(&Schema, &Entry, &Json) -> bool,
) -> Result<VerifySummary, String> {
    let mut summary = VerifySummary::default();
    for (relative, expected) in &group_by_path(atlas, schemas) {
        let schema = expected[0].0;
        let path = root.join(relative);
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let ids: Vec<String> = expected.iter().map(|(schema, entry)| record_id(schema, entry)).collect();
        let mut found = vec![false; expected.len()];
        summary.records += expected.len();
        for (_start, mut record) in split_records(&text) {
            // A parties record carries the project-only `owner` field; it is not part of the atlas value.
            if schema.table == "parties" {
                take_owner(&mut record);
            }
            // `name_<code>` lines are project-only too (T-113); they are not part of the atlas value.
            take_names(&mut record, schema);
            let failure = match parse(schema, &record) {
                Err(e) => format!("{relative}: {e}"),
                // The atlas may repeat an id (dialogs do), so every occurrence is matched once.
                Ok((id, value)) => match ids.iter().enumerate().position(|(i, known)| *known == id && !found[i]) {
                    None if ids.iter().any(|known| *known == id) => format!("{relative}: `{id}` appears twice"),
                    None => format!("{relative}: `{id}` is not in the atlas"),
                    Some(index) => {
                        found[index] = true;
                        if same(expected[index].0, expected[index].1, &value) {
                            summary.exact += 1;
                            continue;
                        }
                        format!("{relative}: `{id}`: the record no longer matches the atlas value")
                    }
                },
            };
            summary.failed += 1;
            if summary.first_failures.len() < FIRST_FAILURES {
                summary.first_failures.push(failure);
            }
        }
        for (index, id) in ids.iter().enumerate() {
            if !found[index] {
                summary.failed += 1;
                if summary.first_failures.len() < FIRST_FAILURES {
                    summary.first_failures.push(format!("{relative}: `{id}` is missing from the file"));
                }
            }
        }
    }
    Ok(summary)
}

/// The records of one project file, split where the braces close (a record may hold nested blocks);
/// `//` comment lines and the blank lines between records are dropped. Each record carries its
/// 1-based first line in the file, so a parse error can point at the file instead of the record.
fn split_records(text: &str) -> Vec<(usize, String)> {
    let mut records = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut start = 0usize;
    for (index, line) in text.lines().enumerate() {
        if depth <= 0 && current.is_empty() && (line.trim().is_empty() || line.trim_start().starts_with("//")) {
            continue;
        }
        if current.is_empty() {
            start = index + 1;
        }
        current.push_str(line);
        current.push('\n');
        scan_braces(line, &mut depth, &mut in_string);
        if depth <= 0 {
            records.push((start, std::mem::take(&mut current)));
            depth = 0;
            in_string = false;
        }
    }
    // A file that ends inside a record still hands the text over, so `parse` can name the problem.
    if !current.is_empty() {
        records.push((start, current));
    }
    records
}

/// Counts the braces of a line, ignoring the ones inside a quoted string.
fn scan_braces(line: &str, depth: &mut i32, in_string: &mut bool) {
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if *in_string {
            match c {
                '\\' => {
                    chars.next();
                }
                '"' => *in_string = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => *in_string = true,
            '{' => *depth += 1,
            '}' => *depth -= 1,
            _ => {}
        }
    }
}

/// Takes the project-only `owner` field out of a parties record, blanking its line so the record's
/// own line numbers still point at the file. `None` when the record has no owner line.
pub(crate) fn take_owner(record: &mut String) -> Option<(String, usize)> {
    let mut taken: Option<(String, usize)> = None;
    let mut kept = String::with_capacity(record.len());
    let mut depth = 0i32;
    let mut in_string = false;
    for (line_number, line) in record.split_inclusive('\n').enumerate() {
        let trimmed = line.trim_start();
        if taken.is_none() && depth == 1 && trimmed.starts_with("owner ") {
            let value = trimmed["owner ".len()..].split_whitespace().next().unwrap_or("").to_owned();
            taken = Some((value, line_number));
            kept.push('\n');
        } else {
            kept.push_str(line);
        }
        scan_braces(line, &mut depth, &mut in_string);
    }
    if taken.is_some() {
        *record = kept;
    }
    taken
}

/// One project-only `name_<code> "…"` line (T-113): the record's text in one language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageName {
    /// The atlas table the record belongs to (`parties`).
    pub table: String,
    /// The full kit id (`p_town_6`), the translation key.
    pub id: String,
    /// The language folder code (`tr`, `de`, …).
    pub language: String,
    /// What the project wants the record called in that language.
    pub text: String,
}

/// Takes the project-only `name_<code> "…"` lines out of a record (T-113): the record's text in one
/// language, the code being letters only. A line whose value is not a quoted string, or whose word is
/// a schema field, is left in place so the schema parser reports it instead of it vanishing silently.
/// Each taken line is blanked, so the record's own line numbers still point at the file.
pub(crate) fn take_names(record: &mut String, schema: &Schema) -> Vec<(String, String)> {
    let mut taken = Vec::new();
    let mut kept = String::with_capacity(record.len());
    let mut depth = 0i32;
    let mut in_string = false;
    for line in record.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if depth == 1 {
            if let Some((language, text)) = name_line(trimmed) {
                if !schema.fields.iter().any(|field| field.name == format!("name_{language}")) {
                    taken.push((language, text));
                    kept.push('\n');
                    scan_braces(line, &mut depth, &mut in_string);
                    continue;
                }
            }
        }
        kept.push_str(line);
        scan_braces(line, &mut depth, &mut in_string);
    }
    if !taken.is_empty() {
        *record = kept;
    }
    taken
}

/// `name_<code> "<text>"` (a trailing `//` comment is allowed) as `(code, text)`.
fn name_line(trimmed: &str) -> Option<(String, String)> {
    let rest = trimmed.strip_prefix("name_")?;
    let end = rest.find(|c: char| !c.is_ascii_alphabetic())?;
    if end == 0 {
        return None;
    }
    let language = &rest[..end];
    let (text, after) = quoted(rest[end..].trim_start())?;
    let after = after.trim_start();
    if after.is_empty() || after.starts_with("//") {
        Some((language.to_owned(), text))
    } else {
        None
    }
}

/// One quoted string at the start of `text`, with what follows it; `\"`, `\\` and `\n` are read.
fn quoted(text: &str) -> Option<(String, &str)> {
    let mut chars = text.char_indices();
    match chars.next()? {
        (_, '"') => {}
        _ => return None,
    }
    let mut out = String::new();
    while let Some((index, c)) = chars.next() {
        match c {
            '"' => return Some((out, &text[index + 1..])),
            '\\' => match chars.next()?.1 {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                'n' => out.push('\n'),
                other => out.push(other),
            },
            _ => out.push(c),
        }
    }
    None
}

/// The relative path of every record, in atlas order: the exporter writes one file per path.
fn group_by_path<'a>(atlas: &'a Atlas, schemas: &'a [Schema]) -> BTreeMap<String, Vec<(&'a Schema, &'a Entry)>> {
    let mut files: BTreeMap<String, Vec<(&Schema, &Entry)>> = BTreeMap::new();
    for schema in schemas {
        let Some(table) = atlas.tables.get(&schema.table) else { continue };
        for entry in &table.entries {
            files.entry(layout::path_for(atlas, schema, entry)).or_default().push((schema, entry));
        }
    }
    files
}

/// The id a record is printed with: the atlas id without the schema's prefix (records::print does
/// the same; the helper there is private). Tables without ids use the record's index.
fn record_id(schema: &Schema, entry: &Entry) -> String {
    match entry.id.as_deref().or(entry.name.as_deref()) {
        Some(id) if !id.is_empty() => id.strip_prefix(&schema.id_prefix).unwrap_or(id).to_owned(),
        _ => entry.index.to_string(),
    }
}

/// `// <display name> — <short Turkish description>`; `None` when neither is known.
fn comment(schema: &Schema, entry: &Entry, descriptions: &HashMap<String, String>) -> Option<String> {
    let clean = |text: &str| text.replace('\n', " ").replace('\r', " ");
    let name = display_name(schema, entry).map(|name| clean(&name));
    let description = entry
        .id
        .as_deref()
        .and_then(|id| descriptions.get(id))
        .or_else(|| {
            let name = entry.name.as_deref()?;
            descriptions.get(&format!("{}{name}", schema.id_prefix))
        })
        .map(|text| clean(text));
    match (name, description) {
        (Some(name), Some(description)) => Some(format!("// {name} — {description}")),
        (Some(name), None) => Some(format!("// {name}")),
        (None, Some(description)) => Some(format!("// {description}")),
        (None, None) => None,
    }
}

/// party id -> faction id, from the `game_start` script and the scripts it calls directly (one
/// level). Empty when the atlas has no `call_script` op (a tiny test atlas).
fn owners(atlas: &Atlas) -> HashMap<String, String> {
    let mut owners = HashMap::new();
    let Some(code) = atlas.op("call_script").map(|op| op.code) else {
        return owners;
    };
    let mut called: Vec<String> = Vec::new();
    for entry in atlas.entries_named("scripts", GAME_START) {
        let Some(ops) = script_body(entry) else { continue };
        for (target, party, faction) in calls(&ops, code) {
            if OWNER_SCRIPTS.contains(&target) {
                owner_pair(party, faction, &mut owners);
            } else if target.starts_with("script_") && !called.iter().any(|seen| seen == target) {
                called.push(target.to_owned());
            }
        }
    }
    for id in called {
        let Some(("scripts", entry)) = atlas.entry_by_id(&id) else { continue };
        let Some(ops) = script_body(entry) else { continue };
        for (target, party, faction) in calls(&ops, code) {
            if OWNER_SCRIPTS.contains(&target) {
                owner_pair(party, faction, &mut owners);
            }
        }
    }
    owners
}

/// The operation block of a scripts entry (`module_scripts.py:22`: name, body).
fn script_body(entry: &Entry) -> Option<Vec<crate::RawOp>> {
    crate::parse_ops(crate::items(&entry.value)?.get(1)?)
}

/// The `(called script, first operand, second operand)` of every `call_script` with three string
/// operands.
fn calls<'a>(ops: &'a [crate::RawOp], code: u64) -> impl Iterator<Item = (&'a str, &'a str, &'a str)> + 'a {
    ops.iter().filter(move |op| op.code == code).filter_map(|op| match op.args.as_slice() {
        [crate::RawArg::Str(target), crate::RawArg::Str(first), crate::RawArg::Str(second), ..] => {
            Some((target.as_str(), first.as_str(), second.as_str()))
        }
        _ => None,
    })
}

/// Adds `party -> faction` when the two operands are literal kit references.
fn owner_pair(party: &str, faction: &str, owners: &mut HashMap<String, String>) {
    if party.starts_with("p_") && faction.starts_with("fac_") {
        owners.insert(party.to_owned(), faction.to_owned());
    }
}

/// `owner <faction>  // who gets it at game start (game_start script); change it here`: the
/// project's game name for the faction (`swadia`) when it has one, the kit id (`fac_kingdom_1`)
/// otherwise.
fn owner_line(atlas: &Atlas, owners: &HashMap<String, String>, entry: &Entry) -> Option<String> {
    let faction = owners.get(entry.id.as_deref()?)?;
    let owner = atlas.game_name(faction).unwrap_or(faction);
    Some(format!("owner {owner}  // who gets it at game start (game_start script); change it here"))
}

/// One `owner` line the build applied to a `game_start` give-center call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerChange {
    /// Party reference (`p_town_1`).
    pub party: String,
    /// The faction the call gave the center to (`fac_kingdom_1`).
    pub from: String,
    /// The faction from the project's `owner` line (`fac_kingdom_4`).
    pub to: String,
}

/// Rewrites the `game_start` give-center calls from the project's `owner` lines (full party and
/// faction references). A center with an owner line but no call is an error: never a silent add.
pub fn apply_owners(
    atlas: &Atlas,
    records: &mut BTreeMap<String, Vec<Json>>,
    owners: &BTreeMap<String, String>,
) -> Result<Vec<OwnerChange>, Vec<String>> {
    if owners.is_empty() {
        return Ok(Vec::new());
    }
    let Some(code) = atlas.op("call_script").map(|op| op.code) else {
        return Err(owners.keys().map(|party| missing_owner_call(party)).collect());
    };
    // (record, statement, party, faction) of every call that gives a center to a faction, in
    // `game_start` and in the scripts it calls directly: the same scan the owner lines come from.
    let mut calls: Vec<(usize, usize, String, String)> = Vec::new();
    {
        let Some(scripts) = records.get("scripts") else {
            return Err(owners.keys().map(|party| missing_owner_call(party)).collect());
        };
        let starts: Vec<usize> = (0..scripts.len())
            .filter(|&index| {
                crate::items(&scripts[index]).and_then(|items| items.first()).and_then(Json::as_str)
                    == Some(GAME_START)
            })
            .collect();
        let mut called: Vec<usize> = Vec::new();
        for &index in &starts {
            for (statement, target, first, second) in script_calls(&scripts[index], code) {
                if OWNER_SCRIPTS.contains(&target.as_str()) {
                    calls.push((index, statement, first, second));
                } else if target.starts_with("script_") {
                    if let Some(("scripts", entry)) = atlas.entry_by_id(&target) {
                        if !called.contains(&entry.index) {
                            called.push(entry.index);
                        }
                    }
                }
            }
        }
        for index in called {
            let Some(value) = scripts.get(index) else { continue };
            for (statement, target, first, second) in script_calls(value, code) {
                if OWNER_SCRIPTS.contains(&target.as_str()) {
                    calls.push((index, statement, first, second));
                }
            }
        }
    }

    let mut wanted: Vec<Option<String>> = vec![None; calls.len()];
    let mut errors = Vec::new();
    let mut changes = Vec::new();
    for (party, faction) in owners {
        let matches: Vec<usize> =
            calls.iter().enumerate().filter(|(_, call)| &call.2 == party).map(|(index, _)| index).collect();
        if matches.is_empty() {
            errors.push(missing_owner_call(party));
            continue;
        }
        let from = calls[matches[0]].3.clone();
        if &from != faction {
            for &index in &matches {
                wanted[index] = Some(faction.clone());
            }
            changes.push(OwnerChange { party: party.clone(), from, to: faction.clone() });
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let scripts = records.get_mut("scripts").expect("present when calls were found");
    for (index, call) in calls.iter().enumerate() {
        let Some(to) = wanted[index].as_ref() else { continue };
        if let Some(Json::Array(parts)) =
            script_statements_mut(&mut scripts[call.0]).and_then(|statements| statements.get_mut(call.1))
        {
            if parts.len() > 3 {
                parts[3] = Json::String(to.clone());
            }
        }
    }
    Ok(changes)
}

/// Every three-operand `call_script` in a script value: `(statement index, target, first, second)`.
fn script_calls(value: &Json, code: u64) -> Vec<(usize, String, String, String)> {
    let Some(statements) = crate::items(value).and_then(|items| items.get(1)).and_then(crate::items) else {
        return Vec::new();
    };
    statements
        .iter()
        .enumerate()
        .filter_map(|(index, statement)| {
            let parts = crate::items(statement)?;
            if parts.first()?.as_u64()? != code {
                return None;
            }
            Some((
                index,
                parts.get(1)?.as_str()?.to_owned(),
                parts.get(2)?.as_str()?.to_owned(),
                parts.get(3)?.as_str()?.to_owned(),
            ))
        })
        .collect()
}

/// The statement list of a script value, for rewriting.
fn script_statements_mut(value: &mut Json) -> Option<&mut Vec<Json>> {
    let items = match value {
        Json::Array(items) => items,
        Json::Object(map) => map.get_mut("list")?.as_array_mut()?,
        _ => return None,
    };
    match items.get_mut(1)? {
        Json::Array(statements) => Some(statements),
        Json::Object(map) => map.get_mut("list")?.as_array_mut(),
        _ => None,
    }
}

/// The Turkish error for an `owner` line whose center the `game_start` script never gives.
fn missing_owner_call(party: &str) -> String {
    format!(
        "{party}: `owner` satırı var ama game_start betiğinde bu merkezi veren bir çağrı yok \
         (sessizce eklenmez)"
    )
}

/// The record's own `name` field when it holds a string (the name a player sees), else the atlas
/// entry's name.
fn display_name(schema: &Schema, entry: &Entry) -> Option<String> {
    let index = schema.fields.iter().position(|field| field.name == "name");
    let from_value = index
        .and_then(|index| crate::items(&entry.value)?.get(index)?.as_str().map(str::to_owned));
    from_value.or_else(|| entry.name.clone().filter(|name| !name.is_empty()))
}

/// Turkish short descriptions: full atlas id -> `kisa` text, from every `kit-meta/aciklama/*.json`
/// (`ops.json` describes operations, not records). A missing folder gives an empty map.
fn descriptions() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Some(dir) = std::env::current_dir()
        .ok()
        .and_then(|cwd| cwd.ancestors().map(|dir| dir.join(DESCRIPTIONS_REL)).find(|path| path.is_dir()))
    else {
        return map;
    };
    let Ok(entries) = std::fs::read_dir(&dir) else { return map };
    for file in entries.flatten() {
        let path = file.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        if path.file_stem().and_then(|stem| stem.to_str()) == Some("ops") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let Ok(Json::Object(records)) = serde_json::from_str::<Json>(&text) else { continue };
        for (id, value) in records {
            if let Some(short) = value.get("kisa").and_then(Json::as_str) {
                if !short.is_empty() {
                    map.insert(id, short.to_owned());
                }
            }
        }
    }
    map
}

/// The `[languages.<code>]` rules a project may set in `uw.toml` (T-113).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LanguageRule {
    /// `town_suffix`: the suffix a town/castle/village name gets (T-094).
    pub town_suffix: Option<String>,
    /// Endings to remove from every translation of this language (`[" Şehri", " Kalesi"]`).
    pub strip_suffix: Vec<String>,
    /// Endings to add to every translation of this language.
    pub add_suffix: Vec<String>,
}

/// Every `[languages.<code>]` section of the project's `uw.toml`, keyed by the language folder name
/// (`tr`). A folder without `uw.toml` has no rules; a broken file, or a suffix rule that is not a list
/// of strings, is an error, so a typo cannot silently do nothing.
pub fn language_rules(dir: &Path) -> Result<BTreeMap<String, LanguageRule>, String> {
    let path = dir.join("uw.toml");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: toml::Value = text.parse().map_err(|e| format!("{}: {e}", path.display()))?;
    let mut found = BTreeMap::new();
    for (language, section) in value.get("languages").and_then(toml::Value::as_table).into_iter().flatten() {
        let rule = LanguageRule {
            town_suffix: section.get("town_suffix").and_then(toml::Value::as_str).map(str::to_owned),
            strip_suffix: string_list(section, "strip_suffix", language, &path)?,
            add_suffix: string_list(section, "add_suffix", language, &path)?,
        };
        if rule != LanguageRule::default() {
            found.insert(language.clone(), rule);
        }
    }
    Ok(found)
}

/// One `[languages.<code>]` key that must be a list of strings.
fn string_list(section: &toml::Value, key: &str, language: &str, path: &Path) -> Result<Vec<String>, String> {
    let Some(value) = section.get(key) else { return Ok(Vec::new()) };
    let message = || format!("{}: [languages.{language}] {key} must be a list of strings", path.display());
    value
        .as_array()
        .ok_or_else(message)?
        .iter()
        .map(|item| item.as_str().map(str::to_owned).ok_or_else(message))
        .collect()
}

/// The per-language town suffix a project may set (T-094):
/// `[languages.<lang>] town_suffix = "..."` in `uw.toml`. A folder without `uw.toml` has no
/// setting; a broken file is an error, so a typo cannot silently do nothing.
pub fn town_suffixes(dir: &Path) -> Result<BTreeMap<String, String>, String> {
    Ok(language_rules(dir)?
        .into_iter()
        .filter_map(|(language, rule)| Some((language, rule.town_suffix?)))
        .collect())
}

/// `uw.lock` (the record order), `uw.toml` (the folder is an export, not a compiled project yet)
/// and the Turkish `README.md`.
fn write_metadata(atlas: &Atlas, schemas: &[Schema], out: &Path) -> Result<(), String> {
    let mut toml_text = String::from(
        "# uw new --from native (P3a): every Native record as UW text. Not compiled yet (P3b).\n\
         # base = \"native-export\": this folder is an export of Native, not a project build.\n\
         base = \"native-export\"\n",
    );
    if let Some(hash) = &atlas.kit_hash {
        toml_text.push_str(&format!("kit_hash = \"{hash}\"\n"));
    }
    std::fs::write(out.join("uw.toml"), toml_text).map_err(|e| format!("{}: {e}", out.join("uw.toml").display()))?;
    load::write_lock(atlas, schemas, out)?;
    std::fs::write(out.join("README.md"), README).map_err(|e| format!("{}: {e}", out.join("README.md").display()))?;
    Ok(())
}

/// Turkish, short: what the folder is, that it is not compiled yet, and how to check it.
const README: &str = "\
# Native'den aktarılmış UW projesi

Bu klasör `uw new --from native` ile yazıldı: Native'in her kaydı UW kayıt metni olarak,
tablo/tür düzenine göre ayrı dosyalarda duruyor. Her dosyanın ilk satırı tabloyu ve kayıt
sayısını söyler; kaydın üstündeki `//` satırı görünen adı ve varsa Türkçe kısa açıklamayı verir.

`uw.toml` içinde `base = \"native-export\"` yazar: bu klasör bir dışa aktarım kopyasıdır.

## Henüz derlenmiyor

Bu klasör **derlenmiyor**: `base = \"project\"` derlemesi (P3b) gelene kadar `uw build` bu
klasörü mod olarak kurmaz. Kayıtlar burada yalnız okunur, denetlenir ve incelenir.

## Denetleme

Klasörü geri okuyup her kaydın atlas (Native) ile birebir aynı olduğunu görmek için:

    uw new --check .

Çıktı `records N, exact N, failed 0` ise her kayıt birebir okundu. Bir kayıt eksik, yinelenmiş
ya da artık ayrıştırılamıyorsa `failed` sıfırdan büyük olur ve ilk hatalar listelenir.

## Notlar

- Bu klasör Native'in kopyasıdır; elle düzenlerseniz `--check` farkı gösterir. Kaynak doğruluk
  Native'in kendisidir.
- Klasör boş değilken `uw new` onu ezmez: yeni bir klasör adı verin.
";
