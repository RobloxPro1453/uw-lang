//! `uw find "<text>"` (T-105): every project text that contains a name, so a rename does not leave
//! one behind. Case-insensitive and whole word by default (`Praven` finds `Praven`, not `Pravenia`);
//! `--part` searches inside a word too. Reads the project's `.uw` record texts and its
//! `languages/**/*.csv` translations; `--json` prints one object per hit (`file, line, record,
//! field, text`) for the editor.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
  uw find \"<text>\" [<dir>] [--part] [--json]
      every project text that contains <text>: the .uw record texts and the
      languages/<lang>/*.csv translations; a whole word by default (Praven does not
      match Pravenia), --part also matches inside a word
      --json: one object per hit (file, line, record, field, text)";

/// The longest text a terminal line shows; a cut side gets an ellipsis.
const SNIPPET: usize = 60;

/// What the command line asked for.
struct Args {
    /// The project folder; none means this folder.
    dir: Option<PathBuf>,
    /// The text to look for.
    needle: String,
    /// `--part`: match inside a word too.
    part: bool,
    /// `--json`: one object per hit, no Turkish summary.
    json: bool,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut words: Vec<String> = Vec::new();
    let mut part = false;
    let mut json = false;
    for arg in args {
        match arg.as_str() {
            "--part" => part = true,
            "--json" => json = true,
            other if other.starts_with('-') => return Err(format!("unknown argument `{other}`\n{USAGE}")),
            other => words.push(other.to_owned()),
        }
    }
    let (needle, dir) = match words.as_slice() {
        [needle] => (needle.clone(), None),
        [needle, dir] => (needle.clone(), Some(PathBuf::from(dir))),
        _ => return Err(USAGE.to_owned()),
    };
    if needle.is_empty() {
        return Err(format!("the text to find may not be empty\n{USAGE}"));
    }
    Ok(Args { dir, needle, part, json })
}

pub fn cmd(args: &[String]) -> ExitCode {
    let args = match parse(args) {
        Ok(args) => args,
        Err(message) => return crate::fail(&message),
    };
    let dir = match resolve(args.dir.as_deref().unwrap_or(Path::new("."))) {
        Ok(dir) => dir,
        Err(message) => return crate::fail(&message),
    };
    match run(&dir, &args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => crate::fail(&message),
    }
}

/// The project folder made absolute (find needs no atlas, so the working folder may stay anywhere).
fn resolve(dir: &Path) -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let dir = if dir.is_absolute() { dir.to_path_buf() } else { cwd.join(dir) };
    if !dir.is_dir() {
        return Err(format!("{} is not a folder", dir.display()));
    }
    Ok(crate::plain(dir.canonicalize().unwrap_or(dir)))
}

fn run(dir: &Path, args: &Args) -> Result<(), String> {
    let mut uw = Vec::new();
    for path in crate::rename::uw_files(dir) {
        let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let file = crate::rename::relative(dir, &path);
        uw.extend(uw_hits(&text, &file, &args.needle, args.part));
    }
    let mut languages = Vec::new();
    let root = dir.join("languages");
    if root.is_dir() {
        for path in crate::rename::files_with(&root, "csv") {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            // A language file that is not UTF-8 is skipped, as `uw rename` does.
            let Ok(text) = String::from_utf8(bytes) else { continue };
            let file = crate::rename::relative(dir, &path);
            languages.extend(csv_hits(&text, &file, &args.needle, args.part));
        }
    }
    for hit in uw.iter().chain(&languages) {
        if args.json {
            println!(
                "{}",
                serde_json::json!({
                    "file": hit.file,
                    "line": hit.line,
                    "record": hit.record(),
                    "field": hit.field,
                    "text": hit.text,
                })
            );
        } else {
            println!("{:<32}  {:<24}  {:<8}  \"{}\"", hit.where_(), hit.record(), hit.field, hit.shown);
        }
    }
    if !args.json {
        if languages.is_empty() {
            println!("{} yer bulundu.", uw.len());
        } else {
            println!("{} + {} yer bulundu.", uw.len(), languages.len());
        }
    }
    Ok(())
}

/// One found text: where it is, which record and field hold it, and how it reads.
struct Hit {
    file: String,
    line: usize,
    /// The top-level record kind (`party`); empty for a translation row.
    kind: String,
    /// The record's own id (`town_1`); a translation row's key.
    id: String,
    field: String,
    /// The whole text, without its quotes.
    text: String,
    /// [`Hit::text`] shortened around the match, for the terminal.
    shown: String,
}

impl Hit {
    /// `file:line` as printed.
    fn where_(&self) -> String {
        format!("{}:{}", self.file, self.line)
    }

    /// `kind id`, or just the key for a translation row.
    fn record(&self) -> String {
        if self.kind.is_empty() {
            self.id.clone()
        } else {
            format!("{} {}", self.kind, self.id)
        }
    }
}

/// One `.uw` file's matching quoted strings, each with its record and its field.
fn uw_hits(text: &str, file: &str, needle: &str, part: bool) -> Vec<Hit> {
    let lines: Vec<&str> = text.lines().collect();
    let mut hits = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        // Between records there are only blank lines and comments.
        if lines[index].trim().is_empty() || lines[index].trim_start().starts_with("//") {
            index += 1;
            continue;
        }
        let start = index;
        let header = header(lines[start]);
        let mut depth = 0i32;
        let mut end = start;
        while end < lines.len() {
            count_braces(lines[end], &mut depth);
            end += 1;
            if depth <= 0 {
                break;
            }
        }
        let kind = header.as_ref().map(|(kind, _)| kind.clone()).unwrap_or_default();
        let id = record_id(&lines[start..end]).or_else(|| header.map(|(_, id)| id)).unwrap_or_default();
        for (offset, line) in lines[start..end].iter().enumerate() {
            let field = field_of(line);
            for quoted in quoted_strings(line) {
                let Some((from, to)) = found(&quoted, needle, part) else { continue };
                hits.push(Hit {
                    file: file.to_owned(),
                    line: start + offset + 1,
                    kind: kind.clone(),
                    id: id.clone(),
                    field: field.clone(),
                    shown: around(&quoted, from, to, SNIPPET),
                    text: quoted,
                });
            }
        }
        index = end;
    }
    hits
}

/// One `key|text` line's matching text; the key names the record, the row is the field's text.
fn csv_hits(text: &str, file: &str, needle: &str, part: bool) -> Vec<Hit> {
    let mut hits = Vec::new();
    for (index, line) in text.split_inclusive('\n').enumerate() {
        let Some(bar) = line.find('|') else { continue };
        let key = line[..bar].trim();
        let value = line[bar + 1..].trim_end_matches(['\r', '\n']);
        let Some((from, to)) = found(value, needle, part) else { continue };
        hits.push(Hit {
            file: file.to_owned(),
            line: index + 1,
            kind: String::new(),
            id: key.to_owned(),
            field: "text".to_owned(),
            shown: around(value, from, to, SNIPPET),
            text: value.to_owned(),
        });
    }
    hits
}

/// The `<kind> <id> {` of a record header line.
fn header(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") {
        return None;
    }
    let brace = trimmed.find('{')?;
    let before = trimmed[..brace].trim_end();
    let mut words = before.splitn(2, char::is_whitespace);
    let kind = words.next()?.trim();
    let id = words.next()?.trim().trim_matches('"');
    if kind.is_empty() || id.is_empty() {
        return None;
    }
    Some((kind.to_owned(), id.to_owned()))
}

/// A record's own id from its `id <value>` field (`town_6`), else none.
fn record_id(lines: &[&str]) -> Option<String> {
    for line in lines {
        let Some(rest) = line.trim_start().strip_prefix("id ") else { continue };
        if let Some(text) = quoted_strings(line).into_iter().next() {
            return Some(text);
        }
        let value = rest.trim().trim_matches('"');
        if !value.is_empty() {
            return Some(value.to_owned());
        }
    }
    None
}

/// The field a quoted string sits in: the word before the line's first quote (`name "…"`).
fn field_of(line: &str) -> String {
    let Some(quote) = line.find('"') else { return String::new() };
    line[..quote].trim_end().split_whitespace().last().unwrap_or_default().to_owned()
}

/// Every `"…"` string on one line, unescaped, stopping at a `//` comment.
fn quoted_strings(line: &str) -> Vec<String> {
    let chars: Vec<char> = line.chars().collect();
    let mut strings = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '/' if chars.get(i + 1) == Some(&'/') => break,
            '"' => {
                let mut out = String::new();
                i += 1;
                while i < chars.len() {
                    match chars[i] {
                        '\\' => {
                            match chars.get(i + 1) {
                                Some('"') => out.push('"'),
                                Some('\\') => out.push('\\'),
                                Some('n') => out.push('\n'),
                                Some(&other) => out.push(other),
                                None => {}
                            }
                            i += 2;
                        }
                        '"' => {
                            i += 1;
                            break;
                        }
                        c => {
                            out.push(c);
                            i += 1;
                        }
                    }
                }
                strings.push(out);
            }
            _ => i += 1,
        }
    }
    strings
}

/// Counts the `{` and `}` of one line, ignoring the ones in a string or a `//` comment.
fn count_braces(line: &str, depth: &mut i32) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '/' if chars.get(i + 1) == Some(&'/') => break,
            '"' => {
                i += 1;
                while i < chars.len() {
                    match chars[i] {
                        '\\' => i += 2,
                        '"' => {
                            i += 1;
                            break;
                        }
                        _ => i += 1,
                    }
                }
            }
            '{' => {
                *depth += 1;
                i += 1;
            }
            '}' => {
                *depth -= 1;
                i += 1;
            }
            _ => i += 1,
        }
    }
}

/// The first occurrence of `needle` in `text` as character indices, whole word unless `part`.
fn found(text: &str, needle: &str, part: bool) -> Option<(usize, usize)> {
    // The lower-case copy plus, per byte, the original character index it came from (a lower-case
    // character may grow, e.g. `İ`), so the match maps back to the real characters.
    let mut lower = String::with_capacity(text.len());
    let mut origin: Vec<usize> = Vec::with_capacity(text.len());
    for (index, c) in text.chars().enumerate() {
        for lower_char in c.to_lowercase() {
            lower.push(lower_char);
            origin.extend(std::iter::repeat(index).take(lower_char.len_utf8()));
        }
    }
    let lowered = needle.to_lowercase();
    if lowered.is_empty() {
        return None;
    }
    let chars: Vec<char> = text.chars().collect();
    let mut from = 0;
    while let Some(at) = lower[from..].find(&lowered) {
        let start_byte = from + at;
        let end_byte = start_byte + lowered.len();
        let start = origin[start_byte];
        let end = origin[end_byte - 1] + 1;
        let before = start.checked_sub(1).and_then(|i| chars.get(i)).is_none_or(|&c| !crate::rename::is_word_char(c));
        let after = chars.get(end).is_none_or(|&c| !crate::rename::is_word_char(c));
        if part || (before && after) {
            return Some((start, end));
        }
        from = end_byte;
    }
    None
}

/// `text` around the match `[start, end)` (character indices), at most `max` chars, with an
/// ellipsis where a side was cut.
fn around(text: &str, start: usize, end: usize, max: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max {
        return text.to_owned();
    }
    let room = max.saturating_sub(end - start);
    let mut from = start.saturating_sub(room / 2);
    let to = (from + max).min(chars.len());
    from = to.saturating_sub(max);
    let mut out = String::new();
    if from > 0 {
        out.push('…');
    }
    out.extend(&chars[from..to]);
    if to < chars.len() {
        out.push('…');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whole_word_ignores_a_longer_word_but_part_takes_it() {
        assert_eq!(found("Merchant of Praven", "praven", false), Some((12, 18)));
        assert_eq!(found("Pravenia is elsewhere", "praven", false), None);
        assert_eq!(found("Pravenia is elsewhere", "praven", true), Some((0, 6)));
        // A translation keeps its suffix and its apostrophe.
        assert_eq!(found("Praven'e katıl", "PRAVEN", false), Some((0, 6)));
    }

    #[test]
    fn a_long_text_is_shortened_around_the_match() {
        let text = "join a caravan to Praven, in the Kingdom of Swadia and far beyond it";
        let (from, to) = found(text, "Praven", false).unwrap();
        let shown = around(text, from, to, 30);
        assert!(shown.contains("Praven"), "{shown}");
        assert!(shown.starts_with('…'), "{shown}");
        assert!(shown.ends_with('…'), "{shown}");
    }
}
