//! Dry run of Native's start scripts.
//!
//! The kit builds the starting world inside `script_game_start`, not in the record tables. This
//! crate runs that script (and every script it calls) over the atlas, without the game, and keeps
//! a trace of every slot and field it writes: (record, key, value, script, op, what it was read
//! from). That trace is the "start snapshot": which cell Native sets at game start, to what, and
//! because of which other cells. Nothing here is hand-written per field: a cell is whatever the
//! kit's own operations write.
//!
//! Rules:
//! - An operation this crate does not know is never guessed. It is counted in
//!   [`Report::unknown_ops`]; its destination (if it stores into one) becomes an `unknown` value,
//!   and every value computed from it stays `unknown`.
//! - Random numbers come from a fixed seed and are marked `random`: the game will pick its own.
//! - Operations are grouped in families (`families/*.rs`); the core here does control flow,
//!   arithmetic, comparisons, script calls and the generic `<kind>_*_slot*` operations.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::rc::Rc;

use serde_json::Value as Json;
use uw_atlas::{Atlas, Entry};
use uw_decomp::records::Schema;

pub mod families;

/// A record cell: a slot (`key = "slot 31"`) or a field of a record (`key = "faction"`).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell {
    pub table: String,
    pub index: i64,
    pub key: String,
}

impl Cell {
    pub fn slot(table: &str, index: i64, slot: i64) -> Cell {
        Cell { table: table.to_owned(), index, key: format!("slot {slot}") }
    }
    pub fn field(table: &str, index: i64, field: &str) -> Cell {
        Cell { table: table.to_owned(), index, key: field.to_owned() }
    }
}

/// The cells a value was computed from.
pub type Deps = Rc<BTreeSet<Cell>>;

/// A value during the run: the number the engine would hold, and how much we know about it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Val {
    pub n: i64,
    /// Came from the random generator (the real game picks another number).
    pub random: bool,
    /// Came from an operation this crate does not model: the number means nothing.
    pub unknown: bool,
    /// Record cells read to make this value.
    pub deps: Deps,
}

impl Val {
    pub fn num(n: i64) -> Val {
        Val { n, ..Val::default() }
    }
    pub fn unknown() -> Val {
        Val { unknown: true, ..Val::default() }
    }
    /// A value made from `n` and the inputs `from` (random/unknown/deps carry over).
    pub fn derived(n: i64, from: &[&Val]) -> Val {
        let mut out = Val::num(n);
        out.random = from.iter().any(|v| v.random);
        out.unknown = from.iter().any(|v| v.unknown);
        if from.iter().any(|v| !v.deps.is_empty()) {
            let mut set = BTreeSet::new();
            for v in from {
                set.extend(v.deps.iter().cloned());
            }
            out.deps = Rc::new(set);
        }
        out
    }
    /// True when the number can be trusted to be what the game computes.
    pub fn certain(&self) -> bool {
        !self.random && !self.unknown
    }
}

/// One write the start scripts make to a record cell.
#[derive(Clone, Debug)]
pub struct Write {
    pub cell: Cell,
    pub value: Val,
    /// Script that made the write and the position of the operation in its block.
    pub script: String,
    pub op: usize,
}

/// What a block did: ran to its end, or a condition failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    Done,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    /// The run took more operations than [`Machine::step_limit`].
    StepLimit(u64),
    /// `call_script` named a script the atlas does not have.
    NoScript(String),
    /// A block's `try_*` / `try_end` do not pair up.
    Unbalanced(String),
    /// Calls nested deeper than 200 (a script calling itself without end).
    TooDeep(String),
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::StepLimit(n) => write!(f, "stopped after {n} operations"),
            RunError::NoScript(s) => write!(f, "no script `{s}` in the atlas"),
            RunError::Unbalanced(s) => write!(f, "script `{s}`: try/try_end do not pair up"),
            RunError::TooDeep(s) => write!(f, "script `{s}`: calls nest deeper than 200"),
        }
    }
}

/// Summary of a run.
#[derive(Clone, Debug, Default)]
pub struct Report {
    pub steps: u64,
    pub writes: usize,
    pub scripts_called: BTreeSet<String>,
    /// Operation name -> how many times it ran unmodelled.
    pub unknown_ops: BTreeMap<String, usize>,
}

struct Frame {
    script: String,
    params: Vec<Val>,
    locals: HashMap<String, Val>,
}

