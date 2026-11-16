//! `uw rename <record> "<new name>"` (T-094): one record's own name and every text that mentions
//! it — the project's other record texts (any quoted string) and its `languages/<lang>/*.csv`
//! translations. The default keeps each text's own pattern: the old name is replaced inside it
//! ("Praven Şehri" -> "Adam Siken Şehri"), so a translation keeps its suffix and its language.
//! `--list` only lists; otherwise each group is asked first (`--yes` writes all). Native and the
//! game folder are never written, and a file is replaced atomically (temp + rename).

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_atlas::{Atlas, Entry};
use uw_decomp::project::layout;
use uw_decomp::records::Schema;

const USAGE: &str = "usage:
  uw rename <record> \"<new name>\" [--list] [--yes] [--texts all|1,3] [--atlas <atlas.json>]
      rename a record and every text that mentions it: its own name, the project's other record
      texts and its languages/<lang>/*.csv translations; the old name is replaced inside each
      text, so \"Praven Şehri\" becomes \"Adam Siken Şehri\"
      --list: only list what would change; --yes: write every group without asking
      --texts all|1,3: write only the numbered other texts (the record's own name is renamed too)
      every changed file is first copied to .uw-backup/rename-<time>/ in the project
  uw rename <dir> <record> \"<new name>\" [--list] [--yes] [--texts all|1,3] [--atlas <atlas.json>]
      the same, for the project folder <dir>
  uw rename --undo [<dir>]
      put back the files of the last rename (only files nobody changed since)";

/// What the command line asked for.
struct Args {
    /// The project folder; none means this folder.
    dir: Option<PathBuf>,
    /// The record to rename: its `names.uw` game name (`praven`) or its kit reference (`p_town_6`).
    record: String,
    new_name: String,
    list: bool,
    yes: bool,
    /// `--texts all|1,3,…`: the numbered other texts to rewrite too.
    texts: Option<String>,
    /// `--undo`: restore the last rename's backup instead.
    undo: bool,
    atlas: Option<PathBuf>,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut words: Vec<String> = Vec::new();
    let mut list = false;
    let mut yes = false;
    let mut texts = None;
    let mut undo = false;
    let mut atlas = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--list" => list = true,
            "--yes" => yes = true,
            "--texts" => match it.next() {
                Some(choice) => texts = Some(choice.clone()),
                None => return Err(format!("--texts needs `all` or numbers (1,3)\n{USAGE}")),
            },
            "--undo" => undo = true,
            "--atlas" => match it.next() {
                Some(path) => atlas = Some(PathBuf::from(path)),
                None => return Err(format!("--atlas needs a path\n{USAGE}")),
            },
            other if other.starts_with('-') => return Err(format!("unknown argument `{other}`\n{USAGE}")),
            other => words.push(other.to_owned()),
        }
    }
    if undo {
        if texts.is_some() {
            return Err(format!("--texts cannot be used with --undo\n{USAGE}"));
        }
        let dir = match words.as_slice() {
            [] => None,
            [dir] => Some(PathBuf::from(dir)),
            _ => return Err(USAGE.to_owned()),
        };
        return Ok(Args { dir, record: String::new(), new_name: String::new(), list, yes, texts, undo, atlas });
    }
    let (dir, record, new_name) = match words.as_slice() {
        [record, new_name] => (None, record.clone(), new_name.clone()),
        [dir, record, new_name] => (Some(PathBuf::from(dir)), record.clone(), new_name.clone()),
        _ => return Err(USAGE.to_owned()),
    };
    if new_name.is_empty()
        || new_name.contains('"')
        || new_name.contains('\\')
        || new_name.chars().any(|c| c == '\n' || c == '\r')
    {
        return Err("the new name may not be empty or hold `\"`, `\\` or a line break".to_owned());
    }
    Ok(Args { dir, record, new_name, list, yes, texts, undo, atlas })
}

pub fn cmd(args: &[String]) -> ExitCode {
    let args = match parse(args) {
        Ok(args) => args,
        Err(message) => return crate::fail(&message),
    };
    let dir = match crate::enter_home(args.dir.as_deref().unwrap_or(Path::new("."))) {
        Ok(dir) => dir,
        Err(message) => return crate::fail(&message),
    };
    let dir = crate::plain(dir.canonicalize().unwrap_or(dir));
    let result = if args.undo { undo(&dir) } else { run(&dir, &args) };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => crate::fail(&message),
    }
}

/// One found text: the file, the 1-based line, the byte range of the text that changes inside the
/// file, and that line (trimmed) for printing.
struct Hit {
    path: PathBuf,
    /// The path as printed: relative to the project folder, forward slashes.
    file: String,
    line: usize,
    start: usize,
    end: usize,
    text: String,
}

fn run(dir: &Path, args: &Args) -> Result<(), String> {
    if !dir.is_dir() {
        return Err(format!("{} is not a folder", dir.display()));
    }
    let (mut atlas, schemas) = crate::new::atlas_and_schemas(args.atlas.clone())?;
    crate::new::install_names(&mut atlas, dir)?;
    let (table, entry) = atlas.resolve_ref(&args.record).ok_or_else(|| {
        format!(
            "no record named `{}`: write its names.uw name or its kit reference (`p_town_6`)",
            args.record
        )
    })?;
    let schema = schemas
        .iter()
        .find(|schema| schema.table == table)
        .ok_or_else(|| format!("no record schema for `{table}` (kit-meta/records)"))?;
    if !schema.fields.iter().any(|field| field.name == "name") {
        return Err(format!("`{}` records have no `name` field to rename", schema.table));
    }
    let own = own_name_field(&atlas, schema, entry, dir);
    let old = old_name(schema, entry, own.as_ref().map(|(_, _, value)| value.as_str()), &args.new_name);
    let label = entry.id.as_deref().unwrap_or(&args.record);
    if old.is_empty() {
        return Err(format!("{label}: its `name` field could not be read from the project"));
    }
    if old == args.new_name {
        println!("{label} already has the name \"{}\"; nothing to rename", args.new_name);
        return Ok(());
    }
    println!("rename   {label} -> \"{}\"   (old name \"{old}\")", args.new_name);
    let (own_hits, text_hits) = record_hits(dir, &old, own.as_ref().map(|(path, line, _)| (path.as_path(), *line)))?;
    let translation_hits = translation_file_hits(dir, &old)?;
    let groups: [(&str, &Vec<Hit>); 3] = [
        ("own name", &own_hits),
        ("other English texts", &text_hits),
        ("translations", &translation_hits),
    ];
    for (name, hits) in groups {
        println!("{name}");
        print_hits(hits);
    }
    let other_hits: Vec<&Hit> = text_hits.iter().chain(translation_hits.iter()).collect();
    print_other_hits(&other_hits, &old, args);
    if args.list {
        println!("--list: nothing written");
        return Ok(());
    }
    let mut written: BTreeMap<PathBuf, Vec<&Hit>> = BTreeMap::new();
    if let Some(texts) = &args.texts {
        let chosen = chosen_texts(texts, other_hits.len())?;
        for hit in &own_hits {
            written.entry(hit.path.clone()).or_default().push(hit);
        }
        for index in chosen {
            let hit = other_hits[index];
            written.entry(hit.path.clone()).or_default().push(hit);
        }
    } else {
        for (name, hits) in groups {
            if hits.is_empty() {
                continue;
            }
            if args.yes || ask(name, hits.len()) {
                for hit in hits {
                    written.entry(hit.path.clone()).or_default().push(hit);
                }
            }
        }
    }
    if written.is_empty() {
        println!("nothing written");
        return Ok(());
    }
    // Every new text first, then the backup, then the writes: nothing is written unless all of it
    // could be computed and every original is safely copied (rule 1: data loss is the worst error).
    let mut planned: Vec<(PathBuf, String)> = Vec::new();
    for (path, hits) in &written {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        planned.push((path.clone(), replace_spans(&text, hits, &old, &args.new_name)));
    }
    let backup = back_up(dir, &planned)?;
    for (path, renamed) in &planned {
        write_atomic(path, renamed, dir)?;
    }
    for hit in written.values().flatten() {
        let changed = replace_in(&hit.text, &old, &args.new_name);
        println!("  changed  {}:{}  \"{changed}\"", hit.file, hit.line);
    }
    let lines: usize = written.values().map(Vec::len).sum();
    println!("wrote {} line(s) in {} file(s)", lines, written.len());
    println!("backup   {}   (undo: uw rename --undo)", relative(dir, &backup));
    Ok(())
}

/// `--texts`: `all` or 1-based numbers over the numbered other texts, as 0-based indices.
fn chosen_texts(texts: &str, count: usize) -> Result<Vec<usize>, String> {
    if texts == "all" {
        return Ok((0..count).collect());
    }
    let mut chosen = Vec::new();
    for part in texts.split(',') {
        let number: usize = part
            .trim()
            .parse()
            .map_err(|_| format!("--texts takes `all` or numbers (1,3), not `{texts}`\n{USAGE}"))?;
        if number == 0 || number > count {
            return Err(format!("--texts {number} is not in the list (1..{count})\n{USAGE}"));
        }
        if !chosen.contains(&(number - 1)) {
            chosen.push(number - 1);
        }
    }
    Ok(chosen)
}

/// The numbered list of the other texts that still hold the old name, and the two ways to change
/// them. The record's own name is not in this list (it is renamed with the record).
fn print_other_hits(hits: &[&Hit], old: &str, args: &Args) {
    if hits.is_empty() {
        return;
    }
    println!("\"{old}\" şu metinlerde de geçiyor (değiştirilmedi):");
    for (index, hit) in hits.iter().enumerate() {
        println!("  {:>2}  {}:{}  \"{}\"", index + 1, hit.file, hit.line, mention(hit));
    }
    let dir = match &args.dir {
        Some(dir) => format!("{} ", dir.display()),
        None => String::new(),
    };
    let command = format!("uw rename {dir}{} \"{}\"", args.record, args.new_name);
    println!("  Hepsini değiştirmek için:  {command} --texts all");
    println!("  Seçerek:                   {command} --texts 1,3");
}

/// A hit's text without a translation's `key|` prefix.
fn mention(hit: &Hit) -> &str {
    hit.text.split_once('|').map_or(&hit.text, |(_, text)| text)
}


/// The project folder that holds every rename backup (a dot folder: no command reads it as records).
const BACKUP_DIR: &str = ".uw-backup";
const MANIFEST: &str = "manifest.tsv";

/// Copies every file about to change into `.uw-backup/rename-<time>/<same relative path>` and writes a
/// manifest (`relative path<TAB>fingerprint of the new text`) so `--undo` knows which files are still
/// exactly what the rename wrote. Returns the backup folder.
fn back_up(dir: &Path, planned: &[(PathBuf, String)]) -> Result<PathBuf, String> {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mut folder = dir.join(BACKUP_DIR).join(format!("rename-{seconds}"));
    let mut n = 1;
    while folder.exists() || folder.with_file_name(format!("{}.undone", folder.file_name().and_then(|f| f.to_str()).unwrap_or(""))).exists() {
        n += 1;
        folder = dir.join(BACKUP_DIR).join(format!("rename-{seconds}-{n}"));
    }
    let mut manifest = String::new();
    for (path, renamed) in planned {
        let rel = path.strip_prefix(dir).map_err(|_| format!("{} is outside the project folder", path.display()))?;
        let target = folder.join(rel);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        std::fs::copy(path, &target).map_err(|e| format!("backup {}: {e}", path.display()))?;
        manifest.push_str(&format!("{}\t{:016x}\n", relative(dir, path), fingerprint(renamed.as_bytes())));
    }
    std::fs::write(folder.join(MANIFEST), manifest).map_err(|e| format!("{}: {e}", folder.display()))?;
    Ok(folder)
}

/// `uw rename --undo`: the newest backup not undone yet goes back. A file changed since the rename
/// (its fingerprint differs) is left alone and named, so no later work of the owner is overwritten.
fn undo(dir: &Path) -> Result<(), String> {
    let root = dir.join(BACKUP_DIR);
    let mut backups: Vec<PathBuf> = std::fs::read_dir(&root)
        .map_err(|_| format!("no rename backup in {}", dir.display()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.join(MANIFEST).is_file())
        .filter(|path| !path.to_string_lossy().ends_with(".undone"))
        .collect();
    // Folder names hold the time (`rename-<seconds>[-n]`): the newest sorts last.
    backups.sort_by_key(|path| {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").trim_start_matches("rename-").to_owned();
        let mut parts = name.split('-');
        let seconds: u64 = parts.next().and_then(|t| t.parse().ok()).unwrap_or(0);
        let n: u64 = parts.next().and_then(|t| t.parse().ok()).unwrap_or(1);
        (seconds, n)
    });
    let Some(last) = backups.pop() else {
        return Err(format!("no rename backup left to undo in {}", dir.display()));
    };
    let manifest = std::fs::read_to_string(last.join(MANIFEST)).map_err(|e| format!("{}: {e}", last.display()))?;
    let (mut restored, mut kept) = (0, 0);
    for line in manifest.lines().filter(|line| !line.is_empty()) {
        let Some((rel, wanted)) = line.split_once('\t') else { continue };
        let current = dir.join(rel);
        let original = last.join(rel);
        let now = std::fs::read(&current).map(|bytes| format!("{:016x}", fingerprint(&bytes))).unwrap_or_default();
        if now != wanted {
            println!("kept     {rel}: changed after the rename; its old copy is {}", relative(dir, &original));
            kept += 1;
            continue;
        }
        let bytes = std::fs::read(&original).map_err(|e| format!("{}: {e}", original.display()))?;
        let text = String::from_utf8(bytes).map_err(|_| format!("{}: not UTF-8 text", original.display()))?;
        write_atomic(&current, &text, dir)?;
        restored += 1;
    }
    // Marked as undone (not deleted): the copies stay until the owner removes .uw-backup.
    let name = last.file_name().and_then(|n| n.to_str()).unwrap_or("rename").to_owned();
    let done = last.with_file_name(format!("{name}.undone"));
    std::fs::rename(&last, &done).map_err(|e| format!("{}: {e}", last.display()))?;
    println!("undo     {restored} file(s) restored, {kept} kept (changed since)");
    Ok(())
}

/// FNV-1a 64: a stable fingerprint of a file's bytes (not for security; only "unchanged since").
fn fingerprint(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

/// The name the project currently uses (`""` when unknown). When it already is the new name, the
/// Native display name is the old one — the owner's hand-renamed town whose other texts stayed old.
fn old_name(schema: &Schema, entry: &Entry, own: Option<&str>, new: &str) -> String {
    let native = field_text(&entry.value, schema, "name").unwrap_or_default();
    match own {
        Some(own) if own != new => own.to_owned(),
        Some(own) => {
            if native.is_empty() { own.to_owned() } else { native }
        }
        None => native,
    }
}

/// The record's own `name` field line in its project file: `(path, 1-based line, value)`. `None`
/// when the file or the field is not there (a hand-written project, say).
fn own_name_field(atlas: &Atlas, schema: &Schema, entry: &Entry, dir: &Path) -> Option<(PathBuf, usize, String)> {
    let index = schema.fields.iter().position(|field| field.name == "name")?;
    let field_name = schema.fields.get(index)?.name.clone();
    let path = dir.join(layout::path_for(atlas, schema, entry));
    let text = std::fs::read_to_string(&path).ok()?;
    let bare = record_id(schema, entry);
    let game = entry.id.as_deref().and_then(|id| atlas.game_name(id));
    let mut inside = false;
    for (number, line) in text.lines().enumerate() {
        if inside {
            if line.starts_with('}') {
                inside = false;
                continue;
            }
            let trimmed = line.trim_start();
            let Some(rest) = trimmed.strip_prefix(field_name.as_str()).and_then(|rest| rest.strip_prefix(' ')) else {
                continue;
            };
            return Some((path, number + 1, string_value(rest.trim())?));
        }
        if is_header(line, &schema.kind, &bare, game) {
            inside = true;
        }
    }
    None
}

/// True for the record header line `party praven {` / `party town_6 {` of this record.
fn is_header(line: &str, kind: &str, bare: &str, game: Option<&str>) -> bool {
    let trimmed = line.trim();
    let Some(rest) = trimmed.strip_suffix('{') else { return false };
    let Some(id) = rest.trim_end().strip_prefix(kind).and_then(|rest| rest.strip_prefix(' ')) else {
        return false;
    };
    let id = id.trim().trim_matches('"');
    id == bare || Some(id) == game
}

/// The bare id a record is printed with (`p_town_6` -> `town_6`).
fn record_id(schema: &Schema, entry: &Entry) -> String {
    match entry.id.as_deref().or(entry.name.as_deref()) {
        Some(id) if !id.is_empty() => id.strip_prefix(&schema.id_prefix).unwrap_or(id).to_owned(),
        _ => entry.index.to_string(),
    }
}

/// The string value of one record field (`[id, "Name", ...]` or `{"list": [...]}`), by schema name.
fn field_text(value: &serde_json::Value, schema: &Schema, name: &str) -> Option<String> {
    let index = schema.fields.iter().position(|field| field.name == name)?;
    let list = match value {
        serde_json::Value::Array(values) => values,
        serde_json::Value::Object(map) => map.get("list")?.as_array()?,
        _ => return None,
    };
    list.get(index)?.as_str().map(str::to_owned)
}

/// One quoted string in the source without its quotes; `\"`, `\\` and `\n` are read.
fn string_value(text: &str) -> Option<String> {
    let mut chars = text.strip_prefix('"')?.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next()? {
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

/// Every quoted string of every project `.uw` file that mentions `old`, split into the record's own
/// name field (the hit at `own`) and all other texts.
fn record_hits(dir: &Path, old: &str, own: Option<(&Path, usize)>) -> Result<(Vec<Hit>, Vec<Hit>), String> {
    let mut own_hits = Vec::new();
    let mut other_hits = Vec::new();
    for path in uw_files(dir) {
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let file = relative(dir, &path);
        for (start, end, line, source) in quoted_hits(&text, old) {
            let hit = Hit { path: path.clone(), file: file.clone(), line, start, end, text: source };
            match own {
                Some((own_path, own_line)) if own_path == path && own_line == hit.line => own_hits.push(hit),
                _ => other_hits.push(hit),
            }
        }
    }
    Ok((own_hits, other_hits))
}

/// Every `key|text` line of the project's `languages/**/*.csv` files that mentions `old`.
fn translation_file_hits(dir: &Path, old: &str) -> Result<Vec<Hit>, String> {
    let mut hits = Vec::new();
    let root = dir.join("languages");
    if !root.is_dir() {
        return Ok(hits);
    }
    for path in files_with(&root, "csv") {
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        // A few Native language files are not UTF-8 (es/party_templates.csv): they cannot hold the
        // name as text we could rewrite safely, so they are left alone and named.
        let Ok(text) = String::from_utf8(bytes) else {
            println!("skipped  {} (not UTF-8 text; left unchanged)", relative(dir, &path));
            continue;
        };
        let file = relative(dir, &path);
        for (start, end, line, source) in translation_hits(&text, old) {
            hits.push(Hit { path: path.clone(), file: file.clone(), line, start, end, text: source });
        }
    }
    Ok(hits)
}

/// One `.uw` file's quoted strings that mention `old`: `(start, end, line, source line)`.
fn quoted_hits(text: &str, old: &str) -> Vec<(usize, usize, usize, String)> {
    let mut hits = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut start = 0;
    let mut in_string = false;
    while i < bytes.len() {
        if in_string {
            match bytes[i] {
                b'\\' => i += 2,
                b'"' => {
                    if !crate::find::word_spans(&text[start..i], old, false).is_empty() {
                        let line = line_of(text, start);
                        hits.push((start, i, line, source_line(text, line)));
                    }
                    in_string = false;
                    i += 1;
                }
                _ => i += 1,
            }
            continue;
        }
        // A `//` comment is not record text.
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'/') {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'"' {
            in_string = true;
            start = i + 1;
        }
        i += 1;
    }
    hits
}

