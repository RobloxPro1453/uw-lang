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
            if schema.table == "parties" {
                if let Some(line) = owner_line(atlas, &owners, entry) {
                    text.push_str(&line);
                    text.push('\n');
                }
            }
            text.push_str(render(schema, entry).trim_end());
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
        for record in split_records(&text) {
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
/// `//` comment lines and the blank lines between records are dropped.
fn split_records(text: &str) -> Vec<String> {
    let mut records = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    for line in text.lines() {
        if depth <= 0 && current.is_empty() && (line.trim().is_empty() || line.trim_start().starts_with("//")) {
            continue;
        }
        current.push_str(line);
        current.push('\n');
        scan_braces(line, &mut depth, &mut in_string);
        if depth <= 0 {
            records.push(std::mem::take(&mut current));
            depth = 0;
            in_string = false;
        }
    }
    // A file that ends inside a record still hands the text over, so `parse` can name the problem.
    if !current.is_empty() {
        records.push(current);
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

/// `// at game start: owned by <faction> (game_start script)`: the project's game name for the
/// faction (`swadia`) when it has one, the kit id (`fac_kingdom_1`) otherwise.
fn owner_line(atlas: &Atlas, owners: &HashMap<String, String>, entry: &Entry) -> Option<String> {
    let faction = owners.get(entry.id.as_deref()?)?;
    let owner = atlas.game_name(faction).unwrap_or(faction);
    Some(format!("// at game start: owned by {owner} (game_start script)"))
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