/// A parsed block: plain operations, `try_begin … else_try … try_end`, and `try_for_* … try_end`.
#[derive(Debug)]
enum Node {
    Op(usize),
    Try(Vec<Vec<Node>>),
    /// A loop body behaves like a `try` block: it may hold `else_try` branches (Native does this).
    Loop(usize, Vec<Vec<Node>>),
}

/// Slot kinds of the `<kind>_set_slot` family and the table each one addresses.
const SLOT_KINDS: &[(&str, &str)] = &[
    ("party_template", "party_templates"),
    ("troop", "troops"),
    ("party", "parties"),
    ("faction", "factions"),
    ("item", "items"),
    ("quest", "quests"),
    ("scene", "scenes"),
];

/// The engine's register operands: `opmask_register | n` (header_common.py).
const REGISTER_TAG: i64 = 1;
const TAG_SHIFT: u32 = 56;

pub struct Machine<'a> {
    pub atlas: &'a Atlas,
    scripts: HashMap<String, &'a [Json]>,
    parsed: HashMap<String, Rc<Vec<Node>>>,
    schemas: HashMap<String, Schema>,
    initial_fields: HashMap<(String, i64), BTreeMap<String, String>>,
    neg: i64,
    this_or_next: i64,
    pub globals: HashMap<String, Val>,
    pub registers: HashMap<i64, Val>,
    /// Current value of every cell written so far (cells never written keep their record value).
    pub cells: BTreeMap<Cell, Val>,
    pub trace: Vec<Write>,
    pub report: Report,
    /// Parties that exist: Native's list plus the ones the run spawns.
    pub party_count: i64,
    pub step_limit: u64,
    rng: u64,
    frames: Vec<Frame>,
    current_op: usize,
}

