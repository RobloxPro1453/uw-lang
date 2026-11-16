//! Game names (P5b): the names a modder writes for Native records — `swadia`, `praven`,
//! `count_klargus` — instead of kit references (`fac_kingdom_1`, `p_town_6`, `trp_knight_1_1`).
//!
//! A project keeps them in one file, `names.uw`, one `game_name = kit_reference` line each. `uw new`
//! derives the file from the kit's own display names ([`derive`], the rule is printed at the top of
//! the file); the modder may rename any line. [`Atlas::set_game_names`] installs the pairs: the checker
//! then accepts the game name wherever the reference was accepted, and the printers write it.
//!
//! Nothing is invented: every name comes from the record's display name in the kit. A name that
//! would be ambiguous (two records, an operation, a constant, a reference) is left out, and that
//! record keeps its kit reference.

use std::collections::{BTreeMap, HashMap, HashSet};

use uw_atlas::Atlas;

/// The project file holding the names.
pub const FILE: &str = "names.uw";

/// Tables whose records get a game name, in file order. Their display name is the 2nd tuple item.
pub const TABLES: &[&str] = &["factions", "parties", "troops", "items", "party_templates"];

/// Words the language keeps for itself; a game name may not be one of them.
const RESERVED: &[&str] = &[
    "if", "elif", "else", "while", "for", "in", "range", "and", "or", "not", "require", "true",
    "false", "menu", "option", "dialog", "script", "trigger", "ops", "id", "name", "native", "player",
    "save", "say", "every", "on", "area", "like", "end", "return", "break", "continue", "goto",
];

/// One game name and the kit reference it stands for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameName {
    pub name: String,
    pub reference: String,
    /// The table of the record (`factions`, ...), for grouping in the file.
    pub table: String,
}

/// The rule written at the top of `names.uw` (and the one [`derive`] follows).
pub const RULE: &str = "\
// names.uw: the names you write for Native records, `game_name = kit_reference`.
// Use the left side anywhere: in records (`faction swadia`) and in code (`praven`).
// Rule used by `uw new`: the record's name as shown in the game, lower case, every run of
// other characters turned into `_` (\"Count Klargus\" -> count_klargus); factions drop a
// leading \"Kingdom of\" / \"The\". Left out (the record keeps its kit reference): names the
// kit marks as never shown ({!}..., \"Error - ...\"), that hold placeholders ({s1}), are not plain ASCII, start with a digit,
// are shared by two records, or equal a language word, an operation, a constant or a reference.
// You may rename any line; every name must stay unique. Delete a line to use the kit reference.
";

/// The game names `uw new` writes for this atlas, in [`TABLES`] order, then record order.
pub fn derive(atlas: &Atlas) -> Vec<GameName> {
    let mut candidates: Vec<GameName> = Vec::new();
    for &table in TABLES {
        let Some(entries) = atlas.tables.get(table).map(|t| &t.entries) else { continue };
        for entry in entries {
            let Some(reference) = entry.id.as_deref() else { continue };
            // Only a reference the kit reads back as this very record (first match wins in the kit).
            match atlas.resolve_ref(reference) {
                Some((_, found)) if found.index == entry.index => {}
                _ => continue,
            }
            let Some(display) = display_name(&entry.value) else { continue };
            let Some(name) = slug(table, display) else { continue };
            candidates.push(GameName { name, reference: reference.to_string(), table: table.to_string() });
        }
    }
    let mut count: HashMap<&str, usize> = HashMap::new();
    for candidate in &candidates {
        *count.entry(candidate.name.as_str()).or_default() += 1;
    }
    let shared: HashSet<String> =
        count.into_iter().filter(|(_, n)| *n > 1).map(|(name, _)| name.to_string()).collect();
    candidates
        .into_iter()
        .filter(|c| !shared.contains(&c.name) && taken_by_kit(atlas, &c.name).is_none())
        .collect()
}

/// Why `name` cannot be a game name in this atlas (it already means something), if it cannot.
pub fn taken_by_kit(atlas: &Atlas, name: &str) -> Option<&'static str> {
    if RESERVED.contains(&name) {
        Some("a language word")
    } else if atlas.op(name).is_some() {
        Some("an operation")
    } else if atlas.constant(name).is_some() {
        Some("a constant")
    } else if atlas.entry_by_id(name).is_some() || atlas.reference_for(name).is_none() && atlas.resolve_ref(name).is_some() {
        Some("a kit reference")
    } else {
        None
    }
}

/// The display name of a record value (`[id, "Name", ...]` or `{"list": [id, "Name", ...]}`).
fn display_name(value: &serde_json::Value) -> Option<&str> {
    let list = match value.get("list") {
        Some(list) => list.as_array()?,
        None => value.as_array()?,
    };
    let display = list.get(1)?.as_str()?;
    // The kit's own marks for names the game never shows.
    if display.starts_with("{!}") || display.starts_with("Error") {
        return None;
    }
    Some(display)
}

