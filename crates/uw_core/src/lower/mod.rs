//! Lowering: [`ast::File`]s → [`Lowered`] Native operations (docs/dil/CORE-v0.md §4).
//!
//! The crate's rule holds here too: no property name is written in Rust. A property
//! (`player.name`, `key.held(alt)`, …) is data in `kit-meta/core/properties.toml`; the code below only knows
//! the language's fixed constructs — variables, assignments, `if`/`for`/`let`/`fn`, `on game.start`.
//!
//! Numbers are the language's fixed-point thousandths ([`crate::ast::Expr::Number`]); a property entry says
//! in which unit the engine wants the value (`unit = "int"`: ÷1000, `"percent"`: the percent as written).

pub mod kit;
pub mod time;
pub mod ui;

use std::collections::BTreeMap;
use std::path::Path as FsPath;

use serde::Deserialize;
use uw_atlas::Atlas;

use crate::ast::{
    AssignOp, BinOp, Block, Call, Clock, Expr, File, Ident, Item, Path, RateUnit, Span, Stmt, TextPart,
    VarDecl,
};
use crate::slots::SlotLock;
use crate::{Arg, Error, Lowered, Op};

/// The spawned agent inside the agent-spawn trigger.
const AGENT_LOCAL: &str = ":uw_agent";

/// Where the property vocabulary lives, relative to the repository root.
pub const PROPERTIES_PATH: &str = "kit-meta/core/properties.toml";

/// The vocabulary as committed (so the tests and the build need no path lookup).
const PROPERTIES_TOML: &str = include_str!("../../../../kit-meta/core/properties.toml");

/// The hand-made property vocabulary of the core language (CORE-v0 §4).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CoreData {
    /// One entry per property (`[[property]]` in the TOML).
    #[serde(default, rename = "property")]
    pub properties: Vec<Property>,
}

/// One property: a path pattern and the operations it stands for.
#[derive(Debug, Clone, Deserialize)]
pub struct Property {
    /// Dotted pattern; `{name}` in the last part marks a call argument (`key.held({key})`).
    pub path: String,
    /// Engine unit of the value at this boundary: `int`, `percent` or `text`.
    #[serde(default)]
    pub unit: Option<String>,
    /// Operations that write the property's value into `{dest}`.
    #[serde(default)]
    pub get: Vec<Step>,
    /// Operations that store `{value}` into the property.
    #[serde(default)]
    pub set: Vec<Step>,
    /// Operations that test the property (a Native condition).
    #[serde(default)]
    pub cond: Vec<Step>,
    /// Operations of an action: run as a statement, with no value (`scene.town_6_center.jump(0)`).
    #[serde(default)]
    pub run: Vec<Step>,
    /// Other spellings accepted as a call argument (`alt` → `left_alt`).
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
    /// The getter writes a position register (`{dest}` is one): the value is a position/vector, read as
    /// `.x .y .z` or, as a number, its length (CORE-v0 §4).
    #[serde(default)]
    pub vector: bool,
    /// Generated from the kit vocabulary (`kit-meta/easy/ops.json`), not written by hand.
    #[serde(skip)]
    pub kit: bool,
}

/// One operation template with its arguments (`{dest}`, `{value}`, `{object}`, a call argument or a literal).
#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl CoreData {
    /// The vocabulary compiled into this build: the hand-made mechanism entries of
    /// `kit-meta/core/properties.toml` followed by the engine's own words from the kit vocabulary
    /// (`kit-meta/easy/ops.json`, [`kit::kit_properties`]).
    pub fn builtin() -> Result<CoreData, Error> {
        let mut data = CoreData::from_toml(PROPERTIES_TOML, PROPERTIES_PATH)?;
        data.properties.extend(kit::kit_properties(&kit::kit_ops()));
        Ok(data)
    }

    /// Only the hand-made entries (`kit-meta/core/properties.toml`).
    pub fn hand_made() -> Result<CoreData, Error> {
        CoreData::from_toml(PROPERTIES_TOML, PROPERTIES_PATH)
    }

    /// Reads a vocabulary file from disk.
    pub fn from_path(path: impl AsRef<FsPath>) -> Result<CoreData, Error> {
        let path = path.as_ref();
        let origin = path.display().to_string();
        let text = std::fs::read_to_string(path).map_err(|error| Error {
            file: origin.clone(),
            line: 0,
            message: error.to_string(),
        })?;
        CoreData::from_toml(&text, &origin)
    }

    /// Parses vocabulary text; `origin` is the file name the errors point at.
    pub fn from_toml(text: &str, origin: &str) -> Result<CoreData, Error> {
        toml::from_str(text).map_err(|error| {
            let at = error.span().map_or(0, |span| span.start);
            let before = text.get(..at).unwrap_or("");
            Error {
                file: origin.to_owned(),
                line: before.matches('\n').count() as u32 + 1,
                message: error.message().to_owned(),
            }
        })
    }
}

/// Lowers every core file into the operations the build writes into the kit.
///
/// Every error is collected (`file:line`, see [`Error`]) instead of stopping at the first one.
pub fn lower(files: &[File], atlas: &Atlas, data: &CoreData) -> Result<Lowered, Vec<Error>> {
    lower_locked(files, atlas, data, &SlotLock::new())
}

/// [`lower`] for a project that has been built before: every field already in `lock` keeps its slot,
/// new fields get numbers above the kit's and the lock's, and [`Lowered::slot_lock`] is the lock to
/// write back §2).
pub fn lower_locked(files: &[File], atlas: &Atlas, data: &CoreData, lock: &SlotLock) -> Result<Lowered, Vec<Error>> {
    let mut ctx = Ctx::new(atlas, data);
    ctx.lock = lock.clone();
    let mut errors = Vec::new();
    ctx.declare(files, &mut errors);
    // A feature variable starts with its declared value when the game starts (CORE-v0 §4), so every
    // `game_start` body sees all of them.
    for file in files {
        ctx.enter(file);
        for item in &file.items {
            let Item::Var(decl) = item else { continue };
            if let (Some(kind), Some(_)) = (&decl.kind, &decl.init) {
                // A field's start value: agents get it when they spawn (CORE-v0 §5.2).
                if let Err(error) = ctx.init_field(kind, decl) {
                    errors.push(error);
                }
                continue;
            }
            if decl.kind.is_some() || decl.cooldown.is_some() {
                continue; // an object field is a slot (a cooldown is an error, reported by `declare`)
            }
            if let Err(error) = ctx.init_var(decl) {
                errors.push(error);
            }
        }
    }
    for file in files {
        ctx.enter(file);
        for item in &file.items {
            if let Err(error) = ctx.item(item) {
                errors.push(error);
            }
        }
    }
    for (feature, name, file, line) in &ctx.cancels {
        if !ctx.after_names.contains_key(&(feature.clone(), name.clone())) {
            errors.push(Error {
                file: file.clone(),
                line: *line,
                message: format!("`cancel {name}`: this feature has no `after … as {name}`"),
            });
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ctx.finish())
}

/// A value and its scale: the language's thousandths, or the engine's own number (an id, a 0/1 flag).
enum Scaled {
    Lang(Arg),
    Engine(Arg),
}

/// A declared place that is written with `assign`/`val_*`: a local or a UW global.
enum Var {
    Local(String),
    Global { name: String, range: Option<(i64, i64)> },
}

/// A declared object field (`var agent.stamina`).
struct Field {
    slot: i64,
    range: Option<(i64, i64)>,
    /// Where it was declared, for the "already declared" message.
    file: String,
    line: u32,
}

/// The property entry that matched a path.
struct Hit {
    prop: usize,
}

/// A value substituted into an operation template: a lowered value, or a bare name (a constant to resolve).
enum SubArg {
    Arg(Arg),
    Name(String),
}

/// What `{…}` placeholders mean while one property entry is written out.
///
/// The object stays a path: it is only lowered when an operation asks for it (`{object}`), so `player.name`
/// never has to make sense of the word `player` on its own.
struct Subst<'p> {
    object: Option<&'p Path>,
    value: Option<Arg>,
    args: Vec<(String, SubArg)>,
}

impl Subst<'_> {
    fn arg(&self, name: &str) -> Option<&Arg> {
        // A call argument wins: a kit action's parameter may itself be called `value` (`raise_skill`).
        if let Some((_, SubArg::Arg(arg))) = self.args.iter().find(|(key, _)| key == name) {
            return Some(arg);
        }
        match name {
            // A value-producing entry writes into `{dest}`; a set entry takes the same argument as `{value}`.
            "dest" | "value" => self.value.as_ref(),
            _ => None,
        }
    }

    fn text(&self, name: &str) -> Option<&str> {
        self.args.iter().find(|(key, _)| key == name).and_then(|(_, arg)| match arg {
            SubArg::Name(text) => Some(text.as_str()),
            SubArg::Arg(_) => None,
        })
    }
}

/// The lowering state: the vocabulary, the declarations of all files, and the block being written.
///
/// Later tasks extend this context (continuous blocks, UI, the start flow); [`Ctx::expr`],
/// [`Ctx::place_set`], [`Ctx::temp`], [`Ctx::slot_for`], [`Ctx::global_for`], [`Ctx::begin_block`] and
/// [`Ctx::end_block`] are the handles they build on.
pub struct Ctx<'a> {
    atlas: &'a Atlas,
    data: &'a CoreData,
    /// Blocks being filled; the last one receives the operations.
    blocks: Vec<Vec<Op>>,
    next_temp: usize,
    /// Vector values handed out so far (rotates over the compiler's position registers).
    next_pos: usize,
    /// The file being lowered (`file:line` in messages).
    file: String,
    /// The feature being lowered (the namespace of its variables and constants).
    feature: String,
    /// Constants per (feature, name).
    consts: BTreeMap<(String, String), Expr>,
    /// User-defined `fn` items per (feature, name) → (arity, returns_value).
    fns: BTreeMap<(String, String), (usize, bool)>,
    /// Feature variables per (feature, name) with their optional range.
    globals: BTreeMap<(String, String), Option<(i64, i64)>>,
    /// Object fields per (kind, name).
    fields: BTreeMap<(String, String), Field>,
    /// Slot numbers already given out, per (kind, name).
    slots: BTreeMap<(String, String), i64>,
    /// Next free slot per kind.
    next_slot: BTreeMap<String, i64>,
    /// Slots kept from earlier builds plus the ones given out now (`slots.lock`).
    lock: SlotLock,
    /// `let`/parameter locals in scope, innermost last.
    locals: Vec<BTreeMap<String, String>>,
    /// Constants being expanded, to catch a cycle.
    resolving: Vec<(String, String)>,
    /// Scripts made from `fn` items.
    scripts: Vec<(String, Vec<Op>)>,
    /// Operations appended to `game_start`.
    start: Vec<Op>,
    /// Continuous blocks (`while`) in source order: the body of the mission dispatcher.
    frame: Vec<Op>,
    frame_items: Vec<time::FrameItem>,
    /// True while lowering a continuous block (a rate `15 per second` is allowed there).
    continuous: bool,
    /// `every`/`after` blocks for the dispatchers.
    timed: Vec<time::Timed>,
    /// Per feature: `every` blocks and unnamed `after`s numbered so far.
    every_count: BTreeMap<String, usize>,
    after_count: BTreeMap<String, usize>,
    /// Named `after`s per (feature, name) → line.
    after_names: BTreeMap<(String, String), u32>,
    /// `cancel NAME`s, checked once every file is lowered: (feature, name, file, line).
    cancels: Vec<(String, String, String, u32)>,
    /// Something reads the mission clock: the mission dispatcher must run even with no block in it.
    mission_clock: bool,
    /// `ui.*` parts of the HUD presentation, in source order.
    hud: Vec<ui::HudPart>,
    /// Per feature: `on key.released` blocks numbered so far (their key-state globals).
    key_count: BTreeMap<String, usize>,
    /// Agent field start values, run by the agent-spawn trigger (the agent is in `:uw_agent`).
    agent_init: Vec<Op>,
    /// `on menu.<name>.enter` blocks per menu (name without its tag), appended to the menu's enter block.
    menu_appends: Vec<(String, Vec<Op>)>,
    /// String registers handed out so far for text values (rotates over the compiler's own `s60..s67`).
    next_str: usize,
    /// Number registers handed out so far for texts with values (rotates over `reg52..reg59`).
    next_text_reg: usize,
}

