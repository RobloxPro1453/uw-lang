//! D2 `every` / `after` / `cancel` / `cooldown`, the two hidden dispatchers and the
//! tick cost report (CORE-v0 §4 "Time", §5.1). Sources go through the real parser.

use uw_core::ast::Clock;
use uw_core::lower::time::{report, CLOCK_MS, FRAME_MS, GAME_CLOCK, MISSION_FRAME};
use uw_core::{Arg, Lowered, Op, ScheduleKind, TriggerWhen};

mod common;
use common::{lower_err, lower_ok};

fn script<'a>(lowered: &'a Lowered, name: &str) -> &'a [Op] {
    let found: Vec<_> = lowered.scripts.iter().filter(|(script, _)| script == name).collect();
    assert_eq!(found.len(), 1, "script {name} once in {:?}", lowered.scripts);
    &found[0].1
}

fn has(ops: &[Op], name: &str, args: &[Arg]) -> bool {
    ops.iter().any(|op| op.name == name && op.args == args)
}

fn global(name: &str) -> Arg {
    Arg::Global(name.to_owned())
}

fn local(name: &str) -> Arg {
    Arg::Local(name.to_owned())
}

#[test]
fn hidden_trigger_count_does_not_grow_with_blocks() {
    let one = lower_ok(&[("a", "var n = 0\nevery 6 hours {\n  n += 1\n}\n")]);
    let many = lower_ok(&[
        (
            "a",
            "var n = 0\nevery 6 hours {\n  n += 1\n}\nevery 1 day {\n  n += 2\n}\nevery 1 week {\n  n = 0\n}\n\
             while in mission {\n  n += 1\n}\nevery 0.5 seconds {\n  n -= 1\n}\n",
        ),
        ("b", "var m = 0\nevery 6 hours {\n  m += 1\n}\nevery 2 minutes {\n  m = 1\n}\nwhile m > 3 {\n  m = 0\n}\n"),
    ]);
    assert_eq!(one.simple_triggers.len(), 1);
    assert!(one.mission_triggers.is_empty(), "no mission block, no mission trigger");
    assert_eq!(many.simple_triggers.len(), 1, "one game-clock trigger for every game block");
    assert_eq!(many.mission_triggers.len(), 2, "one frame trigger + one mission-start trigger");
    assert!(matches!(many.mission_triggers[1].when, TriggerWhen::Event(ref e) if e == "ti_before_mission_start"));
    assert_eq!(many.schedule.len(), 8);
    script(&many, GAME_CLOCK);
    script(&many, MISSION_FRAME);
    let call = &many.simple_triggers[0].body[0];
    assert_eq!(call.name, "call_script");
    assert_eq!(call.args, vec![Arg::Name(format!("script_{GAME_CLOCK}"))]);
}

#[test]
fn no_time_no_dispatcher() {
    let lowered = lower_ok(&[("a", "var n = 1\nfn f() {\n  n = 2\n}\n")]);
    assert!(lowered.simple_triggers.is_empty() && lowered.mission_triggers.is_empty());
    assert!(lowered.schedule.is_empty());
    assert_eq!(report(&lowered), "No timed or continuous blocks: no hidden triggers.\n");
}

#[test]
fn same_cadence_blocks_are_spread_over_the_period() {
    let lowered = lower_ok(&[
        ("a", "var n = 0\nevery 1 day {\n  n += 1\n}\nevery 1 day {\n  n += 2\n}\n"),
        ("b", "var m = 0\nevery 24 hours {\n  m += 1\n}\nevery 6 hours {\n  m += 3\n}\n"),
    ]);
    let phases: Vec<(i64, i64)> = lowered
        .schedule
        .iter()
        .map(|s| match s.kind {
            ScheduleKind::Every { period } => (period, s.phase),
            other => panic!("unexpected {other:?}"),
        })
        .collect();
    // The three daily blocks (two features) get hours 0, 8, 16; the lone 6-hour block keeps 0.
    assert_eq!(phases, vec![(24, 0), (24, 8), (24, 16), (6, 0)]);
    let game = script(&lowered, GAME_CLOCK);
    let now = local(":uw_now");
    assert_eq!(game[0], Op { name: "store_current_hours".into(), args: vec![now.clone()], neg: false, this_or_next: false });
    assert!(has(game, "store_add", &[global("$uw_a_every1"), now.clone(), Arg::Int(8)]), "phase on first run");
    assert!(has(game, "store_add", &[global("$uw_b_every0"), now.clone(), Arg::Int(16)]));
    assert!(has(game, "ge", &[now.clone(), global("$uw_b_every1")]));
    assert!(has(game, "store_add", &[global("$uw_b_every1"), now, Arg::Int(6)]), "next run one period on");

    let text = report(&lowered);
    assert!(text.contains("Game clock: 1 hidden trigger every game hour"), "{text}");
    // Each daily body is 7 wrapper ops + 1 body op + try_end = 9 ops; the 6-hour one too. On hour 0 the
    // first daily and the 6-hour block are due: 2 + 9 + 9 + 6 + 6 = 32. Without the spreading all three
    // daily blocks would fall on hour 0 (2 + 4 × 9 = 38).
    assert!(text.contains("busiest hour: 32 ops (hour 0 of a 24-hour cycle)"), "{text}");
    assert!(text.contains("quietest hour: 26 ops"), "{text}");
}

