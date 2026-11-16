//! Names and shapes: every name in a parsed UW mod is looked up in the Native atlas (nothing is
//! guessed), and the mod becomes a list of checked patches for `uw_lower`.
//!
//! Faz 1 meaning (KARARNAMELER, Sözdizimi 2026-11-15):
//! - `override menu <m> { on enter { ... } }` replaces menu `<m>`'s on-enter operations
//!   (module_game_menus.py:25, field 5 = operations block).
//! - `on game.<hook> { ... }` appends to the engine hook script `game_<hook>`
//!   (module_scripts.py:22, field 2 = operation block).
//! - a statement is a Native operation from header_operations.py; a name argument is a Native
//!   reference (`p_town_6`, written into the operation as a string the kit resolves) or a kit
//!   constant (`reg0`, `tf_male`, imported from the header that defines it).

use std::collections::BTreeMap;

use uw_atlas::Atlas;
use uw_diag::{suggest, Diagnostic, SourceFile, Span};
use uw_syntax::{Arg, Block, Cond, Expr, File, Ident, Item, Member, Place, Stmt, When};
#[allow(unused_imports)]
use uw_syntax::Call;

pub const E0001_UNKNOWN_NAME: &str = uw_diag::E0001_UNKNOWN_NAME;
pub const E0201_NOT_YET: &str = "E0201";
pub const E0202_DUPLICATE: &str = "E0202";
pub const E0203_TOO_FEW: &str = "E0203";
pub const E0204_TEXT: &str = "E0204";
pub const E0205_MOD_NAME: &str = "E0205";
pub const E0206_OPTION_EXISTS: &str = "E0206";
pub const E0207_OPTION_UNKNOWN: &str = "E0207";
pub const E0208_BAD_ID: &str = "E0208";
pub const E0209_NEW_EXISTS: &str = "E0209";
pub const E0210_LOCAL_TEMP: &str = "E0210";
pub const E0211_MEMBER_TWICE: &str = "E0211";
pub const E0212_ASSIGN_TARGET: &str = "E0212";
pub const E0213_FN_ARITY: &str = "E0213";
pub const E0214_FN_OP_NAME: &str = "E0214";
pub const E0215_FN_CYCLE: &str = "E0215";
/// A dotted object word (`player.gold`) the fns do not cover: the fn its form calls is missing, or
/// the form has no verb (T-082).
pub const E0216_OBJECT_WORD: &str = "E0216";
/// A `save` name that a game name, a kit constant or a Native operation already uses (T-085).
pub const E0217_SAVE_NAME: &str = "E0217";

/// How deep `fn` calls may nest before the body is expanded, so a runaway chain stops.
const FN_DEPTH: usize = 16;

/// A parsed source file.
pub struct Source {
    pub file: SourceFile,
    pub tree: File,
}

/// An error in source `file` (index into the sources given to [`check`]).
#[derive(Debug)]
pub struct Error {
    pub file: usize,
    pub diag: Diagnostic,
}

/// Values and operations are defined in `uw_lower` (it turns the checked tree into them).
pub use uw_lower::{Op, Value};

/// What a patch does to its entry. `path` (on [`Patch`]) is the field it works on; `ops` is the
/// lowered operation block where the action has one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Replace the operation block at `path` with `ops`.
    ReplaceBody,
    /// Append `ops` to the operation block at `path`.
    Append,
    /// Replace the text field at `path` (`text "…"`, menu field 2).
    ReplaceText(String),
    /// Add a new option to the menu's option list (`option …`).
    InsertOption { option: NewOption, place: OptionPlace },
    /// Change parts of an existing option (`change option …`); `None` keeps the Native part.
    ChangeOption { id: String, text: Option<String>, condition: Option<Vec<Op>>, consequence: Option<Vec<Op>> },
    /// Remove an existing option (`remove option …`).
    RemoveOption(String),
    /// A new menu appended to the list (`new menu_x …`); `entry` holds its bare name.
    NewMenu { text: String, enter: Vec<Op>, options: Vec<NewOption> },
}

/// A menu option written in UW: kit layout module_game_menus.py:28 (id, condition, text,
/// consequence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOption {
    /// Without `mno_` (the kit adds it).
    pub id: String,
    pub condition: Vec<Op>,
    pub text: String,
    pub consequence: Vec<Op>,
    pub origin: String,
}

/// Where a new option goes: the end of the list, or after/before an option id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionPlace {
    End,
    After(String),
    Before(String),
}

/// One checked change to a Native entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patch {
    /// Python module of the table (`module_game_menus`).
    pub module: String,
    /// List variable in it (`game_menus`).
    pub var: String,
    /// What `uw_runtime.find` takes: the entry's name, `name#n` when the name repeats.
    pub entry: String,
    /// Field path to the operation block inside the entry.
    pub path: Vec<usize>,
    pub action: Action,
    pub ops: Vec<Op>,
    pub origin: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    pub name: String,
    pub patches: Vec<Patch>,
}

/// Fields of a menu entry: module_game_menus.py:20-32 (id, flags, text, mesh, enter block,
/// options) and module_scripts.py:22 (scripts, 2nd field = operation block).
const MENU_TEXT_FIELD: usize = 2;
const MENU_ENTER_FIELD: usize = 4;
const MENU_OPTIONS_FIELD: usize = 5;
const SCRIPT_BODY_FIELD: usize = 1;

fn diag(code: &'static str, message: String, span: Span, help: Option<String>) -> Diagnostic {
    Diagnostic { code, message, span, notes: Vec::new(), help }
}

/// Smallest number of values `op` needs according to its header comment
/// (`(name,<a>,<b>,[c])`: 2), when the comment has that form.
pub fn min_args(op: &uw_atlas::Op) -> Option<usize> {
    let sig = &op.signature;
    let head = format!("({}", op.name);
    let start = sig.find(&head)?;
    let rest = &sig[start + head.len()..];
    let rest = rest.trim_start();
    if !(rest.starts_with(',') || rest.starts_with(')')) {
        return None;
    }
    let close = rest.find(')')?;
    let inner = &rest[..close];
    let mut depth = 0i32;
    let mut required = 0;
    for c in inner.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            '<' if depth == 0 => required += 1,
            _ => {}
        }
    }
    Some(required)
}

/// A `fn` of the mod: where it was written and what it does. `check` collects every one before it
/// checks a file, so a `fn` is visible from all files of the mod (T-078).
struct FnDef<'a> {
    /// Index into the sources given to [`check`]: an inlined body keeps its own file and lines.
    file: usize,
    /// `file:line` of `fn <name>`, for "two fns with one name".
    origin: String,
    params: &'a [Ident],
    body: &'a Block,
}

/// `file:line` of `span` in `file`.
fn origin_in(file: &SourceFile, span: Span) -> String {
    let (line, _) = file.line_col(span.start);
    format!("{}:{line}", file.name)
}

/// Every `fn` of `sources` by name, and the errors its definitions make: a fn may not take the
/// name of a Native operation, and one name is defined once (T-078).
fn collect_fns<'a>(atlas: &Atlas, sources: &'a [Source]) -> (BTreeMap<String, FnDef<'a>>, Vec<Error>) {
    let mut fns: BTreeMap<String, FnDef> = BTreeMap::new();
    let mut errors = Vec::new();
    for (index, source) in sources.iter().enumerate() {
        for item in &source.tree.items {
            let Item::Fn { name, params, body, .. } = item else { continue };
            if atlas.op(&name.text).is_some() {
                errors.push(Error {
                    file: index,
                    diag: diag(
                        E0214_FN_OP_NAME,
                        format!("`fn {}` has the name of a Native operation", name.text),
                        name.span,
                        Some(format!("its calls would be the Native operation; name it e.g. `fn uw_{}`", name.text)),
                    ),
                });
                continue;
            }
            if let Some(first) = fns.get(&name.text) {
                let existing = first.origin.clone();
                errors.push(Error {
                    file: index,
                    diag: diag(
                        E0202_DUPLICATE,
                        format!("`fn {}` is already defined at {existing}", name.text),
                        name.span,
                        Some("a fn name is used once in a mod; rename one of them".into()),
                    ),
                });
                continue;
            }
            let origin = origin_in(&source.file, name.span);
            fns.insert(name.text.clone(), FnDef { file: index, origin, params, body });
        }
    }
    (fns, errors)
}

/// A `save` of the mod (T-085): one named `$global` that lives in the save game, with the value
/// written once at game start.
struct SaveDef {
    name: String,
    start: i64,
    /// `file:line` of `save <name>`, for "two saves with one name" and the start operation.
    origin: String,
}

struct Ctx<'a> {
    atlas: &'a Atlas,
    errors: Vec<Error>,
    file: usize,
    source: &'a SourceFile,
    /// Next `:_uwN` temporary, numbered from 0 per option condition.
    temp: usize,
    /// Native mode ([`check_block`]): names and shapes are taken as Native wrote them, so flagged
    /// names (`neg|eq`) are allowed, the header signature is not counted and text is not checked.
    native: bool,
    /// Every `fn` of the mod, by name (empty in Native mode and in [`check_block`]).
    fns: &'a BTreeMap<String, FnDef<'a>>,
    /// The files `fns` came from, so an inlined body keeps its own file and lines.
    sources: &'a [Source],
    /// `fn` calls being expanded, outermost first (cycle and depth check).
    stack: Vec<String>,
    /// Next `__fnN` suffix, so two calls never share the locals of one body.
    rename: usize,
    /// The `:_…` temporaries lowering made itself, so [`Ctx::local`] accepts them and no other
    /// `:_` name (an object word's `get` fills one through a fn parameter, T-082).
    temps: Vec<String>,
    /// Every `save` of the mod: its bare name means `$<name>` in every file (T-085).
    saves: &'a [SaveDef],
}