/// One `key|text` file's lines whose text mentions `old`: `(start, end, line, source line)`.
fn translation_hits(text: &str, old: &str) -> Vec<(usize, usize, usize, String)> {
    let mut hits = Vec::new();
    let mut offset = 0;
    for (index, line) in text.split_inclusive('\n').enumerate() {
        if let Some(bar) = line.find('|') {
            if !crate::find::word_spans(&line[bar + 1..], old, false).is_empty() {
                let start = offset + bar + 1;
                hits.push((start, offset + line.len(), index + 1, line.trim_end().to_owned()));
            }
        }
        offset += line.len();
    }
    hits
}

/// `text` with the old name replaced by the new one inside each hit's byte range.
fn replace_spans(text: &str, hits: &[&Hit], old: &str, new: &str) -> String {
    let mut spans: Vec<(usize, usize)> = hits.iter().map(|hit| (hit.start, hit.end)).collect();
    spans.sort_unstable();
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for (start, end) in spans {
        let replaced = replace_in(&text[start..end], old, new);
        out.push_str(&text[last..start]);
        out.push_str(&replaced);
        last = end;
    }
    out.push_str(&text[last..]);
    out
}

/// `text` with every whole-word mention of `old` replaced by `new`, matched as `uw find` does.
fn replace_in(text: &str, old: &str, new: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for (start, end) in crate::find::word_spans(text, old, false) {
        out.push_str(&text[last..start]);
        out.push_str(new);
        last = end;
    }
    out.push_str(&text[last..]);
    out
}

