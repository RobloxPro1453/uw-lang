//! The engine's own words.
//!
//! The core language does not invent engine vocabulary: every property it knows about the engine comes from
//! `kit-meta/easy/ops.json`, the easy names generated from the kit's `header_operations.py` (D0: 1060/1060
//! named). This module turns the entries whose parameter list has a plain shape into [`Property`] entries,
//! the same data the hand-made `kit-meta/core/properties.toml` holds; the lowering cannot tell them apart.
//!
//! Shapes taken (anything else is left out until the language has a construct for it):
//! - `property_get (destination, <subject>_id)` → `agent.horse` reads into `{dest}` from `{object}`;
//! - `property_get (destination)` → `mission.timer_c_msec` (no object: the subject is the whole game);
//! - `property_get (position_no, <subject>_id)` → a position value (`agent.speed`, `agent.position`);
//! - `property_set (<subject>_id, value | string…)` and `property_set (value)`;
//! - `condition (<subject>_id)` → `agent.is_alive`;
//! - `method`/`function (<subject>_id, params…)` → an action on a thing: `scene.town_6_center.jump(0)`;
//! - `method`/`function (params…)` → an action of the whole game: `screen.mission()`, `display.message("hi")`.
//!
//! A setter or action parameter that names a thing (`troop_id`, `mission_template_id`) takes the engine's id
//! as it is (`troop.player`, `mission_template.town_center`); a position parameter is not taken yet.
//!
//! Numbers: the engine counts whole units, the language thousandths, so a kit value is `unit = "int"`
//! (× 1000 when read, ÷ 1000 when written); a string parameter is `unit = "text"`.

use serde::Deserialize;

use super::{Property, Step};

/// The generated easy vocabulary as committed (`uw easy` writes it).
pub const OPS_PATH: &str = "kit-meta/easy/ops.json";
const OPS_JSON: &str = include_str!("../../../../kit-meta/easy/ops.json");

#[derive(Debug, Deserialize)]
struct OpsFile {
    ops: Vec<KitOp>,
}

/// One entry of `ops.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct KitOp {
    /// Easy name (`agent.hit_points`): `<subject>.<name>`.
    pub easy: String,
    /// The Native operation (`agent_set_hit_points`).
    pub native: String,
    /// `property_get`, `property_set`, `condition`, `method`, `function`, `text`, `flow`.
    pub kind: String,
    /// The object kind the operation works on (`agent`, `troop`, `mission`, …).
    pub subject: String,
    /// Parameter names from the kit's comment (`destination`, `agent_id`, `value`, …); empty when the kit
    /// documents the operation in a form the generator could not read.
    #[serde(default)]
    pub params: Vec<String>,
    /// The kit's comment; a parameter written `[name]` there is optional (the engine defaults it to 0).
    #[serde(default)]
    pub doc: String,
}

/// The kit's constants by their easy names (`ca.strength` → `ca_strength`), as `uw easy` writes them.
pub const CONSTANTS_PATH: &str = "kit-meta/easy/constants.json";
const CONSTANTS_JSON: &str = include_str!("../../../../kit-meta/easy/constants.json");

#[derive(Debug, Deserialize)]
struct KitConstant {
    easy: String,
    native: String,
}

fn constant_list() -> Vec<KitConstant> {
    serde_json::from_str::<Vec<KitConstant>>(CONSTANTS_JSON).unwrap_or_default()
}

/// The Native name of a kit constant written by its easy name (`ca.strength` → `ca_strength`).
pub fn kit_constant(easy: &str) -> Option<&'static str> {
    use std::collections::HashMap;
    use std::sync::OnceLock;
    static MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
    MAP.get_or_init(|| constant_list().into_iter().map(|c| (c.easy, c.native)).collect())
        .get(easy)
        .map(String::as_str)
}

/// The other way: Native constant → easy name (`ca_strength` → `ca.strength`), for the read-only view.
pub fn kit_constant_names() -> std::collections::HashMap<String, String> {
    constant_list().into_iter().map(|c| (c.native, c.easy)).collect()
}

