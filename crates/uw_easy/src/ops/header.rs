//! Rebuild the atlas `ops` array from the kit's own `header_operations.py`.
//!
//! `py/atlas_dump.py` cannot run in every lane (it needs the kit's Python), so when
//! `target/uw-staging/atlas.json` is missing this module reads the same facts out of the kit
//! header: the declarations (name, code, line), the trailing comment (the signature/doc) and the
//! three name lists `lhs_operations`, `global_lhs_operations`, `can_fail_operations`.
//!
//! The parsing mirrors `py/atlas_dump.py`: `ASSIGN_RE` only accepts a literal code, so a line such
//! as `lt = neg | ge` is not an operation and is skipped, exactly as the dump skips it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde_json::json;
use uw_atlas::{Atlas, Error};

const LISTS: [&str; 3] = ["lhs_operations", "global_lhs_operations", "can_fail_operations"];

/// One `name = code` declaration of the header.
struct Decl {
    name: String,
    code: u64,
    doc: String,
    line: u64,
}

/// Reads `header_operations.py` and turns it into an [`Atlas`] whose `ops` are complete
/// (`name`, `code`, `line`, `signature`, `lhs`, `can_fail`, `global_lhs`); hooks, ranges,
/// constants and tables stay empty.
pub fn atlas_from_header(path: impl AsRef<Path>) -> Result<Atlas, Error> {
    let path = path.as_ref();
    let bytes = std::fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes);
    let mut decls: Vec<Decl> = Vec::new();
    let mut values: BTreeMap<String, u64> = BTreeMap::new();
    let mut lists: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut open: Option<(String, BTreeSet<String>)> = None;
    for (index, raw) in text.lines().enumerate() {
        let line = index as u64 + 1;
        let trimmed = raw.trim();
        match open.take() {
            Some((name, mut names)) => {
                let (body, closed) = match trimmed.split_once(']') {
                    Some((body, _)) => (body, true),
                    None => (trimmed, false),
                };
                push_names(&mut names, body);
                if closed {
                    lists.insert(name, names);
                } else {
                    open = Some((name, names));
                }
            }
            None => {
                let (head, tail) = match raw.split_once('=') {
                    Some(pair) => pair,
                    None => continue,
                };
                let append = head.trim_end().ends_with('+');
                let head = head.trim().trim_end_matches('+').trim();
                if LISTS.contains(&head) && tail.trim_start().starts_with('[') {
                    let mut names = if append {
                        lists.remove(head).unwrap_or_default()
                    } else {
                        BTreeSet::new()
                    };
                    let body = tail.trim_start().trim_start_matches('[');
                    let (body, closed) = match body.split_once(']') {
                        Some((body, _)) => (body, true),
                        None => (body, false),
                    };
                    push_names(&mut names, body);
                    if closed {
                        lists.insert(head.to_string(), names);
                    } else {
                        open = Some((head.to_string(), names));
                    }
                    continue;
                }
                let Some((name, code, rest)) = declaration(raw) else { continue };
                values.insert(name.clone(), code);
                decls.push(Decl { name, code, doc: doc_of(rest), line });
            }
        }
    }
    let codes = |list: &str| -> BTreeSet<u64> {
        lists
            .get(list)
            .into_iter()
            .flatten()
            .filter_map(|name| values.get(name).copied())
            .collect()
    };
    let lhs = codes("lhs_operations");
    let global_lhs = codes("global_lhs_operations");
    let can_fail = codes("can_fail_operations");
    let ops: Vec<_> = decls
        .iter()
        .map(|decl| {
            json!({
                "name": decl.name,
                "code": decl.code,
                "line": decl.line,
                "signature": decl.doc,
                "lhs": lhs.contains(&decl.code),
                "can_fail": can_fail.contains(&decl.code),
                "global_lhs": global_lhs.contains(&decl.code),
            })
        })
        .collect();
    let atlas = json!({
        "schema": 1,
        "kit_hash": null,
        "ops": ops,
        "hooks": [],
        "ranges": [],
        "constants": [],
        "tables": {},
    });
    Atlas::from_str(&atlas.to_string())
}

/// Adds the comma separated operation names of one list chunk.
fn push_names(names: &mut BTreeSet<String>, chunk: &str) {
    for part in chunk.split(',') {
        let name = part.trim().trim_matches(']').trim();
        if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            names.insert(name.to_string());
        }
    }
}

/// `^([A-Za-z_]\w*)\s*=\s*(0x[0-9a-fA-F]+|\d+)\b(.*)$` — `py/atlas_dump.py`'s `ASSIGN_RE`.
fn declaration(line: &str) -> Option<(String, u64, &str)> {
    let bytes = line.as_bytes();
    if bytes.is_empty() || !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return None;
    }
    let mut end = 1;
    while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
        end += 1;
    }
    let name = line[..end].to_string();
    let rest = line[end..].trim_start();
    let rest = rest.strip_prefix('=')?.trim_start();
    let (digits, radix, prefix) = match rest.get(..2) {
        Some("0x") | Some("0X") => (rest, 16, 2),
        _ => (rest, 10, 0),
    };
    let digits_len = digits[prefix..]
        .find(|c: char| !c.is_digit(radix))
        .unwrap_or(digits.len() - prefix);
    if digits_len == 0 {
        return None;
    }
    let code = u64::from_str_radix(&digits[prefix..prefix + digits_len], radix).ok()?;
    let rest = &digits[prefix + digits_len..];
    // `\b`: the literal must end on a word boundary.
    if rest.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((name, code, rest))
}

/// The header's `# ...` remainder, which carries the signature and the doc; exactly what the dump
/// keeps as `signature` (a declaration without a `#` comment has none).
fn doc_of(rest: &str) -> String {
    match rest.trim().strip_prefix('#') {
        Some(text) => text.trim().to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::declaration;

    #[test]
    fn declarations_are_the_dumps() {
        let (name, code, rest) = declaration("try_for_range  = 6 # Works like").unwrap();
        assert_eq!((name.as_str(), code, rest), ("try_for_range", 6, " # Works like"));
        let (_, code, _) = declaration("neg          = 0x80000000\t # (neg|<operation>),").unwrap();
        assert_eq!(code, 0x8000_0000);
        // A computed code is not an operation (py/atlas_dump.py ASSIGN_RE skips it).
        assert!(declaration("lt           = neg | ge # less than").is_none());
        assert!(declaration("##477 used for: server_set_anti_cheat = 477 # text").is_none());
        assert!(declaration("    key_id,").is_none());
    }

    #[test]
    fn plus_equals_appends_to_operation_lists() {
        let dir = std::env::temp_dir().join(format!("uw-header-plus-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("header_operations.py");
        std::fs::write(
            &file,
            "op_a = 1 # (op_a, <dest>),\nop_b = 2 # (op_b, <dest>),\nlhs_operations = [op_a]\nlhs_operations += [\n  op_b,\n]\n",
        )
        .unwrap();
        let atlas = super::atlas_from_header(&file).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(atlas.op("op_a").unwrap().lhs);
        assert!(atlas.op("op_b").unwrap().lhs);
    }
}