/// The 1-based line `index` falls on.
fn line_of(text: &str, index: usize) -> usize {
    text[..index].bytes().filter(|&byte| byte == b'\n').count() + 1
}

fn source_line(text: &str, line: usize) -> String {
    text.lines().nth(line - 1).unwrap_or_default().trim().to_owned()
}

pub(crate) fn relative(dir: &Path, path: &Path) -> String {
    path.strip_prefix(dir).unwrap_or(path).to_string_lossy().replace('\\', "/")
}

fn print_hits(hits: &[Hit]) {
    if hits.is_empty() {
        println!("  (none)");
        return;
    }
    for hit in hits {
        println!("  {}:{}  {}", hit.file, hit.line, hit.text);
    }
}

fn ask(group: &str, count: usize) -> bool {
    print!("{group}: rewrite {count} line(s)? [y/N] ");
    let _ = std::io::stdout().flush();
    let mut answer = String::new();
    if std::io::stdin().lock().read_line(&mut answer).is_err() {
        return false;
    }
    matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

/// Writes `text` next to `path` and renames it over the old file, so a failure leaves it untouched.
/// A path outside the project folder is refused, never written.
fn write_atomic(path: &Path, text: &str, root: &Path) -> Result<(), String> {
    if !path.starts_with(root) {
        return Err(format!("{} is outside the project folder; refusing to write it", path.display()));
    }
    let temp = path.with_extension(format!("uw-rename-{}", std::process::id()));
    std::fs::write(&temp, text).map_err(|e| format!("{}: {e}", temp.display()))?;
    std::fs::rename(&temp, path).map_err(|e| {
        let _ = std::fs::remove_file(&temp);
        format!("{}: {e}", path.display())
    })
}

/// Every `.uw` file under `dir`, in path order.
pub(crate) fn uw_files(dir: &Path) -> Vec<PathBuf> {
    files_with(dir, "uw")
}

/// Every file under `dir` with this extension, in path order.
pub(crate) fn files_with(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return files };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Dot folders (.uw-backup, .git) are never project texts.
            if path.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.')) {
                continue;
            }
            files.extend(files_with(&path, extension));
        } else if path.extension().is_some_and(|ext| ext == extension) {
            files.push(path);
        }
    }
    files.sort();
    files
}

