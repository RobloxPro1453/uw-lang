//! Reads the atlas JSON produced by `py/atlas_dump.py` (Native ops, hooks, ranges, every table entry with name/index/value/source line) and `kit-meta/*.toml`. Read-only data for check, plan and `uw explain`.
//!
//! The JSON schema is the one documented in the docstring of `py/atlas_dump.py` (schema 1). The
//! loader builds the lookup maps once, so `op`, `entry_by_id`, `is_hook` are plain hash lookups.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::path::Path;

use serde::Deserialize;

/// Atlas JSON schema version this crate understands (`schema` in the dump).
pub const SCHEMA: u64 = 1;

/// A header operation (`from header_operations.py`).
#[derive(Debug, Clone, Deserialize)]
pub struct Op {
    pub name: String,
    /// Opcode as written in the header; fits in `u64`.
    pub code: u64,
    /// Source line in `header_operations.py`.
    pub line: Option<u32>,
    /// Free-form signature text, may be empty and may contain tabs/commas.
    pub signature: String,
    /// True when the operation stores into its first operand.
    pub lhs: bool,
    /// True when the operation can fail (conditions).
    pub can_fail: bool,
    /// True when the operation writes a global (`$...`) lhs.
    pub global_lhs: bool,
}

/// A `*_begin` / `*_end` constant (`from module_constants.py`).
#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    pub name: String,
    /// Kept as arbitrary JSON: the dump only guarantees a number.
    pub value: serde_json::Value,
    /// Source line in `module_constants.py`.
    pub line: Option<u32>,
}

/// One entry of a Native table list.
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    /// Index in the table's list (what the kit uses as the entry id).
    pub index: usize,
    /// Entry name; `null` for unnamed tables (dialogs, triggers, simple_triggers).
    pub name: Option<String>,
    /// Identifier from the kit's `ID_*.py`; `null` when shadowed by a later duplicate and for
    /// unnamed tables.
    pub id: Option<String>,
    /// 1-based occurrence, set only when the name repeats; else `null`.
    pub dup: Option<u32>,
    /// Source line in the `module_*.py`; `null` when not found.
    pub line: Option<u32>,
    /// Arbitrary JSON, exactly as dumped.
    pub value: serde_json::Value,
}

impl Entry {
    /// The `id#dup` key this entry answers to, when it is a duplicate occurrence.
    fn dup_key(&self, id: &str) -> Option<String> {
        self.dup.map(|dup| format!("{id}#{dup}"))
    }
}

/// A Native table list together with where its identifiers come from.
#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub module: String,
    pub var: String,
    pub id_file: Option<String>,
    pub id_prefix: Option<String>,
    /// Tag operations use to name this table's entries (`"mnu"` for menus, `"mt"` for mission
    /// templates), read by the dump from the kit's `process_operations.py`. `None` when
    /// operations cannot reference the table, and for atlases dumped before the field existed.
    #[serde(default)]
    pub ref_tag: Option<String>,
    pub entries: Vec<Entry>,
}

/// The whole atlas, with loader-built lookup maps.
#[derive(Debug, Clone, Deserialize)]
pub struct Atlas {
    pub schema: u64,
    pub kit_hash: Option<String>,
    pub ops: Vec<Op>,
    /// Bare script names (`game_start`, ...) — not `script_`-prefixed ids.
    pub hooks: Vec<String>,
    pub ranges: Vec<Range>,
    /// Keyed by table name; ordered so `all_ids` is deterministic.
    pub tables: BTreeMap<String, Table>,
    #[serde(skip)]
    op_index: HashMap<String, usize>,
    /// `id` or `id#dup` -> (table, position in `tables[table].entries`).
    #[serde(skip)]
    id_index: HashMap<String, (String, usize)>,
    #[serde(skip)]
    hook_set: HashSet<String>,
    /// `ref_tag` -> table name.
    #[serde(skip)]
    tag_index: HashMap<String, String>,
    /// Op code -> position in `ops` (the first non-deprecated name for that code).
    #[serde(skip)]
    code_index: HashMap<u64, usize>,
    /// `neg | this_or_next`, from the atlas' own op list.
    #[serde(skip)]
    flag_mask: u64,
}

/// One place where an operation block mentions something (see [`Atlas::references`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// Table of the entry whose value holds the mention.
    pub table: String,
    /// Index of that entry in its table.
    pub index: usize,
    /// Name of the operation the mention is an operand of; `None` outside operation blocks.
    pub op: Option<String>,
}

