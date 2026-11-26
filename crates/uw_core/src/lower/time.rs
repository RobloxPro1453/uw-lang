//! Time (CORE-v0 §4 "Time", §5.1): the two hidden dispatchers and the tick cost report.
//!
//! Every `every`/`after` block and every continuous block (`while`) of every feature ends up in one of two
//! generated scripts, so the number of hidden triggers never grows with the mod §3):
//!
//! - game clock: ONE simple trigger every game hour calls `script_uw_game_clock`, which reads
//!   `store_current_hours` once and runs each block that is due;
//! - mission clock: ONE mission trigger every frame calls `script_uw_mission_frame`, which measures the
//!   frame's milliseconds, adds them to the saved clock `$uw_clock_ms` and runs the due timed blocks and
//!   every continuous block; ONE `ti_before_mission_start` trigger marks the first frame of a mission.
//!
//! An `every` block keeps its next due time in a global (`$uw_<feature>_every<k>`, 0 = not scheduled
//! yet): the first run is placed at a phase so blocks of the same cadence do not all fall on one tick.
//! An `after` keeps its due time in a global as well (0 = nothing pending; `cancel` writes 0).

use std::collections::BTreeMap;

use crate::ast::Clock;
use crate::{Arg, Lowered, MissionTarget, MissionTrigger, Op, ScheduleKind, Scheduled, SimpleTrigger, TriggerWhen};

/// The game-clock dispatcher script (without the `script_` prefix).
pub const GAME_CLOCK: &str = "uw_game_clock";
/// The mission-clock dispatcher script.
pub const MISSION_FRAME: &str = "uw_mission_frame";
/// Milliseconds spent in missions since the game started; saved with the game, never reset.
pub const CLOCK_MS: &str = "$uw_clock_ms";
/// Mission timer at the previous frame; -1 on the first frame of a mission.
pub const FRAME_LAST: &str = "$uw_frame_last";
/// Milliseconds since the previous frame, inside `script_uw_mission_frame`.
pub const FRAME_MS: &str = ":uw_frame_ms";
/// Current hour, inside `script_uw_game_clock`.
const NOW_HOURS: &str = ":uw_now";
/// Mission timer this frame, inside `script_uw_mission_frame`.
const NOW_TIMER: &str = ":uw_timer";
/// A frame longer than this (a loading pause, a hitch) counts as this long, so a stall never fires a
/// burst of catch-up work.
pub const MAX_FRAME_MS: i64 = 1_000;

/// What a timed block is.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TimedKind {
    /// `every`: period in clock ticks (hours or milliseconds).
    Every { period: i64 },
    /// `after`: runs once when its due time comes.
    After,
}

/// One `every`/`after` block waiting for its dispatcher.
#[derive(Debug, Clone)]
pub(crate) struct Timed {
    pub clock: Clock,
    pub kind: TimedKind,
    /// The global holding the next due time.
    pub global: String,
    pub body: Vec<Op>,
    pub file: String,
    pub line: u32,
    /// `every 6 hours`, `after 3 days as truce_end`.
    pub what: String,
}

/// One continuous block (`while`) of the mission dispatcher, for the report.
#[derive(Debug, Clone)]
pub(crate) struct FrameItem {
    pub file: String,
    pub line: u32,
    pub what: String,
    pub ops: usize,
}

fn op(name: &str, args: Vec<Arg>) -> Op {
    Op { name: name.to_owned(), args, neg: false, this_or_next: false }
}

/// The first-run offset of each timed block: the `every` blocks of one clock and cadence are spread
/// evenly over the period (three `every 24 hours` → hours 0, 8, 16). `after` blocks get 0.
pub(crate) fn phases(timed: &[Timed]) -> Vec<i64> {
    let mut groups: BTreeMap<(Clock, i64), Vec<usize>> = BTreeMap::new();
    for (index, block) in timed.iter().enumerate() {
        if let TimedKind::Every { period } = block.kind {
            groups.entry((block.clock, period)).or_default().push(index);
        }
    }
    let mut phases = vec![0; timed.len()];
    for ((_, period), members) in groups {
        let count = members.len() as i64;
        for (position, index) in members.into_iter().enumerate() {
            phases[index] = position as i64 * period / count;
        }
    }
    phases
}