/// Every operation of the committed easy vocabulary.
pub fn kit_ops() -> Vec<KitOp> {
    serde_json::from_str::<OpsFile>(OPS_JSON).map(|file| file.ops).unwrap_or_default()
}

/// Bare nouns that name an engine object directly without `_id` / `_no`.
const BARE_OBJECTS: &[&str] = &[
    "agent", "troop", "party", "party_template", "faction", "item", "quest", "scene", "menu",
    "skill", "sound", "mesh", "presentation", "player", "team", "overlay", "prop_instance",
    "scene_prop", "tableau", "anim", "mission_template", "track", "map_icon",
];

fn base_param(name: &str) -> &str {
    match name.split_once("__dup") {
        Some((stem, _)) => stem,
        None => name,
    }
}

/// True for a parameter that names the object the operation works on (`agent_id`, `troop_no`).
fn is_object_param(raw: &str) -> bool {
    let name = base_param(raw);
    BARE_OBJECTS.contains(&name)
        || ((name.ends_with("_id") || name.ends_with("_no"))
            && !name.starts_with("position")
            && !name.starts_with("string")
            && name != "slot_no")
}

fn is_text_param(raw: &str) -> bool {
    base_param(raw).starts_with("string")
}

fn is_position_param(raw: &str) -> bool {
    let name = base_param(raw);
    name.starts_with("position") || name.starts_with("pos_") || name.ends_with("_position_no")
}

fn is_fixed_param(raw: &str) -> bool {
    base_param(raw).ends_with("_fixed_point")
}

/// How a value goes into a kit parameter: `id` (a thing's reference, as it is), `text`, `fixed`, or `int` (whole units).
pub fn param_unit(name: &str) -> &'static str {
    if is_text_param(name) {
        "text"
    } else if is_fixed_param(name) {
        "fixed"
    } else if is_object_param(name) {
        "id"
    } else {
        "int"
    }
}

/// The unit of a setter's value: a thing's id goes in as it is (no unit), a text, or whole units.
fn value_unit(name: &str) -> Option<String> {
    match param_unit(name) {
        "id" | "fixed" => None,
        unit => Some(unit.to_owned()),
    }
}

/// True when the parameter is the thing the operation works on: an object parameter of the subject's kind
/// (`scene_id` of `scene.jump`), not just any id (`string_id` of `display.message`).
fn is_subject_param(raw: &str, subject: &str) -> bool {
    let name = base_param(raw);
    is_object_param(name)
        && (name == subject
            || name.strip_prefix(subject).is_some_and(|rest| rest.starts_with('_'))
            || (subject == "party" && name.starts_with("center_")))
}

/// Normalizes parameter names into unique identifier placeholders (`duration-in-ms` → `duration_in_ms`,
/// duplicate `value` → `value__dup1`, `value__dup2`); `None` if any parameter is a position register or unusable.
fn clean_params(params: &[&str]) -> Option<Vec<String>> {
    let mut normalized = Vec::with_capacity(params.len());
    for raw in params {
        if raw.is_empty() || is_position_param(raw) {
            return None;
        }
        let mut clean = String::with_capacity(raw.len());
        for ch in raw.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                clean.push(ch);
            } else if ch == '-' || ch == '/' {
                clean.push('_');
            } else {
                return None;
            }
        }
        if clean.is_empty() {
            return None;
        }
        normalized.push(clean);
    }
    let mut out = Vec::with_capacity(normalized.len());
    for (i, name) in normalized.iter().enumerate() {
        let total = normalized.iter().filter(|other| *other == name).count();
        if total == 1 {
            out.push(name.clone());
        } else {
            let ord = normalized[..=i].iter().filter(|other| *other == name).count();
            out.push(format!("{name}__dup{ord}"));
        }
    }
    Some(out)
}

fn sig_key(path: &str) -> (&str, Option<usize>) {
    match path.split_once('(') {
        Some((head, inner)) => {
            let inner = inner.trim_end_matches(')').trim();
            let arity = if inner.is_empty() { 0 } else { inner.split(',').count() };
            (head, Some(arity))
        }
        None => (path, None),
    }
}

fn step(op: &str, args: &[&str]) -> Step {
    Step { op: op.to_owned(), args: args.iter().map(|arg| (*arg).to_owned()).collect() }
}