impl Atlas {
    /// Reads and validates an atlas file.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Atlas, Error> {
        let text = std::fs::read_to_string(path)?;
        Atlas::from_str(&text)
    }

    /// Parses and validates atlas JSON text. Rejects any `schema` other than [`SCHEMA`].
    pub fn from_str(text: &str) -> Result<Atlas, Error> {
        let mut atlas: Atlas = serde_json::from_str(text)?;
        if atlas.schema != SCHEMA {
            return Err(Error::Schema { found: atlas.schema });
        }
        atlas.build_indexes();
        Ok(atlas)
    }

    fn build_indexes(&mut self) {
        let mut op_index = HashMap::with_capacity(self.ops.len());
        for (pos, op) in self.ops.iter().enumerate() {
            op_index.entry(op.name.clone()).or_insert(pos);
        }
        let mut id_index = HashMap::new();
        for (table_name, table) in &self.tables {
            for (pos, entry) in table.entries.iter().enumerate() {
                if let Some(id) = &entry.id {
                    id_index.insert(id.clone(), (table_name.clone(), pos));
                }
            }
            // A repeated name loses its `id` on the shadowed (earlier) occurrences, so the
            // `id#dup` keys are recovered per name group: one member carries the identifier,
            // every member answers to `<that id>#<its dup>`.
            let mut groups: HashMap<&str, Vec<usize>> = HashMap::new();
            for (pos, entry) in table.entries.iter().enumerate() {
                if let (Some(name), Some(_)) = (&entry.name, entry.dup) {
                    groups.entry(name.as_str()).or_default().push(pos);
                }
            }
            for (name, positions) in groups {
                let base = positions
                    .iter()
                    .filter_map(|&pos| table.entries[pos].id.as_deref())
                    .next()
                    .map(str::to_owned)
                    .or_else(|| {
                        table
                            .id_prefix
                            .as_ref()
                            .map(|prefix| format!("{prefix}{name}"))
                    });
                let Some(base) = base else { continue };
                for pos in positions {
                    if let Some(key) = table.entries[pos].dup_key(&base) {
                        id_index.insert(key, (table_name.clone(), pos));
                    }
                }
            }
        }
        let hook_set = self.hooks.iter().cloned().collect();
        let mut tag_index = HashMap::new();
        for (table_name, table) in &self.tables {
            if let Some(tag) = &table.ref_tag {
                tag_index.insert(tag.clone(), table_name.clone());
            }
        }
        let mut code_index: HashMap<u64, usize> = HashMap::new();
        for (pos, op) in self.ops.iter().enumerate() {
            // First name in header order wins, unless it is marked deprecated and a later one
            // is not (`end_try` / `try_end`).
            let deprecated = |op: &Op| op.signature.trim_start().starts_with("deprecated");
            let replace = match code_index.get(&op.code) {
                None => true,
                Some(&old) => deprecated(&self.ops[old]) && !deprecated(op),
            };
            if replace {
                code_index.insert(op.code, pos);
            }
        }
        self.flag_mask = ["neg", "this_or_next"]
            .iter()
            .filter_map(|name| op_index.get(*name).map(|&pos| self.ops[pos].code))
            .fold(0, |mask, code| mask | code);
        self.tag_index = tag_index;
        self.code_index = code_index;
        self.op_index = op_index;
        self.id_index = id_index;
        self.hook_set = hook_set;
    }

    /// The operation with this header name, if the kit has one.
    pub fn op(&self, name: &str) -> Option<&Op> {
        self.ops.get(*self.op_index.get(name)?)
    }

    /// Looks up a table entry by identifier. Accepts a plain `id` (the entry whose `id` field
    /// matches) and `id#n`, where `n` is the 1-based duplicate occurrence.
    pub fn entry_by_id(&self, id: &str) -> Option<(&str, &Entry)> {
        let (table, pos) = self.id_index.get(id)?;
        let entry = self.tables.get(table)?.entries.get(*pos)?;
        Some((table.as_str(), entry))
    }

    /// Every entry of `table` whose name is exactly `name` (duplicates included, in list order).
    pub fn entries_named<'a>(
        &'a self,
        table: &str,
        name: &'a str,
    ) -> impl Iterator<Item = &'a Entry> + 'a {
        self.tables
            .get(table)
            .into_iter()
            .flat_map(|table| table.entries.iter())
            .filter(move |entry| entry.name.as_deref() == Some(name))
    }

    /// True when `name` is a script hook (`game_start`, ...).
    pub fn is_hook(&self, name: &str) -> bool {
        self.hook_set.contains(name)
    }

    /// The table operations reach with `tag` (`"mnu"` -> `"menus"`).
    pub fn table_for_tag(&self, tag: &str) -> Option<&str> {
        self.tag_index.get(tag).map(String::as_str)
    }

    /// Resolves a reference string as the kit does inside operations (`get_identifier_value`):
    /// the part before the first `_` is the tag, the rest is compared case-insensitively with
    /// entry names and the FIRST match wins (`header_common.find_object`). Note the contrast with
    /// [`Atlas::entry_by_id`], where a repeated name keeps only the last occurrence's id.
    pub fn resolve_ref(&self, text: &str) -> Option<(&str, &Entry)> {
        let (tag, rest) = text.split_once('_')?;
        if tag.is_empty() {
            return None;
        }
        let table_name = self.tag_index.get(tag)?;
        let table = self.tables.get(table_name)?;
        let wanted = rest.to_lowercase();
        let entry = table
            .entries
            .iter()
            .find(|entry| entry.name.as_deref().is_some_and(|name| name.to_lowercase() == wanted))?;
        Some((table_name.as_str(), entry))
    }

    /// The operation named for a statement head, with the kit's `neg` / `this_or_next` flag bits
    /// (taken from the atlas' own op list) cleared.
    pub fn op_for_code(&self, code: u64) -> Option<&Op> {
        let pos = self
            .code_index
            .get(&code)
            .or_else(|| self.code_index.get(&(code & !self.flag_mask)))?;
        self.ops.get(*pos)
    }

    /// Every place a string in some entry's value satisfies `matches`, in table order. Inside
    /// operation blocks (`{"list": [...]}` of statements) the statement's operation is named.
    /// Only string mentions are found: data written with the kit's Python names (`itm_x` as an
    /// `ID_*.py` integer) is already a number in the atlas and cannot be traced back.
    pub fn references(&self, matches: impl Fn(&str) -> bool) -> Vec<Reference> {
        let mut out = Vec::new();
        self.for_each_string(|table, entry, op, text| {
            if matches(text) {
                out.push(Reference {
                    table: table.to_owned(),
                    index: entry.index,
                    op: op.map(str::to_owned),
                });
            }
        });
        out
    }

    /// Calls `f(table, entry, op, text)` for every string inside every entry's value, in table
    /// order; `op` names the operation whose statement holds the string (see [`Atlas::references`]).
    pub fn for_each_string<'a>(&'a self, mut f: impl FnMut(&'a str, &'a Entry, Option<&'a str>, &'a str)) {
        for (table_name, table) in &self.tables {
            for entry in &table.entries {
                self.scan(&entry.value, None, false, &mut |op, text| f(table_name, entry, op, text));
            }
        }
    }

    fn scan<'a>(
        &'a self,
        value: &'a serde_json::Value,
        op: Option<&'a str>,
        in_block: bool,
        f: &mut dyn FnMut(Option<&'a str>, &'a str),
    ) {
        use serde_json::Value;
        match value {
            Value::String(text) => f(op, text),
            Value::Array(items) => {
                // A tuple sitting directly in a block, headed by an integer, is a statement.
                let statement_op = if in_block {
                    items
                        .first()
                        .and_then(Value::as_u64)
                        .and_then(|code| self.op_for_code(code))
                        .map(|op| op.name.as_str())
                } else {
                    None
                };
                let op = statement_op.or(op);
                for item in items {
                    self.scan(item, op, false, f);
                }
            }
            Value::Object(map) => {
                for (key, item) in map {
                    match (key.as_str(), item) {
                        ("list", Value::Array(items)) => {
                            for item in items {
                                self.scan(item, op, true, f);
                            }
                        }
                        _ => self.scan(item, op, false, f),
                    }
                }
            }
            _ => {}
        }
    }

    /// Every known identifier, for did-you-mean suggestions. Deterministic order.
    pub fn all_ids(&self) -> impl Iterator<Item = &str> {
        self.tables
            .values()
            .flat_map(|table| table.entries.iter())
            .filter_map(|entry| entry.id.as_deref())
    }
}

/// Everything that can go wrong while loading an atlas.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
    /// The file was dumped by a newer/other `atlas_dump.py` than this crate understands.
    Schema { found: u64 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "cannot read atlas: {err}"),
            Error::Json(err) => write!(f, "cannot parse atlas JSON: {err}"),
            Error::Schema { found } => {
                write!(f, "unsupported atlas schema {found}, expected {SCHEMA}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Json(err) => Some(err),
            Error::Schema { .. } => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