impl<'a> Machine<'a> {
    /// A machine over `atlas`; `schemas` (kit-meta/records) let families read record fields.
    pub fn new(atlas: &'a Atlas, schemas: Vec<Schema>, seed: u64) -> Machine<'a> {
        let mut scripts = HashMap::new();
        if let Some(table) = atlas.tables.get("scripts") {
            for entry in &table.entries {
                if let (Some(name), Some(block)) = (&entry.name, script_block(entry)) {
                    scripts.entry(name.clone()).or_insert(block);
                }
            }
        }
        let flag = |name: &str, fallback: i64| {
            atlas.op(name).map(|op| op.code as i64).or_else(|| atlas.constant(name).and_then(|c| c.value.as_i64())).unwrap_or(fallback)
        };
        let party_count = atlas.tables.get("parties").map_or(0, |t| t.entries.len() as i64);
        Machine {
            atlas,
            scripts,
            parsed: HashMap::new(),
            schemas: schemas.into_iter().map(|s| (s.table.clone(), s)).collect(),
            initial_fields: HashMap::new(),
            neg: flag("neg", 0x8000_0000),
            this_or_next: flag("this_or_next", 0x4000_0000),
            globals: HashMap::new(),
            registers: HashMap::new(),
            cells: BTreeMap::new(),
            trace: Vec::new(),
            report: Report::default(),
            party_count,
            step_limit: 200_000_000,
            rng: seed.max(1),
            frames: Vec::new(),
            current_op: 0,
        }
    }

    /// Runs one script (`game_start`) with the given parameters.
    pub fn run_script(&mut self, name: &str, params: Vec<Val>) -> Result<Flow, RunError> {
        self.call(name, params)
    }

    // ---- operand access (for families) -------------------------------------------------

    /// The value an operand stands for: a number, `:local`, `$global`, a register, or a record
    /// reference (`p_town_1` -> its index).
    pub fn get(&self, arg: &Json) -> Val {
        if let Some(n) = arg.as_i64() {
            if n >> TAG_SHIFT == REGISTER_TAG {
                return self.registers.get(&(n & 0xFF_FFFF)).cloned().unwrap_or_default();
            }
            return Val::num(n);
        }
        let Some(text) = arg.as_str() else { return Val::unknown() };
        if let Some(local) = text.strip_prefix(':') {
            return self.frame().and_then(|f| f.locals.get(local)).cloned().unwrap_or_default();
        }
        if let Some(global) = text.strip_prefix('$') {
            return self.globals.get(global).cloned().unwrap_or_default();
        }
        if text.starts_with('@') {
            return Val::num(0); // quick string: no world value
        }
        match self.resolve(text) {
            Some(n) => Val::num(n),
            None => Val::unknown(),
        }
    }

    /// Stores into a destination operand (`:local`, `$global`, register).
    pub fn set(&mut self, arg: &Json, value: Val) {
        if let Some(n) = arg.as_i64() {
            if n >> TAG_SHIFT == REGISTER_TAG {
                self.registers.insert(n & 0xFF_FFFF, value);
            }
            return;
        }
        let Some(text) = arg.as_str() else { return };
        if let Some(local) = text.strip_prefix(':') {
            if let Some(frame) = self.frames.last_mut() {
                frame.locals.insert(local.to_owned(), value);
            }
        } else if let Some(global) = text.strip_prefix('$') {
            self.globals.insert(global.to_owned(), value);
        }
    }

    /// Index of a record reference, or a named constant (`slot_town_lord`, `fac_kingdoms_end`).
    pub fn resolve(&self, text: &str) -> Option<i64> {
        if let Some((_, entry)) = self.atlas.resolve_ref(text) {
            return Some(entry.index as i64);
        }
        self.atlas.constant(text).and_then(|c| c.value.as_i64())
    }

    /// Reads a record cell: the value written during the run, else the record's start value
    /// (slots start at 0; fields come from the record). The result depends on that cell.
    pub fn read(&mut self, cell: Cell) -> Val {
        let mut value = match self.cells.get(&cell) {
            Some(v) => v.clone(),
            None => self.initial(&cell),
        };
        let mut deps: BTreeSet<Cell> = value.deps.iter().cloned().collect();
        deps.insert(cell);
        value.deps = Rc::new(deps);
        value
    }

    /// Writes a record cell and records it in the trace.
    pub fn write(&mut self, cell: Cell, value: Val) {
        let script = self.frame().map(|f| f.script.clone()).unwrap_or_default();
        self.trace.push(Write { cell: cell.clone(), value: value.clone(), script, op: self.current_op });
        self.report.writes += 1;
        self.cells.insert(cell, value);
    }

    /// A random number in `lo..hi` (kit: `store_random_in_range`), marked random.
    pub fn random(&mut self, lo: i64, hi: i64) -> i64 {
        // xorshift64: deterministic for a seed.
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        if hi <= lo { lo } else { lo + (self.rng % (hi - lo) as u64) as i64 }
    }

    /// Counts an operation the machine does not model; stores `unknown` into its destination.
    pub fn unknown_op(&mut self, name: &str, lhs: bool, args: &[Json]) {
        *self.report.unknown_ops.entry(name.to_owned()).or_default() += 1;
        if lhs {
            if let Some(dest) = args.first() {
                self.set(dest, Val::unknown());
            }
        }
    }

    /// A record field's start value as text, the way `uw new` prints it (`faction fac_kingdom_1`).
    pub fn initial_field_text(&mut self, table: &str, index: i64, field: &str) -> Option<String> {
        let key = (table.to_owned(), index);
        if !self.initial_fields.contains_key(&key) {
            let mut map = BTreeMap::new();
            if let (Some(schema), Some(entry)) = (self.schemas.get(table), self.entry(table, index)) {
                let text = uw_decomp::records::print(schema, entry);
                for line in text.lines().skip(1) {
                    let line = line.trim();
                    if let Some((name, value)) = line.split_once(' ') {
                        map.insert(name.to_owned(), value.trim().to_owned());
                    }
                }
            }
            self.initial_fields.insert(key.clone(), map);
        }
        self.initial_fields[&key].get(field).cloned()
    }

    /// The atlas entry at `index` of `table`.
    pub fn entry(&self, table: &str, index: i64) -> Option<&'a Entry> {
        let table = self.atlas.tables.get(table)?;
        usize::try_from(index).ok().and_then(|i| table.entries.get(i))
    }

    fn initial(&mut self, cell: &Cell) -> Val {
        if cell.key.starts_with("slot ") {
            return Val::num(0);
        }
        let text = self.initial_field_text(&cell.table, cell.index, &cell.key);
        match text {
            Some(text) => match text.parse::<i64>() {
                Ok(n) => Val::num(n),
                Err(_) => self.resolve(&text).map(Val::num).unwrap_or_else(Val::unknown),
            },
            None => Val::unknown(),
        }
    }

    fn frame(&self) -> Option<&Frame> {
        self.frames.last()
    }

    // ---- execution -----------------------------------------------------------------------