fn empty(path: &str) -> Property {
    Property {
        path: path.to_owned(),
        unit: None,
        get: Vec::new(),
        set: Vec::new(),
        cond: Vec::new(),
        run: Vec::new(),
        aliases: Default::default(),
        vector: false,
        kit: true,
    }
}

/// The properties the kit vocabulary gives, one per easy name (a getter and a setter of the same name
/// share one entry, the first operation of a kind wins). `ops` is usually [`kit_ops`].
pub fn kit_properties(ops: &[KitOp]) -> Vec<Property> {
    let mut out: Vec<Property> = Vec::new();
    for op in ops {
        let mut params: Vec<&str> = op.params.iter().map(String::as_str).collect();
        // Trailing optional parameters (`[absolute]`) are left at the engine's default.
        while params.last().is_some_and(|last| op.doc.contains(&format!("[{last}]")) || op.doc.contains(&format!("[<{last}>]"))) {
            params.pop();
        }
        if let Some(entry) = name_getter(op) {
            add_or_merge(&mut out, entry);
        }
        let Some(entry) = shape(op, &params) else { continue };
        let key = sig_key(&entry.path);
        match out.iter_mut().find(|prop| sig_key(&prop.path) == key) {
            Some(prop) if prop.path == entry.path => merge(prop, entry),
            Some(_) => {}
            None => out.push(entry),
        }
    }
    out
}

/// `str_store_<thing>_name(<string_register>, <thing>)` also reads `<thing>.name` (`troop.player.name`), next to
/// the setter `<thing>_set_name` the kit already names `<thing>.name`. A general pairing, the same for every thing.
fn name_getter(op: &KitOp) -> Option<Property> {
    let subject = op.native.strip_prefix("str_store_")?.strip_suffix("_name")?;
    if !BARE_OBJECTS.contains(&subject) {
        return None;
    }
    let [register, object] = op.params.as_slice() else { return None };
    if !base_param(register).starts_with("str") || base_param(object) != format!("{subject}_id") {
        return None;
    }
    let mut prop = empty(&format!("{subject}.name"));
    prop.unit = Some("text".to_owned());
    prop.get = vec![step(&op.native, &["{dest}", "{object}"])];
    Some(prop)
}

fn add_or_merge(out: &mut Vec<Property>, entry: Property) {
    let key = sig_key(&entry.path);
    match out.iter_mut().find(|prop| sig_key(&prop.path) == key) {
        Some(prop) if prop.path == entry.path => merge(prop, entry),
        Some(_) => {}
        None => out.push(entry),
    }
}