#[test]
fn bad_durations_are_explained() {
    let msg = lower_err(&[("a", "every 1.5 hours {\n}\n")]);
    assert!(msg.contains("game time moves in whole hours"), "{msg}");
    let msg = lower_err(&[("a", "every 5 {\n}\n")]);
    assert!(msg.contains("needs a duration such as `6 hours`"), "{msg}");
    // A constant holding a duration is fine.
    let lowered = lower_ok(&[("a", "const TICK = 12 hours\nevery TICK {\n}\n")]);
    assert_eq!(lowered.schedule[0].kind, ScheduleKind::Every { period: 12 });
    // A duration as a value is an amount of its clock: seconds (thousandths) or hours.
    let lowered = lower_ok(&[("a", "var n = 0\nfn f() {\n  n = 5 seconds\n  n = 1 day\n}\n")]);
    let f = script(&lowered, "uw_a_f");
    assert!(has(f, "assign", &[global("$uw_a_n"), Arg::Int(5000)]), "{f:?}");
    assert!(has(f, "assign", &[global("$uw_a_n"), Arg::Int(24_000)]), "1 day = 24 hours: {f:?}");
}

#[test]
fn after_schedules_and_cancel_clears() {
    let lowered = lower_ok(&[(
        "diplomacy",
        "var truce = 0\non game.start {\n  truce = 1\n  after 3 days as truce_end {\n    truce = 0\n  }\n}\n\
         fn break_truce() {\n  cancel truce_end\n}\n",
    )]);
    let due = global("$uw_diplomacy_after_truce_end");
    let start = &lowered.script_appends[0].1;
    let hours = start.iter().find(|op| op.name == "store_current_hours").expect("reads the hour");
    let now = hours.args[0].clone();
    assert!(has(start, "store_add", &[due.clone(), now, Arg::Int(72)]), "due in 72 hours: {start:?}");
    assert!(has(start, "val_max", &[due.clone(), Arg::Int(1)]));
    let cancel = script(&lowered, "uw_diplomacy_break_truce");
    assert!(has(cancel, "assign", &[due.clone(), Arg::Int(0)]));
    let game = script(&lowered, GAME_CLOCK);
    assert!(has(game, "gt", &[due.clone(), Arg::Int(0)]));
    assert!(has(game, "ge", &[local(":uw_now"), due.clone()]));
    let ran = game.iter().position(|op| op.name == "assign" && op.args == vec![due.clone(), Arg::Int(0)]).unwrap();
    assert!(
        game[ran..].iter().any(|op| op.name == "assign" && op.args == vec![global("$uw_diplomacy_truce"), Arg::Int(0)]),
        "the body runs after the due time is cleared"
    );
    assert_eq!(lowered.schedule[0].kind, ScheduleKind::After);
    assert_eq!(lowered.schedule[0].what, "after 3 days as truce_end");

    let msg = lower_err(&[("a", "fn f() {\n  cancel nothing\n}\n")]);
    assert!(msg.contains("this feature has no `after … as nothing`"), "{msg}");
    let msg = lower_err(&[("a", "fn f() {\n  after 1 hour as x {\n  }\n  after 2 hours as x {\n  }\n}\n")]);
    assert!(msg.contains("already used at line 2"), "{msg}");
    let msg = lower_err(&[("a", "while in mission {\n  after 2 seconds {\n  }\n}\n")]);
    assert!(msg.contains("would start again on every frame"), "{msg}");
    // A body cannot see the locals around it: it runs later, from the dispatcher.
    let msg = lower_err(&[("a", "var n = 0\nfn f() {\n  let x = 2\n  after 1 hour {\n    n = x\n  }\n}\n")]);
    assert!(msg.contains("unknown name 'x'"), "{msg}");
}

#[test]
fn mission_time_after_uses_the_saved_clock() {
    let lowered = lower_ok(&[("a", "var n = 0\nfn f() {\n  after 2.5 seconds {\n    n = 1\n  }\n}\n")]);
    let f = script(&lowered, "uw_a_f");
    assert!(has(f, "store_add", &[global("$uw_a_after0"), global(CLOCK_MS), Arg::Int(2500)]));
    let frame = script(&lowered, MISSION_FRAME);
    assert!(has(frame, "ge", &[global(CLOCK_MS), global("$uw_a_after0")]));
    assert!(has(frame, "val_add", &[global(CLOCK_MS), Arg::Local(FRAME_MS.to_owned())]), "a timed block moves the clock");
    assert_eq!(lowered.mission_triggers.len(), 2);
    assert_eq!(lowered.schedule[0].clock, Clock::Mission);
}

