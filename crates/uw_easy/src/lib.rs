//! Easy English names for kit constants and slots.
//!
//! The generator reads the atlas (`uw_atlas::Atlas`) and the prefix dictionary
//! (`kit-meta/easy/prefixes.toml`) and writes `constants.json` / `slots.json`.
//! Nothing here invents a native name: every `native`/`value`/`line` comes from
//! the atlas, and a family is a flag set only when the values say so.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uw_atlas::Atlas;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// One prefix rule from `kit-meta/easy/prefixes.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct Prefix {
    /// Literal prefix, e.g. `key_` or `itp_type_`.
    pub prefix: String,
    /// Easy family, dots allowed, e.g. `item.type`.
    pub family: String,
    /// Optional kit header filter (`header_troops`); `tf_` means different
    /// things in troops and presentations.
    #[serde(default)]
    pub module: Option<String>,
    /// Where the rule was measured, e.g. `header_triggers.py:132 (key_1)`.
    pub evidence: String,
}

#[derive(Debug, Deserialize)]
struct PrefixFile {
    #[serde(default)]
    prefix: Vec<Prefix>,
}

/// The loaded prefix dictionary, longest matching prefix wins.
#[derive(Debug, Clone)]
pub struct Prefixes {
    entries: Vec<Prefix>,
}