impl Ctx<'_> {
    fn error(&mut self, d: Diagnostic) {
        self.errors.push(Error { file: self.file, diag: d });
    }

    fn origin(&self, span: Span) -> String {
        origin_in(self.source, span)
    }

    /// Checks statements written in the file at `index` and goes back to the caller's file
    /// afterwards: an inlined `fn` body reports its own file and lines.
    fn stmts_in(&mut self, stmts: &[Stmt], index: usize) -> Vec<uw_lower::Stmt> {
        let sources = self.sources;
        let (file, source) = (self.file, self.source);
        self.file = index;
        self.source = &sources[index].file;
        let out = self.stmts(stmts);
        self.file = file;
        self.source = source;
        out
    }

    /// Checks a block and lowers it to the operations the kit writes.
    fn ops(&mut self, stmts: &[Stmt]) -> Vec<Op> {
        let tree = self.stmts(stmts);
        uw_lower::lower(&tree)
    }

    /// Checks a block into the tree `uw_lower` lowers.
    fn stmts(&mut self, stmts: &[Stmt]) -> Vec<uw_lower::Stmt> {
        let mut out = Vec::new();
        for stmt in stmts {
            match stmt {
                Stmt::Call(call) => {
                    if self.fns.contains_key(&call.op.text) {
                        out.extend(self.inline(call));
                    } else if let Some(op) = self.call(call) {
                        out.push(uw_lower::Stmt::Op(op));
                    }
                }
                Stmt::Assign { target, op, value, span } => {
                    // `player.gold += 1000` is a call to the fn the word names (T-082).
                    if let Arg::Name(name) = target {
                        if is_dotted(name) {
                            self.dotted_assign(name, *op, value, *span, &mut out);
                            continue;
                        }
                    }
                    // A dotted word in the value is read into a temporary before this statement.
                    let mut pre = Vec::new();
                    let value = self.hoisted_value(value, &mut pre);
                    let checked_target = self.assign_target(target);
                    let checked_value = self.expr(&value);
                    if let (Some(target), Some(value)) = (checked_target, checked_value) {
                        out.extend(pre);
                        out.push(uw_lower::Stmt::Assign {
                            target,
                            op: lower_assign_op(*op),
                            value,
                            origin: self.origin(*span),
                        });
                    }
                }
                Stmt::If { arms, otherwise, span } => {
                    let mut pre = Vec::new();
                    let mut checked_arms = Vec::new();
                    let mut ok = true;
                    for (cond, body) in arms {
                        let checked_cond = self.cond_with_reads(cond, &mut pre);
                        let checked_body = self.stmts(&body.stmts);
                        match checked_cond {
                            Some(cond) => checked_arms.push((cond, checked_body)),
                            None => ok = false,
                        }
                    }
                    let otherwise = otherwise.as_ref().map(|body| self.stmts(&body.stmts));
                    if ok {
                        // The reads of a dotted word come before the condition's operations.
                        out.extend(pre);
                        out.push(uw_lower::Stmt::If { arms: checked_arms, otherwise, origin: self.origin(*span) });
                    }
                }
                Stmt::Require { cond, span } => {
                    let mut pre = Vec::new();
                    if let Some(cond) = self.cond_with_reads(cond, &mut pre) {
                        out.extend(pre);
                        out.push(uw_lower::Stmt::Require { cond, origin: self.origin(*span) });
                    }
                }
                Stmt::Attempt { arms, span } => {
                    let arms = arms.iter().map(|body| self.stmts(&body.stmts)).collect();
                    out.push(uw_lower::Stmt::Attempt { arms, origin: self.origin(*span) });
                }
                Stmt::For { var, kind, args, body, span } => {
                    if let Some(stmt) = self.for_stmt(var, kind, args, body, *span) {
                        out.push(stmt);
                    }
                }
            }
        }
        out
    }

    /// Replaces a `fn` call by its body: each `:param` becomes the argument of this call, every
    /// other `:local` of the body gets a `__fnN` suffix, and a call inside the body is expanded in
    /// turn. A `fn` that reaches itself, or a chain deeper than [`FN_DEPTH`], is one error naming
    /// the chain; the call then adds nothing.
    fn inline(&mut self, call: &Call) -> Vec<uw_lower::Stmt> {
        let name = call.op.text.clone();
        let fns = self.fns;
        let Some(def) = fns.get(&name) else { return Vec::new() };
        if call.args.len() != def.params.len() {
            let params: Vec<&str> = def.params.iter().map(|p| p.text.as_str()).collect();
            let help = if params.is_empty() {
                format!("call it as `{name}()`")
            } else {
                format!("call it as `{name}({})`", params.join(", "))
            };
            self.error(diag(
                E0213_FN_ARITY,
                format!("`fn {name}` takes {} value(s), got {}", def.params.len(), call.args.len()),
                call.span,
                Some(help),
            ));
            return Vec::new();
        }
        let mut chain = self.stack.clone();
        chain.push(name.clone());
        if self.stack.iter().any(|n| n == &name) {
            chain.push(name.clone());
            self.error(diag(
                E0215_FN_CYCLE,
                format!("`fn {name}` never ends: {}", chain.join(" -> ")),
                call.op.span,
                Some("a fn cannot call itself, directly or through other fns".into()),
            ));
            return Vec::new();
        }
        if self.stack.len() >= FN_DEPTH {
            self.error(diag(
                E0215_FN_CYCLE,
                format!("`fn` calls nest more than {FN_DEPTH} deep: {}", chain.join(" -> ")),
                call.op.span,
                Some("shorten the chain of fns, or call a Native operation directly".into()),
            ));
            return Vec::new();
        }
        let binds: Vec<(String, Arg)> =
            def.params.iter().map(|p| p.text.clone()).zip(call.args.iter().cloned()).collect();
        let call_site = self.rename;
        self.rename += 1;
        let body: Vec<Stmt> = def.body.stmts.iter().map(|s| substituted_stmt(s, &binds, call_site)).collect();
        self.stack.push(name);
        let out = self.stmts_in(&body, def.file);
        self.stack.pop();
        out
    }

    /// An assignment target: `$global`, `:local`, or a register kit constant.
    fn assign_target(&mut self, target: &Arg) -> Option<Value> {
        match target {
            Arg::Global(g) | Arg::Local(g) => self.local(g),
            Arg::Name(name) => {
                if let Some(global) = self.save_global(&name.text) {
                    return Some(global);
                }
                let constant = self.atlas.constant(&name.text).map(|c| (c.name.clone(), c.module.clone()));
                match constant {
                    Some((constant, module)) if is_register(&name.text) => Some(Value::Const { name: constant, module }),
                    _ => {
                        self.error(diag(
                            E0212_ASSIGN_TARGET,
                            format!("'{}' cannot be assigned to", name.text),
                            name.span,
                            Some("an assignment target is a $global, a :local or a register such as reg0".into()),
                        ));
                        None
                    }
                }
            }
            Arg::Int(_, span) | Arg::Str(_, span) => {
                self.error(diag(
                    E0212_ASSIGN_TARGET,
                    "an assignment target must be a $global, a :local or a register".into(),
                    *span,
                    None,
                ));
                None
            }
        }
    }

    /// A `$global` / `:local`; `:_` names are reserved for lowering, so only the temporaries
    /// lowering made itself (an object word's `get`, T-082) get through.
    fn local(&mut self, ident: &Ident) -> Option<Value> {
        if ident.text.starts_with(":_") && !self.temps.iter().any(|temp| temp == &ident.text) {
            self.error(diag(
                E0210_LOCAL_TEMP,
                format!("'{}' starts with `:_`, which lowering uses for temporaries", ident.text),
                ident.span,
                Some("user locals may not start with `:_`".into()),
            ));
            None
        } else {
            Some(Value::Ref(ident.text.clone()))
        }
    }

    /// The `$global` a `save` name stands for (`rich` -> `$rich`), if it is one (T-085).
    fn save_global(&self, name: &str) -> Option<Value> {
        self.saves.iter().any(|save| save.name == name).then(|| Value::Ref(format!("${name}")))
    }

    /// A `save` name used as a condition: a bare name is a call without operands (`if rich`).
    fn save_cond(&self, call: &Call) -> Option<Value> {
        if !call.args.is_empty() {
            return None;
        }
        self.save_global(&call.op.text)
    }

    // ----------------------------------------------------------- object words (T-082)

    /// `x.y = e`, `x.y += e`, `x.y -= e`: the fn the word names (`.set`, `.add`, `.remove`) is
    /// called with the value, after the value's own reads.
    fn dotted_assign(
        &mut self,
        name: &Ident,
        op: uw_syntax::AssignOp,
        value: &Expr,
        span: Span,
        out: &mut Vec<uw_lower::Stmt>,
    ) {
        let verb = match op {
            uw_syntax::AssignOp::Set => "set",
            uw_syntax::AssignOp::Add => "add",
            uw_syntax::AssignOp::Sub => "remove",
            // `*=` `/=` `%=` have no verb yet: naming one would be a guess (T-082).
            uw_syntax::AssignOp::Mul | uw_syntax::AssignOp::Div | uw_syntax::AssignOp::Mod => {
                self.error(diag(
                    E0216_OBJECT_WORD,
                    format!("'{}' has no `{}` form", name.text, assign_op_sign(op)),
                    span,
                    Some("a dotted word takes `=`, `+=` and `-=`".into()),
                ));
                return;
            }
        };
        let mut pre = Vec::new();
        let value = self.hoisted_value(value, &mut pre);
        out.extend(pre);
        // A fn takes one value, so an expression becomes a temporary of its own first.
        let arg = match value {
            Expr::Value(arg) => arg,
            other => {
                let Some(value) = self.expr(&other) else { return };
                let temp = self.object_temp("val", span);
                out.push(uw_lower::Stmt::Assign {
                    target: Value::Ref(temp.text.clone()),
                    op: uw_lower::AssignOp::Set,
                    value,
                    origin: self.origin(span),
                });
                Arg::Local(temp)
            }
        };
        self.object_call(name, verb, vec![arg], out);
    }

    /// `x.y` used as a value: `x.y.get(:tmp)` runs before the statement and `:tmp` holds the value.
    fn dotted_read(&mut self, name: &Ident, out: &mut Vec<uw_lower::Stmt>) -> Arg {
        let temp = self.object_temp("get", name.span);
        self.object_call(name, "get", vec![Arg::Local(temp.clone())], out);
        Arg::Local(temp)
    }

    /// The body of `x.y.<verb>(…)` inlined where the word stands; a fn that is not defined is one
    /// error naming it.
    fn object_call(&mut self, word: &Ident, verb: &str, args: Vec<Arg>, out: &mut Vec<uw_lower::Stmt>) {
        let name = format!("{}.{verb}", word.text);
        if !self.fns.contains_key(&name) {
            self.object_fn_error(word, verb);
            return;
        }
        let call = Call { op: Ident { text: name, span: word.span }, args, span: word.span };
        let body = self.inline(&call);
        out.extend(body);
    }

    /// The error for a dotted word whose fn is missing: it names the fn the word would need.
    fn object_fn_error(&mut self, word: &Ident, verb: &str) {
        let name = format!("{}.{verb}", word.text);
        let params = if verb == "get" { ":out" } else { ":value" };
        self.error(diag(
            E0216_OBJECT_WORD,
            format!("'{}' needs `fn {name}`", word.text),
            word.span,
            Some(format!("write `fn {name}({params}) {{ … }}` to give the word its {verb} form")),
        ));
    }

    /// A temporary the object-word form fills before the statement that reads it (`:_uw_get0`).
    /// The word between `:_uw` and the number keeps it apart from the `:_uwN` temporaries
    /// `uw_lower` numbers from 0 in every block.
    fn object_temp(&mut self, kind: &str, span: Span) -> Ident {
        let text = format!("{}_{kind}{}", uw_lower::TEMP_PREFIX, self.temp);
        self.temp += 1;
        self.temps.push(text.clone());
        Ident { text, span }
    }

    /// The expression with every dotted word read into a temporary first; those `get` calls go into
    /// `pre`, in the order the words are read.
    fn hoisted_value(&mut self, expr: &Expr, pre: &mut Vec<uw_lower::Stmt>) -> Expr {
        match expr {
            Expr::Value(Arg::Name(name)) if is_dotted(name) => {
                Expr::Value(self.dotted_read(name, pre))
            }
            Expr::Value(arg) => Expr::Value(arg.clone()),
            Expr::Neg(inner, span) => Expr::Neg(Box::new(self.hoisted_value(inner, pre)), *span),
            Expr::Bin { op, left, right, span } => Expr::Bin {
                op: *op,
                left: Box::new(self.hoisted_value(left, pre)),
                right: Box::new(self.hoisted_value(right, pre)),
                span: *span,
            },
        }
    }

    /// A checked `if`/`require` condition whose dotted words are read into `pre` first, so they are
    /// filled before the condition's own operations.
    fn cond_with_reads(&mut self, cond: &Cond, pre: &mut Vec<uw_lower::Stmt>) -> Option<uw_lower::Cond> {
        let cond = self.hoisted_cond(cond, pre);
        self.cond(&cond)
    }

    fn hoisted_cond(&mut self, cond: &Cond, pre: &mut Vec<uw_lower::Stmt>) -> Cond {
        match cond {
            Cond::Cmp { op, left, right, span } => Cond::Cmp {
                op: *op,
                left: self.hoisted_value(left, pre),
                right: self.hoisted_value(right, pre),
                span: *span,
            },
            // The values of a condition operation are not expressions (T-082 reads only those).
            Cond::Call(call) => Cond::Call(call.clone()),
            Cond::Not(inner, span) => Cond::Not(Box::new(self.hoisted_cond(inner, pre)), *span),
            Cond::And(parts) => Cond::And(parts.iter().map(|part| self.hoisted_cond(part, pre)).collect()),
            Cond::Or(parts) => Cond::Or(parts.iter().map(|part| self.hoisted_cond(part, pre)).collect()),
        }
    }

    /// One value of an operation or expression; a name that does not resolve is an error.
    fn value(&mut self, arg: &Arg) -> Option<Value> {
        match arg {
            Arg::Int(n, _) => Some(Value::Int(*n)),
            Arg::Global(g) | Arg::Local(g) => self.local(g),
            Arg::Str(text, span) => {
                if !self.native {
                    self.check_text(text, *span);
                }
                Some(Value::Text(text.clone()))
            }
            Arg::Name(name) => {
                if let Some(n) = bool_value(&name.text) {
                    return Some(Value::Int(n));
                }
                if let Some(global) = self.save_global(&name.text) {
                    return Some(global);
                }
                if self.atlas.resolve_ref(&name.text).is_some() {
                    // A game name (`praven`, P5b) goes to the kit as its reference (`p_town_6`).
                    let reference = self.atlas.reference_for(&name.text).unwrap_or(&name.text);
                    Some(Value::Ref(reference.to_string()))
                } else if let Some(c) = self.atlas.constant(&name.text) {
                    Some(Value::Const { name: c.name.clone(), module: c.module.clone() })
                } else {
                    let refs: Vec<String> = self
                        .atlas
                        .tables
                        .values()
                        .filter_map(|t| {
                            let tag = t.ref_tag.as_deref()?;
                            Some(t.entries.iter().filter_map(move |e| {
                                e.name.as_deref().map(|n| format!("{tag}_{}", n.to_lowercase()))
                            }))
                        })
                        .flatten()
                        .collect();
                    let candidates = refs
                        .iter()
                        .map(String::as_str)
                        .chain(self.atlas.constants.iter().map(|c| c.name.as_str()))
                        // Game names installed for this mod (P5b) are suggestions too.
                        .chain(refs.iter().filter_map(|r| self.atlas.game_name(r)));
                    let help = suggest(&name.text, candidates).map(|s| format!("did you mean '{s}'?"));
                    self.error(diag(
                        E0001_UNKNOWN_NAME,
                        format!("'{}' is not a Native name or kit constant", name.text),
                        name.span,
                        help.or_else(|| Some("`uw browse` finds Native names by part of the name or in-game text".into())),
                    ));
                    None
                }
            }
        }
    }

    fn check_text(&mut self, text: &str, span: Span) {
        if !text.is_ascii() {
            self.error(diag(
                E0204_TEXT,
                "text with non-ASCII letters is not supported yet".into(),
                span,
                Some("Turkish letters wait for game experiment D5; use plain letters for now".into()),
            ));
        }
    }

    /// A checked integer expression.
    fn expr(&mut self, expr: &Expr) -> Option<uw_lower::Expr> {
        match expr {
            Expr::Value(arg) => self.value(arg).map(uw_lower::Expr::Value),
            Expr::Neg(inner, _) => Some(uw_lower::Expr::Neg(Box::new(self.expr(inner)?))),
            Expr::Bin { op, left, right, .. } => Some(uw_lower::Expr::Bin {
                op: lower_bin_op(*op),
                left: Box::new(self.expr(left)?),
                right: Box::new(self.expr(right)?),
            }),
        }
    }

    /// A checked condition for `if`.
    fn cond(&mut self, cond: &Cond) -> Option<uw_lower::Cond> {
        match cond {
            Cond::Cmp { op, left, right, span } => Some(uw_lower::Cond::Cmp {
                op: lower_cmp_op(*op),
                left: self.expr(left)?,
                right: self.expr(right)?,
                origin: self.origin(*span),
            }),
            // A bare `save` name is `rich != 0`; `not` then flips it to `eq` (T-085).
            Cond::Call(call) => {
                if let Some(global) = self.save_cond(call) {
                    return Some(uw_lower::Cond::Cmp {
                        op: uw_lower::CmpOp::Ne,
                        left: uw_lower::Expr::Value(global),
                        right: uw_lower::Expr::Value(Value::Int(0)),
                        origin: self.origin(call.span),
                    });
                }
                self.call(call).map(uw_lower::Cond::Op)
            }
            Cond::Not(inner, _) => Some(uw_lower::Cond::Not(Box::new(self.cond(inner)?))),
            Cond::And(parts) => {
                let mut out = Vec::new();
                for part in parts {
                    out.push(self.cond(part)?);
                }
                Some(uw_lower::Cond::And(out))
            }
            Cond::Or(parts) => {
                let mut out = Vec::new();
                for part in parts {
                    out.push(self.cond(part)?);
                }
                Some(uw_lower::Cond::Or(out))
            }
        }
    }

    /// A `when <cond>` option condition as the operation list the kit writes (gramer-2 §2).
    /// `uw_lower` only lowers whole statement blocks, so the condition is lowered here.
    fn cond_ops(&mut self, cond: &Cond, span: Span) -> Vec<Op> {
        let mut out = Vec::new();
        self.cond_ops_into(cond, span, &mut out);
        out
    }

    fn cond_ops_into(&mut self, cond: &Cond, fallback: Span, out: &mut Vec<Op>) -> Option<()> {
        match cond {
            Cond::Cmp { op, left, right, span } => {
                let left = self.expr_value(left, out)?;
                let right = self.expr_value(right, out)?;
                out.push(Op {
                    name: cmp_op_name(*op).into(),
                    args: vec![left, right],
                    origin: self.origin(*span),
                });
                Some(())
            }
            Cond::Call(call) => {
                if let Some(global) = self.save_cond(call) {
                    out.push(Op { name: "neq".into(), args: vec![global, Value::Int(0)], origin: self.origin(call.span) });
                    return Some(());
                }
                let op = self.call(call)?;
                out.push(op);
                Some(())
            }
            Cond::Not(inner, _) => self.not_ops_into(inner, fallback, out),
            Cond::And(parts) => {
                for part in parts {
                    self.cond_ops_into(part, fallback, out)?;
                }
                Some(())
            }
            Cond::Or(parts) => self.or_ops_into(parts, fallback, out),
        }
    }

    fn not_ops_into(&mut self, inner: &Cond, fallback: Span, out: &mut Vec<Op>) -> Option<()> {
        match inner {
            Cond::Cmp { op, left, right, span } => {
                let left = self.expr_value(left, out)?;
                let right = self.expr_value(right, out)?;
                out.push(Op {
                    name: not_cmp_op_name(*op).into(),
                    args: vec![left, right],
                    origin: self.origin(*span),
                });
                Some(())
            }
            Cond::Call(call) => {
                if let Some(global) = self.save_cond(call) {
                    out.push(Op { name: "eq".into(), args: vec![global, Value::Int(0)], origin: self.origin(call.span) });
                    return Some(());
                }
                let op = self.call(call)?;
                out.push(Op { name: format!("neg|{}", op.name), args: op.args, origin: op.origin });
                Some(())
            }
            Cond::Not(inner, _) => self.cond_ops_into(inner, fallback, out),
            Cond::And(_) | Cond::Or(_) => {
                let temp = self.next_temp();
                let origin = self.origin(fallback);
                out.push(Op { name: "assign".into(), args: vec![temp.clone(), Value::Int(0)], origin: origin.clone() });
                out.push(Op { name: "try_begin".into(), args: Vec::new(), origin: origin.clone() });
                self.cond_ops_into(inner, fallback, out)?;
                out.push(Op { name: "assign".into(), args: vec![temp.clone(), Value::Int(1)], origin: origin.clone() });
                out.push(Op { name: "try_end".into(), args: Vec::new(), origin: origin.clone() });
                out.push(Op { name: "eq".into(), args: vec![temp, Value::Int(0)], origin });
                Some(())
            }
        }
    }

    fn or_ops_into(&mut self, parts: &[Cond], fallback: Span, out: &mut Vec<Op>) -> Option<()> {
        let mut branches = Vec::new();
        for part in parts {
            let mut branch = Vec::new();
            self.cond_ops_into(part, fallback, &mut branch)?;
            branches.push(branch);
        }
        if branches.iter().all(|branch| branch.len() == 1) {
            let last = branches.len().saturating_sub(1);
            for (index, branch) in branches.into_iter().enumerate() {
                let mut op = branch.into_iter().next().expect("one operation");
                if index < last {
                    op.name = format!("this_or_next|{}", op.name);
                }
                out.push(op);
            }
            Some(())
        } else {
            let temp = self.next_temp();
            let origin = self.origin(fallback);
            out.push(Op { name: "assign".into(), args: vec![temp.clone(), Value::Int(0)], origin: origin.clone() });
            for part in parts {
                out.push(Op { name: "try_begin".into(), args: Vec::new(), origin: origin.clone() });
                self.cond_ops_into(part, fallback, out)?;
                out.push(Op { name: "assign".into(), args: vec![temp.clone(), Value::Int(1)], origin: origin.clone() });
                out.push(Op { name: "try_end".into(), args: Vec::new(), origin: origin.clone() });
            }
            out.push(Op { name: "eq".into(), args: vec![temp, Value::Int(1)], origin });
            Some(())
        }
    }

    /// One expression as a value, writing `store_*` operations for its sub-expressions.
    fn expr_value(&mut self, expr: &Expr, out: &mut Vec<Op>) -> Option<Value> {
        match expr {
            // A dotted word is read before the comparison it takes part in (T-082).
            Expr::Value(Arg::Name(name)) if is_dotted(name) => Some(self.dotted_read_ops(name, out)),
            Expr::Value(arg) => self.value(arg),
            Expr::Neg(inner, span) => {
                if let Expr::Value(Arg::Int(n, _)) = &**inner {
                    return Some(Value::Int(-*n));
                }
                let value = self.expr_value(inner, out)?;
                let temp = self.next_temp();
                out.push(Op {
                    name: "store_sub".into(),
                    args: vec![temp.clone(), Value::Int(0), value],
                    origin: self.origin(*span),
                });
                Some(temp)
            }
            Expr::Bin { op, left, right, span } => {
                let left = self.expr_value(left, out)?;
                let right = self.expr_value(right, out)?;
                let temp = self.next_temp();
                out.push(Op { name: bin_op_name(*op).into(), args: vec![temp.clone(), left, right], origin: self.origin(*span) });
                Some(temp)
            }
        }
    }

    fn next_temp(&mut self) -> Value {
        let name = format!("{}{}", uw_lower::TEMP_PREFIX, self.temp);
        self.temp += 1;
        Value::Ref(name)
    }

    /// `x.y` in a `when` condition: `x.y.get(:tmp)` goes into `out` first — before the condition's
    /// own operations — and `:tmp` is the value read.
    fn dotted_read_ops(&mut self, name: &Ident, out: &mut Vec<Op>) -> Value {
        let temp = self.object_temp("get", name.span);
        let fn_name = format!("{}.get", name.text);
        if !self.fns.contains_key(&fn_name) {
            self.object_fn_error(name, "get");
            return Value::Ref(temp.text);
        }
        let call = Call {
            op: Ident { text: fn_name, span: name.span },
            args: vec![Arg::Local(temp.clone())],
            span: name.span,
        };
        let body = self.inline(&call);
        // The get's own temporaries continue this list's numbering, never restarting at 0.
        let (ops, next) = uw_lower::lower_from(&body, self.temp);
        out.extend(ops);
        self.temp = next;
        Value::Ref(temp.text)
    }

    /// The bare option id; `mno_` is the kit's prefix, never the source form.
    fn option_id(&mut self, id: &Ident) -> Option<String> {
        if !id.text.is_ascii() {
            self.error(diag(
                E0208_BAD_ID,
                format!("option id '{}' has non-ASCII letters", id.text),
                id.span,
                Some("option ids use ASCII letters, digits and _".into()),
            ));
            return None;
        }
        if let Some(bare) = id.text.strip_prefix("mno_") {
            self.error(diag(
                E0208_BAD_ID,
                format!("option id '{}' is written without `mno_`", id.text),
                id.span,
                Some(format!("write '{bare}'; the kit adds `mno_` itself")),
            ));
            return None;
        }
        Some(id.text.clone())
    }

    /// A new option; `native` is the menu's Native option id list when it already exists.
    fn build_option(
        &mut self,
        menu: &str,
        native: Option<&[String]>,
        id: &Ident,
        text: &str,
        when: Option<&When>,
        body: &Block,
        span: Span,
    ) -> Option<NewOption> {
        let id_span = id.span;
        let id = self.option_id(id)?;
        if let Some(native) = native {
            if native.iter().any(|existing| existing == &id) {
                self.error(diag(
                    E0206_OPTION_EXISTS,
                    format!("menu '{menu}' already has option '{id}'"),
                    id_span,
                    Some(format!("change it with `change option {id}`")),
                ));
                return None;
            }
        }
        self.check_text(text, span);
        let condition = match when {
            Some(When::Block(block)) => self.ops(&block.stmts),
            Some(When::Cond(cond)) => self.cond_ops(cond, span),
            None => Vec::new(),
        };
        let consequence = self.ops(&body.stmts);
        Some(NewOption { id, condition, text: text.to_owned(), consequence, origin: self.origin(span) })
    }

    fn option_place(&mut self, place: Option<&Place>) -> Option<OptionPlace> {
        match place {
            None => Some(OptionPlace::End),
            Some(Place::After(id)) => Some(OptionPlace::After(self.option_id(id)?)),
            Some(Place::Before(id)) => Some(OptionPlace::Before(self.option_id(id)?)),
        }
    }

    fn insert_option_patch(
        &mut self,
        menu: &str,
        entry: &str,
        native: &[String],
        id: &Ident,
        text: &str,
        when: Option<&When>,
        place: Option<&Place>,
        body: &Block,
        span: Span,
    ) -> Option<Patch> {
        let place = self.option_place(place)?;
        let option = self.build_option(menu, Some(native), id, text, when, body, span)?;
        Some(Patch {
            module: "module_game_menus".into(),
            var: "game_menus".into(),
            entry: entry.to_owned(),
            path: vec![MENU_OPTIONS_FIELD],
            action: Action::InsertOption { option, place },
            ops: Vec::new(),
            origin: self.origin(span),
        })
    }

    fn change_option_patch(
        &mut self,
        menu: &str,
        entry: &str,
        native: &[String],
        id: &Ident,
        text: &Option<(String, Span)>,
        when: Option<&When>,
        body: Option<&Block>,
        span: Span,
    ) -> Option<Patch> {
        let id_span = id.span;
        let id = self.option_id(id)?;
        if !native.iter().any(|existing| existing == &id) {
            let help = suggest(&id, native.iter().map(String::as_str)).map(|s| format!("did you mean '{s}'?"));
            self.error(diag(
                E0207_OPTION_UNKNOWN,
                format!("menu '{menu}' has no option '{id}'"),
                id_span,
                help.or_else(|| Some("`change option` only changes options the Native menu already has".into())),
            ));
            return None;
        }
        let text = match text {
            Some((text, span)) => {
                self.check_text(text, *span);
                Some(text.clone())
            }
            None => None,
        };
        let condition = match when {
            Some(When::Block(block)) => Some(self.ops(&block.stmts)),
            Some(When::Cond(cond)) => Some(self.cond_ops(cond, span)),
            None => None,
        };
        let consequence = body.map(|body| self.ops(&body.stmts));
        Some(Patch {
            module: "module_game_menus".into(),
            var: "game_menus".into(),
            entry: entry.to_owned(),
            path: vec![MENU_OPTIONS_FIELD],
            action: Action::ChangeOption { id, text, condition, consequence },
            ops: Vec::new(),
            origin: self.origin(span),
        })
    }

    fn remove_option_patch(
        &mut self,
        menu: &str,
        entry: &str,
        native: &[String],
        id: &Ident,
        span: Span,
    ) -> Option<Patch> {
        let id_span = id.span;
        let id = self.option_id(id)?;
        if !native.iter().any(|existing| existing == &id) {
            let help = suggest(&id, native.iter().map(String::as_str)).map(|s| format!("did you mean '{s}'?"));
            self.error(diag(
                E0207_OPTION_UNKNOWN,
                format!("menu '{menu}' has no option '{id}'"),
                id_span,
                help.or_else(|| Some("`remove option` only removes options the Native menu already has".into())),
            ));
            return None;
        }
        Some(Patch {
            module: "module_game_menus".into(),
            var: "game_menus".into(),
            entry: entry.to_owned(),
            path: vec![MENU_OPTIONS_FIELD],
            action: Action::RemoveOption(id),
            ops: Vec::new(),
            origin: self.origin(span),
        })
    }

    /// Checks one operation call: the operation exists, has enough values, every value resolves.
    /// In Native mode (`check_block`) a flagged name (`neg|this_or_next|eq`) is one operation when
    /// every `|`-separated part is, the name is kept as written, and the header signature count is
    /// not enforced.
    fn call(&mut self, stmt: &Call) -> Option<Op> {
        let text = stmt.op.text.as_str();
        let missing = if self.native { missing_op_part(self.atlas, text) } else { None };
        let op = match self.atlas.op(text) {
            Some(op) => Some(op),
            None if self.native && missing.is_none() => None,
            None => {
                let (message, wanted) = match missing {
                    Some(part) => (format!("'{part}' in '{text}' is not a Native operation"), part),
                    None => (format!("'{text}' is not a Native operation"), text),
                };
                let help = suggest(
                    wanted,
                    self.atlas.ops.iter().map(|o| o.name.as_str()).chain(self.fns.keys().map(String::as_str)),
                )
                // A `fn` used where an operation is needed (a condition) is no typo of itself.
                .filter(|suggestion| suggestion != wanted)
                .map(|s| format!("did you mean '{s}'? (`uw browse` lists every operation)"));
                self.error(diag(E0001_UNKNOWN_NAME, message, stmt.op.span, help));
                return None;
            }
        };
        let name = match op {
            Some(op) if !self.native => op.name.clone(),
            _ => text.to_string(),
        };
        if let Some(op) = op.filter(|_| !self.native) {
            if let Some(min) = min_args(op) {
                if stmt.args.len() < min {
                    let mut d = diag(
                        E0203_TOO_FEW,
                        format!("'{name}' needs at least {min} values, got {}", stmt.args.len()),
                        stmt.span,
                        Some(format!("Native: {}", op.signature.trim())),
                    );
                    if let Some(line) = op.line {
                        d.notes.push(format!("header_operations.py:{line}"));
                    }
                    self.error(d);
                }
            }
        }
        let mut args = Vec::new();
        for arg in &stmt.args {
            if let Some(value) = self.value(arg) {
                args.push(value);
            }
        }
        Some(Op { name, args, origin: self.origin(stmt.span) })
    }

    /// Checks a `for` loop: the operation is `try_for_<kind>` (it must be in the atlas), the loop
    /// variable is a `$global` or a `:local`, and the values are checked like the values of a call.
    fn for_stmt(
        &mut self,
        var: &Arg,
        kind: &Ident,
        args: &[Arg],
        body: &Block,
        span: Span,
    ) -> Option<uw_lower::Stmt> {
        let name = format!("try_for_{}", kind.text);
        let body = self.stmts(&body.stmts);
        let var = match var {
            Arg::Global(g) | Arg::Local(g) => self.local(g),
            Arg::Name(name) => {
                self.error(diag(
                    E0212_ASSIGN_TARGET,
                    format!("'{}' cannot be a loop variable", name.text),
                    name.span,
                    Some("the loop variable must be a $global or a :local".into()),
                ));
                None
            }
            Arg::Int(_, span) | Arg::Str(_, span) => {
                self.error(diag(
                    E0212_ASSIGN_TARGET,
                    "the loop variable must be a $global or a :local".into(),
                    *span,
                    None,
                ));
                None
            }
        };
        let mut values = Vec::new();
        for arg in args {
            if let Some(value) = self.value(arg) {
                values.push(value);
            }
        }
        let op = match self.atlas.op(&name) {
            Some(op) => op,
            None => {
                let kinds: Vec<&str> = self
                    .atlas
                    .ops
                    .iter()
                    .filter(|o| o.name.starts_with("try_for_"))
                    .map(|o| o.name.as_str())
                    .collect();
                self.error(diag(
                    E0001_UNKNOWN_NAME,
                    format!("'{name}' is not a Native operation"),
                    kind.span,
                    Some(format!("the loop kinds are: {}", kinds.join(", "))),
                ));
                return None;
            }
        };
        // The operation takes the loop variable first, so the values count is one more than `args`.
        let given = 1 + args.len();
        if let Some(min) = min_args(op) {
            if given < min {
                let mut d = diag(
                    E0203_TOO_FEW,
                    format!("'{name}' needs at least {min} values, got {given}"),
                    span,
                    Some(format!("Native: {}", op.signature.trim())),
                );
                if let Some(line) = op.line {
                    d.notes.push(format!("header_operations.py:{line}"));
                }
                self.error(d);
            }
        }
        Some(uw_lower::Stmt::For { op: name, var: var?, args: values, body, origin: self.origin(span) })
    }
}

