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
  uw rename <record> \"<new name>\" [--list] [--yes] [--atlas <atlas.json>]
      rename a record and every text that mentions it: its own name, the project's other record
      texts and its languages/<lang>/*.csv translations; the old name is replaced inside each
      text, so \"Praven Şehri\" becomes \"Adam Siken Şehri\"
      --list: only list what would change; --yes: write every group without asking
  uw rename <dir> <record> \"<new name>\" [--list] [--yes] [--atlas <atlas.json>]
      the same, for the project folder <dir>";

/// What the command line asked for.
struct Args {
    /// The project folder; none means this folder.
    dir: Option<PathBuf>,
    /// The record to rename: its `names.uw` game name (`praven`) or its kit reference (`p_town_6`).
    record: String,
    new_name: String,
    list: bool,
    yes: bool,
    atlas: Option<PathBuf>,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut words: Vec<String> = Vec::new();
    let mut list = false;
    let mut yes = false;
    let mut atlas = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--list" => list = true,
            "--yes" => yes = true,
            "--atlas" => match it.next() {
                Some(path) => atlas = Some(PathBuf::from(path)),
                None => return Err(format!("--atlas needs a path\n{USAGE}")),
            },
            other if other.starts_with('-') => return Err(format!("unknown argument `{other}`\n{USAGE}")),
            other => words.push(other.to_owned()),
        }
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
    Ok(Args { dir, record, new_name, list, yes, atlas })
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
    let dir = dir.canonicalize().unwrap_or(dir);
    match run(&dir, &args) {
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
    if args.list {
        println!("--list: nothing written");
        return Ok(());
    }
    let mut written: BTreeMap<PathBuf, Vec<&Hit>> = BTreeMap::new();
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
    if written.is_empty() {
        println!("nothing written");
        return Ok(());
    }
    for (path, hits) in &written {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let renamed = replace_spans(&text, hits, &old, &args.new_name);
        write_atomic(path, &renamed, dir)?;
    }
    let lines: usize = written.values().map(Vec::len).sum();
    println!("wrote {} line(s) in {} file(s)", lines, written.len());
    Ok(())
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
        let text = String::from_utf8(bytes).map_err(|_| format!("{}: not UTF-8 text", path.display()))?;
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
                    if contains_word(&text[start..i], old) {
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
            if contains_word(&line[bar + 1..], old) {
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
        let replaced = replace_word(&text[start..end], old, new).unwrap_or_else(|| text[start..end].to_owned());
        out.push_str(&text[last..start]);
        out.push_str(&replaced);
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

fn relative(dir: &Path, path: &Path) -> String {
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
fn uw_files(dir: &Path) -> Vec<PathBuf> {
    files_with(dir, "uw")
}

/// Every file under `dir` with this extension, in path order.
fn files_with(dir: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return files };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
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