/// The dispatcher's operations for one timed block, reading the clock from `now`.
pub(crate) fn timed_ops(block: &Timed, phase: i64, now: &Arg) -> Vec<Op> {
    let due = Arg::Global(block.global.clone());
    let mut ops = Vec::new();
    match block.kind {
        TimedKind::Every { period } => {
            // Not scheduled yet (a new game, or an old save that did not have this block): first run at
            // now + phase.
            ops.push(op("try_begin", vec![]));
            ops.push(op("eq", vec![due.clone(), Arg::Int(0)]));
            ops.push(op("store_add", vec![due.clone(), now.clone(), Arg::Int(phase)]));
            ops.push(op("try_end", vec![]));
            ops.push(op("try_begin", vec![]));
            ops.push(op("ge", vec![now.clone(), due.clone()]));
            // Next run one period from now, not from the missed due time: a long pause never fires a
            // burst of catch-up runs.
            ops.push(op("store_add", vec![due, now.clone(), Arg::Int(period)]));
        }
        TimedKind::After => {
            ops.push(op("try_begin", vec![]));
            ops.push(op("gt", vec![due.clone(), Arg::Int(0)]));
            ops.push(op("ge", vec![now.clone(), due.clone()]));
            ops.push(op("assign", vec![due, Arg::Int(0)]));
        }
    }
    ops.extend(block.body.iter().cloned());
    ops.push(op("try_end", vec![]));
    ops
}

/// Writes the dispatchers into `lowered`: scripts, the hidden triggers and the schedule for the report.
pub(crate) fn build(lowered: &mut Lowered, timed: &[Timed], frame: Vec<Op>, frame_items: &[FrameItem], mission_clock: bool) {
    let phases = phases(timed);
    let game_now = Arg::Local(NOW_HOURS.to_owned());
    let mission_now = Arg::Global(CLOCK_MS.to_owned());

    // Game clock.
    let mut game = vec![op("store_current_hours", vec![game_now.clone()])];
    for (block, phase) in timed.iter().zip(&phases).filter(|(block, _)| block.clock == Clock::Game) {
        let ops = timed_ops(block, *phase, &game_now);
        lowered.schedule.push(scheduled(block, *phase, ops.len()));
        game.extend(ops);
    }
    if game.len() > 1 {
        lowered.scripts.push((GAME_CLOCK.to_owned(), game));
        lowered.simple_triggers.push(SimpleTrigger {
            interval: 1.0,
            body: vec![op("call_script", vec![Arg::Name(format!("script_{GAME_CLOCK}"))])],
        });
    }

    // Mission clock.
    let elapsed = Arg::Local(FRAME_MS.to_owned());
    let timer = Arg::Local(NOW_TIMER.to_owned());
    let last = Arg::Global(FRAME_LAST.to_owned());
    let mut mission = vec![
        op("store_mission_timer_c_msec", vec![timer.clone()]),
        op("assign", vec![elapsed.clone(), Arg::Int(0)]),
        op("try_begin", vec![]),
        op("ge", vec![last.clone(), Arg::Int(0)]),
        op("store_sub", vec![elapsed.clone(), timer.clone(), last.clone()]),
        // `val_clamp`'s upper bound is exclusive (header_operations).
        op("val_clamp", vec![elapsed.clone(), Arg::Int(0), Arg::Int(MAX_FRAME_MS + 1)]),
        op("try_end", vec![]),
        op("assign", vec![last.clone(), timer]),
    ];
    // The saved clock moves only when something reads it (a timed block or a cooldown in seconds).
    if mission_clock || timed.iter().any(|block| block.clock == Clock::Mission) {
        mission.push(op("val_add", vec![mission_now.clone(), elapsed]));
    }
    let header = mission.len();
    for (block, phase) in timed.iter().zip(&phases).filter(|(block, _)| block.clock == Clock::Mission) {
        let ops = timed_ops(block, *phase, &mission_now);
        lowered.schedule.push(scheduled(block, *phase, ops.len()));
        mission.extend(ops);
    }
    for item in frame_items {
        lowered.schedule.push(Scheduled {
            file: item.file.clone(),
            line: item.line,
            clock: Clock::Mission,
            what: item.what.clone(),
            kind: ScheduleKind::Continuous,
            phase: 0,
            ops: item.ops,
        });
    }
    mission.extend(frame);
    if mission.len() > header || mission_clock {
        lowered.scripts.push((MISSION_FRAME.to_owned(), mission));
        lowered.mission_triggers.push(MissionTrigger {
            when: TriggerWhen::Interval(0.0),
            conditions: Vec::new(),
            body: vec![op("call_script", vec![Arg::Name(format!("script_{MISSION_FRAME}"))])],
            target: MissionTarget::WithPlayer,
        });
        lowered.mission_triggers.push(MissionTrigger {
            when: TriggerWhen::Event("ti_before_mission_start".to_owned()),
            conditions: Vec::new(),
            body: vec![op("assign", vec![last, Arg::Int(-1)])],
            target: MissionTarget::WithPlayer,
        });
    }
}

