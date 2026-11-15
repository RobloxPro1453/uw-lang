//! `uw graph --json`: the whole menu flow in the `uw-flow/1` format that graph_viewer reads
//! (docs/graph_viewer/DATA.md, schema.json). Same rules as py/flow_export.py, which it replaces:
//!
//! * every menu is a node with its in-game text and its options (buttons) in display order;
//! * arrows come from operations in the menu's opening code (field 4) and in each option's
//!   consequence block: `jump_to_menu`, `call_script`, `start_presentation`, `jump_to_scene`,
//!   `set_jump_mission` (to the entry they name) and `change_screen_*` (to `screen:<rest>`);
//! * every arrow target that is not a menu becomes a small `stub` node, so `from`/`to` always
//!   name a node.
//!
//! Operation codes come from the atlas' own op list (flags cleared), never typed in.
//! Compatibility: within `uw-flow/1` fields are only added; see DATA.md.

use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use uw_atlas::Atlas;

/// Plain-language descriptions (kit-meta/aciklama/<table>.json, written by py/describe.py from
/// the kit's source): node id or op name -> (short, long, confidence).
#[derive(Default)]
pub struct Descriptions {
    pub nodes: HashMap<String, (String, String, String)>,
    pub ops: HashMap<String, (String, String, String)>,
}

impl Descriptions {
    /// Reads every `*.json` of the folder; `ops.json` describes operations. Missing folder = empty.
    pub fn load(dir: &Path) -> Descriptions {
        let mut d = Descriptions::default();
        let Ok(read) = std::fs::read_dir(dir) else { return d };
        for file in read.flatten() {
            let path = file.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let Ok(Value::Object(map)) = serde_json::from_str::<Value>(&text) else { continue };
            let is_ops = path.file_stem().and_then(|s| s.to_str()) == Some("ops");
            for (id, v) in map {
                let get = |k: &str| v[k].as_str().unwrap_or("").to_owned();
                let entry = (get("kisa"), get("uzun"), get("guven"));
                if is_ops { d.ops.insert(id, entry) } else { d.nodes.insert(id, entry) };
            }
        }
        d
    }
}

pub const FORMAT: &str = "uw-flow/1";

/// Operations whose first argument names the thing the arrow points to.
const EDGE_OPS: [&str; 5] = ["jump_to_menu", "call_script", "start_presentation", "jump_to_scene", "set_jump_mission"];

/// Menu layout (module_game_menus.py:20): id, flags, text, mesh, opening code, options.
/// Option layout (module_game_menus.py:28): id, condition block, text, consequence block.
const MENU_TEXT: usize = 2;
const MENU_ON_ENTER: usize = 4;
const MENU_OPTIONS: usize = 5;

/// One root: where a view starts and why.
pub struct Root {
    pub id: String,
    pub why: String,
}

/// The default roots: menu #0 (the engine opens it on New Game) and start_phase_2 (the engine
/// opens it after character creation, proven in game: D4). Only roots that exist are returned.
pub fn default_roots(atlas: &Atlas) -> Vec<Root> {
    let mut roots = Vec::new();
    if let Some(first) = atlas.tables.get("menus").and_then(|t| t.entries.first()) {
        roots.push(Root { id: menu_label(first), why: "menu #0: the engine opens it on New Game".into() });
    }
    if let Some((_, entry)) = atlas.entry_by_id("menu_start_phase_2") {
        let id = menu_label(entry);
        if roots.iter().all(|r| r.id != id) {
            roots.push(Root { id, why: "the engine opens it after character creation (D4)".into() });
        }
    }
    roots
}

/// The node id of a menu: its kit id, or `menu_<name>#<dup>` for a shadowed duplicate.
fn menu_label(entry: &uw_atlas::Entry) -> String {
    match &entry.id {
        Some(id) => id.clone(),
        None => {
            let name = entry.name.as_deref().unwrap_or("").to_lowercase();
            match entry.dup {
                Some(dup) => format!("menu_{name}#{dup}"),
                None => format!("menu_{name}"),
            }
        }
    }
}

/// `{"list": [...]}` -> the list; a bare list stays as it is; anything else is empty.
fn block(value: &Value) -> &[Value] {
    match value {
        Value::Object(map) => map.get("list").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]),
        Value::Array(items) => items,
        _ => &[],
    }
}