/// The first `|`-separated part of a Native operation name that is not an operation; `None` when
/// every part is one (`uw_decomp::to_raw` splits flagged names the same way).
fn missing_op_part<'t>(atlas: &Atlas, text: &'t str) -> Option<&'t str> {
    text.split('|').find(|part| atlas.op(part).is_none())
}

/// A register the kit defines as a constant: `reg0`, `reg1`, ... (not `tf_male` or other constants).
fn is_register(name: &str) -> bool {
    name.strip_prefix("reg").is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

fn lower_assign_op(op: uw_syntax::AssignOp) -> uw_lower::AssignOp {
    match op {
        uw_syntax::AssignOp::Set => uw_lower::AssignOp::Set,
        uw_syntax::AssignOp::Add => uw_lower::AssignOp::Add,
        uw_syntax::AssignOp::Sub => uw_lower::AssignOp::Sub,
        uw_syntax::AssignOp::Mul => uw_lower::AssignOp::Mul,
        uw_syntax::AssignOp::Div => uw_lower::AssignOp::Div,
        uw_syntax::AssignOp::Mod => uw_lower::AssignOp::Mod,
    }
}

/// `=` `+=` … as the source spells the operator of an assignment target (T-082).
fn assign_op_sign(op: uw_syntax::AssignOp) -> &'static str {
    match op {
        uw_syntax::AssignOp::Set => "=",
        uw_syntax::AssignOp::Add => "+=",
        uw_syntax::AssignOp::Sub => "-=",
        uw_syntax::AssignOp::Mul => "*=",
        uw_syntax::AssignOp::Div => "/=",
        uw_syntax::AssignOp::Mod => "%=",
    }
}

