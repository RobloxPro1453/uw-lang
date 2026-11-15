//! What `uw explain` and `uw browse` show, computed once from the atlas: a search index over
//! every entry / global / operation / constant, the in-game name of entries, and a reverse index
//! of who mentions whom.

use std::collections::{BTreeMap, HashMap};

use uw_atlas::{Atlas, Entry};

/// Something the tool can explain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Target {
    Entry { table: String, index: usize },
    Global(String),
    Op(String),
    Const(String),
}

/// One search result row.
#[derive(Debug, Clone)]
pub struct Hit {
    pub target: Target,
    pub label: String,
    /// In-game name or text, when the kit's layout gives one.
    pub hint: Option<String>,
    pub kind: String,
}

/// Everything shown about one target.
#[derive(Debug, Clone)]
pub struct Detail {
    pub label: String,
    pub kind: String,
    pub hint: Option<String>,
    pub notes: Vec<String>,
    pub defined: Option<String>,
    pub value: Option<String>,
    pub uses: Vec<UseGroup>,
    pub total_mentions: usize,
}

#[derive(Debug, Clone)]
pub struct UseGroup {
    pub table: String,
    pub sites: Vec<UseSite>,
}

#[derive(Debug, Clone)]
pub struct UseSite {
    pub target: Target,
    pub label: String,
    pub hint: Option<String>,
    pub place: String,
    /// Operation name (or "data") -> how many mentions.
    pub ops: Vec<(String, usize)>,
}