/// The node id an operation argument points to, as the kit resolves it: the FIRST entry with
/// that name (`find_object`). A shadowed duplicate has no kit id; it gets `<prefix><name>#<dup>`,
/// the same label its own node carries.
fn target_id(atlas: &Atlas, text: &str) -> Option<String> {
    let (table, entry) = atlas.resolve_ref(text)?;
    if let Some(id) = &entry.id {
        return Some(id.clone());
    }
    let prefix = atlas.tables.get(table)?.id_prefix.clone().unwrap_or_default();
    let name = entry.name.as_deref()?.to_lowercase();
    Some(match entry.dup {
        Some(dup) => format!("{prefix}{name}#{dup}"),
        None => format!("{prefix}{name}"),
    })
}

/// The arrows (`{op, to}`) leaving an operation block, in statement order.
fn goes_of(atlas: &Atlas, ops: &Value) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for statement in block(ops) {
        let Some(items) = statement.as_array() else { continue };
        let Some(code) = items.first().and_then(Value::as_u64) else { continue };
        let Some(op) = atlas.op_for_code(code) else { continue };
        let name = op.name.as_str();
        if let Some(screen) = name.strip_prefix("change_screen_") {
            out.push((name.to_owned(), format!("screen:{screen}")));
        } else if EDGE_OPS.contains(&name) {
            if let Some(to) = items.get(1).and_then(Value::as_str).and_then(|t| target_id(atlas, t)) {
                out.push((name.to_owned(), to));
            }
        }
    }
    out
}

/// Every statement of an operation block as `{op, args, not, or_next, text_tr}`: what a button
/// does or when it is shown, in order. `args` are written as in the kit (`"$x"`, `"trp_player"`,
/// numbers as digits); `text_tr` is the operation's short description, if known.
fn statements(atlas: &Atlas, ops: &Value, desc: &Descriptions) -> Value {
    let flag = |name: &str| atlas.op(name).map(|op| op.code).unwrap_or(0);
    let (neg, or_next) = (flag("neg"), flag("this_or_next"));
    let mut out = Vec::new();
    for statement in block(ops) {
        let Some(items) = statement.as_array() else { continue };
        let Some(code) = items.first().and_then(Value::as_u64) else { continue };
        let Some(op) = atlas.op_for_code(code) else { continue };
        let args: Vec<Value> = items[1..]
            .iter()
            .map(|a| Value::String(match a {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            }))
            .collect();
        let text = desc.ops.get(&op.name).map(|d| d.0.clone()).unwrap_or_default();
        out.push(json!({
            "op": op.name,
            "args": args,
            "not": neg != 0 && code & neg != 0,
            "or_next": or_next != 0 && code & or_next != 0,
            "text_tr": text,
        }));
    }
    Value::Array(out)
}

/// The description fields every node carries (empty strings when unknown).
fn describe_node(node: &mut Value, desc: &Descriptions) {
    let id = node["id"].as_str().unwrap_or("").to_owned();
    // a screen stub is described by the operation that opens it (`screen:map` <- change_screen_map)
    let found = match id.strip_prefix("screen:") {
        Some(rest) => desc.ops.get(&format!("change_screen_{rest}")),
        None => desc.nodes.get(&id),
    };
    let (short, long, conf) = found.cloned().unwrap_or_default();
    node["summary_tr"] = Value::String(short);
    node["detail_tr"] = Value::String(long);
    node["confidence"] = Value::String(conf);
}

fn goes_json(goes: &[(String, String)]) -> Value {
    Value::Array(goes.iter().map(|(op, to)| json!({"op": op, "to": to})).collect())
}

/// Stub kind from the id's prefix; unknown prefixes are `other` (the viewer draws them generically).
fn stub_kind(id: &str) -> &'static str {
    if id.starts_with("screen:") {
        return "screen";
    }
    match id.split('_').next().unwrap_or("") {
        "script" => "script",
        "mst" => "mission_template",
        "scn" => "scene",
        "prsnt" => "presentation",
        "menu" => "menu",
        _ => "other",
    }
}