/// True when `word` appears in `text` as a whole word (`Praven` in `Praven Şehri`).
pub(crate) fn contains_word(text: &str, word: &str) -> bool {
    replace_word(text, word, word).is_some()
}

/// `text` with every whole-word `old` replaced by `new`; `None` when there is none.
pub(crate) fn replace_word(text: &str, old: &str, new: &str) -> Option<String> {
    if old.is_empty() {
        return None;
    }
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    let mut found = false;
    for (index, _) in text.match_indices(old) {
        let before = text[..index].chars().next_back().is_none_or(|c| !is_word_char(c));
        let end = index + old.len();
        let after = text[end..].chars().next().is_none_or(|c| !is_word_char(c));
        if before && after {
            out.push_str(&text[last..index]);
            out.push_str(new);
            last = end;
            found = true;
        }
    }
    if !found {
        return None;
    }
    out.push_str(&text[last..]);
    Some(out)
}

/// A character a word continues with; `Praven's` still matches `Praven`.
pub(crate) fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uw-rename-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("menus")).unwrap();
        dir
    }

    #[test]
    fn undo_puts_back_exactly_what_the_rename_changed() {
        let dir = temp("undo");
        let a = dir.join("menus").join("a.uw");
        let b = dir.join("b.uw");
        std::fs::write(&a, "text \"to Praven\"\r\n").unwrap();
        std::fs::write(&b, "name \"Praven\"\n").unwrap();
        let planned = vec![(a.clone(), "text \"to Adam\"\r\n".to_owned()), (b.clone(), "name \"Adam\"\n".to_owned())];
        let backup = back_up(&dir, &planned).unwrap();
        for (path, text) in &planned {
            write_atomic(path, text, &dir).unwrap();
        }
        assert!(backup.join("menus").join("a.uw").is_file());
        undo(&dir).unwrap();
        assert_eq!(std::fs::read(&a).unwrap(), b"text \"to Praven\"\r\n");
        assert_eq!(std::fs::read(&b).unwrap(), b"name \"Praven\"\n");
        // Undone once: nothing left to undo, and the copies are kept (marked, not deleted).
        assert!(undo(&dir).is_err());
        assert!(backup.with_file_name(format!("{}.undone", backup.file_name().unwrap().to_str().unwrap())).is_dir());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn undo_never_overwrites_a_file_changed_after_the_rename() {
        let dir = temp("kept");
        let a = dir.join("a.uw");
        std::fs::write(&a, "name \"Praven\"\n").unwrap();
        let planned = vec![(a.clone(), "name \"Adam\"\n".to_owned())];
        back_up(&dir, &planned).unwrap();
        write_atomic(&a, &planned[0].1, &dir).unwrap();
        // The owner keeps working on the file after the rename.
        std::fs::write(&a, "name \"Adam\"\nlevel 5\n").unwrap();
        undo(&dir).unwrap();
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "name \"Adam\"\nlevel 5\n");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn backups_are_not_read_as_project_texts() {
        let dir = temp("dot");
        std::fs::create_dir_all(dir.join(".uw-backup").join("rename-1")).unwrap();
        std::fs::write(dir.join(".uw-backup").join("rename-1").join("x.uw"), "old").unwrap();
        std::fs::write(dir.join("y.uw"), "new").unwrap();
        let files = uw_files(&dir);
        assert_eq!(files, vec![dir.join("y.uw")]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