/// The rule of [`RULE`]: `"Count Klargus"` -> `count_klargus`; `None` when left out.
pub fn slug(table: &str, display: &str) -> Option<String> {
    if display.contains('{') || !display.is_ascii() {
        return None;
    }
    let mut out = String::new();
    for ch in display.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('_') {
            out.push('_');
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if table == "factions" {
        for prefix in ["the_", "kingdom_of_"] {
            if let Some(rest) = out.strip_prefix(prefix) {
                out = rest.to_string();
            }
        }
    }
    if out.is_empty() || out.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    Some(out)
}

/// `names.uw` text: [`RULE`], then one block per table.
pub fn print(names: &[GameName]) -> String {
    let mut out = String::from(RULE);
    let mut by_table: BTreeMap<usize, Vec<&GameName>> = BTreeMap::new();
    for name in names {
        let order = TABLES.iter().position(|t| *t == name.table).unwrap_or(TABLES.len());
        by_table.entry(order).or_default().push(name);
    }
    for (order, names) in by_table {
        let table = TABLES.get(order).copied().unwrap_or("other");
        out.push_str(&format!("\n// {table}\n"));
        for name in names {
            out.push_str(&format!("{} = {}\n", name.name, name.reference));
        }
    }
    out
}

/// Reads `names.uw` text and checks it against the atlas (which must not have names installed yet).
/// Errors are one line each, `line N: ...`, in Turkish (the modder reads them).
pub fn parse(atlas: &Atlas, text: &str) -> Result<Vec<GameName>, Vec<String>> {
    let mut names = Vec::new();
    let mut errors = Vec::new();
    let mut seen_name: HashMap<String, usize> = HashMap::new();
    let mut seen_ref: HashMap<String, usize> = HashMap::new();
    for (index, raw) in text.lines().enumerate() {
        let number = index + 1;
        let line = raw.split("//").next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((name, reference)) = line.split_once('=') else {
            errors.push(format!("satır {number}: `ad = kit_kimliği` bekleniyordu: `{line}`"));
            continue;
        };
        let (name, reference) = (name.trim(), reference.trim());
        if !is_name(name) {
            errors.push(format!(
                "satır {number}: `{name}` ad olamaz (küçük harf, rakam ve `_`; rakamla başlamaz)"
            ));
            continue;
        }
        let table = match atlas.resolve_ref(reference) {
            Some((table, _)) => table.to_string(),
            None => {
                errors.push(format!("satır {number}: `{reference}` kitte bir kayıt değil"));
                continue;
            }
        };
        if let Some(why) = taken_by_kit(atlas, name) {
            errors.push(format!("satır {number}: `{name}` zaten {why}; başka bir ad seçin"));
            continue;
        }
        if let Some(first) = seen_name.insert(name.to_string(), number) {
            errors.push(format!("satır {number}: `{name}` adı {first}. satırda da var; her ad tek olmalı"));
            continue;
        }
        if let Some(first) = seen_ref.insert(reference.to_string(), number) {
            errors.push(format!("satır {number}: `{reference}` kaydına {first}. satırda zaten ad verildi"));
            continue;
        }
        names.push(GameName { name: name.to_string(), reference: reference.to_string(), table });
    }
    if errors.is_empty() { Ok(names) } else { Err(errors) }
}

/// The pairs [`Atlas::set_game_names`] takes.
pub fn pairs(names: &[GameName]) -> Vec<(String, String)> {
    names.iter().map(|n| (n.name.clone(), n.reference.clone())).collect()
}

fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_lowercase() || c == '_')
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_follows_the_written_rule() {
        assert_eq!(slug("troops", "Count Klargus").as_deref(), Some("count_klargus"));
        assert_eq!(slug("factions", "Kingdom of Swadia").as_deref(), Some("swadia"));
        assert_eq!(slug("factions", "The Kingdom of Swadia").as_deref(), Some("swadia"));
        assert_eq!(slug("parties", "Praven").as_deref(), Some("praven"));
        assert_eq!(slug("items", "Leather Jerkin (good)").as_deref(), Some("leather_jerkin_good"));
        assert_eq!(slug("parties", "{!}temp_party"), None);
        assert_eq!(slug("quests", "Escort {s13}"), None);
        assert_eq!(slug("items", "2 Hand Sword"), None);
        assert_eq!(slug("troops", "Çelik"), None);
        assert_eq!(slug("troops", "  "), None);
    }

    #[test]
    fn names_are_lower_case_words() {
        assert!(is_name("count_klargus"));
        assert!(is_name("_x1"));
        assert!(!is_name("Count"));
        assert!(!is_name("1st"));
        assert!(!is_name("a-b"));
    }
}