    fn call(&mut self, name: &str, params: Vec<Val>) -> Result<Flow, RunError> {
        let Some(code) = self.scripts.get(name).copied() else {
            return Err(RunError::NoScript(name.to_owned()));
        };
        if self.frames.len() >= 200 {
            return Err(RunError::TooDeep(name.to_owned()));
        }
        let nodes = match self.parsed.get(name) {
            Some(nodes) => nodes.clone(),
            None => {
                let nodes = Rc::new(self.parse(name, code)?);
                self.parsed.insert(name.to_owned(), nodes.clone());
                nodes
            }
        };
        self.report.scripts_called.insert(name.to_owned());
        self.frames.push(Frame { script: name.to_owned(), params, locals: HashMap::new() });
        let flow = self.block(code, &nodes);
        self.frames.pop();
        flow
    }

    fn op_name(&self, op: &Json) -> Option<(&'a str, bool, bool, bool, bool)> {
        let head = match op {
            Json::Array(items) => items.first()?.as_i64()?,
            other => other.as_i64()?,
        };
        let neg = head & self.neg != 0;
        let or_next = head & self.this_or_next != 0;
        let found = self.atlas.op_for_code(head as u64)?;
        // The kit names some codes twice (header_operations.py): use the one this crate matches on.
        let name = match found.name.as_str() {
            "end_try" => "try_end",
            "else_try_begin" => "else_try",
            other => other,
        };
        Some((name, neg, or_next, found.can_fail, found.lhs))
    }

    fn parse(&self, script: &str, code: &[Json]) -> Result<Vec<Node>, RunError> {
        let mut pos = 0;
        let nodes = self.parse_until(script, code, &mut pos, false)?;
        if pos < code.len() {
            return Err(RunError::Unbalanced(script.to_owned()));
        }
        Ok(nodes.0)
    }

    /// Parses until `try_end` (when `inner`) or the end; returns the nodes and the split points of
    /// `else_try` (as separate branches).
    fn parse_until(
        &self,
        script: &str,
        code: &[Json],
        pos: &mut usize,
        inner: bool,
    ) -> Result<(Vec<Node>, Vec<Vec<Node>>), RunError> {
        let mut branches: Vec<Vec<Node>> = Vec::new();
        let mut current: Vec<Node> = Vec::new();
        while *pos < code.len() {
            let at = *pos;
            *pos += 1;
            let name = self.op_name(&code[at]).map(|o| o.0).unwrap_or("");
            match name {
                "try_begin" => {
                    let (last, mut more) = self.parse_until(script, code, pos, true)?;
                    more.push(last);
                    current.push(Node::Try(more));
                }
                n if n.starts_with("try_for_") => {
                    let (last, mut more) = self.parse_until(script, code, pos, true)?;
                    more.push(last);
                    current.push(Node::Loop(at, more));
                }
                "else_try" if inner => branches.push(std::mem::take(&mut current)),
                "try_end" if inner => return Ok((current, branches)),
                "try_end" | "else_try" => return Err(RunError::Unbalanced(format!("{script} at op {at} ({name})"))),
                _ => current.push(Node::Op(at)),
            }
        }
        if inner {
            return Err(RunError::Unbalanced(format!("{script}: block not closed")));
        }
        Ok((current, branches))
    }

    fn block(&mut self, code: &'a [Json], nodes: &[Node]) -> Result<Flow, RunError> {
        let mut or_ok = false; // an earlier `this_or_next` condition held
        for node in nodes {
            match node {
                Node::Op(at) => {
                    self.report.steps += 1;
                    if self.report.steps > self.step_limit {
                        return Err(RunError::StepLimit(self.step_limit));
                    }
                    self.current_op = *at;
                    let op = &code[*at];
                    let Some((name, neg, or_next, can_fail, lhs)) = self.op_name(op) else {
                        *self.report.unknown_ops.entry(format!("code {op}")).or_default() += 1;
                        continue;
                    };
                    let args: &[Json] = match op {
                        Json::Array(items) => &items[1..],
                        _ => &[],
                    };
                    let flow = self.exec(name, lhs, args)?;
                    let is_condition = can_fail || name == "call_script";
                    if !is_condition {
                        continue;
                    }
                    let mut held = flow == Flow::Done;
                    if neg {
                        held = !held;
                    }
                    if or_next {
                        or_ok |= held;
                        continue;
                    }
                    let passed = held || or_ok;
                    or_ok = false;
                    if !passed {
                        return Ok(Flow::Fail);
                    }
                }
                Node::Try(branches) => {
                    for branch in branches {
                        if self.block(code, branch)? == Flow::Done {
                            break;
                        }
                    }
                }
                Node::Loop(at, body) => {
                    self.current_op = *at;
                    self.run_loop(code, *at, body)?;
                }
            }
        }
        Ok(Flow::Done)
    }