/// `true`/`false` as the number they stand for, wherever a value is written (T-085).
fn bool_value(name: &str) -> Option<i64> {
    match name {
        "true" => Some(1),
        "false" => Some(0),
        _ => None,
    }
}

/// `player.gold`: a name the parser read across dots, an object word (T-082).
fn is_dotted(name: &Ident) -> bool {
    name.text.contains('.')
}

fn lower_bin_op(op: uw_syntax::BinOp) -> uw_lower::BinOp {
    match op {
        uw_syntax::BinOp::Add => uw_lower::BinOp::Add,
        uw_syntax::BinOp::Sub => uw_lower::BinOp::Sub,
        uw_syntax::BinOp::Mul => uw_lower::BinOp::Mul,
        uw_syntax::BinOp::Div => uw_lower::BinOp::Div,
        uw_syntax::BinOp::Mod => uw_lower::BinOp::Mod,
    }
}

fn lower_cmp_op(op: uw_syntax::CmpOp) -> uw_lower::CmpOp {
    match op {
        uw_syntax::CmpOp::Eq => uw_lower::CmpOp::Eq,
        uw_syntax::CmpOp::Ne => uw_lower::CmpOp::Ne,
        uw_syntax::CmpOp::Gt => uw_lower::CmpOp::Gt,
        uw_syntax::CmpOp::Ge => uw_lower::CmpOp::Ge,
        uw_syntax::CmpOp::Lt => uw_lower::CmpOp::Lt,
        uw_syntax::CmpOp::Le => uw_lower::CmpOp::Le,
    }
}

fn cmp_op_name(op: uw_syntax::CmpOp) -> &'static str {
    match op {
        uw_syntax::CmpOp::Eq => "eq",
        uw_syntax::CmpOp::Ne => "neq",
        uw_syntax::CmpOp::Gt => "gt",
        uw_syntax::CmpOp::Ge => "ge",
        uw_syntax::CmpOp::Lt => "lt",
        uw_syntax::CmpOp::Le => "le",
    }
}

/// A `fn` body copied for one call site: `binds` holds the call's argument per parameter and
/// `site` numbers the copy, so the body's own `:locals` stay apart from every other call.
fn substituted_stmt(stmt: &Stmt, binds: &[(String, Arg)], site: usize) -> Stmt {
    match stmt {
        Stmt::Call(call) => Stmt::Call(substituted_call(call, binds, site)),
        Stmt::Assign { target, op, value, span } => Stmt::Assign {
            target: substituted_arg(target, binds, site),
            op: *op,
            value: substituted_expr(value, binds, site),
            span: *span,
        },
        Stmt::If { arms, otherwise, span } => Stmt::If {
            arms: arms
                .iter()
                .map(|(cond, body)| (substituted_cond(cond, binds, site), substituted_block(body, binds, site)))
                .collect(),
            otherwise: otherwise.as_ref().map(|body| substituted_block(body, binds, site)),
            span: *span,
        },
        Stmt::Require { cond, span } => {
            Stmt::Require { cond: substituted_cond(cond, binds, site), span: *span }
        }
        Stmt::Attempt { arms, span } => Stmt::Attempt {
            arms: arms.iter().map(|body| substituted_block(body, binds, site)).collect(),
            span: *span,
        },
        Stmt::For { var, kind, args, body, span } => Stmt::For {
            var: substituted_arg(var, binds, site),
            kind: kind.clone(),
            args: args.iter().map(|arg| substituted_arg(arg, binds, site)).collect(),
            body: substituted_block(body, binds, site),
            span: *span,
        },
    }
}

fn substituted_block(block: &Block, binds: &[(String, Arg)], site: usize) -> Block {
    Block { stmts: block.stmts.iter().map(|s| substituted_stmt(s, binds, site)).collect(), span: block.span }
}

fn substituted_call(call: &Call, binds: &[(String, Arg)], site: usize) -> Call {
    Call {
        op: call.op.clone(),
        args: call.args.iter().map(|arg| substituted_arg(arg, binds, site)).collect(),
        span: call.span,
    }
}

fn substituted_cond(cond: &Cond, binds: &[(String, Arg)], site: usize) -> Cond {
    match cond {
        Cond::Cmp { op, left, right, span } => Cond::Cmp {
            op: *op,
            left: substituted_expr(left, binds, site),
            right: substituted_expr(right, binds, site),
            span: *span,
        },
        Cond::Call(call) => Cond::Call(substituted_call(call, binds, site)),
        Cond::Not(inner, span) => Cond::Not(Box::new(substituted_cond(inner, binds, site)), *span),
        Cond::And(list) => Cond::And(list.iter().map(|c| substituted_cond(c, binds, site)).collect()),
        Cond::Or(list) => Cond::Or(list.iter().map(|c| substituted_cond(c, binds, site)).collect()),
    }
}

fn substituted_expr(expr: &Expr, binds: &[(String, Arg)], site: usize) -> Expr {
    match expr {
        Expr::Value(arg) => Expr::Value(substituted_arg(arg, binds, site)),
        Expr::Neg(inner, span) => Expr::Neg(Box::new(substituted_expr(inner, binds, site)), *span),
        Expr::Bin { op, left, right, span } => Expr::Bin {
            op: *op,
            left: Box::new(substituted_expr(left, binds, site)),
            right: Box::new(substituted_expr(right, binds, site)),
            span: *span,
        },
    }
}

/// A parameter becomes the call's argument (a number, `$global`, `:local`, name or text); any
/// other `:local` of the body becomes `:name__fn<site>`.
fn substituted_arg(arg: &Arg, binds: &[(String, Arg)], site: usize) -> Arg {
    match arg {
        Arg::Local(ident) => match binds.iter().find(|(param, _)| param == &ident.text) {
            Some((_, value)) => value.clone(),
            None => Arg::Local(Ident { text: format!("{}__fn{site}", ident.text), span: ident.span }),
        },
        other => other.clone(),
    }
}


/// `not a < b` flips the operator (`lt = neg|ge`, `neq = neg|eq`, `le = neg|gt`).
fn not_cmp_op_name(op: uw_syntax::CmpOp) -> &'static str {
    match op {
        uw_syntax::CmpOp::Eq => "neq",
        uw_syntax::CmpOp::Ne => "eq",
        uw_syntax::CmpOp::Gt => "le",
        uw_syntax::CmpOp::Ge => "lt",
        uw_syntax::CmpOp::Lt => "ge",
        uw_syntax::CmpOp::Le => "gt",
    }
}

fn bin_op_name(op: uw_syntax::BinOp) -> &'static str {
    match op {
        uw_syntax::BinOp::Add => "store_add",
        uw_syntax::BinOp::Sub => "store_sub",
        uw_syntax::BinOp::Mul => "store_mul",
        uw_syntax::BinOp::Div => "store_div",
        uw_syntax::BinOp::Mod => "store_mod",
    }
}

fn strip_menu_prefix(text: &str) -> Option<&str> {
    text.strip_prefix("menu_").or_else(|| text.strip_prefix("mnu_"))
}

/// The table a full id belongs to: menu ids (`menu_`, `mnu_`) first, then every table's `id_prefix`.
fn id_kind(atlas: &Atlas, text: &str) -> Option<String> {
    if strip_menu_prefix(text).is_some_and(|bare| !bare.is_empty()) {
        return Some("menus".into());
    }
    atlas.tables.iter().find_map(|(name, table)| {
        let prefix = table.id_prefix.as_deref()?;
        (text.len() > prefix.len() && text.starts_with(prefix)).then(|| name.clone())
    })
}

/// The Native menu entry behind `find_menu`'s label (`name` or `name#dup`).
fn menu_entry<'a>(atlas: &'a Atlas, entry: &str) -> Option<&'a uw_atlas::Entry> {
    let (name, dup) = match entry.split_once('#') {
        Some((name, dup)) => (name, Some(dup.parse::<u32>().ok()?)),
        None => (entry, None),
    };
    atlas.tables.get("menus")?.entries.iter().find(|e| e.name.as_deref() == Some(name) && e.dup == dup)
}

/// Option ids in a menu entry's field 5: `{"list": [[id, condition, text, consequence], ...]}`.
fn native_option_ids(entry: &uw_atlas::Entry) -> Vec<String> {
    let Some(list) = entry.value.get(MENU_OPTIONS_FIELD).and_then(|field| field.get("list")).and_then(|list| list.as_array()) else {
        return Vec::new();
    };
    list.iter()
        .filter_map(|option| option.as_array()?.first()?.as_str())
        .map(str::to_owned)
        .collect()
}

/// `start_phase_2`, `menu_start_phase_2`, `mnu_start_phase_2`, `camp#2` -> (entry ref for the
/// runtime, label) of a menu.
fn find_menu(atlas: &Atlas, text: &str) -> Option<String> {
    let (base, occ) = match text.split_once('#') {
        Some((b, n)) => (b, Some(n)),
        None => (text, None),
    };
    let bare = base
        .strip_prefix("menu_")
        .or_else(|| base.strip_prefix("mnu_"))
        .unwrap_or(base)
        .to_lowercase();
    let table = atlas.tables.get("menus")?;
    let hits: Vec<&uw_atlas::Entry> = table
        .entries
        .iter()
        .filter(|e| e.name.as_deref().is_some_and(|n| n.to_lowercase() == bare))
        .collect();
    let entry = match occ {
        Some(n) => *hits.get(n.parse::<usize>().ok()?.checked_sub(1)?)?,
        None if hits.len() == 1 => hits[0],
        None => return None,
    };
    let name = entry.name.clone()?;
    Some(match entry.dup {
        Some(n) => format!("{name}#{n}"),
        None => name,
    })
}