impl UseSite {
    pub fn ops_text(&self) -> String {
        self.ops
            .iter()
            .map(|(op, n)| if *n > 1 { format!("{op} x{n}") } else { op.clone() })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

struct Row {
    target: Target,
    label: String,
    hint: Option<String>,
    kind: String,
    /// Other exact names this row answers to (code form `mnu_x`, hook name `game_start`).
    alts: Vec<String>,
    label_lc: String,
    hint_lc: String,
    /// label, alts, kind and hint, lower case: what word search looks in.
    all_lc: String,
}

/// The index. Owns the atlas.
pub struct Model {
    pub atlas: Atlas,
    rows: Vec<Row>,
    /// Target mentioned -> (table, index, op) of each mention, in atlas order.
    mentions: HashMap<Target, Vec<(String, usize, Option<String>)>>,
    /// Entry -> (target it mentions, op), in value order: the forward edges of the flow graph.
    outgoing: HashMap<Target, Vec<(Target, Option<String>)>>,
}

/// Field that holds the in-game name/text, per the kit's own layout comments:
/// module_troops.py:14, module_items.py:11, module_parties.py:12, module_factions.py:7,
/// module_quests.py:6, module_party_templates.py:13, module_skills.py:7 (field 2 = name);
/// module_strings.py:10 (field 2 = text); module_game_menus.py:20 (field 3 = text);
/// module_dialogs.py:39 (field 4 = text).
fn hint_field(table: &str) -> Option<usize> {
    match table {
        "troops" | "items" | "parties" | "factions" | "quests" | "party_templates" | "skills"
        | "strings" => Some(1),
        "menus" => Some(2),
        "dialogs" => Some(3),
        _ => None,
    }
}

/// Readable kind for a table.
pub fn kind_of(table: &str) -> String {
    match table {
        "parties" => "party".into(),
        "dialogs" => "dialog line".into(),
        "strings" => "text string".into(),
        "simple_triggers" => "simple trigger".into(),
        "mission_templates" => "mission template".into(),
        "party_templates" => "party template".into(),
        "scene_props" => "scene prop".into(),
        "tableau_materials" => "tableau material".into(),
        "particle_systems" => "particle system".into(),
        "postfx_params" => "postfx setting".into(),
        "info_pages" => "info page".into(),
        "map_icons" => "map icon".into(),
        "music" => "music track".into(),
        other => other.strip_suffix('s').unwrap_or(other).replace('_', " "),
    }
}

fn fields(value: &serde_json::Value) -> Option<&Vec<serde_json::Value>> {
    match value {
        serde_json::Value::Array(items) => Some(items),
        serde_json::Value::Object(map) => map.get("list")?.as_array(),
        _ => None,
    }
}

/// In-game name/text of an entry, if its table has one (see [`hint_field`]).
pub fn entry_hint(table: &str, entry: &Entry) -> Option<String> {
    let text = fields(&entry.value)?.get(hint_field(table)?)?.as_str()?;
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}

/// Shortens to `max` chars with an ellipsis.
pub fn short(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_owned()
    } else {
        let cut: String = text.chars().take(max.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}

impl Model {
    pub fn new(atlas: Atlas) -> Model {
        let mut rows = Vec::new();
        // tag -> lower-case name -> first entry index (header_common.find_object: first wins).
        let mut by_tag: HashMap<&str, (&str, HashMap<String, usize>)> = HashMap::new();
        for (table_name, table) in &atlas.tables {
            if let Some(tag) = table.ref_tag.as_deref() {
                let mut names = HashMap::new();
                for entry in &table.entries {
                    if let Some(name) = &entry.name {
                        names.entry(name.to_lowercase()).or_insert(entry.index);
                    }
                }
                by_tag.insert(tag, (table_name.as_str(), names));
            }
            for entry in &table.entries {
                let label = entry_label(&atlas, table_name, entry);
                let mut alts = Vec::new();
                if let (Some(tag), Some(name)) = (&table.ref_tag, &entry.name) {
                    alts.push(format!("{tag}_{}", name.to_lowercase()));
                }
                if table_name == "scripts" {
                    if let Some(name) = entry.name.as_deref().filter(|n| atlas.is_hook(n)) {
                        alts.push(name.to_owned());
                    }
                }
                rows.push(Row::new(
                    Target::Entry { table: table_name.clone(), index: entry.index },
                    label,
                    entry_hint(table_name, entry),
                    kind_of(table_name),
                    alts,
                ));
            }
        }

        let mut mentions: HashMap<Target, Vec<(String, usize, Option<String>)>> = HashMap::new();
        let mut outgoing: HashMap<Target, Vec<(Target, Option<String>)>> = HashMap::new();
        let mut globals: BTreeMap<String, ()> = BTreeMap::new();
        atlas.for_each_string(|table, entry, op, text| {
            let target = if text.starts_with('$') && text.len() > 1 {
                globals.insert(text.to_owned(), ());
                Target::Global(text.to_owned())
            } else {
                let Some((tag, rest)) = text.split_once('_') else { return };
                let Some((ref_table, names)) = by_tag.get(tag) else { return };
                let Some(&index) = names.get(&rest.to_lowercase()) else { return };
                Target::Entry { table: (*ref_table).to_owned(), index }
            };
            outgoing
                .entry(Target::Entry { table: table.to_owned(), index: entry.index })
                .or_default()
                .push((target.clone(), op.map(str::to_owned)));
            mentions
                .entry(target)
                .or_default()
                .push((table.to_owned(), entry.index, op.map(str::to_owned)));
        });

        for global in globals.into_keys() {
            rows.push(Row::new(Target::Global(global.clone()), global, None, "global variable".into(), vec![]));
        }
        for op in &atlas.ops {
            let hint = (!op.signature.is_empty()).then(|| op.signature.clone());
            rows.push(Row::new(Target::Op(op.name.clone()), op.name.clone(), hint, "operation".into(), vec![]));
        }
        for range in &atlas.ranges {
            rows.push(Row::new(
                Target::Const(range.name.clone()),
                range.name.clone(),
                Some(format!("= {}", range.value)),
                "constant".into(),
                vec![],
            ));
        }
        Model { atlas, rows, mentions, outgoing }
    }

    /// Every mention of `target`: (table, index, op) of the entry that holds it. Empty if none.
    pub fn incoming(&self, target: &Target) -> &[(String, usize, Option<String>)] {
        self.mentions.get(target).map_or(&[], Vec::as_slice)
    }

    /// Everything entry `source` mentions by name, with the operation (None outside operation
    /// blocks), in the order it appears in the entry's value. Empty for non-entries.
    pub fn outgoing(&self, source: &Target) -> &[(Target, Option<String>)] {
        self.outgoing.get(source).map_or(&[], Vec::as_slice)
    }

    /// Entries the engine itself starts from (nothing in Native has to mention them): every hook
    /// script (`game_*`, header comments in module_scripts.py), menu #0 (the first menu,
    /// docs/bilgi/baslangic-akisi.md) and menu `start_phase_2` (opened by the engine after character
    /// creation, D4 [DENEY]). Triggers and simple triggers run by themselves too.
    pub fn engine_roots(&self) -> Vec<Target> {
        let mut roots = Vec::new();
        for (table_name, table) in &self.atlas.tables {
            for entry in &table.entries {
                let root = match table_name.as_str() {
                    "scripts" => entry.name.as_deref().is_some_and(|n| self.atlas.is_hook(n)),
                    "menus" => entry.index == 0 || entry.name.as_deref() == Some("start_phase_2"),
                    "triggers" | "simple_triggers" => true,
                    _ => false,
                };
                if root {
                    roots.push(Target::Entry { table: table_name.clone(), index: entry.index });
                }
            }
        }
        roots
    }

    /// The target `query` names exactly (id, `id#n`, code form, hook, global, op, constant).
    pub fn resolve(&self, query: &str) -> Option<Target> {
        let query = query.trim();
        if let Some((table, entry)) = self.atlas.entry_by_id(query).or_else(|| self.atlas.resolve_ref(query)) {
            return Some(Target::Entry { table: table.to_owned(), index: entry.index });
        }
        if let Some(row) = self.rows.iter().find(|row| row.label == query || row.alts.iter().any(|alt| alt == query)) {
            return Some(row.target.clone());
        }
        // A bare name without its id prefix ("start_phase_2" -> menu_start_phase_2), when exactly
        // one table has an entry with that id.
        let mut bare = self.atlas.tables.values().filter_map(|table| {
            let id = format!("{}{query}", table.id_prefix.as_deref()?);
            self.atlas.entry_by_id(&id)
        });
        if let (Some((table, entry)), None) = (bare.next(), bare.next()) {
            return Some(Target::Entry { table: table.to_owned(), index: entry.index });
        }
        // An in-game name that belongs to exactly one thing ("Praven" -> p_town_6).
        let q = query.to_lowercase();
        let mut named = self.rows.iter().filter(|row| !row.hint_lc.is_empty() && row.hint_lc == q);
        match (named.next(), named.next()) {
            (Some(row), None) => Some(row.target.clone()),
            _ => None,
        }
    }

    /// Rows whose label, code form or in-game name contains `query` (case-insensitive), best
    /// first: exact, then label prefix, then label contains, then name contains.
    pub fn search(&self, query: &str, limit: usize) -> Vec<Hit> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return Vec::new();
        }
        // "merchant_quest" / "merchant quest": every word somewhere (label, code form, kind, name).
        let words: Vec<String> = q
            .split(|c: char| c == '_' || c.is_whitespace())
            .filter(|w| w.len() > 1)
            .map(str::to_owned)
            .collect();
        let mut scored: Vec<(u8, usize, &Row)> = Vec::new();
        for row in &self.rows {
            let rank = if row.label_lc == q || row.hint_lc == q || row.alts.iter().any(|alt| alt.to_lowercase() == q) {
                0
            } else if row.label_lc.starts_with(&q) {
                1
            } else if row.label_lc.contains(&q) || row.alts.iter().any(|alt| alt.contains(&q)) {
                2
            } else if row.hint_lc.contains(&q) {
                3
            } else if words.len() > 1 && words.iter().all(|w| row.all_lc.contains(w.as_str())) {
                4
            } else {
                continue;
            };
            scored.push((rank, row.label.len(), row));
        }
        scored.sort_by(|a, b| (a.0, a.1, &a.2.label).cmp(&(b.0, b.1, &b.2.label)));
        scored
            .into_iter()
            .take(limit)
            .map(|(_, _, row)| Hit {
                target: row.target.clone(),
                label: row.label.clone(),
                hint: row.hint.clone(),
                kind: row.kind.clone(),
            })
            .collect()
    }

    /// Number of rows that match `query` (for "N results").
    pub fn count(&self, query: &str) -> usize {
        self.search(query, usize::MAX).len()
    }

    pub fn label(&self, target: &Target) -> String {
        match target {
            Target::Entry { table, index } => {
                entry_label(&self.atlas, table, &self.atlas.tables[table].entries[*index])
            }
            Target::Global(name) | Target::Op(name) | Target::Const(name) => name.clone(),
        }
    }

    pub fn describe(&self, target: &Target) -> Detail {
        let mut detail = Detail {
            label: self.label(target),
            kind: String::new(),
            hint: None,
            notes: Vec::new(),
            defined: None,
            value: None,
            uses: Vec::new(),
            total_mentions: 0,
        };
        match target {
            Target::Entry { table, index } => {
                let t = &self.atlas.tables[table];
                let entry = &t.entries[*index];
                detail.kind = format!("{} ({table} #{index})", kind_of(table));
                detail.hint = entry_hint(table, entry);
                if table == "scripts" && entry.name.as_deref().is_some_and(|n| self.atlas.is_hook(n)) {
                    detail.notes.push("engine hook: the game itself calls this script".into());
                }
                if let (Some(tag), Some(name)) = (&t.ref_tag, &entry.name) {
                    let code = format!("{tag}_{}", name.to_lowercase());
                    match self.atlas.resolve_ref(&code) {
                        Some((_, first)) if first.index == entry.index => {
                            detail.notes.push(format!("written in code as \"{code}\""));
                        }
                        Some((_, first)) => detail.notes.push(format!(
                            "unreachable from code: \"{code}\" means the first entry with this name (#{})",
                            first.index
                        )),
                        None => {}
                    }
                }
                detail.defined = Some(match entry.line {
                    Some(line) => format!("{}.py:{line}", t.module),
                    None => format!("{}.py (entry #{index} of {})", t.module, t.var),
                });
                detail.value = Some(short(&entry.value.to_string(), 200));
            }
            Target::Global(_) => detail.kind = "global variable".into(),
            Target::Op(name) => {
                detail.kind = "operation".into();
                if let Some(op) = self.atlas.op(name) {
                    detail.kind = format!("operation (code {})", op.code);
                    detail.defined = op.line.map(|line| format!("header_operations.py:{line}"));
                    if !op.signature.is_empty() {
                        detail.hint = Some(op.signature.clone());
                    }
                    if op.can_fail {
                        detail.notes.push("condition: can fail and stop the block".into());
                    }
                    if op.lhs {
                        detail.notes.push("writes its first operand".into());
                    }
                }
            }
            Target::Const(name) => {
                detail.kind = "constant".into();
                if let Some(range) = self.atlas.ranges.iter().find(|r| &r.name == name) {
                    detail.hint = Some(format!("= {}", range.value));
                    detail.defined = range.line.map(|line| format!("module_constants.py:{line}"));
                }
            }
        }
        if let Some(list) = self.mentions.get(target) {
            detail.total_mentions = list.len();
            detail.uses = self.group(list);
        }
        detail
    }

    fn group(&self, list: &[(String, usize, Option<String>)]) -> Vec<UseGroup> {
        // table -> index -> op -> count, all in atlas order.
        let mut by_table: BTreeMap<&str, BTreeMap<usize, BTreeMap<&str, usize>>> = BTreeMap::new();
        for (table, index, op) in list {
            *by_table
                .entry(table)
                .or_default()
                .entry(*index)
                .or_default()
                .entry(op.as_deref().unwrap_or("data"))
                .or_default() += 1;
        }
        let mut groups: Vec<UseGroup> = by_table
            .into_iter()
            .map(|(table, sites)| UseGroup {
                table: table.to_owned(),
                sites: sites
                    .into_iter()
                    .map(|(index, ops)| {
                        let t = &self.atlas.tables[table];
                        let entry = &t.entries[index];
                        UseSite {
                            target: Target::Entry { table: table.to_owned(), index },
                            label: entry_label(&self.atlas, table, entry),
                            hint: entry_hint(table, entry),
                            place: match entry.line {
                                Some(line) => format!("{}.py:{line}", t.module),
                                None => format!("{}.py", t.module),
                            },
                            ops: ops.into_iter().map(|(op, n)| (op.to_owned(), n)).collect(),
                        }
                    })
                    .collect(),
            })
            .collect();
        groups.sort_by(|a, b| b.sites.len().cmp(&a.sites.len()).then(a.table.cmp(&b.table)));
        groups
    }
}

impl Row {
    fn new(target: Target, label: String, hint: Option<String>, kind: String, alts: Vec<String>) -> Row {
        let all_lc = format!(
            "{} {} {} {}",
            label,
            alts.join(" "),
            kind,
            hint.as_deref().unwrap_or("")
        )
        .to_lowercase();
        Row {
            all_lc,
            label_lc: label.to_lowercase(),
            hint_lc: hint.as_deref().unwrap_or("").to_lowercase(),
            target,
            label,
            hint,
            kind,
            alts,
        }
    }
}

/// `p_town_6`, `script_game_start`, `itm_x#1`, `dlga_start:close_window`, `triggers #5`.
pub fn entry_label(atlas: &Atlas, table: &str, entry: &Entry) -> String {
    if let Some(id) = &entry.id {
        return id.clone();
    }
    let t = &atlas.tables[table];
    match (&t.id_prefix, &entry.name, entry.dup) {
        (Some(prefix), Some(name), Some(dup)) => format!("{prefix}{}#{dup}", name.to_lowercase()),
        _ => format!("{table} #{}", entry.index),
    }
}