    fn run_loop(&mut self, code: &'a [Json], at: usize, body: &[Vec<Node>]) -> Result<(), RunError> {
        let op = &code[at];
        let name = self.op_name(op).map(|o| o.0).unwrap_or("");
        let args: Vec<Json> = match op {
            Json::Array(items) => items[1..].to_vec(),
            _ => Vec::new(),
        };
        let dest = args.first().cloned().unwrap_or(Json::Null);
        let values: Vec<i64> = match name {
            "try_for_range" | "try_for_range_backwards" => {
                let lo = args.get(1).map(|a| self.get(a)).unwrap_or_default();
                let hi = args.get(2).map(|a| self.get(a)).unwrap_or_default();
                if lo.unknown || hi.unknown {
                    *self.report.unknown_ops.entry(format!("{name} (unknown bounds)")).or_default() += 1;
                    return Ok(());
                }
                let range = lo.n..hi.n;
                if name == "try_for_range" { range.collect() } else { range.rev().collect() }
            }
            "try_for_parties" => (0..self.party_count).collect(),
            // No agents, players or scene props exist before the first mission.
            "try_for_agents" | "try_for_players" | "try_for_prop_instances" => Vec::new(),
            other => {
                *self.report.unknown_ops.entry(other.to_owned()).or_default() += 1;
                Vec::new()
            }
        };
        for v in values {
            self.set(&dest, Val::num(v));
            for branch in body {
                if self.block(code, branch)? == Flow::Done {
                    break;
                }
            }
        }
        Ok(())
    }

    fn exec(&mut self, name: &'a str, lhs: bool, args: &[Json]) -> Result<Flow, RunError> {
        let arg = |i: usize| args.get(i).cloned().unwrap_or(Json::Null);
        // Script calls and parameters.
        match name {
            "call_script" => {
                let target = arg(0);
                let script = target.as_str().map(|s| s.strip_prefix("script_").unwrap_or(s).to_owned());
                let Some(script) = script else {
                    self.unknown_op(name, false, args);
                    return Ok(Flow::Done);
                };
                let params = args[1.min(args.len())..].iter().map(|a| self.get(a)).collect();
                return self.call(&script, params);
            }
            "store_script_param_1" | "store_script_param_2" | "store_script_param" => {
                let index = match name {
                    "store_script_param_1" => 1,
                    "store_script_param_2" => 2,
                    _ => self.get(&arg(1)).n,
                };
                let value = self
                    .frame()
                    .and_then(|f| f.params.get((index - 1).max(0) as usize))
                    .cloned()
                    .unwrap_or_default();
                self.set(&arg(0), value);
                return Ok(Flow::Done);
            }
            _ => {}
        }
        if let Some(flow) = self.core(name, args) {
            return Ok(flow);
        }
        if let Some(flow) = self.slots(name, args) {
            return Ok(flow);
        }
        if let Some(flow) = families::exec(self, name, args) {
            return Ok(flow);
        }
        self.unknown_op(name, lhs, args);
        Ok(Flow::Done)
    }