/// Checks every source; on success the patches in source order.
pub fn check(atlas: &Atlas, sources: &[Source], fallback_name: &str) -> Result<Checked, Vec<Error>> {
    let mut errors = Vec::new();
    let mut name: Option<String> = None;
    let mut patches = Vec::new();
    let mut overridden: BTreeMap<String, String> = BTreeMap::new();

    // Every `fn` of the mod is visible from every file, so the table comes first; a call is
    // replaced by the body when the block that holds it is checked (`Ctx::inline`).
    let (fns, fn_errors) = collect_fns(atlas, sources);
    errors.extend(fn_errors);

    // A `save` names one `$global` of the whole mod too, so its names come first as well: a bare
    // name means `$<name>` in every file and its start value is written once, at game start
    // (T-085). A name the kit already uses would make the bare name mean two things.
    let mut saves: Vec<SaveDef> = Vec::new();
    for (index, source) in sources.iter().enumerate() {
        for item in &source.tree.items {
            let Item::Save { name, value, .. } = item else { continue };
            let clash = if atlas.op(&name.text).is_some() {
                Some("a Native operation")
            } else if atlas.constant(&name.text).is_some() {
                Some("a kit constant")
            } else if atlas.resolve_ref(&name.text).is_some() {
                Some("a kit name")
            } else {
                None
            };
            if let Some(what) = clash {
                errors.push(Error {
                    file: index,
                    diag: diag(
                        E0217_SAVE_NAME,
                        format!("`save {}` has the name of {what}", name.text),
                        name.span,
                        Some(format!("a save is also written as `${}`, so pick a name the kit does not use", name.text)),
                    ),
                });
                continue;
            }
            if let Some(first) = saves.iter().find(|save| save.name == name.text) {
                let existing = first.origin.clone();
                errors.push(Error {
                    file: index,
                    diag: diag(
                        E0202_DUPLICATE,
                        format!("`save {}` is already defined at {existing}", name.text),
                        name.span,
                        Some("a save name is used once in a mod; rename one of them".into()),
                    ),
                });
                continue;
            }
            // The parser only lets an integer (or `true`/`false`) through here.
            let Expr::Value(Arg::Int(start, _)) = value else { continue };
            let origin = origin_in(&source.file, name.span);
            saves.push(SaveDef { name: name.text.clone(), start: *start, origin });
        }
    }
    // The start values go to the `game_start` script, the way `on game.start` is emitted; they are
    // the first patches, so a `game_start` handler of the mod already reads them.
    if let Some(first) = saves.first() {
        let ops = saves
            .iter()
            .map(|save| Op {
                name: "assign".into(),
                args: vec![Value::Ref(format!("${}", save.name)), Value::Int(save.start)],
                origin: save.origin.clone(),
            })
            .collect();
        patches.push(Patch {
            module: "module_scripts".into(),
            var: "scripts".into(),
            entry: "game_start".into(),
            path: vec![SCRIPT_BODY_FIELD],
            action: Action::Append,
            ops,
            origin: first.origin.clone(),
        });
    }

    for (index, source) in sources.iter().enumerate() {
        let mut ctx = Ctx {
            atlas,
            errors: Vec::new(),
            file: index,
            source: &source.file,
            temp: 0,
            native: false,
            fns: &fns,
            sources,
            stack: Vec::new(),
            rename: 0,
            temps: Vec::new(),
            saves: &saves,
        };
        if let Some(m) = &source.tree.module {
            match &name {
                Some(existing) if existing != &m.text => ctx.error(diag(
                    E0205_MOD_NAME,
                    format!("this mod is already named '{existing}' in another file"),
                    m.span,
                    Some("name the mod in one file only".into()),
                )),
                _ => name = Some(m.text.clone()),
            }
        }
        for item in &source.tree.items {
            match item {
                Item::Override { kind, name: target, handlers, members, span } => {
                    let kind = match kind {
                        Some(kind) => kind.text.clone(),
                        None => match id_kind(atlas, &target.text) {
                            Some(table) => table,
                            None => {
                                let menus = atlas.tables.get("menus");
                                let names: Vec<String> = menus
                                    .map(|t| t.entries.iter().filter_map(|e| e.name.clone()).collect())
                                    .unwrap_or_default();
                                let help = suggest(&target.text, names.iter().map(String::as_str))
                                    .map(|s| format!("did you mean 'menu_{s}'?"))
                                    .or_else(|| Some("write the full Native id, e.g. `override menu_start_phase_2`".into()));
                                ctx.error(diag(
                                    E0001_UNKNOWN_NAME,
                                    format!("'{}' has no Native id prefix", target.text),
                                    target.span,
                                    help,
                                ));
                                continue;
                            }
                        },
                    };
                    if kind != "menu" && kind != "menus" {
                        ctx.error(diag(
                            E0201_NOT_YET,
                            format!("`override {kind}` is not in UW yet; Faz 1 has `override menu`"),
                            target.span,
                            None,
                        ));
                        continue;
                    }
                    if let Some(bare) = strip_menu_prefix(&target.text) {
                        if let Some(once) = strip_menu_prefix(bare) {
                            ctx.error(diag(
                                E0208_BAD_ID,
                                format!("'{}' has the menu prefix twice", target.text),
                                target.span,
                                Some(format!("write it once: 'menu_{once}'")),
                            ));
                            continue;
                        }
                    }
                    let Some(entry) = find_menu(atlas, &target.text) else {
                        let menus = atlas.tables.get("menus");
                        let names: Vec<String> = menus
                            .map(|t| t.entries.iter().filter_map(|e| e.name.clone()).collect())
                            .unwrap_or_default();
                        let help = suggest(&target.text, names.iter().map(String::as_str))
                            .map(|s| format!("did you mean '{s}'?"))
                            .or_else(|| Some("`uw browse menu_` lists the Native menus".into()));
                        ctx.error(diag(
                            E0001_UNKNOWN_NAME,
                            format!("'{}' is not a Native menu (or its name repeats; add #1, #2)", target.text),
                            target.span,
                            help,
                        ));
                        continue;
                    };
                    let native = menu_entry(atlas, &entry).map(native_option_ids).unwrap_or_default();
                    let origin = ctx.origin(*span);
                    if let Some(first) = overridden.insert(entry.clone(), origin.clone()) {
                        ctx.error(diag(
                            E0202_DUPLICATE,
                            format!("menu '{entry}' is already overridden at {first}"),
                            target.span,
                            Some("put both changes in one `override` block".into()),
                        ));
                        continue;
                    }
                    let mut found: Vec<(usize, Patch)> = Vec::new();
                    let mut enter_origin: Option<String> = None;
                    for handler in handlers {
                        if handler.event.text != "enter" {
                            let help = suggest(&handler.event.text, ["enter"]).map(|s| format!("did you mean '{s}'?"));
                            ctx.error(diag(
                                E0201_NOT_YET,
                                format!("`on {}` is not in UW yet; menus have `on enter`", handler.event.text),
                                handler.event.span,
                                help,
                            ));
                            continue;
                        }
                        let here = ctx.origin(handler.body.span);
                        if let Some(first) = &enter_origin {
                            ctx.error(diag(
                                E0211_MEMBER_TWICE,
                                format!("menu '{entry}' has two `on enter` blocks: {first} and {here}"),
                                handler.event.span,
                                Some("put all enter operations in one `on enter` block".into()),
                            ));
                            continue;
                        }
                        enter_origin = Some(here);
                        let ops = ctx.ops(&handler.body.stmts);
                        found.push((
                            handler.body.span.start,
                            Patch {
                                module: "module_game_menus".into(),
                                var: "game_menus".into(),
                                entry: entry.clone(),
                                path: vec![MENU_ENTER_FIELD],
                                action: Action::ReplaceBody,
                                ops,
                                origin: origin.clone(),
                            },
                        ));
                    }
                    let mut text_origin: Option<String> = None;
                    for member in members {
                        match member {
                            Member::Text { text, span } => {
                                let here = ctx.origin(*span);
                                if let Some(first) = &text_origin {
                                    ctx.error(diag(
                                        E0211_MEMBER_TWICE,
                                        format!("menu '{entry}' has two `text` lines: {first} and {here}"),
                                        *span,
                                        Some("keep one `text` line per menu".into()),
                                    ));
                                    continue;
                                }
                                text_origin = Some(here.clone());
                                ctx.check_text(text, *span);
                                found.push((
                                    span.start,
                                    Patch {
                                        module: "module_game_menus".into(),
                                        var: "game_menus".into(),
                                        entry: entry.clone(),
                                        path: vec![MENU_TEXT_FIELD],
                                        action: Action::ReplaceText(text.clone()),
                                        ops: Vec::new(),
                                        origin: here,
                                    },
                                ));
                            }
                            Member::Option { id, text, when, place, body, span } => {
                                if let Some(patch) = ctx.insert_option_patch(
                                    &entry,
                                    &entry,
                                    &native,
                                    id,
                                    text,
                                    when.as_ref(),
                                    place.as_ref(),
                                    body,
                                    *span,
                                ) {
                                    found.push((span.start, patch));
                                }
                            }
                            Member::ChangeOption { id, text, when, body, span } => {
                                if let Some(patch) = ctx.change_option_patch(
                                    &entry,
                                    &entry,
                                    &native,
                                    id,
                                    text,
                                    when.as_ref(),
                                    body.as_ref(),
                                    *span,
                                ) {
                                    found.push((span.start, patch));
                                }
                            }
                            Member::RemoveOption { id, span } => {
                                if let Some(patch) = ctx.remove_option_patch(&entry, &entry, &native, id, *span) {
                                    found.push((span.start, patch));
                                }
                            }
                        }
                    }
                    found.sort_by_key(|(start, _)| *start);
                    patches.extend(found.into_iter().map(|(_, patch)| patch));
                }
                Item::New { id, text, handlers, members, span } => {
                    let id_span = id.span;
                    let Some(bare) = id.text.strip_prefix("menu_") else {
                        let help = id
                            .text
                            .strip_prefix("mnu_")
                            .map(|rest| format!("write 'menu_{rest}'"))
                            .or_else(|| Some("a new menu id starts with `menu_`, e.g. `new menu_uw_ask`".into()));
                        ctx.error(diag(
                            E0208_BAD_ID,
                            format!("'{}' is not a full menu id", id.text),
                            id_span,
                            help,
                        ));
                        continue;
                    };
                    if let Some(once) = strip_menu_prefix(bare) {
                        ctx.error(diag(
                            E0208_BAD_ID,
                            format!("'{}' has the menu prefix twice", id.text),
                            id_span,
                            Some(format!("write it once: 'menu_{once}'")),
                        ));
                        continue;
                    }
                    if !id.text.is_ascii() {
                        ctx.error(diag(
                            E0208_BAD_ID,
                            format!("menu id '{}' has non-ASCII letters", id.text),
                            id_span,
                            Some("menu ids use ASCII letters, digits and _".into()),
                        ));
                        continue;
                    }
                    if find_menu(atlas, &id.text).is_some() || atlas.entry_by_id(&id.text).is_some() {
                        ctx.error(diag(
                            E0209_NEW_EXISTS,
                            format!("menu '{}' already exists", id.text),
                            id_span,
                            Some(format!("change it with `override {}`", id.text)),
                        ));
                        continue;
                    }
                    let mut menu_text = String::new();
                    let mut text_origin: Option<String> = None;
                    if let Some((text, text_span)) = text {
                        ctx.check_text(text, *text_span);
                        menu_text = text.clone();
                        text_origin = Some(ctx.origin(*text_span));
                    }
                    let mut enter = Vec::new();
                    let mut enter_origin: Option<String> = None;
                    for handler in handlers {
                        if handler.event.text != "enter" {
                            let help = suggest(&handler.event.text, ["enter"]).map(|s| format!("did you mean '{s}'?"));
                            ctx.error(diag(
                                E0201_NOT_YET,
                                format!("`on {}` is not in UW yet; new menus have `on enter`", handler.event.text),
                                handler.event.span,
                                help,
                            ));
                            continue;
                        }
                        let here = ctx.origin(handler.body.span);
                        if let Some(first) = &enter_origin {
                            ctx.error(diag(
                                E0211_MEMBER_TWICE,
                                format!("menu '{bare}' has two `on enter` blocks: {first} and {here}"),
                                handler.event.span,
                                Some("put all enter operations in one `on enter` block".into()),
                            ));
                            continue;
                        }
                        enter_origin = Some(here);
                        enter = ctx.ops(&handler.body.stmts);
                    }
                    let mut placed = Vec::new();
                    let menu_name = bare.to_lowercase();
                    for member in members {
                        match member {
                            Member::Text { text, span } => {
                                let here = ctx.origin(*span);
                                if let Some(first) = &text_origin {
                                    ctx.error(diag(
                                        E0211_MEMBER_TWICE,
                                        format!("menu '{menu_name}' has two `text` lines: {first} and {here}"),
                                        *span,
                                        Some("keep one `text` per menu".into()),
                                    ));
                                    continue;
                                }
                                ctx.check_text(text, *span);
                                menu_text = text.clone();
                                text_origin = Some(here);
                            }
                            Member::Option { id, text, when, place, body, span } => {
                                let Some(place) = ctx.option_place(place.as_ref()) else { continue };
                                if let Some(option) =
                                    ctx.build_option(&menu_name, None, id, text, when.as_ref(), body, *span)
                                {
                                    placed.push((place, option));
                                }
                            }
                            Member::ChangeOption { id, span, .. } => {
                                ctx.error(diag(
                                    E0207_OPTION_UNKNOWN,
                                    format!("a new menu has no existing option '{}'", id.text),
                                    *span,
                                    Some("add an option with `option <id> \"text\" { ... }`".into()),
                                ));
                            }
                            Member::RemoveOption { id, span } => {
                                ctx.error(diag(
                                    E0207_OPTION_UNKNOWN,
                                    format!("a new menu has no existing option '{}' to remove", id.text),
                                    *span,
                                    Some("add an option with `option <id> \"text\" { ... }`".into()),
                                ));
                            }
                        }
                    }
                    let mut options: Vec<NewOption> = Vec::new();
                    for (place, option) in placed {
                        let at = match place {
                            OptionPlace::End => None,
                            OptionPlace::After(id) => options.iter().position(|o| o.id == id).map(|i| i + 1),
                            OptionPlace::Before(id) => options.iter().position(|o| o.id == id),
                        };
                        match at {
                            Some(index) => options.insert(index, option),
                            None => options.push(option),
                        }
                    }
                    patches.push(Patch {
                        module: "module_game_menus".into(),
                        var: "game_menus".into(),
                        entry: menu_name,
                        path: Vec::new(),
                        action: Action::NewMenu { text: menu_text, enter, options },
                        ops: Vec::new(),
                        origin: ctx.origin(*span),
                    });
                }
                // A `fn` adds no patch of its own: its body lands wherever it is called.
                Item::Fn { .. } => {}
                // A `save` adds no patch here: its start value is written at game start above.
                Item::Save { .. } => {}
                Item::On { path, body, span } => {
                    let words: Vec<&str> = path.iter().map(|p| p.text.as_str()).collect();
                    let hook = match words.as_slice() {
                        ["game", rest] => format!("game_{rest}"),
                        _ => String::new(),
                    };
                    if !atlas.is_hook(&hook) {
                        let hooks: Vec<String> = atlas.hooks.iter().map(|h| h.replacen("game_", "game.", 1)).collect();
                        let asked = words.join(".");
                        let help = suggest(&asked, hooks.iter().map(String::as_str))
                            .map(|s| format!("did you mean 'on {s}'?"))
                            .or_else(|| Some("engine hooks are `on game.<name>`, e.g. `on game.start`".into()));
                        let whole = Span { start: path[0].span.start, end: path[path.len() - 1].span.end };
                        ctx.error(diag(
                            E0001_UNKNOWN_NAME,
                            format!("'{asked}' is not an engine hook"),
                            whole,
                            help,
                        ));
                        continue;
                    }
                    let ops = ctx.ops(&body.stmts);
                    patches.push(Patch {
                        module: "module_scripts".into(),
                        var: "scripts".into(),
                        entry: hook,
                        path: vec![SCRIPT_BODY_FIELD],
                        action: Action::Append,
                        ops,
                        origin: ctx.origin(*span),
                    });
                }
            }
        }
        errors.extend(ctx.errors);
    }
    if errors.is_empty() {
        Ok(Checked { name: name.unwrap_or_else(|| fallback_name.to_owned()), patches })
    } else {
        Err(errors)
    }
}

/// Checks and lowers one statement block exactly as Native wrote it (raw mode): a decompiled block
/// can be checked with the same rules and compared with the original operations.
///
/// `atlas` resolves every operation and name, `file` only names the block's source (its `name` and
/// lines go into each operation's `origin` and into the errors) and `stmts` is one parsed block (a
/// body, an option condition or consequence), so no module header is needed. Unlike [`check`]: a
/// flagged operation name (`neg|this_or_next|eq`) is accepted when every part is a Native operation
/// and is kept as written, the header signature count is not enforced (Native is the reference) and
/// non-ASCII text is allowed. Names still resolve in the atlas.
pub fn check_block(atlas: &Atlas, file: &SourceFile, stmts: &[Stmt]) -> Result<Vec<Op>, Vec<Diagnostic>> {
    check_block_with(atlas, file, stmts, &[])
}