#[test]
fn mission_frame_measures_and_caps_the_frame_time() {
    let lowered = lower_ok(&[("a", "var n = 0\nwhile in mission {\n  n += 2 per second\n}\n")]);
    let frame = script(&lowered, MISSION_FRAME);
    let elapsed = local(FRAME_MS);
    let last = global("$uw_frame_last");
    assert_eq!(frame[0].name, "store_mission_timer_c_msec");
    assert!(has(frame, "ge", &[last.clone(), Arg::Int(0)]), "first frame of a mission counts 0");
    assert!(has(frame, "val_clamp", &[elapsed.clone(), Arg::Int(0), Arg::Int(1001)]), "a stall counts 1 s at most");
    assert!(!has(frame, "val_add", &[global(CLOCK_MS), elapsed.clone()]), "nothing reads the clock: it stays");
    // `n += 2 per second`: 2000 thousandths × elapsed ms ÷ 1000.
    let mul = frame.iter().position(|op| op.name == "store_mul" && op.args[1..] == [Arg::Int(2000), elapsed.clone()]);
    let mul = mul.expect("rate × frame time");
    assert_eq!(frame[mul + 1].name, "val_div");
    assert_eq!(frame[mul + 1].args[1], Arg::Int(1000));
    assert!(has(frame, "val_add", &[global("$uw_a_n"), frame[mul].args[0].clone()]));
    let reset = &lowered.mission_triggers[1].body;
    assert!(has(reset, "assign", &[last, Arg::Int(-1)]));

    let msg = lower_err(&[("a", "var n = 0\nevery 1 hour {\n  n += 2 per second\n}\n")]);
    assert!(msg.contains("only makes sense in a continuous block"), "{msg}");
    let msg = lower_err(&[("a", "while in battle {\n}\n")]);
    assert!(msg.contains("only `while in mission` exists"), "{msg}");
}

#[test]
fn alex_stamina_and_sprint_lower_into_one_frame() {
    let stamina = "var agent.stamina = 100 in 0..100\n\nwhile in mission {\n    player.agent.stamina += 15 per second\n}\n";
    let sprint = std::fs::read_to_string(format!("{}/tests/alex/features/sprint.uw", env!("CARGO_MANIFEST_DIR")))
        .expect("the Alex sprint feature");
    let lowered = lower_ok(&[("stamina", stamina), ("sprint", &sprint)]);
    let frame = script(&lowered, MISSION_FRAME);
    assert!(frame.iter().any(|op| op.name == "key_is_down"), "the sprint condition is in the frame");
    assert!(frame.iter().any(|op| op.name == "else_try"), "`otherwise` is the else branch");
    assert!(frame.iter().any(|op| op.name == "agent_set_speed_modifier"));
    // Every continuous block sits in its own try block: begins and ends balance.
    let begins = frame.iter().filter(|op| op.name == "try_begin").count();
    let ends = frame.iter().filter(|op| op.name == "try_end").count();
    assert_eq!(begins, ends);
    let whats: Vec<&str> = lowered.schedule.iter().map(|s| s.what.as_str()).collect();
    assert_eq!(whats, vec!["while in mission", "while … otherwise"]);
    let text = report(&lowered);
    assert!(text.contains("Mission clock: 1 hidden trigger every frame + 1 at mission start"), "{text}");
    assert!(text.contains("(every frame)"), "{text}");
}

#[test]
fn a_ready_time_is_written_with_the_clock() {
    // no `cooldown` word; the modder keeps a ready time.
    let lowered = lower_ok(&[(
        "dash",
        "var agent.dash_at = 0\nwhile in mission {\n  if mission.time >= player.agent.dash_at {\n    \
         player.agent.dash_at = mission.time + 5 seconds\n  }\n}\n",
    )]);
    let frame = script(&lowered, MISSION_FRAME);
    assert!(
        frame.iter().any(|op| op.name == "store_add" && op.args[2] == Arg::Int(5000)),
        "ready time = now + 5000 ms: {frame:?}"
    );
    assert!(frame.iter().any(|op| op.name == "assign" && op.args[1] == global(CLOCK_MS)), "mission.time reads the clock");
    assert!(frame.iter().any(|op| op.name == "val_add" && op.args[0] == global(CLOCK_MS)), "the clock moves");
    let msg = lower_err(&[("a", "var agent.dash cooldown\n")]);
    assert!(msg.contains("`cooldown` is not a word of the language"), "{msg}");
    assert!(msg.contains("mission.time + 2 seconds"), "the message shows the way: {msg}");
}

#[test]
fn time_words_stay_usable_as_names() {
    // `after`/`cancel` are statements only when no `=`, `(` or `.` follows.
    let lowered = lower_ok(&[("a", "var after = 0\nvar cancel = 0\nfn f() {\n  after = 3\n  cancel += 1\n}\n")]);
    let f = script(&lowered, "uw_a_f");
    assert!(has(f, "assign", &[global("$uw_a_after"), Arg::Int(3000)]));
    assert!(lowered.schedule.is_empty());
}