fn scheduled(block: &Timed, phase: i64, ops: usize) -> Scheduled {
    Scheduled {
        file: block.file.clone(),
        line: block.line,
        clock: block.clock,
        what: block.what.clone(),
        kind: match block.kind {
            TimedKind::Every { period } => ScheduleKind::Every { period },
            TimedKind::After => ScheduleKind::After,
        },
        phase,
        ops,
    }
}

/// Operations a block costs on a tick where it is not due (the checks before its body).
fn idle_cost(entry: &Scheduled) -> usize {
    match entry.kind {
        ScheduleKind::Every { .. } => 6, // try_begin eq try_end try_begin ge (fails) + the skipped try_end
        ScheduleKind::After => 3,        // try_begin gt/ge (fails) + the skipped try_end
        ScheduleKind::Continuous => entry.ops,
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// The longest cycle the report walks through (one year of game hours).
const REPORT_CYCLE_CAP: i64 = 24 * 365;

/// The cost report §3): how many operations each scheduled block runs, and how
/// heavy the busiest tick of each clock gets. Counted statically: a loop's body counts once.
pub fn report(lowered: &Lowered) -> String {
    let mut out = String::new();
    let game: Vec<&Scheduled> = lowered.schedule.iter().filter(|s| s.clock == Clock::Game).collect();
    let mission: Vec<&Scheduled> = lowered.schedule.iter().filter(|s| s.clock == Clock::Mission).collect();
    if game.is_empty() && mission.is_empty() {
        return "No timed or continuous blocks: no hidden triggers.\n".to_owned();
    }
    if !game.is_empty() {
        out.push_str(&format!("Game clock: 1 hidden trigger every game hour (script_{GAME_CLOCK})\n"));
        for entry in &game {
            let when = match entry.kind {
                ScheduleKind::Every { .. } => format!("first run at hour +{}", entry.phase),
                _ => "once, when due".to_owned(),
            };
            out.push_str(&format!("  {}:{}  {}  ({when})  {} ops\n", entry.file, entry.line, entry.what, entry.ops));
        }
        let cycle = game
            .iter()
            .filter_map(|entry| match entry.kind {
                ScheduleKind::Every { period } => Some(period),
                _ => None,
            })
            .fold(1i64, |acc, period| (acc / gcd(acc, period)).saturating_mul(period))
            .min(REPORT_CYCLE_CAP);
        let mut busiest = (0usize, 0i64);
        let mut quietest = usize::MAX;
        for hour in 0..cycle {
            let mut cost = 2; // the trigger's call + store_current_hours
            for entry in &game {
                cost += match entry.kind {
                    ScheduleKind::Every { period } if (hour - entry.phase).rem_euclid(period) == 0 => entry.ops,
                    _ => idle_cost(entry),
                };
            }
            if cost > busiest.0 {
                busiest = (cost, hour);
            }
            quietest = quietest.min(cost);
        }
        out.push_str(&format!(
            "  busiest hour: {} ops (hour {} of a {cycle}-hour cycle); quietest hour: {quietest} ops\n",
            busiest.0, busiest.1
        ));
        if game.iter().any(|entry| entry.kind == ScheduleKind::After) {
            out.push_str("  an `after` adds its full cost on the hour it comes due\n");
        }
    }
    if !mission.is_empty() {
        out.push_str(&format!(
            "Mission clock: 1 hidden trigger every frame + 1 at mission start (script_{MISSION_FRAME})\n"
        ));
        let mut every_frame = 11; // the trigger's call + the frame-time header
        let mut worst = 11;
        for entry in &mission {
            let when = match entry.kind {
                ScheduleKind::Continuous => "every frame".to_owned(),
                ScheduleKind::After => "once, when due".to_owned(),
                ScheduleKind::Every { .. } => format!("first run at +{} ms", entry.phase),
            };
            out.push_str(&format!("  {}:{}  {}  ({when})  {} ops\n", entry.file, entry.line, entry.what, entry.ops));
            every_frame += idle_cost(entry);
            worst += entry.ops;
        }
        out.push_str(&format!(
            "  a usual frame: {every_frame} ops; worst frame (every timed block due at once): {worst} ops\n"
        ));
    }
    out.push_str("(counted statically: a loop's body counts as if it ran once)\n");
    out
}