    /// Assignment, arithmetic and comparisons.
    fn core(&mut self, name: &str, args: &[Json]) -> Option<Flow> {
        let v = |m: &Self, i: usize| args.get(i).map(|a| m.get(a)).unwrap_or_default();
        let dest = args.first().cloned().unwrap_or(Json::Null);
        let cond = |held: bool| Some(if held { Flow::Done } else { Flow::Fail });
        match name {
            "assign" => {
                let x = v(self, 1);
                self.set(&dest, x);
                Some(Flow::Done)
            }
            "store_add" | "store_sub" | "store_mul" | "store_div" | "store_mod" => {
                let (a, b) = (v(self, 1), v(self, 2));
                let n = arith(&name[6..], a.n, b.n);
                self.set(&dest, Val::derived(n, &[&a, &b]));
                Some(Flow::Done)
            }
            "val_add" | "val_sub" | "val_mul" | "val_div" | "val_mod" | "val_min" | "val_max" => {
                let (a, b) = (v(self, 0), v(self, 1));
                let n = arith(&name[4..], a.n, b.n);
                self.set(&dest, Val::derived(n, &[&a, &b]));
                Some(Flow::Done)
            }
            "val_abs" => {
                let a = v(self, 0);
                self.set(&dest, Val::derived(a.n.abs(), &[&a]));
                Some(Flow::Done)
            }
            "val_clamp" => {
                // val_clamp: lower bound inclusive, upper bound exclusive (header_operations).
                let (a, lo, hi) = (v(self, 0), v(self, 1), v(self, 2));
                let n = a.n.max(lo.n).min(hi.n - 1);
                self.set(&dest, Val::derived(n, &[&a, &lo, &hi]));
                Some(Flow::Done)
            }
            "store_random_in_range" => {
                let (lo, hi) = (v(self, 1), v(self, 2));
                let n = self.random(lo.n, hi.n);
                let mut out = Val::derived(n, &[&lo, &hi]);
                out.random = true;
                self.set(&dest, out);
                Some(Flow::Done)
            }
            "store_random" => {
                let hi = v(self, 1);
                let n = self.random(0, hi.n);
                let mut out = Val::derived(n, &[&hi]);
                out.random = true;
                self.set(&dest, out);
                Some(Flow::Done)
            }
            "eq" => cond(v(self, 0).n == v(self, 1).n),
            "neq" => cond(v(self, 0).n != v(self, 1).n),
            "gt" => cond(v(self, 0).n > v(self, 1).n),
            "ge" => cond(v(self, 0).n >= v(self, 1).n),
            "lt" => cond(v(self, 0).n < v(self, 1).n),
            "le" => cond(v(self, 0).n <= v(self, 1).n),
            "is_between" => {
                let (x, lo, hi) = (v(self, 0).n, v(self, 1).n, v(self, 2).n);
                cond(lo <= x && x < hi)
            }
            _ => None,
        }
    }

    /// `<kind>_set_slot`, `<kind>_get_slot`, `<kind>_slot_eq`, `<kind>_slot_ge` for record kinds.
    fn slots(&mut self, name: &str, args: &[Json]) -> Option<Flow> {
        let (kind, table) = SLOT_KINDS.iter().find(|(kind, _)| {
            name.strip_prefix(kind).is_some_and(|rest| rest.starts_with("_set_slot") || rest.starts_with("_get_slot") || rest.starts_with("_slot_"))
        })?;
        let rest = &name[kind.len()..];
        let v = |m: &Self, i: usize| args.get(i).map(|a| m.get(a)).unwrap_or_default();
        match rest {
            "_set_slot" => {
                let (rec, slot, value) = (v(self, 0), v(self, 1), v(self, 2));
                if rec.unknown || slot.unknown {
                    self.unknown_op(&format!("{name} (unknown target)"), false, args);
                    return Some(Flow::Done);
                }
                let value = Val::derived(value.n, &[&value, &rec, &slot]);
                self.write(Cell::slot(table, rec.n, slot.n), value);
                Some(Flow::Done)
            }
            "_get_slot" => {
                let (rec, slot) = (v(self, 1), v(self, 2));
                let got = if rec.unknown || slot.unknown {
                    Val::unknown()
                } else {
                    let read = self.read(Cell::slot(table, rec.n, slot.n));
                    Val::derived(read.n, &[&read, &rec, &slot])
                };
                let dest = args.first().cloned().unwrap_or(Json::Null);
                self.set(&dest, got);
                Some(Flow::Done)
            }
            "_slot_eq" | "_slot_ge" => {
                let (rec, slot, want) = (v(self, 0), v(self, 1), v(self, 2));
                let got = self.read(Cell::slot(table, rec.n, slot.n));
                let held = if rest == "_slot_eq" { got.n == want.n } else { got.n >= want.n };
                Some(if held { Flow::Done } else { Flow::Fail })
            }
            _ => None,
        }
    }
}

fn arith(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "add" => a.wrapping_add(b),
        "sub" => a.wrapping_sub(b),
        "mul" => a.wrapping_mul(b),
        "div" => if b == 0 { 0 } else { a / b },
        "mod" => if b == 0 { 0 } else { a % b },
        "min" => a.min(b),
        "max" => a.max(b),
        _ => 0,
    }
}

/// The operation list of a script entry (`["game_start", {"list": [...]}]`).
fn script_block(entry: &Entry) -> Option<&[Json]> {
    let items = entry.value.as_array()?;
    let block = items.get(1)?;
    let list = block.get("list").unwrap_or(block).as_array()?;
    Some(list.as_slice())
}