impl Prefixes {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_toml(&std::fs::read_to_string(path)?)
    }

    pub fn from_toml(text: &str) -> Result<Self> {
        let file: PrefixFile = toml::from_str(text)?;
        let mut entries = file.prefix;
        for entry in &entries {
            if entry.prefix.is_empty() || entry.family.is_empty() {
                return Err("prefix dictionary entry with empty prefix or family".into());
            }
        }
        // Longest prefix first; a module-filtered rule beats the generic rule
        // with the same prefix.
        entries.sort_by(|a, b| {
            b.prefix
                .len()
                .cmp(&a.prefix.len())
                .then(b.module.is_some().cmp(&a.module.is_some()))
                .then(a.prefix.cmp(&b.prefix))
        });
        Ok(Self { entries })
    }

    fn lookup(&self, name: &str, module: &str) -> Option<&Prefix> {
        self.entries.iter().find(|entry| {
            name.starts_with(&entry.prefix)
                && entry.module.as_deref().is_none_or(|m| m == module)
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> impl Iterator<Item = &Prefix> {
        self.entries.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Automatic flag-set / enum decision for one family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// Every non-zero value of the family is a power of two.
    Flag,
    /// Anything else (small consecutive integers, masks, negatives, texts).
    Enum,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstantRecord {
    pub easy: String,
    pub native: String,
    pub value: Value,
    pub family: String,
    pub kind: Kind,
    pub doc: String,
    pub line: Option<u32>,
    /// For `same family + same value`: the one easy name of the concept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_of: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SlotRecord {
    pub easy: String,
    pub native: String,
    pub value: Value,
    pub family: String,
    pub doc: String,
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_of: Option<String>,
}

/// Counts for the finish report.
#[derive(Debug, Clone, Default)]
pub struct Report {
    pub constants: usize,
    pub slots: usize,
    pub constant_aliases: usize,
    pub slot_aliases: usize,
    pub constant_families: usize,
    pub flag_families: usize,
    pub enum_families: usize,
    pub slot_families: usize,
    pub dictionary_hits: usize,
    pub default_hits: usize,
    /// `(family, value, native names)` for same-family/same-value slot groups.
    pub slot_shared_values: Vec<(String, String, Vec<String>)>,
}

#[derive(Debug, Clone)]
struct RawConstant {
    native: String,
    value: Value,
    family: String,
    easy: String,
    doc: String,
    line: Option<u32>,
}

/// Slot families measured in `module_constants.py`; longest first so
/// `slot_party_template_*` is never read as a party slot.
const SLOT_KINDS: &[&str] = &[
    "party_template",
    "production",
    "faction",
    "village",
    "player",
    "castle",
    "cattle",
    "center",
    "troop",
    "party",
    "agent",
    "scene",
    "quest",
    "item",
    "lady",
    "lord",
    "team",
    "town",
];

fn documented(module: &str, line: Option<u32>, native: &str, value: &Value) -> String {
    let line = line.map_or_else(|| "?".to_owned(), |l| l.to_string());
    format!("{native} = {value} ({module}.py:{line})")
}

/// The decimal text of a JSON integer; `arbitrary_precision` keeps big
/// 128-bit kit constants (attributes, knowledges) exact.
fn value_text(value: &Value) -> Option<String> {
    let text = match value {
        Value::Number(_) => value.to_string(),
        _ => return None,
    };
    let negative = text.starts_with('-');
    let digits = text.trim_start_matches(['-', '+']).trim_start_matches('0');
    let digits = if digits.is_empty() { "0" } else { digits };
    Some(if negative && digits != "0" {
        format!("-{digits}")
    } else {
        digits.to_owned()
    })
}

fn is_zero(text: &str) -> bool {
    text == "0"
}

fn is_power_of_two(text: &str) -> bool {
    if text.starts_with('-') || is_zero(text) {
        return false;
    }
    if !text.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }
    let mut digits: Vec<u8> = text.bytes().map(|b| b - b'0').collect();
    while !(digits.len() == 1 && digits[0] == 1) {
        if digits.last().is_none_or(|d| d % 2 != 0) {
            return false;
        }
        let mut carry = 0u8;
        for digit in &mut digits {
            let current = carry * 10 + *digit;
            *digit = current / 2;
            carry = current % 2;
        }
        let first = digits.iter().position(|d| *d != 0).unwrap_or(digits.len());
        digits.drain(..first);
        if digits.is_empty() {
            return false;
        }
    }
    true
}

fn family_kind(values: impl IntoIterator<Item = String>) -> Kind {
    let mut non_zero = 0usize;
    for value in values {
        if is_zero(&value) {
            continue;
        }
        if !is_power_of_two(&value) {
            return Kind::Enum;
        }
        non_zero += 1;
    }
    if non_zero == 0 {
        Kind::Enum
    } else {
        Kind::Flag
    }
}

fn easy_name(name: &str, module: &str, prefixes: &Prefixes) -> (String, String) {
    if let Some(entry) = prefixes.lookup(name, module) {
        let stem = &name[entry.prefix.len()..];
        // A member may not start with a digit (`key_1`, `str_3`): keep the
        // whole native word after the family instead.
        let member = if stem.is_empty() || stem.as_bytes()[0].is_ascii_digit() {
            name
        } else {
            stem
        };
        return (entry.family.clone(), format!("{}.{}", entry.family, member));
    }
    match name.split_once('_') {
        Some((family, stem)) if !family.is_empty() && !stem.is_empty() => {
            let member = if stem.as_bytes()[0].is_ascii_digit() {
                name
            } else {
                stem
            };
            (family.to_owned(), format!("{family}.{member}"))
        }
        _ => ("constant".to_owned(), name.to_owned()),
    }
}

fn slot_name(name: &str) -> (String, String) {
    let rest = name.strip_prefix("slot_").unwrap_or(name);
    let known = SLOT_KINDS
        .iter()
        .filter(|kind| {
            rest.len() > kind.len()
                && rest.starts_with(**kind)
                && rest.as_bytes()[kind.len()] == b'_'
        })
        .max_by_key(|kind| kind.len());
    if let Some(kind) = known {
        return (
            (*kind).to_owned(),
            format!("{kind}.{}", &rest[kind.len() + 1..]),
        );
    }
    match rest.split_once('_') {
        Some((kind, stem)) if !kind.is_empty() && !stem.is_empty() => {
            (kind.to_owned(), format!("{kind}.{stem}"))
        }
        _ => ("slot".to_owned(), format!("slot.{rest}")),
    }
}

fn raw_constants(atlas: &Atlas, prefixes: &Prefixes) -> Vec<RawConstant> {
    let mut seen = BTreeSet::new();
    let mut raw = Vec::new();
    for constant in &atlas.constants {
        if constant.name.starts_with("slot_") || !seen.insert(constant.name.clone()) {
            continue;
        }
        let (family, easy) = easy_name(&constant.name, &constant.module, prefixes);
        raw.push(RawConstant {
            native: constant.name.clone(),
            value: constant.value.clone(),
            family,
            easy,
            doc: documented(
                &constant.module,
                constant.line,
                &constant.name,
                &constant.value,
            ),
            line: constant.line,
        });
    }
    raw
}

fn assign_aliases<T>(
    records: &mut [T],
    key: impl Fn(&T) -> (String, String),
    primary_order: impl Fn(&T) -> (u32, String),
    set_alias: impl Fn(&mut T, Option<String>),
    get_easy: impl Fn(&T) -> String,
) where
    T: Clone,
{
    let mut groups: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (index, record) in records.iter().enumerate() {
        groups.entry(key(record)).or_default().push(index);
    }
    for indexes in groups.values() {
        if indexes.len() < 2 {
            continue;
        }
        let primary = indexes
            .iter()
            .copied()
            .min_by_key(|index| primary_order(&records[*index]))
            .expect("non-empty group");
        let canonical = get_easy(&records[primary]);
        for &index in indexes {
            if index == primary {
                set_alias(&mut records[index], None);
            } else {
                let mut record = records[index].clone();
                set_alias(&mut record, Some(canonical.clone()));
                records[index] = record;
            }
        }
    }
}

/// One record per unique constant, + `slot_*` excluded; aliases carry the
/// primary easy name in `alias_of`.
pub fn constants(atlas: &Atlas, prefixes: &Prefixes) -> Vec<ConstantRecord> {
    let raw = raw_constants(atlas, prefixes);

    let mut family_values: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for record in &raw {
        let value = value_text(&record.value).unwrap_or_else(|| "text".to_owned());
        family_values
            .entry(record.family.clone())
            .or_default()
            .push(value);
    }
    let kinds: BTreeMap<String, Kind> = family_values
        .into_iter()
        .map(|(family, values)| (family, family_kind(values)))
        .collect();

    let mut records: Vec<ConstantRecord> = raw
        .into_iter()
        .map(|record| ConstantRecord {
            easy: record.easy,
            native: record.native,
            value: record.value,
            kind: kinds[&record.family],
            family: record.family,
            doc: record.doc,
            line: record.line,
            alias_of: None,
        })
        .collect();

    assign_aliases(
        &mut records,
        |record| {
            (
                record.family.clone(),
                value_text(&record.value).unwrap_or_else(|| "text".to_owned()),
            )
        },
        |record| {
            (
                record.line.unwrap_or(u32::MAX),
                record.native.clone(),
            )
        },
        |record, alias| {
            if let Some(canonical) = &alias {
                record.easy = canonical.clone();
            }
            record.alias_of = alias;
        },
        |record| record.easy.clone(),
    );

    records.sort_by(|a, b| {
        a.family
            .cmp(&b.family)
            .then(a.easy.cmp(&b.easy))
            .then(a.native.cmp(&b.native))
    });
    records
}

/// One record per numeric `slot_*` constant in `module_constants.py`.
pub fn slots(atlas: &Atlas) -> Vec<SlotRecord> {
    let mut seen = BTreeSet::new();
    let mut records = Vec::new();
    for constant in &atlas.constants {
        if !constant.name.starts_with("slot_") || !seen.insert(constant.name.clone()) {
            continue;
        }
        let (family, easy) = slot_name(&constant.name);
        records.push(SlotRecord {
            easy,
            native: constant.name.clone(),
            value: constant.value.clone(),
            family,
            doc: documented(
                &constant.module,
                constant.line,
                &constant.name,
                &constant.value,
            ),
            line: constant.line,
            alias_of: None,
        });
    }

    assign_aliases(
        &mut records,
        |record| {
            (
                record.family.clone(),
                value_text(&record.value).unwrap_or_else(|| "text".to_owned()),
            )
        },
        |record| {
            (
                record.line.unwrap_or(u32::MAX),
                record.native.clone(),
            )
        },
        |record, alias| {
            if let Some(canonical) = &alias {
                record.easy = canonical.clone();
            }
            record.alias_of = alias;
        },
        |record| record.easy.clone(),
    );

    records.sort_by(|a, b| {
        a.family
            .cmp(&b.family)
            .then(a.easy.cmp(&b.easy))
            .then(a.native.cmp(&b.native))
    });
    records
}

/// Summary of the generated records (used by the generator and the report).
pub fn report(constants: &[ConstantRecord], slots: &[SlotRecord]) -> Report {
    let mut families: BTreeMap<&str, Kind> = BTreeMap::new();
    for record in constants {
        families.insert(&record.family, record.kind);
    }
    let mut slot_families = BTreeSet::new();
    let mut shared: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for record in slots {
        slot_families.insert(record.family.clone());
        if let Some(value) = value_text(&record.value) {
            shared
                .entry((record.family.clone(), value))
                .or_default()
                .push(record.native.clone());
        }
    }
    let slot_shared_values = shared
        .into_iter()
        .filter(|(_, names)| names.len() > 1)
        .map(|((family, value), mut names)| {
            names.sort();
            (family, value, names)
        })
        .collect();
    Report {
        constants: constants.len(),
        slots: slots.len(),
        constant_aliases: constants.iter().filter(|r| r.alias_of.is_some()).count(),
        slot_aliases: slots.iter().filter(|r| r.alias_of.is_some()).count(),
        constant_families: families.len(),
        flag_families: families.values().filter(|k| **k == Kind::Flag).count(),
        enum_families: families.values().filter(|k| **k == Kind::Enum).count(),
        slot_families: slot_families.len(),
        dictionary_hits: 0,
        default_hits: 0,
        slot_shared_values,
    }
}

/// Generate both JSON files from an atlas and the prefix dictionary.
pub fn generate(atlas_path: impl AsRef<Path>, prefixes_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) -> Result<Report> {
    let atlas = Atlas::from_path(atlas_path)?;
    let prefixes = Prefixes::from_path(prefixes_path)?;
    let out_dir = out_dir.as_ref();
    std::fs::create_dir_all(out_dir)?;

    let constants = constants(&atlas, &prefixes);
    let slots = slots(&atlas);
    let mut summary = report(&constants, &slots);
    let mut seen = BTreeSet::new();
    summary.dictionary_hits = atlas
        .constants
        .iter()
        .filter(|constant| !constant.name.starts_with("slot_") && seen.insert(constant.name.clone()))
        .filter(|constant| prefixes.lookup(&constant.name, &constant.module).is_some())
        .count();
    summary.default_hits = summary.constants - summary.dictionary_hits;

    write_json(out_dir.join("constants.json"), &constants)?;
    write_json(out_dir.join("slots.json"), &slots)?;
    Ok(summary)
}

fn write_json<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let mut text = serde_json::to_string_pretty(value)?;
    text.push('\n');
    std::fs::write(path, text)?;
    Ok(())
}

/// Easy names for kit operations (T-130): the operation dictionary behind the easy command.
pub mod ops;