impl<'a> Ctx<'a> {
    pub fn new(atlas: &'a Atlas, data: &'a CoreData) -> Ctx<'a> {
        Ctx {
            atlas,
            data,
            blocks: vec![Vec::new()],
            next_temp: 0,
            next_pos: 0,
            file: String::new(),
            feature: String::new(),
            consts: BTreeMap::new(),
            fns: BTreeMap::new(),
            globals: BTreeMap::new(),
            fields: BTreeMap::new(),
            slots: BTreeMap::new(),
            next_slot: BTreeMap::new(),
            lock: SlotLock::new(),
            locals: Vec::new(),
            resolving: Vec::new(),
            scripts: Vec::new(),
            start: Vec::new(),
            frame: Vec::new(),
            frame_items: Vec::new(),
            continuous: false,
            timed: Vec::new(),
            every_count: BTreeMap::new(),
            after_count: BTreeMap::new(),
            after_names: BTreeMap::new(),
            cancels: Vec::new(),
            mission_clock: false,
            hud: Vec::new(),
            key_count: BTreeMap::new(),
            agent_init: Vec::new(),
            menu_appends: Vec::new(),
            next_str: 0,
            next_text_reg: 0,
        }
    }

    /// A fresh local for an intermediate value.
    pub fn temp(&mut self) -> Arg {
        let name = format!(":_uw{}", self.next_temp);
        self.next_temp += 1;
        Arg::Local(name)
    }

    /// The Native global of a feature variable (CORE-v0 §4: `$uw_<feature>_<name>`).
    pub fn global_for(feature: &str, name: &str) -> String {
        format!("$uw_{feature}_{name}")
    }

    /// The slot of a field (CORE-v0 §4): its number in `slots.lock` if it has one, otherwise one above
    /// every `slot_<kind>_*` constant of the atlas and every number the lock already holds.
    pub fn slot_for(&mut self, kind: &str, name: &str) -> i64 {
        let key = (kind.to_owned(), name.to_owned());
        if let Some(slot) = self.slots.get(&key) {
            return *slot;
        }
        let slot = match self.lock.get(kind, name) {
            Some(locked) => locked,
            None => {
                let above_kit = self.highest_slot(kind) + 1;
                let above_lock = self.lock.highest(kind).map_or(above_kit, |h| h + 1);
                let next = self.next_slot.get(kind).copied().unwrap_or(above_kit);
                let slot = above_kit.max(above_lock).max(next);
                self.lock.insert(kind, name, slot);
                slot
            }
        };
        let next = self.next_slot.get(kind).copied().unwrap_or(i64::MIN).max(slot + 1);
        self.next_slot.insert(kind.to_owned(), next);
        self.slots.insert(key, slot);
        slot
    }

    /// Starts a nested operation block (a condition, a hook body, a script).
    pub fn begin_block(&mut self) {
        self.blocks.push(Vec::new());
    }

    /// Closes the innermost block and gives its operations back.
    pub fn end_block(&mut self) -> Vec<Op> {
        self.blocks.pop().unwrap_or_default()
    }

    /// The value an expression stands for, written into the current block.
    pub fn expr(&mut self, e: &Expr) -> Result<Arg, Error> {
        match e {
            Expr::Number { milli, .. } => Ok(Arg::Int(*milli)),
            // A percent stays the number as written (`150%` → 150): that is the engine's percent unit.
            // `150%` is the number 150 (CORE-v0 §4): the engine's percent fields take it as written.
            Expr::Percent { value, .. } => Ok(Arg::Int(*value * 1000)),
            Expr::Color { rgb, .. } => Ok(Arg::Int(i64::from(*rgb))),
            Expr::Bool { value, .. } => Ok(Arg::Int(i64::from(*value))),
            Expr::Rate { milli, unit, span } => self.rate_amount(*milli, *unit, *span),
            // A duration as a value is an amount of its clock (CORE-v0 §4 "Time"): seconds for mission time
            // (`mission.time + 2 seconds`), hours for game time (`game.current_hours + 1 day`).
            Expr::Duration { milli, unit, .. } => Ok(Arg::Int(match unit.clock() {
                Clock::Mission => milli.saturating_mul(unit.ticks()) / 1_000,
                Clock::Game => milli.saturating_mul(unit.ticks()),
            })),
            Expr::Text { parts, span } => self.text_value(parts, *span),
            Expr::Tuple { span, .. } => Err(self.error(*span, "a tuple is only a range, in `for`")),
            Expr::Path(path) => self.value_path(path),
            Expr::Call(call) => self.call_value(call),
            Expr::Unary { neg, not, expr, span } => match (neg, not) {
                (_, true) => self.cond_value(e),
                (true, false) => {
                    let value = self.expr(expr)?;
                    self.arith(Arg::Int(0), "store_sub", value, *span)
                }
                (false, false) => self.expr(expr),
            },
            Expr::Binary { op, left, right, span } => match op {
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                    self.arith_expr(*op, left, right, *span)
                }
                BinOp::Or | BinOp::And | BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt
                | BinOp::Ge => self.cond_value(e),
            },
        }
    }

    /// Writes `value` into `place` with `op`, clamping where the place declares a range (CORE-v0 §4).
    pub fn place_set(&mut self, place: &Path, op: AssignOp, value: &Expr) -> Result<(), Error> {
        let span = place.span;
        let parts: Vec<String> = place.parts.iter().map(|part| part.text.clone()).collect();
        if let Some(var) = self.var_target(&parts) {
            return self.assign_var(var, op, value, span);
        }
        if let Some((kind, slot, range)) = self.field_target(&parts) {
            let object = self.object_of(place)?;
            return self.assign_field(&kind, slot, range, object, op, value, span);
        }
        if let Some(hit) = self.find_property(&parts, None) {
            let prop = self.data.properties[hit.prop].clone();
            if prop.set.is_empty() {
                return Err(self.error(span, &format!("'{}' cannot be written", prop.path)));
            }
            if op != AssignOp::Set {
                return Err(self.error(span, &format!("'{}' only takes `=`", prop.path)));
            }
            let value = self.expr(value)?;
            let value = self.unit_value(value, prop.unit.as_deref(), span)?;
            let subst = Subst { object: Some(place), value: Some(value), args: Vec::new() };
            return self.emit_steps(&prop.set, &subst, span);
        }
        Err(self.error(span, &format!("unknown place '{}'", parts.join("."))))
    }

    fn finish(mut self) -> Lowered {
        let mut lowered = Lowered::default();
        if !self.agent_init.is_empty() {
            // CORE-v0 §5.2: one `ti_on_agent_spawn` trigger per mission template gives every new agent
            // the start values of the agent fields.
            let agent = Arg::Local(AGENT_LOCAL.to_owned());
            let mut body = vec![Op { name: "store_trigger_param_1".into(), args: vec![agent], neg: false, this_or_next: false }];
            body.append(&mut self.agent_init);
            lowered.mission_triggers.push(crate::MissionTrigger {
                when: crate::TriggerWhen::Event("ti_on_agent_spawn".to_owned()),
                conditions: Vec::new(),
                body,
                target: crate::MissionTarget::WithPlayer,
            });
        }
        let mut frame = std::mem::take(&mut self.frame);
        if let Some((presentation, keep_up)) = self.hud_presentation() {
            // The HUD is started (again) first thing in every frame where it is not up.
            let parts = self.hud.len();
            let per_frame = presentation.on_run.len();
            self.frame_items.insert(0, time::FrameItem {
                file: "(hud)".to_owned(),
                line: 0,
                what: format!("keep prsnt_{} up ({parts} parts; it runs {per_frame} ops per frame itself)", ui::HUD),
                ops: keep_up.len(),
            });
            frame.splice(0..0, keep_up);
            lowered.presentations.push(presentation);
        }
        time::build(&mut lowered, &self.timed, frame, &self.frame_items, self.mission_clock);
        for (kind, name, slot) in self.lock.fields() {
            if !self.fields.contains_key(&(kind.to_owned(), name.to_owned())) {
                lowered.warnings.push(Error {
                    file: "slots.lock".to_owned(),
                    line: 0,
                    message: format!(
                        "field '{kind}.{name}' is no longer declared; its slot {slot} stays reserved so old saves \
                         are not read by another field (delete the line only if no saved game uses it)"
                    ),
                });
            }
        }
        lowered.slot_lock = self.lock.clone();
        lowered.menu_appends = std::mem::take(&mut self.menu_appends);
        let mut scripts = std::mem::take(&mut self.scripts);
        scripts.append(&mut lowered.scripts);
        lowered.scripts = scripts;
        if !self.start.is_empty() {
            lowered
                .script_appends
                .push(("game_start".to_owned(), std::mem::take(&mut self.start)));
        }
        lowered
    }

    fn enter(&mut self, file: &File) {
        self.feature = file.feature.clone();
        self.file = file.feature.clone();
        self.locals.clear();
    }

    fn error(&self, span: Span, message: &str) -> Error {
        Error { file: self.file.clone(), line: span.line, message: message.to_owned() }
    }

    fn emit(&mut self, name: &str, args: Vec<Arg>, span: Span) -> Result<(), Error> {
        // `lt`, `neq`, `le` are not operations of their own: the kit defines them as `neg|ge`,
        // `neg|eq`, `neg|gt` (header_operations.py:444-446), so the atlas has no entry for them.
        let alias = match name {
            "lt" => Some("ge"),
            "neq" => Some("eq"),
            "le" => Some("gt"),
            _ => None,
        };
        if let Some(base) = alias.filter(|_| self.atlas.op(name).is_none()) {
            if self.atlas.op(base).is_none() {
                return Err(self.error(span, &format!("the atlas has no operation '{base}'")));
            }
            self.push(Op { name: base.to_owned(), args, neg: true, this_or_next: false });
            return Ok(());
        }
        if self.atlas.op(name).is_none() {
            return Err(self.error(span, &format!("the atlas has no operation '{name}'")));
        }
        self.push(Op { name: name.to_owned(), args, neg: false, this_or_next: false });
        Ok(())
    }

    fn push(&mut self, op: Op) {
        match self.blocks.last_mut() {
            Some(block) => block.push(op),
            None => self.blocks.push(vec![op]),
        }
    }

    /// True when the operation is a Native condition (it fails the block when it does not hold).
    fn can_fail(&self, op: &Op) -> bool {
        op.name == "call_script" || self.atlas.op(&op.name).is_some_and(|known| known.can_fail)
    }

    /// One above the highest `slot_<kind>_*` constant of the atlas.
    fn highest_slot(&self, kind: &str) -> i64 {
        let prefix = format!("slot_{kind}_");
        self.atlas
            .constants
            .iter()
            .filter(|constant| constant.name.starts_with(&prefix))
            .filter_map(|constant| constant.value.as_i64())
            .max()
            .unwrap_or(-1)
    }

    /// A range bound: a number, or a constant that is one.
    fn const_value(&self, e: &Expr) -> Result<i64, Error> {
        match e {
            Expr::Number { milli, .. } => Ok(*milli),
            Expr::Path(path) if path.parts.len() == 1 => {
                let key = (self.feature.clone(), path.parts[0].text.clone());
                match self.consts.get(&key) {
                    Some(Expr::Number { milli, .. }) => Ok(*milli),
                    _ => Err(self.error(path.span, "a range bound must be a number")),
                }
            }
            other => Err(self.error(other.span(), "a range bound must be a number")),
        }
    }

    // ---- declarations -------------------------------------------------------------------------

    /// Collects constants, functions, feature variables and object fields of every file before lowering starts.
    fn declare(&mut self, files: &[File], errors: &mut Vec<Error>) {
        // Constants and user functions first: range bounds and cross-feature calls may name them.
        for file in files {
            for item in &file.items {
                match item {
                    Item::Const { name, value, span } => {
                        let key = (file.feature.clone(), name.text.clone());
                        if self.consts.insert(key, value.clone()).is_some() {
                            errors.push(Error {
                                file: file.feature.clone(),
                                line: span.line,
                                message: format!("constant '{}' is declared twice", name.text),
                            });
                        }
                    }
                    Item::Fn { name, params, body, span } => {
                        if params.len() > 15 {
                            errors.push(Error {
                                file: file.feature.clone(),
                                line: span.line,
                                message: format!(
                                    "function '{}' has {} parameters; the engine allows at most 15",
                                    name.text,
                                    params.len()
                                ),
                            });
                        }
                        let key = (file.feature.clone(), name.text.clone());
                        if self.fns.insert(key, (params.len(), block_returns_value(body))).is_some() {
                            errors.push(Error {
                                file: file.feature.clone(),
                                line: span.line,
                                message: format!("function '{}' is declared twice", name.text),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        for file in files {
            self.enter(file);
            for item in &file.items {
                let Item::Var(decl) = item else { continue };
                if decl.cooldown.is_some() {
                    // a cooldown is a design the modder
                    // writes with the clock; the std library (D9) will offer one written in UW.
                    errors.push(Error {
                        file: file.feature.clone(),
                        line: decl.span.line,
                        message: format!(
                            "`cooldown` is not a word of the language: keep a ready time yourself, e.g. `var agent.{n}_until = 0`, start it with `player.agent.{n}_until = mission.time + 2 seconds` and test `mission.time >= player.agent.{n}_until`",
                            n = decl.name.text
                        ),
                    });
                    continue;
                }
                let range = match &decl.range {
                    Some((lo, hi)) => match (self.const_value(lo), self.const_value(hi)) {
                        (Ok(lo), Ok(hi)) => Some((lo, hi)),
                        (Err(error), _) | (_, Err(error)) => {
                            errors.push(error);
                            None
                        }
                    },
                    None => None,
                };
                match &decl.kind {
                    Some(kind) => {
                        let key = (kind.text.clone(), decl.name.text.clone());
                        if let Some(old) = self.fields.get(&key) {
                            errors.push(Error {
                                file: file.feature.clone(),
                                line: decl.span.line,
                                message: format!(
                                    "field '{}.{}' is already declared at {}:{}",
                                    kind.text, decl.name.text, old.file, old.line
                                ),
                            });
                            continue;
                        }
                        let slot = self.slot_for(&kind.text, &decl.name.text);
                        let native_top = self.highest_slot(&kind.text);
                        if slot <= native_top {
                            errors.push(Error {
                                file: file.feature.clone(),
                                line: decl.span.line,
                                message: format!(
                                    "field '{}.{}' has slot {slot} in slots.lock, but the kit now uses slot_{}_* up to {native_top}; \
                                     the kit changed under this project (old saves would mix the two values)",
                                    kind.text, decl.name.text, kind.text
                                ),
                            });
                            continue;
                        }
                        let field =
                            Field { slot, range, file: file.feature.clone(), line: decl.span.line };
                        self.fields.insert(key, field);
                    }
                    None => {
                        self.globals.insert((file.feature.clone(), decl.name.text.clone()), range);
                    }
                }
            }
        }
    }

    // ---- items --------------------------------------------------------------------------------

    fn item(&mut self, item: &Item) -> Result<(), Error> {
        match item {
            // Constants only stand for their value.
            Item::Const { .. } => Ok(()),
            // Handled by `lower` (`init_var`) and `declare`.
            Item::Var(_) => Ok(()),
            Item::Fn { name, params, body, .. } => self.fn_item(name, params, body),
            Item::On { event, bound, when, body, span } => {
                self.on_item(event, bound, when, body, *span)
            }
            Item::WhileIn { context, body, span } => {
                let name: Vec<&str> = context.parts.iter().map(|part| part.text.as_str()).collect();
                if name != ["mission"] {
                    return Err(self.error(
                        context.span,
                        &format!("`while in {}`: only `while in mission` exists", name.join(".")),
                    ));
                }
                self.continuous_item(None, body, None, "while in mission".to_owned(), *span)
            }
            Item::While { cond, body, otherwise, span } => {
                let what = if otherwise.is_some() { "while … otherwise" } else { "while …" };
                self.continuous_item(Some(cond), body, otherwise.as_ref(), what.to_owned(), *span)
            }
            Item::Every { period, body, span } => self.every_item(period, body, *span),
            Item::Ui { part, name, props, span } => self.ui_item(part, name, props, *span),
        }
    }

    /// `var agent.stamina = 100`: the start value written into the spawned agent's slot, clamped.
    fn init_field(&mut self, kind: &Ident, decl: &VarDecl) -> Result<(), Error> {
        if kind.text != "agent" {
            return Err(self.error(
                decl.span,
                &format!(
                    "a start value for `{}` fields is not built yet (agent fields get theirs when the agent                      spawns); leave it out, a new field starts at 0",
                    kind.text
                ),
            ));
        }
        let Some(init) = &decl.init else { return Ok(()) };
        let key = (kind.text.clone(), decl.name.text.clone());
        let Some(field) = self.fields.get(&key) else { return Ok(()) };
        let (slot, range) = (field.slot, field.range);
        self.begin_block();
        let result = (|| -> Result<(), Error> {
            let value = self.expr(init)?;
            let target = self.temp();
            self.emit("assign", vec![target.clone(), value], decl.span)?;
            self.clamp(&target, range, decl.span)?;
            self.emit("agent_set_slot", vec![Arg::Local(AGENT_LOCAL.to_owned()), Arg::Int(slot), target], decl.span)
        })();
        let ops = self.end_block();
        result?;
        self.agent_init.extend(ops);
        Ok(())
    }

    /// `var x = 5` at game start: the UW global gets its declared value, clamped to its range.
    fn init_var(&mut self, decl: &VarDecl) -> Result<(), Error> {
        self.begin_block();
        let result = self.init_var_ops(decl);
        let ops = self.end_block();
        self.start.extend(ops);
        result
    }

    fn init_var_ops(&mut self, decl: &VarDecl) -> Result<(), Error> {
        let name = Self::global_for(&self.feature, &decl.name.text);
        let target = Arg::Global(name);
        if let Some(init) = &decl.init {
            let value = self.expr(init)?;
            self.emit("assign", vec![target.clone(), value], decl.span)?;
        }
        let key = (self.feature.clone(), decl.name.text.clone());
        let range = self.globals.get(&key).copied().flatten();
        self.clamp(&target, range, decl.span)
    }

    fn fn_item(&mut self, name: &Ident, params: &[Ident], body: &Block) -> Result<(), Error> {
        let script = format!("uw_{}_{}", self.feature, name.text);
        self.begin_block();
        self.locals.push(BTreeMap::new());
        let mut result = Ok(());
        for (index, param) in params.iter().enumerate() {
            let local = format!(":{}", param.text);
            if let Some(scope) = self.locals.last_mut() {
                scope.insert(param.text.clone(), local.clone());
            }
            // (store_script_param,<destination>,<script_param_no>)
            result = self.emit(
                "store_script_param",
                vec![Arg::Local(local), Arg::Int(index as i64 + 1)],
                param.span,
            );
            if result.is_err() {
                break;
            }
        }
        if result.is_ok() {
            result = self.stmts(body);
        }
        self.locals.pop();
        let ops = self.end_block();
        result?;
        self.scripts.push((script, ops));
        Ok(())
    }

    fn on_item(
        &mut self,
        event: &Path,
        bound: &Option<Ident>,
        when: &Option<Expr>,
        body: &Block,
        span: Span,
    ) -> Result<(), Error> {
        let name: Vec<&str> = event.parts.iter().map(|part| part.text.as_str()).collect();
        if let ["key", edge @ ("pressed" | "released")] = name.as_slice() {
            let pressed = *edge == "pressed";
            return self.key_item(pressed, event, bound.as_ref(), when.as_ref(), body, span);
        }
        if let ["menu", menu, "enter"] = name.as_slice() {
            return self.menu_enter_item(menu, event, bound.as_ref(), when.as_ref(), body, span);
        }
        if name != ["game", "start"] {
            return Err(self.error(
                event.span,
                &format!("`on {}` is not lowered yet", name.join(".")),
            ));
        }
        if bound.is_some() {
            return Err(self.error(span, "`on game.start` takes no bound object"));
        }
        if !self.atlas.is_hook("game_start") {
            return Err(self.error(event.span, "the atlas has no `game_start` hook"));
        }
        self.begin_block();
        let mut result = Ok(());
        if let Some(condition) = when {
            result = self.cond(condition);
        }
        if result.is_ok() {
            result = self.block(body);
        }
        let ops = self.end_block();
        result?;
        self.start.extend(ops);
        Ok(())
    }

    /// `on menu.start_game_0.enter { … }`: the block runs whenever the kit's menu opens, after the menu's own
    /// enter operations (CORE-v0 §4 "Kit things"). The menu is the kit's: its name comes from the atlas.
    fn menu_enter_item(
        &mut self,
        menu: &str,
        event: &Path,
        bound: Option<&Ident>,
        when: Option<&Expr>,
        body: &Block,
        span: Span,
    ) -> Result<(), Error> {
        if bound.is_some() {
            return Err(self.error(span, "`on menu.<name>.enter` takes no bound object"));
        }
        if self.record_ref(&["menu".to_owned(), menu.to_owned()]).is_none() {
            return Err(self.error(event.span, &format!("the kit has no menu '{menu}'")));
        }
        self.begin_block();
        let mut result = Ok(());
        if let Some(condition) = when {
            result = self.cond(condition);
        }
        if result.is_ok() {
            result = self.block(body);
        }
        let ops = self.end_block();
        result?;
        match self.menu_appends.iter_mut().find(|(name, _)| name == menu) {
            Some((_, list)) => list.extend(ops),
            None => self.menu_appends.push((menu.to_owned(), ops)),
        }
        Ok(())
    }

    /// A thing of the kit named by its kind and its name in the kit (`party.town_6` → `p_town_6`,
    /// `menu.start_game_0` → `mnu_start_game_0`): the reference operations take.
    /// The kind is the singular of the atlas table (`parties` → `party`); names are never invented, the atlas
    /// must have the entry.
    fn record_ref(&self, parts: &[String]) -> Option<String> {
        let [kind, name] = parts else { return None };
        for (table_name, table) in &self.atlas.tables {
            let Some(tag) = &table.ref_tag else { continue };
            if singular(table_name) != *kind {
                continue;
            }
            let reference = format!("{tag}_{name}");
            return match self.atlas.resolve_ref(&reference) {
                Some((found, _)) if found == table_name => Some(reference),
                _ => None,
            };
        }
        None
    }

    // ---- time (CORE-v0 §4 "Time") -------------------------------------------------------------

    /// A `while` block: one `try_begin … try_end` inside the mission dispatcher, so a failing condition
    /// never stops another feature.
    fn continuous_item(
        &mut self,
        cond: Option<&Expr>,
        body: &Block,
        otherwise: Option<&Block>,
        what: String,
        span: Span,
    ) -> Result<(), Error> {
        self.begin_block();
        let was = std::mem::replace(&mut self.continuous, true);
        let result = self.continuous_ops(cond, body, otherwise, span);
        self.continuous = was;
        let ops = self.end_block();
        result?;
        self.frame_items.push(time::FrameItem { file: self.file.clone(), line: span.line, what, ops: ops.len() });
        self.frame.extend(ops);
        Ok(())
    }

    fn continuous_ops(
        &mut self,
        cond: Option<&Expr>,
        body: &Block,
        otherwise: Option<&Block>,
        span: Span,
    ) -> Result<(), Error> {
        self.emit("try_begin", Vec::new(), span)?;
        if let Some(cond) = cond {
            self.cond(cond)?;
        }
        self.block(body)?;
        if let Some(otherwise) = otherwise {
            self.emit("else_try", Vec::new(), span)?;
            self.block(otherwise)?;
        }
        self.emit("try_end", Vec::new(), span)
    }

    /// Lowers a block that runs later from a dispatcher: its own scope (no outer `let`), not continuous.
    fn detached_block(&mut self, body: &Block) -> Result<Vec<Op>, Error> {
        self.begin_block();
        let locals = std::mem::take(&mut self.locals);
        let was = std::mem::replace(&mut self.continuous, false);
        let result = self.block(body);
        self.continuous = was;
        self.locals = locals;
        let ops = self.end_block();
        result.map(|()| ops)
    }

    fn every_item(&mut self, period: &Expr, body: &Block, span: Span) -> Result<(), Error> {
        let (clock, ticks, text) = self.duration_ticks(period, "every")?;
        let count = self.every_count.entry(self.feature.clone()).or_default();
        let global = format!("$uw_{}_every{}", self.feature, count);
        *count += 1;
        let body = self.detached_block(body)?;
        self.timed.push(time::Timed {
            clock,
            kind: time::TimedKind::Every { period: ticks },
            global,
            body,
            file: self.file.clone(),
            line: span.line,
            what: format!("every {text}"),
        });
        Ok(())
    }

    /// `after 3 days (as NAME) { … }`: here the due time is written; the block runs from the dispatcher.
    fn after_stmt(&mut self, delay: &Expr, name: Option<&Ident>, body: &Block, span: Span) -> Result<(), Error> {
        if self.continuous {
            return Err(self.error(
                span,
                "`after` inside a continuous block would start again on every frame and never run; \
                 use a cooldown or `every`",
            ));
        }
        let (clock, ticks, text) = self.duration_ticks(delay, "after")?;
        let (global, what) = match name {
            Some(name) => {
                let key = (self.feature.clone(), name.text.clone());
                if let Some(line) = self.after_names.get(&key) {
                    return Err(self.error(
                        name.span,
                        &format!("`after … as {}` is already used at line {line} of this feature", name.text),
                    ));
                }
                self.after_names.insert(key, span.line);
                (format!("$uw_{}_after_{}", self.feature, name.text), format!("after {text} as {}", name.text))
            }
            None => {
                let count = self.after_count.entry(self.feature.clone()).or_default();
                let global = format!("$uw_{}_after{}", self.feature, count);
                *count += 1;
                (global, format!("after {text}"))
            }
        };
        let now = self.clock_now(clock, span)?;
        let due = Arg::Global(global.clone());
        self.emit("store_add", vec![due.clone(), now, Arg::Int(ticks)], span)?;
        // 0 means "nothing pending", so a due time is never 0.
        self.emit("val_max", vec![due, Arg::Int(1)], span)?;
        let body = self.detached_block(body)?;
        self.timed.push(time::Timed {
            clock,
            kind: time::TimedKind::After,
            global,
            body,
            file: self.file.clone(),
            line: span.line,
            what,
        });
        Ok(())
    }

    /// A duration as (clock, ticks, text): `6 hours` → (game, 6), `0.5 seconds` → (mission, 500 ms).
    /// A constant naming a duration works too.
    fn duration_ticks(&mut self, e: &Expr, what: &str) -> Result<(Clock, i64, String), Error> {
        let needs = format!("`{what}` needs a duration such as `6 hours` or `0.5 seconds`");
        let (milli, unit, span) = match e {
            Expr::Duration { milli, unit, span } => (*milli, *unit, *span),
            Expr::Path(path) if path.parts.len() == 1 => {
                let key = (self.feature.clone(), path.parts[0].text.clone());
                match self.consts.get(&key) {
                    Some(Expr::Duration { milli, unit, .. }) => (*milli, *unit, path.span),
                    _ => return Err(self.error(path.span, &needs)),
                }
            }
            other => return Err(self.error(other.span(), &needs)),
        };
        let scaled = milli.saturating_mul(unit.ticks());
        let text = format!("{} {}", milli as f64 / 1000.0, unit.word());
        if scaled % 1000 != 0 || scaled < 1000 {
            let message = match unit.clock() {
                Clock::Game => format!(
                    "game time moves in whole hours: `{what} {text}` is not possible (hours, days and weeks \
                     are campaign time; seconds and minutes are mission time)"
                ),
                Clock::Mission => format!("`{what} {text}` is shorter than a millisecond"),
            };
            return Err(self.error(span, &message));
        }
        Ok((unit.clock(), scaled / 1000, text))
    }

    /// The clock's current value: game hours (`store_current_hours`) or mission milliseconds (`$uw_clock_ms`).
    fn clock_now(&mut self, clock: Clock, span: Span) -> Result<Arg, Error> {
        match clock {
            Clock::Mission => {
                self.mission_clock = true;
                Ok(Arg::Global(time::CLOCK_MS.to_owned()))
            }
            Clock::Game => {
                let now = self.temp();
                self.emit("store_current_hours", vec![now.clone()], span)?;
                Ok(now)
            }
        }
    }

    /// `15 per second` inside a continuous block: the amount for this frame, in thousandths.
    fn rate_amount(&mut self, milli: i64, unit: RateUnit, span: Span) -> Result<Arg, Error> {
        if !self.continuous {
            return Err(self.error(
                span,
                "a rate (`15 per second`) only makes sense in a continuous block (`while …`), where time \
                 passes between runs; in `every`/`after` write the amount for one run",
            ));
        }
        let per = match unit {
            RateUnit::Second => 1_000,
            RateUnit::Minute => 60_000,
            RateUnit::Hour => 3_600_000,
        };
        let amount = self.temp();
        self.emit(
            "store_mul",
            vec![amount.clone(), Arg::Int(milli), Arg::Local(time::FRAME_MS.to_owned())],
            span,
        )?;
        self.emit("val_div", vec![amount.clone(), Arg::Int(per)], span)?;
        Ok(amount)
    }

    // ---- keys (CORE-v0 §4 "Keys") -------------------------------------------------------------

    /// `on key.pressed(q) { … }` / `on key.released(q) { … }`: runs in the mission dispatcher on the frame
    /// the key goes down (the engine's own edge: `key_clicked`) or comes up (the dispatcher keeps the key's
    /// state of the previous frame in a global). A name the kit has as `key_<name>` is a keyboard key;
    /// otherwise `gk_<name>` is a game control the player can rebind (`kick`, `jump`, …).
    fn key_item(
        &mut self,
        pressed: bool,
        event: &Path,
        key: Option<&Ident>,
        when: Option<&Expr>,
        body: &Block,
        span: Span,
    ) -> Result<(), Error> {
        let edge = if pressed { "pressed" } else { "released" };
        let Some(key) = key else {
            return Err(self.error(event.span, &format!("`on key.{edge}` needs the key: `on key.{edge}(q)`")));
        };
        let (family, name) = self.resolve_key(key)?;
        let what = format!("on key.{edge}({})", key.text);
        self.begin_block();
        let was = std::mem::replace(&mut self.continuous, false);
        let locals = std::mem::take(&mut self.locals);
        let result = (|| -> Result<(), Error> {
            if pressed {
                self.emit("try_begin", Vec::new(), span)?;
                self.key_cond(family, "pressed", &name, span)?;
            } else {
                let count = self.key_count.entry(self.feature.clone()).or_default();
                let previous = Arg::Global(format!("$uw_{}_key{}", self.feature, count));
                *count += 1;
                let down = self.temp();
                self.emit("assign", vec![down.clone(), Arg::Int(0)], span)?;
                self.emit("try_begin", Vec::new(), span)?;
                self.key_cond(family, "held", &name, span)?;
                self.emit("assign", vec![down.clone(), Arg::Int(1)], span)?;
                self.emit("try_end", Vec::new(), span)?;
                // Remember this frame's state before the body, so a failing body cannot skip it.
                let was_down = self.temp();
                self.emit("assign", vec![was_down.clone(), previous.clone()], span)?;
                self.emit("assign", vec![previous, down.clone()], span)?;
                self.emit("try_begin", Vec::new(), span)?;
                self.emit("eq", vec![was_down, Arg::Int(1)], span)?;
                self.emit("eq", vec![down, Arg::Int(0)], span)?;
            }
            if let Some(condition) = when {
                self.cond(condition)?;
            }
            self.block(body)?;
            self.emit("try_end", Vec::new(), span)
        })();
        self.locals = locals;
        self.continuous = was;
        let ops = self.end_block();
        result?;
        self.frame_items.push(time::FrameItem { file: self.file.clone(), line: span.line, what, ops: ops.len() });
        self.frame.extend(ops);
        Ok(())
    }

    /// A key name → (`key` or `control`, the name as the kit spells it). Spellings from the `key.held`
    /// property's aliases (`alt` → `left_alt`) are accepted.
    fn resolve_key(&self, key: &Ident) -> Result<(&'static str, String), Error> {
        let mut name = key.text.clone();
        if let Some(prop) = self.data.properties.iter().find(|p| p.path.starts_with("key.held(")) {
            if let Some(alias) = prop.aliases.get(&name) {
                name = alias.clone();
            }
        }
        if self.atlas.constant(&format!("key_{name}")).is_some() {
            return Ok(("key", name));
        }
        if self.atlas.constant(&format!("gk_{name}")).is_some() {
            return Ok(("control", name));
        }
        Err(self.error(
            key.span,
            &format!("unknown key `{}`: the kit has neither `key_{name}` nor `gk_{name}`", key.text),
        ))
    }

    /// The condition of the `<family>.<edge>({…})` property (kit-meta/core/properties.toml) for one key.
    fn key_cond(&mut self, family: &str, edge: &str, name: &str, span: Span) -> Result<(), Error> {
        let prefix = format!("{family}.{edge}(");
        let Some(prop) = self.data.properties.iter().find(|p| p.path.starts_with(&prefix)).cloned() else {
            return Err(self.error(span, &format!("properties.toml has no `{prefix}…)` entry")));
        };
        let placeholder = placeholders(&prop.path).into_iter().next().unwrap_or_default();
        let subst = Subst { object: None, value: None, args: vec![(placeholder, SubArg::Name(name.to_owned()))] };
        self.emit_steps(&prop.cond, &subst, span)
    }

    /// A constant's expression (`BAR_COLOR` → `#3FA34D`); any other expression as it is.
    fn resolve_const(&self, e: &Expr) -> Expr {
        if let Expr::Path(path) = e {
            if path.parts.len() == 1 {
                let key = (self.feature.clone(), path.parts[0].text.clone());
                if let Some(value) = self.consts.get(&key) {
                    return value.clone();
                }
            }
        }
        e.clone()
    }

    // ---- statements ---------------------------------------------------------------------------

    fn stmts(&mut self, body: &Block) -> Result<(), Error> {
        for (index, stmt) in body.iter().enumerate() {
            match stmt {
                Stmt::Let { name, value, span } => {
                    let value = self.expr(value)?;
                    let local = format!(":{}", name.text);
                    self.emit("assign", vec![Arg::Local(local.clone()), value], *span)?;
                    if let Some(scope) = self.locals.last_mut() {
                        scope.insert(name.text.clone(), local);
                    }
                }
                Stmt::Assign { place, op, value, .. } => self.place_set(place, *op, value)?,
                Stmt::Call(call) => self.call_stmt(call)?,
                Stmt::If { cond, then, otherwise, span } => {
                    self.emit("try_begin", Vec::new(), *span)?;
                    self.cond(cond)?;
                    self.block(then)?;
                    if let Some(otherwise) = otherwise {
                        self.emit("else_try", Vec::new(), *span)?;
                        self.block(otherwise)?;
                    }
                    self.emit("try_end", Vec::new(), *span)?;
                }
                Stmt::For { var, iter, filter, body, span } => {
                    let (lo, hi) = match iter {
                        Expr::Tuple { items, .. } if items.len() == 2 => {
                            (self.expr(&items[0])?, self.expr(&items[1])?)
                        }
                        other => {
                            return Err(self.error(
                                other.span(),
                                "`for` needs a range: `for i in (lo, hi)`",
                            ))
                        }
                    };
                    // (try_for_range,<destination>,<lower bound>,<upper bound>), upper exclusive.
                    let local = format!(":{}", var.text);
                    self.emit("try_for_range", vec![Arg::Local(local.clone()), lo, hi], *span)?;
                    self.locals.push(BTreeMap::new());
                    if let Some(scope) = self.locals.last_mut() {
                        scope.insert(var.text.clone(), local);
                    }
                    let mut result = Ok(());
                    if let Some(condition) = filter {
                        result = self.cond(condition);
                    }
                    if result.is_ok() {
                        result = self.stmts(body);
                    }
                    self.locals.pop();
                    self.emit("try_end", Vec::new(), *span)?;
                    result?;
                }
                Stmt::After { delay, name, body, span } => self.after_stmt(delay, name.as_ref(), body, *span)?,
                Stmt::Cancel { name, span } => {
                    let global = format!("$uw_{}_after_{}", self.feature, name.text);
                    self.emit("assign", vec![Arg::Global(global), Arg::Int(0)], *span)?;
                    self.cancels.push((self.feature.clone(), name.text.clone(), self.file.clone(), span.line));
                }
                Stmt::Return { value, span } => {
                    if index + 1 != body.len() {
                        return Err(self.error(
                            *span,
                            "`return` must end its block (a Native script cannot stop early)",
                        ));
                    }
                    if let Some(value) = value {
                        let value = self.expr(value)?;
                        self.emit("assign", vec![Arg::Reg(0), value], *span)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn block(&mut self, body: &Block) -> Result<(), Error> {
        self.locals.push(BTreeMap::new());
        let result = self.stmts(body);
        self.locals.pop();
        result
    }

    /// A statement call: a user function, a condition property gating the block, or an action/getter.
    fn call_stmt(&mut self, call: &Call) -> Result<(), Error> {
        let parts: Vec<String> = call.callee.parts.iter().map(|part| part.text.clone()).collect();
        if let Some((script, arity, _)) = self.user_fn(&parts) {
            return self.emit_user_call(&parts, &script, arity, &call.args, call.span);
        }
        if self.cond_entry(call)? {
            return Ok(());
        }
        let (prop, args) = self.resolve_call(call)?;
        if !prop.run.is_empty() {
            let subst = Subst { object: Some(&call.callee), value: None, args };
            return self.emit_steps(&prop.run, &subst, call.span);
        }
        if prop.get.is_empty() {
            return Err(self.error(call.span, &format!("'{}' has no operations to run", prop.path)));
        }
        let dest = self.temp();
        let subst = Subst { object: Some(&call.callee), value: Some(dest), args };
        self.emit_steps(&prop.get, &subst, call.span)
    }

    fn user_fn(&self, parts: &[String]) -> Option<(String, usize, bool)> {
        match parts {
            [name] => {
                let &(arity, has_ret) = self.fns.get(&(self.feature.clone(), name.clone()))?;
                Some((format!("script_uw_{}_{}", self.feature, name), arity, has_ret))
            }
            [feature, name] => {
                let &(arity, has_ret) = self.fns.get(&(feature.clone(), name.clone()))?;
                Some((format!("script_uw_{feature}_{name}"), arity, has_ret))
            }
            _ => None,
        }
    }

    fn emit_user_call(
        &mut self,
        parts: &[String],
        script: &str,
        arity: usize,
        args: &[Expr],
        span: Span,
    ) -> Result<(), Error> {
        if args.len() != arity {
            return Err(self.error(
                span,
                &format!("'{}' takes {arity} argument(s)", parts.join(".")),
            ));
        }
        let mut call_args = Vec::with_capacity(1 + args.len());
        call_args.push(Arg::Name(script.to_owned()));
        for arg in args {
            call_args.push(self.expr(arg)?);
        }
        self.emit("call_script", call_args, span)
    }

    // ---- places -------------------------------------------------------------------------------

    fn assign_var(&mut self, var: Var, op: AssignOp, value: &Expr, span: Span) -> Result<(), Error> {
        let (target, range) = match var {
            Var::Local(name) => (Arg::Local(name), None),
            Var::Global { name, range } => (Arg::Global(name), range),
        };
        let value = self.expr(value)?;
        self.emit_compound(&target, op, value, span)?;
        self.clamp(&target, range, span)
    }

    /// `agent.stamina = v`: read, combine, clamp, write the slot.
    fn assign_field(
        &mut self,
        kind: &str,
        slot: i64,
        range: Option<(i64, i64)>,
        object: Arg,
        op: AssignOp,
        value: &Expr,
        span: Span,
    ) -> Result<(), Error> {
        let get_op = format!("{kind}_get_slot");
        let set_op = format!("{kind}_set_slot");
        let target = self.temp();
        match op {
            AssignOp::Set => {
                let value = self.expr(value)?;
                self.emit("assign", vec![target.clone(), value], span)?;
            }
            _ => {
                self.emit(&get_op, vec![target.clone(), object.clone(), Arg::Int(slot)], span)?;
                let value = self.expr(value)?;
                self.emit_compound(&target, op, value, span)?;
            }
        }
        self.clamp(&target, range, span)?;
        self.emit(&set_op, vec![object, Arg::Int(slot), target], span)
    }

    /// Combines `value` into `target` while preserving UW's fixed-point thousandths scale.
    fn emit_compound(&mut self, target: &Arg, op: AssignOp, value: Arg, span: Span) -> Result<(), Error> {
        match op {
            AssignOp::Set => self.emit("assign", vec![target.clone(), value], span),
            AssignOp::Add => self.emit("val_add", vec![target.clone(), value], span),
            AssignOp::Sub => self.emit("val_sub", vec![target.clone(), value], span),
            AssignOp::Mul => {
                self.emit("val_mul", vec![target.clone(), value], span)?;
                self.emit("val_div", vec![target.clone(), Arg::Int(1000)], span)
            }
            AssignOp::Div => {
                if matches!(value, Arg::Int(0)) {
                    return Err(self.error(span, "division by zero"));
                }
                self.emit("val_mul", vec![target.clone(), Arg::Int(1000)], span)?;
                self.emit("val_div", vec![target.clone(), value], span)
            }
        }
    }

    /// Emits `val_clamp` when the place declares a range. `in lo..hi` includes `hi`, while the operation's
    /// upper bound is exclusive (header_operations), so the bound goes in as `hi + 1`.
    fn clamp(&mut self, target: &Arg, range: Option<(i64, i64)>, span: Span) -> Result<(), Error> {
        match range {
            Some((lo, hi)) => self.emit(
                "val_clamp",
                vec![target.clone(), Arg::Int(lo), Arg::Int(hi.saturating_add(1))],
                span,
            ),
            None => Ok(()),
        }
    }

    /// The declared variable a path names (a `let`/parameter local, or a UW global).
    fn var_target(&self, parts: &[String]) -> Option<Var> {
        match parts {
            [name] => {
                if let Some(local) = self.local(name) {
                    return Some(Var::Local(local));
                }
                let key = (self.feature.clone(), name.clone());
                self.globals.get(&key).map(|range| Var::Global {
                    name: Self::global_for(&self.feature, name),
                    range: *range,
                })
            }
            [feature, name] => {
                let key = (feature.clone(), name.clone());
                self.globals.get(&key).map(|range| Var::Global {
                    name: Self::global_for(feature, name),
                    range: *range,
                })
            }
            _ => None,
        }
    }

    fn local(&self, name: &str) -> Option<String> {
        self.locals.iter().rev().find_map(|scope| scope.get(name).cloned())
    }

    /// The field a path names: `(kind, slot, range)`. The last part is the field name and its kind must end
    /// the object, so `player.agent.stamina` is the stamina of the player's agent.
    fn field_target(&self, parts: &[String]) -> Option<(String, i64, Option<(i64, i64)>)> {
        let last = parts.last()?;
        let object = &parts[..parts.len() - 1];
        let mut found = None;
        for ((kind, name), field) in &self.fields {
            if name != last || !object.last().is_some_and(|tail| tail == kind) {
                continue;
            }
            if found.is_some() {
                return None; // two kinds declare this name
            }
            found = Some((kind.clone(), field.slot, field.range));
        }
        found
    }

    /// The object in front of a place: the path without its last part (`player.agent` of `player.agent.speed`).
    fn object_of(&mut self, path: &Path) -> Result<Arg, Error> {
        if path.parts.len() < 2 {
            return Err(self.error(
                path.span,
                &format!("'{}' needs an object in front of it", dotted(path)),
            ));
        }
        let mut parts = path.parts.clone();
        parts.pop();
        self.value_path_in(&Path { parts, span: path.span }, true)
    }

    // ---- values -------------------------------------------------------------------------------

    fn value_path(&mut self, path: &Path) -> Result<Arg, Error> {
        self.value_path_in(path, false)
    }

    /// The engine's number for a language value (thousandths → whole units).
    fn to_engine(&mut self, value: Arg, span: Span) -> Result<Arg, Error> {
        match value {
            Arg::Int(milli) => Ok(Arg::Int(milli / 1000)),
            other => {
                let temp = self.temp();
                self.emit("store_div", vec![temp.clone(), other, Arg::Int(1000)], span)?;
                Ok(temp)
            }
        }
    }

    /// A position register for a vector value (`agent.speed`). The registers are the compiler's own
    /// (CORE-v0 §4: registers never show); they rotate over the top of the kit's `pos0..pos63`.
    fn pos_reg(&mut self) -> Arg {
        let reg = format!("pos{}", 56 + self.next_pos % 8);
        self.next_pos += 1;
        Arg::Name(reg)
    }

    /// Reads a vector property (`player.agent.speed`) into a fresh position register.
    fn vector_get(&mut self, path: &Path, prop: &Property) -> Result<Arg, Error> {
        let pos = self.pos_reg();
        let subst = Subst { object: Some(path), value: Some(pos.clone()), args: Vec::new() };
        self.emit_steps(&prop.get, &subst, path.span)?;
        Ok(pos)
    }

    /// `path` names a vector property: its index.
    fn vector_property(&self, parts: &[String]) -> Option<usize> {
        let hit = self.find_property(parts, None)?;
        self.data.properties[hit.prop].vector.then_some(hit.prop)
    }

    /// A value path. `raw` asks for the engine's own number (an object id for `{object}`): language numbers
    /// are thousandths, so a variable is divided and a kit getter is left unscaled.
    fn value_path_in(&mut self, path: &Path, raw: bool) -> Result<Arg, Error> {
        let value = self.value_path_lang(path, raw)?;
        match value {
            Scaled::Lang(arg) if raw => self.to_engine(arg, path.span),
            Scaled::Lang(arg) | Scaled::Engine(arg) => Ok(arg),
        }
    }

    fn value_path_lang(&mut self, path: &Path, raw: bool) -> Result<Scaled, Error> {
        let parts: Vec<String> = path.parts.iter().map(|part| part.text.clone()).collect();
        let span = path.span;
        if let Some(var) = self.var_target(&parts) {
            return Ok(Scaled::Lang(match var {
                Var::Local(name) => Arg::Local(name),
                Var::Global { name, .. } => Arg::Global(name),
            }));
        }
        if let [name] = parts.as_slice() {
            let key = (self.feature.clone(), name.clone());
            if let Some(constant) = self.consts.get(&key).cloned() {
                if self.resolving.contains(&key) {
                    return Err(self.error(span, &format!("constant '{name}' uses itself")));
                }
                self.resolving.push(key);
                let value = self.expr(&constant);
                self.resolving.pop();
                return value.map(Scaled::Lang);
            }
        }
        if let Some((kind, slot, _range)) = self.field_target(&parts) {
            let object = self.object_of(path)?;
            let dest = self.temp();
            self.emit(
                &format!("{kind}_get_slot"),
                vec![dest.clone(), object, Arg::Int(slot)],
                span,
            )?;
            return Ok(Scaled::Lang(dest));
        }
        // A kit thing by name (`troop.player`): the engine's reference, as it is (an id, not a number).
        if let Some(reference) = self.record_ref(&parts) {
            return Ok(Scaled::Engine(Arg::Name(reference)));
        }
        // `speed.x`: one component of a vector value, in metres (thousandths, like every number).
        if let Some((axis, prefix)) = parts.split_last() {
            if matches!(axis.as_str(), "x" | "y" | "z") && prefix.len() >= 2 {
                if let Some(index) = self.vector_property(prefix) {
                    let prop = self.data.properties[index].clone();
                    let vector_path = Path { parts: path.parts[..prefix.len()].to_vec(), span };
                    let pos = self.vector_get(&vector_path, &prop)?;
                    self.emit("set_fixed_point_multiplier", vec![Arg::Int(1000)], span)?;
                    let dest = self.temp();
                    self.emit(&format!("position_get_{axis}"), vec![dest.clone(), pos], span)?;
                    return Ok(Scaled::Lang(dest));
                }
            }
        }
        if let Some(hit) = self.find_property(&parts, None) {
            let prop = self.data.properties[hit.prop].clone();
            if prop.get.is_empty() && !prop.cond.is_empty() {
                // A yes/no property read as a value: 1 when its condition holds, 0 otherwise.
                let dest = self.temp();
                self.emit("assign", vec![dest.clone(), Arg::Int(0)], span)?;
                self.emit("try_begin", Vec::new(), span)?;
                let subst = Subst { object: Some(path), value: None, args: Vec::new() };
                self.emit_steps(&prop.cond, &subst, span)?;
                self.emit("assign", vec![dest.clone(), Arg::Int(1)], span)?;
                self.emit("try_end", Vec::new(), span)?;
                return Ok(Scaled::Engine(dest));
            }
            if prop.get.is_empty() {
                return Err(self.error(span, &format!("'{}' has no value", prop.path)));
            }
            if prop.vector {
                // A vector used as a number is its length (metres): the distance from the origin.
                let pos = self.vector_get(path, &prop)?;
                let origin = self.pos_reg();
                self.emit("init_position", vec![origin.clone()], span)?;
                let dest = self.temp();
                self.emit("get_distance_between_positions", vec![dest.clone(), pos, origin], span)?;
                self.emit("val_mul", vec![dest.clone(), Arg::Int(10)], span)?; // centimetres → thousandths of a metre
                return Ok(Scaled::Lang(dest));
            }
            let dest = self.temp();
            let subst = Subst { object: Some(path), value: Some(dest.clone()), args: Vec::new() };
            self.emit_steps(&prop.get, &subst, span)?;
            if prop.unit.as_deref() == Some("int") && !raw {
                // The engine's whole units → the language's thousandths.
                self.emit("val_mul", vec![dest.clone(), Arg::Int(1000)], span)?;
                return Ok(Scaled::Lang(dest));
            }
            return Ok(Scaled::Engine(dest));
        }
        // A kit constant by its easy name (`ca.strength`): the engine's value, as it is. The atlas must know it.
        if let Some(native) = kit::kit_constant(&parts.join(".")) {
            if self.atlas.constant(native).is_some() {
                return Ok(Scaled::Engine(Arg::Name(native.to_owned())));
            }
        }
        Err(self.error(span, &format!("unknown name '{}'", parts.join("."))))
    }

    fn call_value(&mut self, call: &Call) -> Result<Arg, Error> {
        let parts: Vec<String> = call.callee.parts.iter().map(|part| part.text.clone()).collect();
        if let Some((script, arity, has_ret)) = self.user_fn(&parts) {
            if !has_ret {
                return Err(self.error(
                    call.span,
                    &format!("'{}' returns no value: call it as a statement of its own", parts.join(".")),
                ));
            }
            self.emit_user_call(&parts, &script, arity, &call.args, call.span)?;
            let dest = self.temp();
            self.emit("assign", vec![dest.clone(), Arg::Reg(0)], call.span)?;
            return Ok(dest);
        }
        let (prop, args) = self.resolve_call(call)?;
        let object = Some(&call.callee);
        if !prop.get.is_empty() {
            let dest = self.temp();
            let subst = Subst { object, value: Some(dest.clone()), args };
            self.emit_steps(&prop.get, &subst, call.span)?;
            if prop.unit.as_deref() == Some("int") {
                self.emit("val_mul", vec![dest.clone(), Arg::Int(1000)], call.span)?;
            }
            return Ok(dest);
        }
        if !prop.cond.is_empty() {
            let dest = self.temp();
            self.emit("assign", vec![dest.clone(), Arg::Int(0)], call.span)?;
            self.emit("try_begin", Vec::new(), call.span)?;
            let subst = Subst { object, value: None, args };
            self.emit_steps(&prop.cond, &subst, call.span)?;
            self.emit("assign", vec![dest.clone(), Arg::Int(1)], call.span)?;
            self.emit("try_end", Vec::new(), call.span)?;
            return Ok(dest);
        }
        if !prop.run.is_empty() {
            return Err(self.error(
                call.span,
                &format!("'{}' is an action: write it as a statement of its own, it has no value", prop.path),
            ));
        }
        Err(self.error(call.span, &format!("'{}' has no value", prop.path)))
    }

    /// `"Gold: {gold}"` → a quick string the engine fills in when it shows it. A number goes into one of
    /// the compiler's own registers (`reg52..reg59`, never used by Native) as whole units; a text value is
    /// inlined (a literal) or referenced by its string register (`{s60}`). Registers never show (CORE-v0 §4).
    fn text_value(&mut self, parts: &[TextPart], span: Span) -> Result<Arg, Error> {
        let mut text = String::from("@");
        let mut regs = 0usize;
        for part in parts {
            match part {
                TextPart::Literal(literal) => text.push_str(literal),
                TextPart::Expr(expr) => match self.expr(expr)? {
                    Arg::Text(inner) => text.push_str(inner.strip_prefix('@').unwrap_or(&inner)),
                    Arg::Name(name) if is_str_reg(&name) => {
                        text.push('{');
                        text.push_str(&name);
                        text.push('}');
                    }
                    number => {
                        if regs == TEXT_REGS {
                            return Err(self.error(
                                span,
                                &format!("a text can show at most {TEXT_REGS} numbers; split it into two texts"),
                            ));
                        }
                        // Rotating over the whole build keeps two texts of one operation apart.
                        let reg = TEXT_REG_FIRST + (self.next_text_reg % TEXT_REGS) as u8;
                        self.next_text_reg += 1;
                        regs += 1;
                        let whole = self.to_engine(number, span)?;
                        self.emit("assign", vec![Arg::Reg(reg), whole], span)?;
                        text.push_str(&format!("{{reg{reg}}}"));
                    }
                },
            }
        }
        Ok(Arg::Text(text))
    }

    fn arith_expr(&mut self, op: BinOp, left: &Expr, right: &Expr, span: Span) -> Result<Arg, Error> {
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        let name = match op {
            BinOp::Add => "store_add",
            BinOp::Sub => "store_sub",
            BinOp::Mul => "store_mul",
            BinOp::Div => "store_div",
            BinOp::Or | BinOp::And | BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt
            | BinOp::Ge => return Err(self.error(span, "this operator has no value")),
        };
        self.arith(left, name, right, span)
    }

    /// Folds two numbers; otherwise writes the operation into a fresh local, keeping thousandths scale.
    fn arith(&mut self, left: Arg, name: &str, right: Arg, span: Span) -> Result<Arg, Error> {
        if name == "store_div" && matches!(right, Arg::Int(0)) {
            return Err(self.error(span, "division by zero"));
        }
        if let (Arg::Int(left), Arg::Int(right)) = (&left, &right) {
            let folded = match name {
                "store_add" => Some(left.wrapping_add(*right)),
                "store_sub" => Some(left.wrapping_sub(*right)),
                "store_mul" => Some(((*left as i128 * *right as i128) / 1000) as i64),
                "store_div" => Some(((*left as i128 * 1000) / (*right as i128)) as i64),
                _ => None,
            };
            if let Some(folded) = folded {
                return Ok(Arg::Int(folded));
            }
        }
        let dest = self.temp();
        match name {
            "store_mul" => {
                self.emit("store_mul", vec![dest.clone(), left, right], span)?;
                self.emit("val_div", vec![dest.clone(), Arg::Int(1000)], span)?;
            }
            "store_div" => {
                self.emit("store_mul", vec![dest.clone(), left, Arg::Int(1000)], span)?;
                self.emit("val_div", vec![dest.clone(), right], span)?;
            }
            _ => {
                self.emit(name, vec![dest.clone(), left, right], span)?;
            }
        }
        Ok(dest)
    }

    /// A condition as a value: 1 when it holds, 0 when it does not.
    fn cond_value(&mut self, e: &Expr) -> Result<Arg, Error> {
        let span = e.span();
        let flag = self.temp();
        self.emit("assign", vec![flag.clone(), Arg::Int(0)], span)?;
        self.emit("try_begin", Vec::new(), span)?;
        self.cond(e)?;
        self.emit("assign", vec![flag.clone(), Arg::Int(1)], span)?;
        self.emit("try_end", Vec::new(), span)?;
        Ok(flag)
    }

    // ---- conditions ---------------------------------------------------------------------------

    /// Writes the conditions of an expression into the current block: they hold, or the block fails.
    fn cond(&mut self, e: &Expr) -> Result<(), Error> {
        match e {
            Expr::Binary { op, left, right, span } => match op {
                BinOp::And => {
                    self.cond(left)?;
                    self.cond(right)
                }
                BinOp::Or => self.cond_or(left, right, *span),
                BinOp::Eq => self.compare("eq", left, right, *span),
                BinOp::Ne => self.compare("neq", left, right, *span),
                BinOp::Lt => self.compare("lt", left, right, *span),
                BinOp::Le => self.compare("le", left, right, *span),
                BinOp::Gt => self.compare("gt", left, right, *span),
                BinOp::Ge => self.compare("ge", left, right, *span),
                BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => self.cond_of_value(e),
            },
            Expr::Unary { not: true, expr, span, .. } => self.cond_not(expr, *span),
            Expr::Call(call) => {
                if self.cond_entry(call)? {
                    return Ok(());
                }
                self.cond_of_value(e)
            }
            Expr::Path(path) => {
                if self.cond_path(path)? {
                    return Ok(());
                }
                self.cond_of_value(e)
            }
            other => self.cond_of_value(other),
        }
    }

    /// A property with a condition and no value used as a condition (`player.agent.moving`): emits the
    /// condition directly. `false` when the path is a variable, a field, a constant or another property.
    fn cond_path(&mut self, path: &Path) -> Result<bool, Error> {
        let parts: Vec<String> = path.parts.iter().map(|part| part.text.clone()).collect();
        if self.var_target(&parts).is_some() || self.field_target(&parts).is_some() {
            return Ok(false);
        }
        if parts.len() == 1 && self.consts.contains_key(&(self.feature.clone(), parts[0].clone())) {
            return Ok(false);
        }
        let Some(hit) = self.find_property(&parts, None) else { return Ok(false) };
        let prop = self.data.properties[hit.prop].clone();
        if prop.cond.is_empty() || !prop.get.is_empty() {
            return Ok(false);
        }
        let subst = Subst { object: Some(path), value: None, args: Vec::new() };
        self.emit_steps(&prop.cond, &subst, path.span)?;
        Ok(true)
    }

    /// A value used as a condition: it holds when it is not zero.
    fn cond_of_value(&mut self, e: &Expr) -> Result<(), Error> {
        let span = e.span();
        let value = self.expr(e)?;
        self.emit("neq", vec![value, Arg::Int(0)], span)
    }

    fn compare(&mut self, name: &str, left: &Expr, right: &Expr, span: Span) -> Result<(), Error> {
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        self.emit(name, vec![left, right], span)
    }

    /// `a or b`. One condition on the left joins the group of `b` (`this_or_next|`); a left side with
    /// several conditions needs the try-block form.
    fn cond_or(&mut self, left: &Expr, right: &Expr, span: Span) -> Result<(), Error> {
        self.begin_block();
        let result = self.cond(left);
        let mut left_ops = self.end_block();
        result?;
        let mut joined = false;
        if let Some(mut last) = left_ops.pop() {
            if self.can_fail(&last) && left_ops.iter().all(|op| !self.can_fail(op)) {
                last.this_or_next = true;
                left_ops.push(last);
                joined = true;
            } else {
                left_ops.push(last);
            }
        }
        if joined {
            for op in left_ops {
                self.push(op);
            }
            return self.cond(right);
        }
        self.begin_block();
        let result = self.cond(right);
        let right_ops = self.end_block();
        result?;
        let flag = self.temp();
        self.emit("assign", vec![flag.clone(), Arg::Int(0)], span)?;
        self.emit("try_begin", Vec::new(), span)?;
        for op in left_ops {
            self.push(op);
        }
        self.emit("assign", vec![flag.clone(), Arg::Int(1)], span)?;
        self.emit("else_try", Vec::new(), span)?;
        for op in right_ops {
            self.push(op);
        }
        self.emit("assign", vec![flag.clone(), Arg::Int(1)], span)?;
        self.emit("try_end", Vec::new(), span)?;
        self.emit("neq", vec![flag, Arg::Int(0)], span)
    }

    /// `not e`: one condition flips its `neg`; several are measured and the opposite is required.
    fn cond_not(&mut self, inner: &Expr, span: Span) -> Result<(), Error> {
        self.begin_block();
        let result = self.cond(inner);
        let mut ops = self.end_block();
        result?;
        if let Some(mut last) = ops.pop() {
            if self.can_fail(&last) && ops.iter().all(|op| !self.can_fail(op)) {
                last.neg = !last.neg;
                for op in ops {
                    self.push(op);
                }
                self.push(last);
                return Ok(());
            }
            ops.push(last);
        }
        let flag = self.temp();
        self.emit("assign", vec![flag.clone(), Arg::Int(0)], span)?;
        self.emit("try_begin", Vec::new(), span)?;
        for op in ops {
            self.push(op);
        }
        self.emit("assign", vec![flag.clone(), Arg::Int(1)], span)?;
        self.emit("try_end", Vec::new(), span)?;
        self.emit("eq", vec![flag, Arg::Int(0)], span)
    }

    // ---- property entries ---------------------------------------------------------------------

    /// Emits the property's condition, if the call names one; `false` when it does not.
    fn cond_entry(&mut self, call: &Call) -> Result<bool, Error> {
        let parts: Vec<String> = call.callee.parts.iter().map(|part| part.text.clone()).collect();
        let Some(hit) = self.find_property(&parts, Some(call.args.len())) else {
            return Ok(false);
        };
        if self.data.properties[hit.prop].cond.is_empty() {
            return Ok(false);
        }
        let (prop, args) = self.resolve_call(call)?;
        let subst = Subst { object: Some(&call.callee), value: None, args };
        self.emit_steps(&prop.cond, &subst, call.span)?;
        Ok(true)
    }

    /// The property a call names, with its arguments.
    fn resolve_call(&mut self, call: &Call) -> Result<(Property, Vec<(String, SubArg)>), Error> {
        let parts: Vec<String> = call.callee.parts.iter().map(|part| part.text.clone()).collect();
        let Some(hit) = self.find_property(&parts, Some(call.args.len())) else {
            return Err(self.error(call.span, &format!("unknown property '{}()'", parts.join("."))));
        };
        let prop = self.data.properties[hit.prop].clone();
        let args = self.call_bindings(&prop, &call.args, call.span)?;
        Ok((prop, args))
    }

    /// Binds call arguments to the placeholders of the pattern; a bare name stays a name (`key.held(alt)`).
    fn call_bindings(
        &mut self,
        prop: &Property,
        args: &[Expr],
        span: Span,
    ) -> Result<Vec<(String, SubArg)>, Error> {
        let names = placeholders(&prop.path);
        if names.len() != args.len() {
            return Err(self.error(
                span,
                &format!("'{}' takes {} argument(s)", prop.path, names.len()),
            ));
        }
        let mut out = Vec::with_capacity(names.len());
        for (name, arg) in names.into_iter().zip(args) {
            if prop.kit {
                // A kit action's parameter takes the engine's own value: an id as it is, a number in
                // whole units, a text through a string register (CORE-v0 §4).
                let value = match (kit::param_unit(&name), arg) {
                    ("text", other) => {
                        let value = self.expr(other)?;
                        self.unit_value(value, Some("text"), span)?
                    }
                    ("fixed", other) => self.expr(other)?,
                    (_, Expr::Path(path)) => self.value_path_in(path, true)?,
                    (_, other) => {
                        let value = self.expr(other)?;
                        self.to_engine(value, span)?
                    }
                };
                out.push((name, SubArg::Arg(value)));
                continue;
            }
            let sub = match arg {
                Expr::Path(path) if path.parts.len() == 1 && self.is_bare_name(&path.parts[0].text) => {
                    let text = path.parts[0].text.clone();
                    SubArg::Name(prop.aliases.get(&text).cloned().unwrap_or(text))
                }
                other => SubArg::Arg(self.expr(other)?),
            };
            out.push((name, sub));
        }
        Ok(out)
    }

    /// A name that stands for itself (not a local, a global or a constant of this feature).
    fn is_bare_name(&self, name: &str) -> bool {
        let key = (self.feature.clone(), name.to_owned());
        self.local(name).is_none() && !self.globals.contains_key(&key) && !self.consts.contains_key(&key)
    }

    /// The entry whose pattern names the path's last part, with the pattern's other parts ending the object
    /// (`player.agent.speed` matches `agent.speed`). A longer pattern wins.
    fn find_property(&self, parts: &[String], arity: Option<usize>) -> Option<Hit> {
        let last = parts.last()?;
        let object = &parts[..parts.len() - 1];
        let mut best: Option<(usize, usize)> = None;
        for (index, prop) in self.data.properties.iter().enumerate() {
            let pattern: Vec<&str> = prop.path.split('.').collect();
            let (name, count) = match pattern.last() {
                Some(last) => match call_signature(last) {
                    Some((name, count)) => (name, count),
                    None => ((*last).to_owned(), 0),
                },
                None => continue,
            };
            if name != *last {
                continue;
            }
            match arity {
                Some(arity) if arity != count => continue,
                // A value or a place never matches a call pattern (`scene.jump({entry_no})`).
                None if pattern.last().is_some_and(|last| last.contains('(')) => continue,
                _ => {}
            }
            let prefix = &pattern[..pattern.len() - 1];
            if prefix.len() > object.len() {
                continue;
            }
            let tail = &object[object.len() - prefix.len()..];
            // `troop.player.name` matches `troop.name`: the object is a kit thing, and only the words of its
            // own kind apply to it (`player.face_keys` is a multiplayer player's, not `troop.player`'s).
            let matches = if object.len() == 2 && self.record_ref(object).is_some() {
                prefix.len() == 1 && object[0] == prefix[0]
            } else {
                prefix.iter().zip(tail).all(|(a, b)| *a == b.as_str())
            };
            if !matches {
                continue;
            }
            if best.is_none_or(|(_, len)| pattern.len() > len) {
                best = Some((index, pattern.len()));
            }
        }
        best.map(|(prop, _)| Hit { prop })
    }

    /// Emits the operations of a property entry.
    fn emit_steps(&mut self, steps: &[Step], subst: &Subst, span: Span) -> Result<(), Error> {
        for step in steps {
            let mut args = Vec::with_capacity(step.args.len());
            for token in &step.args {
                args.push(self.template(token, subst, span)?);
            }
            self.emit(&step.op, args, span)?;
        }
        Ok(())
    }

    /// One template argument: a whole placeholder substitutes a value or a name, otherwise the text is
    /// filled in (`key_{key}` → `key_left_alt`).
    fn template(&mut self, token: &str, subst: &Subst, span: Span) -> Result<Arg, Error> {
        if let Some(name) = whole_placeholder(token) {
            if name == "object" {
                return match subst.object {
                    Some(path) => self.object_of(path),
                    None => Err(self.error(span, "this entry needs an object in front of the property")),
                };
            }
            if let Some(arg) = subst.arg(name) {
                return Ok(arg.clone());
            }
            if let Some(text) = subst.text(name) {
                return self.literal(text, span);
            }
            return Err(self.error(span, &format!("'{{{name}}}' is not available here")));
        }
        let name = substitute_text(token, subst).map_err(|missing| {
            self.error(span, &format!("'{{{missing}}}' cannot be written inside a name"))
        })?;
        self.literal(&name, span)
    }

    /// A literal template argument: `@text` (a quick string), a global (`$uw_x`), a number, or an atlas name.
    fn literal(&mut self, token: &str, span: Span) -> Result<Arg, Error> {
        if token.starts_with('@') {
            return Ok(Arg::Text(token.to_owned())); // a quick string keeps its `@`, like `text_value`
        }
        if let Some(name) = token.strip_prefix('$') {
            if token == time::CLOCK_MS {
                self.mission_clock = true; // `mission.time` reads the saved clock: keep it moving
            }
            return Ok(Arg::Global(format!("${name}")));
        }
        if let Ok(number) = token.parse::<i64>() {
            return Ok(Arg::Int(number));
        }
        if self.atlas.constant(token).is_some() || self.atlas.entry_by_id(token).is_some() {
            return Ok(Arg::Name(token.to_owned()));
        }
        Err(self.error(span, &format!("the atlas has no '{token}'")))
    }

    /// A UW value in the unit the property declares (CORE-v0 §4).
    fn unit_value(&mut self, value: Arg, unit: Option<&str>, span: Span) -> Result<Arg, Error> {
        match unit {
            // The engine counts whole units; UW numbers are thousandths.
            Some("int") => match value {
                Arg::Int(milli) => Ok(Arg::Int(milli / 1000)),
                other => {
                    let temp = self.temp();
                    self.emit("store_div", vec![temp.clone(), other, Arg::Int(1000)], span)?;
                    Ok(temp)
                }
            },
            // A text goes in through one of the compiler's string registers: every kit operation that reads
            // a string reads a register (`troop_set_face_keys` reads nothing else in Native).
            Some("text") => match value {
                Arg::Text(text) => {
                    let reg = Arg::Name(format!("s{}", 60 + self.next_str % 8));
                    self.next_str += 1;
                    self.emit("str_store_string", vec![reg.clone(), Arg::Text(text)], span)?;
                    Ok(reg)
                }
                other => Ok(other),
            },
            // A percent is the number as written.
            Some("percent") | None => Ok(value),
            Some(other) => {
                Err(self.error(span, &format!("unknown unit '{other}' (int, percent or text)")))
            }
        }
    }
}

/// First of the registers a text's numbers go through (`reg52..reg59`: unused by Native's module files).
const TEXT_REG_FIRST: u8 = 52;
/// How many numbers one text can show.
const TEXT_REGS: usize = 8;

/// `s0`..`s67`: a string register name.
fn is_str_reg(name: &str) -> bool {
    name.strip_prefix('s').is_some_and(|digits| !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
}

/// True when any path inside `body` returns a value (`return <expr>`).
fn block_returns_value(body: &Block) -> bool {
    body.iter().any(|stmt| match stmt {
        Stmt::Return { value: Some(_), .. } => true,
        Stmt::If { then, otherwise, .. } => {
            block_returns_value(then) || otherwise.as_ref().is_some_and(block_returns_value)
        }
        Stmt::For { body, .. } => block_returns_value(body),
        _ => false,
    })
}

/// The kind word of an atlas table: its singular (`parties` → `party`, `meshes` → `mesh`, `troops` → `troop`).
pub fn singular(table: &str) -> String {
    if let Some(stem) = table.strip_suffix("ies") {
        return format!("{stem}y");
    }
    for ending in ["ches", "shes", "sses"] {
        if table.ends_with(ending) {
            return table[..table.len() - 2].to_owned();
        }
    }
    table.strip_suffix('s').unwrap_or(table).to_owned()
}

/// `name` and the number of arguments of a call pattern (`held({key})` → `held`, 1).
fn call_signature(pattern: &str) -> Option<(String, usize)> {
    let (name, inner) = pattern.split_once('(')?;
    let inner = inner.strip_suffix(')')?;
    let count = if inner.trim().is_empty() { 0 } else { inner.split(',').count() };
    Some((name.to_owned(), count))
}

/// The placeholder names of a pattern's last part, in order (`key.held({key})` → `["key"]`).
fn placeholders(pattern: &str) -> Vec<String> {
    let Some(last) = pattern.rsplit('.').next() else { return Vec::new() };
    let Some((_, inner)) = last.split_once('(') else { return Vec::new() };
    inner
        .trim_end_matches(')')
        .split(',')
        .map(|part| part.trim().trim_start_matches('{').trim_end_matches('}').to_owned())
        .filter(|name| !name.is_empty())
        .collect()
}

/// `{name}` when the whole token is one placeholder.
fn whole_placeholder(token: &str) -> Option<&str> {
    let inner = token.strip_prefix('{')?.strip_suffix('}')?;
    if inner.contains('{') || inner.contains('}') {
        return None;
    }
    Some(inner.trim())
}

/// Fills `{name}` inside a name; the error carries the missing placeholder.
fn substitute_text(token: &str, subst: &Subst) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = token;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let Some(close) = rest[open..].find('}') else {
            return Err(rest[open..].trim_matches('{').to_owned());
        };
        let name = rest[open + 1..open + close].trim();
        match subst.text(name) {
            Some(text) => out.push_str(text),
            None => return Err(name.to_owned()),
        }
        rest = &rest[open + close + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

fn dotted(path: &Path) -> String {
    path.parts.iter().map(|part| part.text.as_str()).collect::<Vec<_>>().join(".")
}
