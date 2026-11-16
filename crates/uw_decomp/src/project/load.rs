//! P3b: a `uw new` project folder read back into whole tables, in `uw.lock` order. Filled by T-056.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use uw_atlas::Atlas;

use crate::records::Schema;

/// The lock file in the project root (next to `uw.toml`).
pub(super) const LOCK_FILE: &str = "uw.lock";

/// `uw.lock`: the kit the project was made from, and every table's record ids in atlas order.
#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct Lock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kit_hash: Option<String>,
    #[serde(default)]
    tables: BTreeMap<String, Vec<String>>,
}

/// Writes `uw.lock` into `out`: `kit_hash` and, per table, the record ids (`record_id`) in atlas
/// order. The order is the whole point: it is what the kit numbers records with.
pub(super) fn write_lock(atlas: &Atlas, schemas: &[Schema], out: &Path) -> Result<(), String> {
    let mut lock = Lock { kit_hash: atlas.kit_hash.clone(), tables: BTreeMap::new() };
    for schema in schemas {
        let Some(table) = atlas.tables.get(&schema.table) else { continue };
        let ids = table.entries.iter().map(|entry| super::record_id(schema, entry)).collect();
        lock.tables.insert(schema.table.clone(), ids);
    }
    let body = toml::to_string(&lock).map_err(|e| format!("{}: {e}", out.join(LOCK_FILE).display()))?;
    let path = out.join(LOCK_FILE);
    let text = format!(
        "# uw.lock: every table's record ids in atlas order (P3b).\n\
         # A record's position never moves; load reads the order from here, not from the files.\n\
         {body}"
    );
    std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// One record read from a project file, with where it came from (for order and error messages).
struct Parsed {
    table: String,
    id: String,
    value: Json,
    file: PathBuf,
    /// Global read position: file-path order, then text order inside the file.
    seq: usize,
}

/// Every table's record values, in the order the kit will number them: the `uw.lock` order first,
/// then records that are new in the project (appended, never inserted). `parse` reads one record's
/// text (records::parse_with_blocks). Errors are one line each, with `file:` and the record id.
pub fn load(
    atlas: &Atlas,
    schemas: &[Schema],
    dir: &Path,
    parse: &mut dyn FnMut(&Schema, &str) -> Result<(String, Json), String>,
) -> Result<BTreeMap<String, Vec<Json>>, Vec<String>> {
    let lock_path = dir.join(LOCK_FILE);
    let text = std::fs::read_to_string(&lock_path).map_err(|e| vec![format!("{}: {e}", lock_path.display())])?;
    let lock: Lock = toml::from_str(&text).map_err(|e| vec![format!("{}: {e}", lock_path.display())])?;
    if lock.kit_hash != atlas.kit_hash {
        let found = lock.kit_hash.as_deref().unwrap_or("<yok>");
        let expected = atlas.kit_hash.as_deref().unwrap_or("<yok>");
        return Err(vec![format!(
            "{}: kit_hash {found}, atlas {expected}: kilit başka bir kite ait; sessiz yeniden sıralama yok",
            lock_path.display()
        )]);
    }

    let mut errors = Vec::new();
    let by_kind: HashMap<&str, &Schema> = schemas.iter().map(|schema| (schema.kind.as_str(), schema)).collect();
    let mut files = Vec::new();
    collect_uw_files(dir, &mut files);
    files.sort();
    let mut records: Vec<Parsed> = Vec::new();
    let mut seq = 0usize;
    for file in &files {
        let text = match std::fs::read_to_string(file) {
            Ok(text) => text,
            Err(e) => {
                errors.push(format!("{}: {e}", file.display()));
                continue;
            }
        };
        for (start, record) in super::split_records(&text) {
            // The record word (`faction`, `troop`, …) names its table exactly; prefixes collide
            // (`party` / `party_template`), so this is a whole-word match.
            let kind = record.trim_start().split_whitespace().next().unwrap_or("");
            let Some(schema) = by_kind.get(kind) else {
                errors.push(format!("{}: line {start}: `{kind}` kayıt türü tanınmıyor", file.display()));
                continue;
            };
            match parse(schema, &record) {
                Ok((id, value)) => {
                    records.push(Parsed { table: schema.table.clone(), id, value, file: file.clone(), seq });
                    seq += 1;
                }
                Err(e) => errors.push(format!("{}: {}", file.display(), at_line(start, &e))),
            }
        }
    }

    let mut by_table: BTreeMap<String, Vec<Parsed>> = BTreeMap::new();
    for parsed in records {
        by_table.entry(parsed.table.clone()).or_default().push(parsed);
    }

    let mut result: BTreeMap<String, Vec<Json>> = BTreeMap::new();
    for schema in schemas {
        let table_records = by_table.remove(&schema.table).unwrap_or_default();
        let lock_ids = lock.tables.get(&schema.table).cloned().unwrap_or_default();
        // The atlas value at each id occurrence: duplicate ids (duplicate names do repeat) need it
        // to pick the occurrence the lock means, since the id alone cannot tell them apart.
        let mut expected: HashMap<String, Vec<&Json>> = HashMap::new();
        if let Some(table) = atlas.tables.get(&schema.table) {
            for entry in &table.entries {
                expected.entry(super::record_id(schema, entry)).or_default().push(&entry.value);
            }
        }

        let mut used = vec![false; table_records.len()];
        let mut ordered: Vec<Json> = Vec::new();
        let mut seen: HashMap<String, usize> = HashMap::new();
        for id in &lock_ids {
            let occurrence = *seen.get(id).unwrap_or(&0);
            seen.insert(id.clone(), occurrence + 1);
            let candidates: Vec<usize> = (0..table_records.len())
                .filter(|&i| !used[i] && table_records[i].id == *id)
                .collect();
            if candidates.is_empty() {
                errors.push(format!(
                    "{} {id}: kayıt silinmiş; silme henüz yok (P3b v1), kaydı geri koy",
                    schema.table
                ));
                continue;
            }
            let wanted = expected.get(id).and_then(|values| values.get(occurrence));
            let chosen = candidates
                .iter()
                .copied()
                .find(|&i| wanted.is_some_and(|value| *value == &table_records[i].value))
                .unwrap_or(candidates[0]);
            used[chosen] = true;
            ordered.push(table_records[chosen].value.clone());
        }

        // Records the lock does not name are new; append them in file-path then text order. More
        // records with one id than the lock lists is a duplicate, not a new record.
        let mut remaining: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, _) in table_records.iter().enumerate() {
            if !used[i] {
                remaining.entry(table_records[i].id.clone()).or_default().push(i);
            }
        }
        let mut appended: Vec<usize> = Vec::new();
        for (id, indexes) in &remaining {
            let locked = lock_ids.iter().filter(|known| *known == id).count();
            if locked > 0 {
                let extra = indexes[0];
                let other = (0..table_records.len())
                    .find(|&i| used[i] && table_records[i].id == *id)
                    .unwrap_or(extra);
                errors.push(format!(
                    "{} {id}: iki kayıt aynı id: {} ve {}",
                    schema.table,
                    table_records[extra].file.display(),
                    table_records[other].file.display()
                ));
            } else if indexes.len() > 1 {
                errors.push(format!(
                    "{} {id}: iki kayıt aynı id: {} ve {}",
                    schema.table,
                    table_records[indexes[0]].file.display(),
                    table_records[indexes[1]].file.display()
                ));
            } else {
                appended.push(indexes[0]);
            }
        }
        appended.sort_by_key(|&i| table_records[i].seq);
        for i in appended {
            ordered.push(table_records[i].value.clone());
        }
        result.insert(schema.table.clone(), ordered);
    }

    if errors.is_empty() {
        Ok(result)
    } else {
        Err(errors)
    }
}