/// The whole flow document. `roots` empty = [`default_roots`].
pub fn flow(atlas: &Atlas, roots: Vec<Root>, generator: &str, desc: &Descriptions) -> Value {
    let roots = if roots.is_empty() { default_roots(atlas) } else { roots };
    let mut nodes: Vec<Value> = Vec::new();
    let mut edges: Vec<Value> = Vec::new();
    let empty = Vec::new();
    let menus = atlas.tables.get("menus").map(|t| &t.entries).unwrap_or(&empty);
    for entry in menus {
        let fields = block(&entry.value);
        let label = menu_label(entry);
        let mut options = Vec::new();
        if let Some(list) = fields.get(MENU_OPTIONS) {
            for option in block(list) {
                let parts = block(option);
                if parts.len() < 4 {
                    continue;
                }
                let index = options.len();
                let id = format!("mno_{}", scalar_text(&parts[0]));
                let goes = goes_of(atlas, &parts[3]);
                for (op, to) in &goes {
                    edges.push(json!({"from": label, "to": to, "op": op, "via_option": id, "option_index": index}));
                }
                options.push(json!({
                    "index": index,
                    "id": id,
                    "text": parts[2].as_str().unwrap_or(""),
                    "conditional": !block(&parts[1]).is_empty(),
                    "goes_to": goes_json(&goes),
                    "condition": statements(atlas, &parts[1], desc),
                    "does": statements(atlas, &parts[3], desc),
                }));
            }
        }
        let on_enter = fields.get(MENU_ON_ENTER).map(|ops| goes_of(atlas, ops)).unwrap_or_default();
        for (op, to) in &on_enter {
            edges.push(json!({"from": label, "to": to, "op": op, "via_option": null, "option_index": null}));
        }
        let source = match entry.line {
            Some(line) => format!("module_game_menus.py:{line}"),
            None => "module_game_menus.py".to_owned(),
        };
        nodes.push(json!({
            "id": label,
            "kind": "menu",
            "index": entry.index,
            "text": fields.get(MENU_TEXT).and_then(Value::as_str).unwrap_or(""),
            "source": source,
            "stub": false,
            "on_enter_goes_to": goes_json(&on_enter),
            "on_enter_does": fields.get(MENU_ON_ENTER).map(|ops| statements(atlas, ops, desc)).unwrap_or(json!([])),
            "options": options,
        }));
    }
    // Every arrow target gets a node: what is not a menu becomes a small stub.
    let mut known: HashSet<String> = nodes.iter().filter_map(|n| n["id"].as_str().map(str::to_owned)).collect();
    let mut stubs = Vec::new();
    for edge in &edges {
        let to = edge["to"].as_str().unwrap_or_default();
        if known.insert(to.to_owned()) {
            stubs.push(json!({
                "id": to, "kind": stub_kind(to), "index": -1, "text": "", "source": "", "stub": true,
                "on_enter_goes_to": [], "on_enter_does": [], "options": [],
            }));
        }
    }
    nodes.extend(stubs);
    for node in &mut nodes {
        describe_node(node, desc);
    }
    let mut doc = Map::new();
    doc.insert("format".into(), FORMAT.into());
    doc.insert("generator".into(), generator.into());
    doc.insert("kit_hash".into(), atlas.kit_hash.clone().map(Value::from).unwrap_or(Value::Null));
    doc.insert(
        "roots".into(),
        Value::Array(roots.into_iter().map(|r| json!({"id": r.id, "why": r.why})).collect()),
    );
    doc.insert("nodes".into(), Value::Array(nodes));
    doc.insert("edges".into(), Value::Array(edges));
    Value::Object(doc)
}

/// Option ids are written bare in the kit (`"start_male"`); numbers are kept as their digits.
fn scalar_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Short summary line for the terminal: `nodes N, options N, edges N`.
pub fn summary(doc: &Value) -> String {
    let nodes = doc["nodes"].as_array().map(Vec::len).unwrap_or(0);
    let options: usize = doc["nodes"]
        .as_array()
        .map(|ns| ns.iter().map(|n| n["options"].as_array().map(Vec::len).unwrap_or(0)).sum())
        .unwrap_or(0);
    let edges = doc["edges"].as_array().map(Vec::len).unwrap_or(0);
    format!("nodes {nodes}, options {options}, edges {edges}")
}
