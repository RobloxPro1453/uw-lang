//! UW core language v0.
//!
//! Pipeline: `parse` (source → [`ast::File`]) → `lower` (files → [`Lowered`]: Native operations and the
//! generated infrastructure) → the build writes [`Lowered`] into the kit's module files.
//!
//! Rule of the crate: no feature-specific code. A general verb or property is data in
//! `kit-meta/core/*.toml` or the generated easy vocabulary `kit-meta/easy/*.json`; the compiler only knows
//! the language's fixed constructs.

pub mod ast;
pub mod lex;
pub mod lower;
pub mod parse;
pub mod slots;
pub mod view;

pub use parse::parse;

/// One Native operation, written the way the kit's Python writes it (`(agent_set_speed_modifier, ":a", 150)`).
#[derive(Debug, Clone, PartialEq)]
pub struct Op {
    /// Operation name from header_operations.py, with `neg|` / `this_or_next|` written as flags below.
    pub name: String,
    pub args: Vec<Arg>,
    pub neg: bool,
    pub this_or_next: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Int(i64),
    /// `":name"` script/trigger local.
    Local(String),
    /// `"$name"` global variable.
    Global(String),
    /// `reg0` … register.
    Reg(u8),
    /// A kit reference or constant written by name (`"p_town_6"`, `"trp_player"`, `key_left_alt`,
    /// `slot_town_lord`, `"script_uw_mission_frame"`). Never invented: resolved from the atlas.
    Name(String),
    /// Quick string (`"@Stamina"`) or string register text.
    Text(String),
}

/// A trigger added to mission templates.
#[derive(Debug, Clone, PartialEq)]
pub struct MissionTrigger {
    /// Interval (`0` = every frame) or a `ti_*` event name such as `ti_on_agent_spawn`.
    pub when: TriggerWhen,
    pub conditions: Vec<Op>,
    pub body: Vec<Op>,
    /// Which mission templates receive it.
    pub target: MissionTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriggerWhen {
    Interval(f64),
    Event(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MissionTarget {
    /// Every mission template whose groups include the player (the build decides from the atlas).
    WithPlayer,
    Named(Vec<String>),
}

/// A simple trigger added to `module_simple_triggers` (interval in game hours).
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleTrigger {
    pub interval: f64,
    pub body: Vec<Op>,
}

/// How a scheduled block runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleKind {
    /// `every`: again and again, `period` clock ticks apart (hours or milliseconds).
    Every { period: i64 },
    /// `after`: once, when due.
    After,
    /// `while`: every frame of a mission.
    Continuous,
}

/// One block a hidden dispatcher runs, for the cost report ([`lower::time::report`]).
#[derive(Debug, Clone, PartialEq)]
pub struct Scheduled {
    pub file: String,
    pub line: u32,
    pub clock: ast::Clock,
    /// `every 6 hours`, `after 3 days as truce_end`, `while …`.
    pub what: String,
    pub kind: ScheduleKind,
    /// First run offset in clock ticks (`every` only).
    pub phase: i64,
    /// Operations the block runs when it is due (counted statically, loops once).
    pub ops: usize,
}

/// A presentation the build adds (`prsnt_<name>`).
#[derive(Debug, Clone, PartialEq)]
pub struct Presentation {
    pub name: String,
    /// Flag names, e.g. `prsntf_read_only`.
    pub flags: Vec<String>,
    pub mesh: Option<String>,
    pub on_load: Vec<Op>,
    pub on_run: Vec<Op>,
}

/// Everything a set of UW core files turns into.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lowered {
    /// Operations appended to the end of an existing Native script (`game_start`).
    pub script_appends: Vec<(String, Vec<Op>)>,
    /// Operations appended to the enter block of an existing Native menu (`on menu.start_game_0.enter`), per
    /// menu name without its `mnu_` tag, in source order.
    pub menu_appends: Vec<(String, Vec<Op>)>,
    /// New scripts (`uw_mission_frame`, user `fn`s as `uw_<feature>_<name>`).
    pub scripts: Vec<(String, Vec<Op>)>,
    pub mission_triggers: Vec<MissionTrigger>,
    /// Hidden campaign triggers (the game-clock dispatcher: at most one).
    pub simple_triggers: Vec<SimpleTrigger>,
    /// Every block the dispatchers run, for the cost report.
    pub schedule: Vec<Scheduled>,
    pub presentations: Vec<Presentation>,
    /// Native menus/scripts/triggers replaced or disabled (`disable(native.character_creation)`),
    /// as (table, id, new ops or None to empty it).
    pub overrides: Vec<(String, String, Option<Vec<Op>>)>,
    /// The slot lock after this build: the one given in plus every new field (write it back).
    pub slot_lock: slots::SlotLock,
    /// Things that build but deserve a look (a field still in `slots.lock` but no longer declared).
    pub warnings: Vec<Error>,
}

/// A message with its place (`features/sprint.uw:3: …`).
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub file: String,
    pub line: u32,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}