/// One kit operation as a property, if its parameters have a shape the language can use.
fn shape(op: &KitOp, params: &[&str]) -> Option<Property> {
    if op.easy.split('.').count() != 2 || op.easy.contains('(') {
        return None;
    }
    let native = op.native.as_str();
    if matches!(native, "eq" | "neq" | "gt" | "ge" | "lt" | "le" | "is_between") {
        return None;
    }
    let mut prop = empty(&op.easy);
    match (op.kind.as_str(), params) {
        ("property_get", ["destination", object]) if is_object_param(object) => {
            prop.unit = Some("int".to_owned());
            prop.get = vec![step(native, &["{dest}", "{object}"])];
        }
        ("property_get", ["destination"]) => {
            prop.unit = Some("int".to_owned());
            prop.get = vec![step(native, &["{dest}"])];
        }
        ("property_get", ["position_no", object]) if is_object_param(object) => {
            prop.vector = true;
            prop.get = vec![step(native, &["{dest}", "{object}"])];
        }
        ("property_get", ["destination_fixed_point", object, rest @ ..])
            if is_subject_param(object, &op.subject) =>
        {
            let clean = clean_params(rest)?;
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{dest}".to_owned(), "{object}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.get = vec![
                step("set_fixed_point_multiplier", &["1000"]),
                Step { op: native.to_owned(), args },
            ];
        }
        ("property_get", ["destination_fixed_point", rest @ ..]) => {
            let clean = clean_params(rest)?;
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{dest}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.get = vec![
                step("set_fixed_point_multiplier", &["1000"]),
                Step { op: native.to_owned(), args },
            ];
        }
        ("property_get", ["destination", object, rest @ ..])
            if !rest.is_empty() && is_subject_param(object, &op.subject) =>
        {
            let clean = clean_params(rest)?;
            prop.unit = Some("int".to_owned());
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{dest}".to_owned(), "{object}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.get = vec![Step { op: native.to_owned(), args }];
        }
        ("property_get", ["destination", rest @ ..]) if !rest.is_empty() => {
            let clean = clean_params(rest)?;
            prop.unit = Some("int".to_owned());
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{dest}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.get = vec![Step { op: native.to_owned(), args }];
        }
        ("property_set", [object, value]) if is_object_param(object) && !is_position_param(value) => {
            prop.unit = value_unit(value);
            prop.set = vec![step(native, &["{object}", "{value}"])];
        }
        ("property_set", [value]) if !is_position_param(value) => {
            prop.unit = value_unit(value);
            prop.set = vec![step(native, &["{value}"])];
        }
        ("property_set" | "method" | "function", [object, rest @ ..])
            if is_subject_param(object, &op.subject) =>
        {
            let clean = clean_params(rest)?;
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{object}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.run = vec![Step { op: native.to_owned(), args }];
        }
        ("property_set" | "method" | "function", params) => {
            if params.first().is_some_and(|first| is_subject_param(first, &op.subject)) {
                return None;
            }
            let clean = clean_params(params)?;
            prop.path = action_path(&op.easy, &clean);
            prop.run = vec![Step {
                op: native.to_owned(),
                args: clean.iter().map(|name| format!("{{{name}}}")).collect(),
            }];
        }
        // `str_store_troop_name(<string_register>, <troop_id>)` → a value: `text.store_troop_name(troop.player)`
        // is a text (the register is the compiler's, CORE-v0 §4). Operations that change a register in place
        // (`str_clear`, `str_encode_url`) or test one (`str_is_empty`) have no use while registers never show.
        ("text", [register, rest @ ..]) if native.starts_with("str_store_") && base_param(register).starts_with("str") => {
            let clean = clean_params(rest)?;
            prop.unit = Some("text".to_owned());
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{dest}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.get = vec![Step { op: native.to_owned(), args }];
        }
        ("condition", [object]) if is_object_param(object) => {
            prop.cond = vec![step(native, &["{object}"])];
        }
        ("condition", [object, rest @ ..])
            if !rest.is_empty() && is_subject_param(object, &op.subject) =>
        {
            let clean = clean_params(rest)?;
            prop.path = action_path(&op.easy, &clean);
            let mut args = vec!["{object}".to_owned()];
            args.extend(clean.iter().map(|name| format!("{{{name}}}")));
            prop.cond = vec![Step { op: native.to_owned(), args }];
        }
        ("condition", params) => {
            let clean = clean_params(params)?;
            prop.path = action_path(&op.easy, &clean);
            prop.cond = vec![Step {
                op: native.to_owned(),
                args: clean.iter().map(|name| format!("{{{name}}}")).collect(),
            }];
        }
        _ => return None,
    }
    Some(prop)
}

/// The call pattern of an action: `scene.jump` with `entry_no` → `scene.jump({entry_no})`.
fn action_path(easy: &str, params: &[String]) -> String {
    let inner: Vec<String> = params.iter().map(|name| format!("{{{name}}}")).collect();
    format!("{easy}({})", inner.join(", "))
}

/// Adds the operations of `more` to `prop` where `prop` has none of that kind (getter + setter pairs).
fn merge(prop: &mut Property, more: Property) {
    if prop.get.is_empty() && !more.get.is_empty() {
        prop.get = more.get;
        prop.vector = more.vector;
        if prop.unit.is_none() {
            prop.unit = more.unit.clone();
        }
    }
    if prop.set.is_empty() && !more.set.is_empty() {
        prop.set = more.set;
        if prop.unit.is_none() {
            prop.unit = more.unit;
        }
    }
    if prop.cond.is_empty() && !more.cond.is_empty() {
        prop.cond = more.cond;
    }
    if prop.run.is_empty() && !more.run.is_empty() {
        prop.run = more.run;
    }
}