/// Moves a record-relative `line N: message` to the file's own line (the record starts at `start`).
fn at_line(start: usize, message: &str) -> String {
    let Some(rest) = message.strip_prefix("line ") else { return message.to_string() };
    let Some((line, tail)) = rest.split_once(':') else { return message.to_string() };
    let Ok(line) = line.parse::<usize>() else { return message.to_string() };
    format!("line {}:{tail}", start + line - 1)
}

/// Every `.uw` file under `dir`, so a user may move records between folders. `names.uw` and the
/// `fns/` words folder hold no records (T-084).
fn collect_uw_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().and_then(|name| name.to_str()) != Some("fns") {
                collect_uw_files(&path, files);
            }
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("uw")
            && path.file_name().and_then(|name| name.to_str()) != Some(crate::names::FILE)
        {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_keeps_its_first_file_line_and_errors_shift_to_it() {
        let records = super::super::split_records("// header\n\nfaction a {}\nfaction b {\n  x\n}\n");
        assert_eq!(records[0].0, 3);
        assert_eq!(records[1].0, 4);
        assert_eq!(at_line(10, "line 3: expected `}`"), "line 12: expected `}`");
        assert_eq!(at_line(10, "line 3:4: error[E0001]: unknown name"), "line 12:4: error[E0001]: unknown name");
        assert_eq!(at_line(10, "read error"), "read error");
    }

    #[test]
    fn the_fns_folder_is_not_read_as_records() {
        let dir = std::env::temp_dir().join(format!("uw-load-fns-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("fns")).unwrap();
        std::fs::write(dir.join("fns").join("mine.uw"), "fn give_gold() {}\n").unwrap();
        std::fs::write(dir.join("record.uw"), "faction x {}\n").unwrap();
        std::fs::write(dir.join(crate::names::FILE), "x = y\n").unwrap();
        let mut files = Vec::new();
        collect_uw_files(&dir, &mut files);
        assert_eq!(files, vec![dir.join("record.uw")]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