/// [`check_block`] with the `fn`s of `sources` visible: a block of a project may call a word from
/// `lib/*.uw` or its own `fns/*.uw`, and an object word is opened by the fn it names (T-087). The
/// fns are collected by the same rules as [`check`]. Without sources this is exactly
/// [`check_block`].
pub fn check_block_with(
    atlas: &Atlas,
    file: &SourceFile,
    stmts: &[Stmt],
    sources: &[Source],
) -> Result<Vec<Op>, Vec<Diagnostic>> {
    let (fns, mut errors) = collect_fns(atlas, sources);
    let mut ctx = Ctx {
        atlas,
        errors: Vec::new(),
        file: 0,
        source: file,
        temp: 0,
        native: true,
        fns: &fns,
        sources,
        stack: Vec::new(),
        rename: 0,
        temps: Vec::new(),
        saves: &[],
    };
    let ops = ctx.ops(stmts);
    errors.extend(ctx.errors);
    if errors.is_empty() {
        Ok(ops)
    } else {
        Err(errors.into_iter().map(|e| e.diag).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atlas() -> Atlas {
        let text = r#"{
          "schema": 1, "kit_hash": "t",
          "ops": [
            {"name":"party_relocate_near_party","code":1623,"line":667,"signature":"(party_relocate_near_party,<party_id>,<target_party_id>,<value_spawn_radius>),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"change_screen_map","code":2052,"line":1093,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"display_message","code":1106,"line":482,"signature":"(display_message,<string_id>,[hex_colour_code]),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"party_is_active","code":2243,"line":7000,"signature":"(party_is_active,<party_id>),","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"assign","code":2133,"line":40,"signature":"had to put this here","lhs":true,"can_fail":false,"global_lhs":false},
            {"name":"try_end","code":3,"line":16,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"try_begin","code":4,"line":17,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"try_for_range","code":6,"line":19,"signature":"(try_for_range,<destination>,<lower_bound>,<upper_bound>),","lhs":true,"can_fail":false,"global_lhs":false},
            {"name":"eq","code":31,"line":41,"signature":"","lhs":false,"can_fail":true,"global_lhs":false},
            {"name":"neg","code":2147483648,"line":440,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"this_or_next","code":1073741824,"line":441,"signature":"","lhs":false,"can_fail":false,"global_lhs":false}
          ],
          "hooks": ["game_start"],
          "ranges": [],
          "constants": [
            {"name":"reg0","value":1,"module":"header_common","line":563},
            {"name":"tf_male","value":0,"module":"header_common","line":100}
          ],
          "tables": {
            "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu",
              "entries": [
                {"index":0,"name":"start_game_0","id":"menu_start_game_0","dup":null,"line":39,"value":[]},
                {"index":1,"name":"start_phase_2","id":"menu_start_phase_2","dup":null,"line":55,"value":[null,null,"Start",null,[],{"list":[["camp_wait_here",[],"Wait here",[]],["leave",[],"Leave",[]]]}]},
                {"index":2,"name":"camp","id":null,"dup":1,"line":60,"value":[]},
                {"index":3,"name":"camp","id":"menu_camp","dup":2,"line":70,"value":[]}
              ]},
            "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","ref_tag":"p",
              "entries": [
                {"index":0,"name":"main_party","id":"p_main_party","dup":null,"line":1,"value":[]},
                {"index":26,"name":"town_6","id":"p_town_6","dup":null,"line":82,"value":[]}
              ]},
            "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
              "entries": [{"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,"value":[]}]}
          }
        }"#;
        Atlas::from_str(text).unwrap()
    }

    fn run(text: &str) -> Result<Checked, Vec<String>> {
        let file = SourceFile::new("start.uw", text);
        let tree = uw_syntax::parse(text).map_err(|d| vec![d.render(&file)])?;
        check(&atlas(), &[Source { file: file.clone(), tree }], "folder").map_err(|errs| {
            errs.iter().map(|e| e.diag.render(&file)).collect()
        })
    }

    #[test]
    fn praven_becomes_two_patches() {
        let checked = run("mod praven_start\noverride menu start_phase_2 {\n on enter {\n  party_relocate_near_party(p_main_party, p_town_6, 3)\n  change_screen_map()\n }\n}\non game.start { assign($g_x, reg0); display_message(\"@hi\") }\n").unwrap();
        assert_eq!(checked.name, "praven_start");
        assert_eq!(checked.patches.len(), 2);
        let menu = &checked.patches[0];
        assert_eq!((menu.module.as_str(), menu.entry.as_str(), menu.path.as_slice()), ("module_game_menus", "start_phase_2", &[4][..]));
        assert_eq!(menu.action, Action::ReplaceBody);
        assert_eq!(menu.ops[0].args, [Value::Ref("p_main_party".into()), Value::Ref("p_town_6".into()), Value::Int(3)]);
        assert_eq!(menu.ops[0].origin, "start.uw:4");
        let script = &checked.patches[1];
        assert_eq!((script.entry.as_str(), &script.action), ("game_start", &Action::Append));
        assert_eq!(script.ops[0].args[1], Value::Const { name: "reg0".into(), module: "header_common".into() });
    }

    #[test]
    fn menu_names_in_every_spelling_and_repeats() {
        for name in ["start_phase_2", "menu_start_phase_2", "mnu_start_phase_2"] {
            let c = run(&format!("override menu {name} {{ on enter {{ change_screen_map() }} }}")).unwrap();
            assert_eq!(c.patches[0].entry, "start_phase_2");
            assert_eq!(c.name, "folder");
        }
        let c = run("override menu camp#2 { on enter { change_screen_map() } }").unwrap();
        assert_eq!(c.patches[0].entry, "camp#2");
        let errs = run("override menu camp { on enter { change_screen_map() } }").unwrap_err();
        assert!(errs[0].contains("add #1, #2"), "{errs:?}");
    }

    #[test]
    fn every_unknown_name_is_reported_with_a_suggestion() {
        let errs = run("override menu start_phase_3 { on enter { change_screan_map()\n party_relocate_near_party(p_main_party, p_town_7, 3) } }\non game.strat { }\n").unwrap_err();
        let all = errs.join("\n");
        assert!(all.contains("'start_phase_3' is not a Native menu"), "{all}");
        assert!(all.contains("did you mean 'start_phase_2'?"), "{all}");
        assert!(all.contains("'game.strat' is not an engine hook"), "{all}");
        assert!(all.contains("did you mean 'on game.start'?"), "{all}");
        let errs = run("override menu start_phase_2 { on enter { change_screan_map()\n party_relocate_near_party(p_main_party, p_town_7, 3) } }").unwrap_err();
        let all = errs.join("\n");
        assert!(all.contains("did you mean 'change_screen_map'?"), "{all}");
        assert!(all.contains("'p_town_7' is not a Native name"), "{all}");
        assert!(all.contains("did you mean 'p_town_6'?"), "{all}");
    }

    #[test]
    fn an_unknown_name_suggests_the_installed_game_names_too() {
        // With `names.uw` installed, `praven` means `p_town_6`; a typo of it should hear `praven`.
        let mut atlas = atlas();
        atlas.set_game_names(&[("praven".to_owned(), "p_town_6".to_owned())]);
        let file = SourceFile::new("start.uw", "on game.start { party_relocate_near_party(pravn, p_main_party, 3) }");
        let tree = uw_syntax::parse(&file.text).expect("parses");
        let errs = check(&atlas, &[Source { file: file.clone(), tree }], "folder").unwrap_err();
        let all: Vec<String> = errs.iter().map(|e| e.diag.render(&file)).collect();
        assert!(all.join("\n").contains("did you mean 'praven'?"), "{all:?}");
    }

    #[test]
    fn too_few_values_quote_the_kit() {
        let errs = run("on game.start { party_relocate_near_party(p_main_party, p_town_6) }").unwrap_err();
        assert!(errs[0].contains("needs at least 3 values, got 2"), "{errs:?}");
        assert!(errs[0].contains("header_operations.py:667"), "{errs:?}");
        // Optional [..] values and unparsable comments are not counted.
        assert!(run("on game.start { display_message(\"@x\") }").is_ok());
        assert!(run("on game.start { assign($a, 1) }").is_ok());
    }

    #[test]
    fn duplicates_events_and_text_are_checked() {
        let errs = run("override menu start_phase_2 { on enter { } }\noverride menu mnu_start_phase_2 { on enter { } }").unwrap_err();
        assert!(errs[0].contains("already overridden at start.uw:1"), "{errs:?}");
        let errs = run("override menu start_phase_2 { on entre { } }").unwrap_err();
        assert!(errs[0].contains("did you mean 'enter'?"), "{errs:?}");
        let errs = run("override troop x { on enter { } }").unwrap_err();
        assert!(errs[0].contains("`override troop` is not in UW yet"), "{errs:?}");
        let errs = run("on game.start { display_message(\"@Ağa\") }").unwrap_err();
        assert!(errs[0].contains("non-ASCII"), "{errs:?}");
    }

    #[test]
    fn min_args_reads_header_comments() {
        let a = atlas();
        assert_eq!(min_args(a.op("party_relocate_near_party").unwrap()), Some(3));
        assert_eq!(min_args(a.op("display_message").unwrap()), Some(1));
        assert_eq!(min_args(a.op("change_screen_map").unwrap()), None);
        assert_eq!(min_args(a.op("assign").unwrap()), None);
    }

    // ---------------------------------------------------------------- hand-built AST

    const SOURCE: &str = "aaaa\nbbbb\ncccc\ndddd\neeee\nffff\ngggg\nhhhh\n";

    fn line_span(line: usize) -> Span {
        Span { start: (line - 1) * 5, end: (line - 1) * 5 + 1 }
    }

    fn ident(text: &str, line: usize) -> Ident {
        Ident { text: text.into(), span: line_span(line) }
    }

    fn block(stmts: Vec<Stmt>) -> Block {
        Block { stmts, span: line_span(1) }
    }

    fn op_call(name: &str, args: Vec<Arg>) -> Stmt {
        Stmt::Call(Call { op: ident(name, 1), args, span: line_span(1) })
    }

    fn int(n: i64) -> Arg {
        Arg::Int(n, line_span(1))
    }

    fn file_with(items: Vec<Item>) -> File {
        File { module: None, items }
    }

    fn run_items(items: Vec<Item>) -> Result<Checked, Vec<String>> {
        let file = SourceFile::new("start.uw", SOURCE);
        check(&atlas(), &[Source { file: file.clone(), tree: file_with(items) }], "folder")
            .map_err(|errs| errs.iter().map(|e| e.diag.render(&file)).collect())
    }

    fn run_stmts(stmts: Vec<Stmt>) -> Result<Vec<uw_lower::Stmt>, Vec<String>> {
        let file = SourceFile::new("start.uw", SOURCE);
        let a = atlas();
        let fns = BTreeMap::new();
        let mut ctx = Ctx {
            atlas: &a,
            errors: Vec::new(),
            file: 0,
            source: &file,
            temp: 0,
            native: false,
            fns: &fns,
            sources: &[],
            stack: Vec::new(),
            rename: 0,
            temps: Vec::new(),
            saves: &[],
        };
        let tree = ctx.stmts(&stmts);
        if ctx.errors.is_empty() {
            Ok(tree)
        } else {
            Err(ctx.errors.iter().map(|e| e.diag.render(&file)).collect())
        }
    }

    fn assign(target: Arg, op: uw_syntax::AssignOp, value: Expr) -> Stmt {
        Stmt::Assign { target, op, value, span: line_span(1) }
    }

    fn cmp(op: uw_syntax::CmpOp, left: Expr, right: Expr) -> Cond {
        Cond::Cmp { op, left, right, span: line_span(1) }
    }

    fn for_stmt(var: Arg, kind: &str, args: Vec<Arg>, body: Vec<Stmt>) -> Stmt {
        Stmt::For { var, kind: ident(kind, 1), args, body: block(body), span: line_span(1) }
    }

    fn run_block(stmts: Vec<Stmt>) -> Result<Vec<Op>, Vec<String>> {
        let file = SourceFile::new("start.uw", SOURCE);
        check_block(&atlas(), &file, &stmts).map_err(|errs| errs.iter().map(|e| e.render(&file)).collect())
    }

    #[test]
    fn check_block_keeps_flags() {
        let ops = run_block(vec![op_call(
            "neg|this_or_next|eq",
            vec![Arg::Global(ident("$x", 1)), int(1)],
        )])
        .unwrap();
        assert_eq!(ops.len(), 1, "{ops:?}");
        assert_eq!(ops[0].name, "neg|this_or_next|eq");
        assert_eq!(ops[0].args, [Value::Ref("$x".into()), Value::Int(1)]);
        // Every flagged part must still be a Native operation.
        let errs = run_block(vec![op_call("neg|change_screan_map", vec![])]).unwrap_err();
        assert!(errs[0].contains("'change_screan_map' in 'neg|change_screan_map' is not a Native operation"), "{errs:?}");
    }

    #[test]
    fn check_block_accepts_control_ops() {
        let ops = run_block(vec![
            op_call("try_begin", vec![]),
            op_call("try_for_range", vec![Arg::Local(ident(":i", 1)), int(0), int(3)]),
            op_call("try_end", vec![]),
        ])
        .unwrap();
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["try_begin", "try_for_range", "try_end"]);
        assert_eq!(ops[1].args, [Value::Ref(":i".into()), Value::Int(0), Value::Int(3)]);
        // Native is the reference: the header signature count is not enforced here.
        assert!(run_block(vec![op_call("display_message", vec![])]).is_ok());
        assert!(run("on game.start { display_message() }").is_err());
    }

    #[test]
    fn check_block_native_text_ok() {
        let ops = run_block(vec![op_call("display_message", vec![Arg::Str("@Ağa".into(), line_span(1))])]).unwrap();
        assert_eq!(ops[0].args, [Value::Text("@Ağa".into())]);
        // The same text in a user file is still E0204.
        let errs = run("on game.start { display_message(\"@Ağa\") }").unwrap_err();
        assert!(errs[0].contains("non-ASCII"), "{errs:?}");
    }

    #[test]
    fn check_block_if_matches_lowering() {
        let ops = run_block(vec![Stmt::If {
            arms: vec![(
                cmp(
                    uw_syntax::CmpOp::Gt,
                    Expr::Value(Arg::Global(ident("$x", 1))),
                    Expr::Value(int(1)),
                ),
                block(vec![op_call("change_screen_map", vec![])]),
            )],
            otherwise: Some(block(vec![op_call("display_message", vec![Arg::Str("@hi".into(), line_span(1))])])),
            span: line_span(1),
        }])
        .unwrap();
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(
            names,
            ["try_begin", "gt", "change_screen_map", "else_try", "display_message", "try_end"]
        );
        assert_eq!(ops[1].args, [Value::Ref("$x".into()), Value::Int(1)]);
        // A plain comparison needs no temporary (§7.6).
        let temp = |v: &Value| matches!(v, Value::Ref(name) if name.starts_with(uw_lower::TEMP_PREFIX));
        assert!(ops.iter().flat_map(|o| &o.args).all(|v| !temp(v)), "{ops:?}");
    }

    #[test]
    fn assignments_become_checked_statements() {
        let tree = run_stmts(vec![
            assign(Arg::Global(ident("$g_x", 1)), uw_syntax::AssignOp::Set, Expr::Value(int(1))),
            assign(
                Arg::Local(ident(":x", 2)),
                uw_syntax::AssignOp::Add,
                Expr::Value(Arg::Name(ident("reg0", 2))),
            ),
            assign(
                Arg::Name(ident("reg0", 3)),
                uw_syntax::AssignOp::Set,
                Expr::Bin {
                    op: uw_syntax::BinOp::Add,
                    left: Box::new(Expr::Value(Arg::Global(ident("$g", 3)))),
                    right: Box::new(Expr::Value(int(2))),
                    span: line_span(3),
                },
            ),
        ])
        .unwrap();
        assert_eq!(tree.len(), 3);
        match &tree[0] {
            uw_lower::Stmt::Assign { target, op, value, origin } => {
                assert_eq!(target, &Value::Ref("$g_x".into()));
                assert_eq!(*op, uw_lower::AssignOp::Set);
                assert_eq!(value, &uw_lower::Expr::Value(Value::Int(1)));
                assert_eq!(origin, "start.uw:1");
            }
            other => panic!("{other:?}"),
        }
        match &tree[1] {
            uw_lower::Stmt::Assign { target, op, value, .. } => {
                assert_eq!(target, &Value::Ref(":x".into()));
                assert_eq!(*op, uw_lower::AssignOp::Add);
                assert_eq!(
                    value,
                    &uw_lower::Expr::Value(Value::Const { name: "reg0".into(), module: "header_common".into() })
                );
            }
            other => panic!("{other:?}"),
        }
        match &tree[2] {
            uw_lower::Stmt::Assign { target, value, .. } => {
                assert_eq!(target, &Value::Const { name: "reg0".into(), module: "header_common".into() });
                match value {
                    uw_lower::Expr::Bin { op, left, right } => {
                        assert_eq!(*op, uw_lower::BinOp::Add);
                        assert_eq!(**left, uw_lower::Expr::Value(Value::Ref("$g".into())));
                        assert_eq!(**right, uw_lower::Expr::Value(Value::Int(2)));
                    }
                    other => panic!("{other:?}"),
                }
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn assignment_targets_are_strict() {
        let bad = |target| {
            run_stmts(vec![assign(target, uw_syntax::AssignOp::Set, Expr::Value(int(1)))]).unwrap_err()
        };
        assert!(bad(Arg::Name(ident("p_town_6", 1)))[0].contains("cannot be assigned to"));
        assert!(bad(Arg::Name(ident("tf_male", 1)))[0].contains("cannot be assigned to"));
        assert!(bad(Arg::Int(1, line_span(1)))[0].contains("must be a $global"));
        assert!(bad(Arg::Local(ident(":_x", 1)))[0].contains("`:_`"));
        let errs = run_stmts(vec![op_call("display_message", vec![Arg::Local(ident(":_bad", 1))])]).unwrap_err();
        assert!(errs[0].contains("`:_`"), "{errs:?}");
    }

    #[test]
    fn for_becomes_a_loop_statement_that_lowers_to_the_kit_operation() {
        let tree = run_stmts(vec![for_stmt(
            Arg::Local(ident(":i", 1)),
            "range",
            vec![int(0), int(10)],
            vec![op_call("change_screen_map", vec![])],
        )])
        .unwrap();
        assert_eq!(
            tree,
            vec![uw_lower::Stmt::For {
                op: "try_for_range".into(),
                var: Value::Ref(":i".into()),
                args: vec![Value::Int(0), Value::Int(10)],
                body: vec![uw_lower::Stmt::Op(Op {
                    name: "change_screen_map".into(),
                    args: Vec::new(),
                    origin: "start.uw:1".into(),
                })],
                origin: "start.uw:1".into(),
            }]
        );
        let ops = uw_lower::lower(&tree);
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["try_for_range", "change_screen_map", "try_end"]);
        assert_eq!(ops[0].args, [Value::Ref(":i".into()), Value::Int(0), Value::Int(10)]);
    }

    #[test]
    fn for_loop_variable_must_be_a_global_or_a_local() {
        for bad in [Arg::Name(ident("i", 1)), Arg::Int(1, line_span(1))] {
            let errs =
                run_stmts(vec![for_stmt(bad, "range", vec![int(0), int(3)], vec![])]).unwrap_err();
            assert!(errs[0].contains("loop variable"), "{errs:?}");
        }
        let tree = run_stmts(vec![for_stmt(
            Arg::Global(ident("$i", 1)),
            "range",
            vec![int(0), int(3)],
            vec![],
        )])
        .unwrap();
        assert!(
            matches!(&tree[0], uw_lower::Stmt::For { var, .. } if var == &Value::Ref("$i".into())),
            "{tree:?}"
        );
    }

    #[test]
    fn unknown_for_kind_lists_the_loop_operations() {
        let errs =
            run_stmts(vec![for_stmt(Arg::Local(ident(":i", 1)), "rang", vec![int(0), int(3)], vec![])])
                .unwrap_err();
        assert!(errs[0].contains("'try_for_rang' is not a Native operation"), "{errs:?}");
        assert!(errs[0].contains("try_for_range"), "{errs:?}");
        // The bare operation name is not a loop kind: the `try_for_` name must exist.
        let errs = run_stmts(vec![for_stmt(Arg::Local(ident(":i", 1)), "begin", vec![], vec![])])
            .unwrap_err();
        assert!(errs[0].contains("'try_for_begin' is not a Native operation"), "{errs:?}");
    }

    #[test]
    fn for_values_are_counted_like_call_values() {
        let errs = run_stmts(vec![for_stmt(Arg::Local(ident(":i", 1)), "range", vec![int(3)], vec![])])
            .unwrap_err();
        assert!(errs[0].contains("'try_for_range' needs at least 3 values, got 2"), "{errs:?}");
        assert!(errs[0].contains("header_operations.py:19"), "{errs:?}");
        // A value that does not resolve is an error, like any other value.
        let errs = run_stmts(vec![for_stmt(
            Arg::Local(ident(":i", 1)),
            "range",
            vec![int(0), Arg::Name(ident("no_such_name", 1))],
            vec![],
        )])
        .unwrap_err();
        assert!(errs[0].contains("'no_such_name'"), "{errs:?}");
    }

    #[test]
    fn if_becomes_a_statement_tree() {
        let tree = run_stmts(vec![Stmt::If {
            arms: vec![
                (
                    cmp(
                        uw_syntax::CmpOp::Ge,
                        Expr::Value(Arg::Global(ident("$gold", 1))),
                        Expr::Value(int(100)),
                    ),
                    block(vec![op_call("change_screen_map", vec![])]),
                ),
                (
                    Cond::Call(Call {
                        op: ident("party_is_active", 2),
                        args: vec![Arg::Name(ident("p_town_6", 2))],
                        span: line_span(2),
                    }),
                    block(vec![]),
                ),
            ],
            otherwise: Some(block(vec![op_call(
                "display_message",
                vec![Arg::Str("@hi".into(), line_span(3))],
            )])),
            span: line_span(1),
        }])
        .unwrap();
        match &tree[0] {
            uw_lower::Stmt::If { arms, otherwise, origin } => {
                assert_eq!(origin, "start.uw:1");
                assert_eq!(
                    arms[0].0,
                    uw_lower::Cond::Cmp {
                        op: uw_lower::CmpOp::Ge,
                        left: uw_lower::Expr::Value(Value::Ref("$gold".into())),
                        right: uw_lower::Expr::Value(Value::Int(100)),
                        origin: "start.uw:1".into(),
                    }
                );
                match &arms[0].1[0] {
                    uw_lower::Stmt::Op(op) => assert_eq!(op.name, "change_screen_map"),
                    other => panic!("{other:?}"),
                }
                match &arms[1].0 {
                    uw_lower::Cond::Op(op) => {
                        assert_eq!(op.name, "party_is_active");
                        assert_eq!(op.args, [Value::Ref("p_town_6".into())]);
                    }
                    other => panic!("{other:?}"),
                }
                assert_eq!(otherwise.as_ref().unwrap().len(), 1);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn condition_combinators_are_checked() {
        let tree = run_stmts(vec![Stmt::If {
            arms: vec![(
                Cond::And(vec![
                    Cond::Call(Call {
                        op: ident("party_is_active", 1),
                        args: vec![Arg::Name(ident("p_town_6", 1))],
                        span: line_span(1),
                    }),
                    Cond::Not(
                        Box::new(cmp(
                            uw_syntax::CmpOp::Lt,
                            Expr::Value(Arg::Global(ident("$x", 1))),
                            Expr::Value(int(3)),
                        )),
                        line_span(1),
                    ),
                ]),
                block(vec![]),
            )],
            otherwise: None,
            span: line_span(1),
        }])
        .unwrap();
        match &tree[0] {
            uw_lower::Stmt::If { arms, .. } => match &arms[0].0 {
                uw_lower::Cond::And(parts) => {
                    assert!(matches!(&parts[0], uw_lower::Cond::Op(op) if op.name == "party_is_active"));
                    assert!(matches!(&parts[1], uw_lower::Cond::Not(inner)
                        if matches!(&**inner, uw_lower::Cond::Cmp { op: uw_lower::CmpOp::Lt, .. })));
                }
                other => panic!("{other:?}"),
            },
            other => panic!("{other:?}"),
        }
    }

    fn override_item(kind: Option<&str>, target: &str, handlers: Vec<uw_syntax::Handler>, members: Vec<Member>) -> Item {
        Item::Override {
            kind: kind.map(|k| ident(k, 1)),
            name: ident(target, 1),
            handlers,
            members,
            span: line_span(1),
        }
    }

    fn enter(line: usize) -> uw_syntax::Handler {
        uw_syntax::Handler {
            event: ident("enter", line),
            body: Block { stmts: vec![op_call("change_screen_map", vec![])], span: line_span(line) },
        }
    }

    #[test]
    fn override_id_and_menu_members_become_patches() {
        let item = override_item(
            None,
            "menu_start_phase_2",
            vec![enter(2)],
            vec![
                Member::Text { text: "New text".into(), span: line_span(3) },
                Member::Option {
                    id: ident("uw_skip", 4),
                    text: "Skip".into(),
                    when: Some(When::Cond(Cond::Cmp {
                        op: uw_syntax::CmpOp::Lt,
                        left: Expr::Value(Arg::Global(ident("$gold", 4))),
                        right: Expr::Value(int(10)),
                        span: line_span(4),
                    })),
                    place: Some(Place::After(ident("leave", 4))),
                    body: block(vec![op_call("change_screen_map", vec![])]),
                    span: line_span(4),
                },
                Member::ChangeOption {
                    id: ident("leave", 5),
                    text: Some(("Leave now".into(), line_span(5))),
                    when: Some(When::Block(Block {
                        stmts: vec![op_call("display_message", vec![Arg::Str("@x".into(), line_span(5))])],
                        span: line_span(5),
                    })),
                    body: Some(block(vec![op_call("change_screen_map", vec![])])),
                    span: line_span(5),
                },
                Member::RemoveOption { id: ident("camp_wait_here", 6), span: line_span(6) },
            ],
        );
        let checked = run_items(vec![item]).unwrap();
        let patches = &checked.patches;
        assert_eq!(patches.len(), 5);
        assert!(patches.iter().all(|p| p.entry == "start_phase_2"));
        assert_eq!((patches[0].action.clone(), patches[0].path.clone()), (Action::ReplaceBody, vec![MENU_ENTER_FIELD]));
        assert_eq!(
            (patches[1].action.clone(), patches[1].path.clone(), patches[1].origin.as_str()),
            (Action::ReplaceText("New text".into()), vec![MENU_TEXT_FIELD], "start.uw:3")
        );
        assert_eq!(patches[2].path, vec![MENU_OPTIONS_FIELD]);
        match &patches[2].action {
            Action::InsertOption { option, place } => {
                assert_eq!(option.id, "uw_skip");
                assert_eq!(option.text, "Skip");
                assert_eq!(
                    option.condition,
                    vec![Op {
                        name: "lt".into(),
                        args: vec![Value::Ref("$gold".into()), Value::Int(10)],
                        origin: "start.uw:4".into(),
                    }]
                );
                assert_eq!(option.consequence.len(), 1);
                assert_eq!(option.origin, "start.uw:4");
                assert_eq!(*place, OptionPlace::After("leave".into()));
            }
            other => panic!("{other:?}"),
        }
        match &patches[3].action {
            Action::ChangeOption { id, text, condition, consequence } => {
                assert_eq!(id, "leave");
                assert_eq!(text.as_deref(), Some("Leave now"));
                assert_eq!(condition.as_ref().unwrap()[0].name, "display_message");
                assert_eq!(consequence.as_ref().unwrap()[0].name, "change_screen_map");
            }
            other => panic!("{other:?}"),
        }
        assert_eq!(patches[4].action, Action::RemoveOption("camp_wait_here".into()));
    }

    #[test]
    fn old_override_form_accepts_members() {
        let item = override_item(
            Some("menu"),
            "start_phase_2",
            vec![],
            vec![Member::Text { text: "Old form".into(), span: line_span(2) }],
        );
        let checked = run_items(vec![item]).unwrap();
        assert_eq!(checked.patches[0].action, Action::ReplaceText("Old form".into()));
    }

    #[test]
    fn new_menu_becomes_one_action() {
        let item = Item::New {
            id: ident("menu_uw_ask", 1),
            text: Some(("Where from?".into(), line_span(1))),
            handlers: vec![uw_syntax::Handler {
                event: ident("enter", 2),
                body: Block {
                    stmts: vec![op_call("display_message", vec![Arg::Str("@hi".into(), line_span(2))])],
                    span: line_span(2),
                },
            }],
            members: vec![
                Member::Option {
                    id: ident("north", 3),
                    text: "North".into(),
                    when: None,
                    place: None,
                    body: block(vec![op_call("change_screen_map", vec![])]),
                    span: line_span(3),
                },
                Member::Option {
                    id: ident("south", 4),
                    text: "South".into(),
                    when: None,
                    place: Some(Place::Before(ident("north", 4))),
                    body: block(vec![op_call("change_screen_map", vec![])]),
                    span: line_span(4),
                },
            ],
            span: line_span(1),
        };
        let checked = run_items(vec![item]).unwrap();
        assert_eq!(checked.patches.len(), 1);
        let patch = &checked.patches[0];
        assert_eq!(
            (patch.module.as_str(), patch.var.as_str(), patch.entry.as_str(), patch.path.as_slice()),
            ("module_game_menus", "game_menus", "uw_ask", &[][..])
        );
        match &patch.action {
            Action::NewMenu { text, enter, options } => {
                assert_eq!(text, "Where from?");
                assert_eq!(enter.len(), 1);
                assert_eq!(enter[0].name, "display_message");
                assert_eq!(options.len(), 2);
                assert_eq!((options[0].id.as_str(), options[0].text.as_str()), ("south", "South"));
                assert_eq!((options[1].id.as_str(), options[1].text.as_str()), ("north", "North"));
                assert_eq!(options[1].consequence[0].name, "change_screen_map");
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn option_and_menu_ids_are_strict() {
        fn option(id: &str, line: usize) -> Member {
            Member::Option {
                id: ident(id, line),
                text: "x".into(),
                when: None,
                place: None,
                body: block(vec![]),
                span: line_span(line),
            }
        }
        let menu = |members| override_item(None, "menu_start_phase_2", vec![], members);

        let errs = run_items(vec![menu(vec![option("leave", 2)])]).unwrap_err();
        assert!(errs[0].contains("already has option 'leave'"), "{errs:?}");
        assert!(errs[0].contains("change option leave"), "{errs:?}");

        let errs = run_items(vec![menu(vec![Member::ChangeOption {
            id: ident("cam_wait_here", 2),
            text: None,
            when: None,
            body: None,
            span: line_span(2),
        }])])
        .unwrap_err();
        assert!(errs[0].contains("has no option 'cam_wait_here'"), "{errs:?}");
        assert!(errs[0].contains("did you mean 'camp_wait_here'?"), "{errs:?}");

        let errs = run_items(vec![menu(vec![Member::RemoveOption {
            id: ident("nope", 2),
            span: line_span(2),
        }])])
        .unwrap_err();
        assert!(errs[0].contains("has no option 'nope'"), "{errs:?}");

        let errs = run_items(vec![menu(vec![option("mno_camp", 2)])]).unwrap_err();
        assert!(errs[0].contains("without `mno_`"), "{errs:?}");
        assert!(errs[0].contains("write 'camp'"), "{errs:?}");

        let errs = run_items(vec![menu(vec![option("seç", 2)])]).unwrap_err();
        assert!(errs[0].contains("non-ASCII"), "{errs:?}");

        let checked = run_items(vec![menu(vec![
            option("first", 2),
            Member::Option {
                id: ident("second", 3),
                text: "Second".into(),
                when: None,
                place: Some(Place::After(ident("first", 3))),
                body: block(vec![]),
                span: line_span(3),
            },
        ])])
        .unwrap();
        match &checked.patches[1].action {
            Action::InsertOption { place, .. } => assert_eq!(*place, OptionPlace::After("first".into())),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn new_menu_and_duplicate_members_are_strict() {
        let new = |id: &str| Item::New {
            id: ident(id, 1),
            text: Some(("x".into(), line_span(1))),
            handlers: vec![],
            members: vec![],
            span: line_span(1),
        };
        let errs = run_items(vec![new("menu_start_phase_2")]).unwrap_err();
        assert!(errs[0].contains("already exists"), "{errs:?}");
        let errs = run_items(vec![new("menu_menu_x")]).unwrap_err();
        assert!(errs[0].contains("prefix twice"), "{errs:?}");
        let errs = run_items(vec![new("troop_x")]).unwrap_err();
        assert!(errs[0].contains("not a full menu id"), "{errs:?}");

        let errs = run_items(vec![override_item(None, "menu_menu_start_phase_2", vec![], vec![])]).unwrap_err();
        assert!(errs[0].contains("prefix twice"), "{errs:?}");

        let errs = run_items(vec![override_item(
            None,
            "menu_start_phase_2",
            vec![],
            vec![
                Member::Text { text: "one".into(), span: line_span(2) },
                Member::Text { text: "two".into(), span: line_span(3) },
            ],
        )])
        .unwrap_err();
        assert!(errs[0].contains("two `text` lines"), "{errs:?}");
        assert!(errs[0].contains("start.uw:2") && errs[0].contains("start.uw:3"), "{errs:?}");

        let errs = run_items(vec![override_item(None, "menu_start_phase_2", vec![enter(2), enter(3)], vec![])])
            .unwrap_err();
        assert!(errs[0].contains("two `on enter` blocks"), "{errs:?}");
        assert!(errs[0].contains("start.uw:2") && errs[0].contains("start.uw:3"), "{errs:?}");
    }

    #[test]
    fn when_conditions_become_operations() {
        let option = |when: When| Member::Option {
            id: ident("uw_opt", 2),
            text: "Opt".into(),
            when: Some(when),
            place: None,
            body: block(vec![]),
            span: line_span(2),
        };
        let condition = |member: Member| {
            run_items(vec![override_item(None, "menu_start_phase_2", vec![], vec![member])]).unwrap()
        };
        let with_cond = |cond: Cond| {
            match &condition(option(When::Cond(cond))).patches[0].action {
                Action::InsertOption { option, .. } => option.condition.clone(),
                other => panic!("{other:?}"),
            }
        };

        let ops = with_cond(Cond::Call(Call {
            op: ident("party_is_active", 2),
            args: vec![Arg::Name(ident("p_town_6", 2))],
            span: line_span(2),
        }));
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].name, "party_is_active");
        assert_eq!(ops[0].args, [Value::Ref("p_town_6".into())]);

        let ops = with_cond(Cond::Not(
            Box::new(cmp(uw_syntax::CmpOp::Lt, Expr::Value(int(1)), Expr::Value(int(2)))),
            line_span(2),
        ));
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].name, "ge");

        let ops = with_cond(Cond::Or(vec![
            Cond::Call(Call {
                op: ident("party_is_active", 2),
                args: vec![Arg::Name(ident("p_town_6", 2))],
                span: line_span(2),
            }),
            Cond::Call(Call {
                op: ident("party_is_active", 2),
                args: vec![Arg::Name(ident("p_main_party", 2))],
                span: line_span(2),
            }),
        ]));
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].name, "this_or_next|party_is_active");
        assert_eq!(ops[1].name, "party_is_active");

        let ops = with_cond(cmp(
            uw_syntax::CmpOp::Lt,
            Expr::Bin {
                op: uw_syntax::BinOp::Add,
                left: Box::new(Expr::Value(Arg::Global(ident("$gold", 2)))),
                right: Box::new(Expr::Value(int(1))),
                span: line_span(2),
            },
            Expr::Value(int(10)),
        ));
        assert_eq!(ops.len(), 2);
        assert_eq!(ops[0].name, "store_add");
        assert_eq!(ops[0].args[0], Value::Ref(":_uw0".into()));
        assert_eq!(ops[1].name, "lt");
        assert_eq!(ops[1].args[0], Value::Ref(":_uw0".into()));
    }

    #[test]
    fn fn_name_mistakes_are_reported() {
        // A fn may not take the name of a Native operation: that call would be the operation.
        let errs = run("fn display_message() { }\non game.start { display_message(\"@x\") }").unwrap_err();
        assert!(errs[0].contains("`fn display_message` has the name of a Native operation"), "{errs:?}");
        // Two fns with one name.
        let errs = run("fn give() { }\nfn give() { }").unwrap_err();
        assert!(errs[0].contains("`fn give` is already defined at start.uw:1"), "{errs:?}");
        // A typo in a fn name is suggested like an unknown operation.
        let errs = run("fn give_gold(:who) { troop_add_gold(:who, 5) }\non game.start { give_gol(trp_player) }").unwrap_err();
        assert!(errs[0].contains("'give_gol' is not a Native operation"), "{errs:?}");
        assert!(errs[0].contains("did you mean 'give_gold'?"), "{errs:?}");
    }

    #[test]
    fn fn_calls_check_their_argument_count() {
        let errs = run("fn give(:amount) { display_message(\"@x\") }\non game.start { give() }").unwrap_err();
        assert!(errs[0].contains("`fn give` takes 1 value(s), got 0"), "{errs:?}");
        assert!(errs[0].contains("call it as `give(:amount)`"), "{errs:?}");
        let errs = run("fn give() { display_message(\"@x\") }\non game.start { give(1, 2) }").unwrap_err();
        assert!(errs[0].contains("`fn give` takes 0 value(s), got 2"), "{errs:?}");
        assert!(errs[0].contains("call it as `give()`"), "{errs:?}");
    }

    #[test]
    fn fn_cycles_name_the_chain() {
        let errs = run("fn a() { a() }\non game.start { a() }").unwrap_err();
        assert!(errs[0].contains("never ends: a -> a"), "{errs:?}");
        let errs = run("fn a() { b() }\nfn b() { a() }\non game.start { a() }").unwrap_err();
        assert!(errs[0].contains("never ends: a -> b -> a"), "{errs:?}");
        // Deep, but not a cycle: the chain still stops at the limit.
        let mut text = String::new();
        for i in 0..=FN_DEPTH {
            let call = if i == FN_DEPTH { "change_screen_map()".to_string() } else { format!("f{}()", i + 1) };
            text.push_str(&format!("fn f{i}() {{ {call} }}\n"));
        }
        text.push_str("on game.start { f0() }");
        let errs = run(&text).unwrap_err();
        assert!(errs[0].contains(&format!("nest more than {FN_DEPTH} deep: f0 -> f1")), "{errs:?}");
    }

    #[test]
    fn object_words_name_the_fn_they_need() {
        let errs = run("on game.start {\n player.gold += 1000\n}").unwrap_err();
        assert!(errs[0].starts_with("error[E0216]"), "{errs:?}");
        assert!(errs[0].contains("'player.gold' needs `fn player.gold.add`"), "{errs:?}");
        assert!(errs[0].contains("write `fn player.gold.add(:value)"), "{errs:?}");
        let errs = run("on game.start {\n player.gold -= 50\n}").unwrap_err();
        assert!(errs[0].contains("needs `fn player.gold.remove`"), "{errs:?}");
        let errs = run("on game.start {\n player.level = 5\n}").unwrap_err();
        assert!(errs[0].contains("needs `fn player.level.set`"), "{errs:?}");
        // A word read as a value needs its `get`.
        let errs = run("on game.start {\n $g_total = player.gold + 1\n}").unwrap_err();
        assert!(errs[0].contains("needs `fn player.gold.get`"), "{errs:?}");
        assert!(errs[0].contains("write `fn player.gold.get(:out)"), "{errs:?}");
        // `*=` has no verb yet: no fn name is guessed for it.
        let errs = run("on game.start {\n player.gold *= 2\n}").unwrap_err();
        assert!(errs[0].contains("'player.gold' has no `*=` form"), "{errs:?}");
    }

    #[test]
    fn an_object_word_takes_a_computed_value_through_a_temporary() {
        // A fn takes one value, so `player.gold += $g_bonus * 2` stores the product first. The
        // temporary is not one of `uw_lower`'s `:_uwN` numbers: both land in the same block.
        let checked = run(
            "fn player.gold.add(:amount) { $g_gold += :amount }\n\
             on game.start {\n player.gold += $g_bonus * 2\n}",
        )
        .unwrap();
        let ops = &checked.patches[0].ops;
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["store_mul", "val_add"], "{ops:?}");
        assert_eq!(
            ops[0].args,
            [Value::Ref(":_uw_val0".into()), Value::Ref("$g_bonus".into()), Value::Int(2)]
        );
        assert_eq!(ops[1].args, [Value::Ref("$g_gold".into()), Value::Ref(":_uw_val0".into())]);
    }

    #[test]
    fn a_dotted_word_in_an_if_is_read_before_the_comparison() {
        let checked = run(
            "fn player.gold.get(:out) { assign(:out, $g_gold) }\n\
             on game.start {\n if player.gold >= 100 { change_screen_map() }\n}",
        )
        .unwrap();
        let ops = &checked.patches[0].ops;
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["assign", "try_begin", "ge", "change_screen_map", "try_end"], "{ops:?}");
        // The `get` fills the temporary before the comparison reads it.
        assert_eq!(ops[0].args, [Value::Ref(":_uw_get0".into()), Value::Ref("$g_gold".into())]);
        assert_eq!(ops[2].args, [Value::Ref(":_uw_get0".into()), Value::Int(100)]);
    }

    #[test]
    fn check_block_with_opens_object_words_from_the_lib() {
        // A project's record block is checked on its own, so the lib's fns come with it (T-087):
        // the `get` of `player.gold` in an `if` condition fills the temporary before the comparison.
        let lib = SourceFile::new("lib/player.uw", "fn player.gold.get(:out) { assign(:out, $g_gold) }");
        let tree = uw_syntax::parse(&lib.text).expect("parses");
        let sources = [Source { file: lib, tree }];
        let file = SourceFile::new("block.uw", "if player.gold >= 100 { change_screen_map() }");
        let stmts = uw_syntax::parse_block(&file.text).expect("parses");
        let ops = check_block_with(&atlas(), &file, &stmts, &sources).unwrap();
        let names: Vec<&str> = ops.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["assign", "try_begin", "ge", "change_screen_map", "try_end"], "{ops:?}");
        assert_eq!(ops[0].args, [Value::Ref(":_uw_get0".into()), Value::Ref("$g_gold".into())]);
        assert_eq!(ops[2].args, [Value::Ref(":_uw_get0".into()), Value::Int(100)]);
        // Without the lib the word has no `get` fn: E0216, as `check_block` says.
        let errs = check_block(&atlas(), &file, &stmts).unwrap_err();
        assert!(errs[0].render(&file).contains("needs `fn player.gold.get`"), "{errs:?}");
    }

    #[test]
    fn a_dotted_word_in_a_when_is_read_before_the_condition() {
        let checked = run(
            "fn player.level.get(:out) { assign(:out, $g_level) }\n\
             override menu start_phase_2 {\n\
             option check_level \"Gold?\" when player.level >= 5 { display_message(\"@ok\") }\n}",
        )
        .unwrap();
        let Action::InsertOption { option, .. } = &checked.patches[0].action else {
            panic!("{:?}", checked.patches[0].action)
        };
        let names: Vec<&str> = option.condition.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(names, ["assign", "ge"], "{:?}", option.condition);
        assert_eq!(option.condition[0].args, [Value::Ref(":_uw_get0".into()), Value::Ref("$g_level".into())]);
        assert_eq!(option.condition[1].args[0], Value::Ref(":_uw_get0".into()));
        assert_eq!(option.condition[1].args[1], Value::Int(5));
    }

    #[test]
    fn a_save_name_means_a_dollar_global_and_starts_at_game_start() {
        let checked = run(
            "mod saves\n\
             save rich = false\n\
             save gold = 5\n\
             on game.start {\n rich = true\n gold += 1\n if rich { change_screen_map() }\n}\n\
             override menu start_phase_2 {\n option uw_poor \"Poor?\" when not rich { display_message(\"@no\") }\n}",
        )
        .unwrap();
        // The start values are the first patch, appended to the `game_start` script.
        let start = &checked.patches[0];
        assert_eq!((start.entry.as_str(), &start.action), ("game_start", &Action::Append));
        assert_eq!(start.path, vec![SCRIPT_BODY_FIELD]);
        let args: Vec<&[Value]> = start.ops.iter().map(|op| op.args.as_slice()).collect();
        assert_eq!(
            args,
            [
                &[Value::Ref("$rich".into()), Value::Int(0)][..],
                &[Value::Ref("$gold".into()), Value::Int(5)][..],
            ]
        );
        // Every bare name is the global: an assignment target, an operand and a condition.
        let hook = &checked.patches[1];
        let names: Vec<&str> = hook.ops.iter().map(|op| op.name.as_str()).collect();
        assert_eq!(names, ["assign", "val_add", "try_begin", "neq", "change_screen_map", "try_end"]);
        assert_eq!(hook.ops[0].args, [Value::Ref("$rich".into()), Value::Int(1)]);
        assert_eq!(hook.ops[1].args, [Value::Ref("$gold".into()), Value::Int(1)]);
        assert_eq!(hook.ops[3].args, [Value::Ref("$rich".into()), Value::Int(0)]);
        let Action::InsertOption { option, .. } = &checked.patches[2].action else {
            panic!("{:?}", checked.patches[2].action)
        };
        assert_eq!(option.condition[0].name, "eq");
        assert_eq!(option.condition[0].args, [Value::Ref("$rich".into()), Value::Int(0)]);
    }

    #[test]
    fn a_save_name_is_used_once_and_never_for_a_kit_name() {
        let errs = run("save rich = 0\nsave rich = 1").unwrap_err();
        assert!(errs[0].starts_with("error[E0202]"), "{errs:?}");
        assert!(errs[0].contains("already defined at start.uw:1"), "{errs:?}");
        let errs = run("save eq = 0").unwrap_err();
        assert!(errs[0].starts_with("error[E0217]"), "{errs:?}");
        assert!(errs[0].contains("a Native operation"), "{errs:?}");
        let errs = run("save tf_male = 0").unwrap_err();
        assert!(errs[0].contains("a kit constant"), "{errs:?}");
        // A name `names.uw` installed is taken as well (T-077).
        let mut atlas = atlas();
        atlas.set_game_names(&[("praven".to_owned(), "p_town_6".to_owned())]);
        let file = SourceFile::new("start.uw", "save praven = 0");
        let tree = uw_syntax::parse(&file.text).expect("parses");
        let errs = check(&atlas, &[Source { file: file.clone(), tree }], "folder").unwrap_err();
        assert!(errs[0].diag.message.contains("a kit name"), "{:?}", errs[0].diag);
    }
}
