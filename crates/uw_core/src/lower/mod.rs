//! Lowering: [`ast::File`]s → [`Lowered`] Native operations (docs/dil/CORE-v0.md §4).
//!
//! The crate's rule (KARAR-DIL §2b) holds here too: no property name is written in Rust. A property
//! (`player.name`, `key.held(alt)`, …) is data in `kit-meta/core/properties.toml`; the code below only knows
//! the language's fixed constructs — variables, assignments, `if`/`for`/`let`/`fn`, `on game.start`.
//!
//! Numbers are the language's fixed-point thousandths ([`crate::ast::Expr::Number`]); a property entry says
//! in which unit the engine wants the value (`unit = "int"`: ÷1000, `"percent"`: the percent as written).

use std::collections::BTreeMap;
use std::path::Path as FsPath;

use serde::Deserialize;
use uw_atlas::Atlas;

use crate::ast::{
    AssignOp, BinOp, Block, Call, Expr, File, Ident, Item, Path, Span, Stmt, TextPart, VarDecl,
};
use crate::slots::SlotLock;
use crate::{Arg, Error, Lowered, Op};

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
    /// Other spellings accepted as a call argument (`alt` → `left_alt`).
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
}

/// One operation template with its arguments (`{dest}`, `{value}`, `{object}`, a call argument or a literal).
#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub op: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl CoreData {
    /// The vocabulary compiled into this build (`kit-meta/core/properties.toml`).
    pub fn builtin() -> Result<CoreData, Error> {
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
/// write back (meclis "maden" KARAR §2).
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
            if decl.kind.is_some() {
                continue; // an object field is a slot, not a variable with a start value
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
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ctx.finish())
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
        match name {
            // A value-producing entry writes into `{dest}`; a set entry takes the same argument as `{value}`.
            "dest" | "value" => self.value.as_ref(),
            _ => self.args.iter().find(|(key, _)| key == name).and_then(|(_, arg)| match arg {
                SubArg::Arg(arg) => Some(arg),
                SubArg::Name(_) => None,
            }),
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
    /// The file being lowered (`file:line` in messages).
    file: String,
    /// The feature being lowered (the namespace of its variables and constants).
    feature: String,
    /// Constants per (feature, name).
    consts: BTreeMap<(String, String), Expr>,
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
}

impl<'a> Ctx<'a> {
    pub fn new(atlas: &'a Atlas, data: &'a CoreData) -> Ctx<'a> {
        Ctx {
            atlas,
            data,
            blocks: vec![Vec::new()],
            next_temp: 0,
            file: String::new(),
            feature: String::new(),
            consts: BTreeMap::new(),
            globals: BTreeMap::new(),
            fields: BTreeMap::new(),
            slots: BTreeMap::new(),
            next_slot: BTreeMap::new(),
            lock: SlotLock::new(),
            locals: Vec::new(),
            resolving: Vec::new(),
            scripts: Vec::new(),
            start: Vec::new(),
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
            Expr::Percent { value, .. } => Ok(Arg::Int(*value)),
            Expr::Color { rgb, .. } => Ok(Arg::Int(i64::from(*rgb))),
            Expr::Bool { value, .. } => Ok(Arg::Int(i64::from(*value))),
            Expr::Rate { span, .. } => {
                Err(self.error(*span, "a rate needs a continuous block (`while`), not lowered yet"))
            }
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
        lowered.scripts = std::mem::take(&mut self.scripts);
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

    /// Collects constants, feature variables and object fields of every file before lowering starts.
    fn declare(&mut self, files: &[File], errors: &mut Vec<Error>) {
        // Constants first: a range bound may name one.
        for file in files {
            for item in &file.items {
                let Item::Const { name, value, span } = item else { continue };
                let key = (file.feature.clone(), name.text.clone());
                if self.consts.insert(key, value.clone()).is_some() {
                    errors.push(Error {
                        file: file.feature.clone(),
                        line: span.line,
                        message: format!("constant '{}' is declared twice", name.text),
                    });
                }
            }
        }
        for file in files {
            self.enter(file);
            for item in &file.items {
                let Item::Var(decl) = item else { continue };
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
            Item::WhileIn { span, .. } => {
                Err(self.error(*span, "`while in` (continuous blocks) is not lowered yet"))
            }
            Item::While { span, .. } => {
                Err(self.error(*span, "`while` (continuous blocks) is not lowered yet"))
            }
            Item::Ui { span, .. } => Err(self.error(*span, "`ui` is not lowered yet")),
        }
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

    /// A statement call: a condition property gates the rest of the block; a value property is computed.
    fn call_stmt(&mut self, call: &Call) -> Result<(), Error> {
        if self.cond_entry(call)? {
            return Ok(());
        }
        let (prop, args) = self.resolve_call(call)?;
        if prop.get.is_empty() {
            return Err(self.error(call.span, &format!("'{}' has no operations to run", prop.path)));
        }
        let dest = self.temp();
        let subst = Subst { object: Some(&call.callee), value: Some(dest), args };
        self.emit_steps(&prop.get, &subst, call.span)
    }

    // ---- places -------------------------------------------------------------------------------

    fn assign_var(&mut self, var: Var, op: AssignOp, value: &Expr, span: Span) -> Result<(), Error> {
        let (target, range) = match var {
            Var::Local(name) => (Arg::Local(name), None),
            Var::Global { name, range } => (Arg::Global(name), range),
        };
        let value = self.expr(value)?;
        match val_op(op) {
            None => self.emit("assign", vec![target.clone(), value], span)?,
            Some(name) => self.emit(name, vec![target.clone(), value], span)?,
        }
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
                let name = val_op(op).unwrap_or("val_add");
                self.emit(name, vec![target.clone(), value], span)?;
            }
        }
        self.clamp(&target, range, span)?;
        self.emit(&set_op, vec![object, Arg::Int(slot), target], span)
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
        self.value_path(&Path { parts, span: path.span })
    }

    // ---- values -------------------------------------------------------------------------------

    fn value_path(&mut self, path: &Path) -> Result<Arg, Error> {
        let parts: Vec<String> = path.parts.iter().map(|part| part.text.clone()).collect();
        let span = path.span;
        if let Some(var) = self.var_target(&parts) {
            return Ok(match var {
                Var::Local(name) => Arg::Local(name),
                Var::Global { name, .. } => Arg::Global(name),
            });
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
                return value;
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
            return Ok(dest);
        }
        if let Some(hit) = self.find_property(&parts, None) {
            let prop = self.data.properties[hit.prop].clone();
            if prop.get.is_empty() {
                return Err(self.error(span, &format!("'{}' has no value", prop.path)));
            }
            let dest = self.temp();
            let subst = Subst { object: Some(path), value: Some(dest.clone()), args: Vec::new() };
            self.emit_steps(&prop.get, &subst, span)?;
            return Ok(dest);
        }
        Err(self.error(span, &format!("unknown name '{}'", parts.join("."))))
    }

    fn call_value(&mut self, call: &Call) -> Result<Arg, Error> {
        let (prop, args) = self.resolve_call(call)?;
        let object = Some(&call.callee);
        if !prop.get.is_empty() {
            let dest = self.temp();
            let subst = Subst { object, value: Some(dest.clone()), args };
            self.emit_steps(&prop.get, &subst, call.span)?;
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
        Err(self.error(call.span, &format!("'{}' has no value", prop.path)))
    }

    fn text_value(&mut self, parts: &[TextPart], span: Span) -> Result<Arg, Error> {
        let mut text = String::from("@");
        for part in parts {
            match part {
                TextPart::Literal(literal) => text.push_str(literal),
                TextPart::Expr(_) => {
                    return Err(self.error(
                        span,
                        "a string with `{…}` needs string registers, not lowered yet",
                    ))
                }
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

    /// Folds two numbers; otherwise writes the operation into a fresh local.
    fn arith(&mut self, left: Arg, name: &str, right: Arg, span: Span) -> Result<Arg, Error> {
        if let (Arg::Int(left), Arg::Int(right)) = (&left, &right) {
            let folded = match name {
                "store_add" => Some(left.wrapping_add(*right)),
                "store_sub" => Some(left.wrapping_sub(*right)),
                "store_mul" => Some(left.wrapping_mul(*right)),
                "store_div" if *right == 0 => return Err(self.error(span, "division by zero")),
                "store_div" => Some(left.wrapping_div(*right)),
                _ => None,
            };
            if let Some(folded) = folded {
                return Ok(Arg::Int(folded));
            }
        }
        let dest = self.temp();
        self.emit(name, vec![dest.clone(), left, right], span)?;
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
            other => self.cond_of_value(other),
        }
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
            if let Some(arity) = arity {
                if arity != count {
                    continue;
                }
            }
            let prefix = &pattern[..pattern.len() - 1];
            if prefix.len() > object.len() {
                continue;
            }
            let tail = &object[object.len() - prefix.len()..];
            if !prefix.iter().zip(tail).all(|(a, b)| *a == b.as_str()) {
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
    fn literal(&self, token: &str, span: Span) -> Result<Arg, Error> {
        if let Some(text) = token.strip_prefix('@') {
            return Ok(Arg::Text(text.to_owned()));
        }
        if let Some(name) = token.strip_prefix('$') {
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
            // A percent is the number as written; text goes through as the string it is.
            Some("percent") | Some("text") | None => Ok(value),
            Some(other) => {
                Err(self.error(span, &format!("unknown unit '{other}' (int, percent or text)")))
            }
        }
    }
}

/// `val_add` & co. for an assignment operator (`None` for `=`).
fn val_op(op: AssignOp) -> Option<&'static str> {
    match op {
        AssignOp::Set => None,
        AssignOp::Add => Some("val_add"),
        AssignOp::Sub => Some("val_sub"),
        AssignOp::Mul => Some("val_mul"),
        AssignOp::Div => Some("val_div"),
    }
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
