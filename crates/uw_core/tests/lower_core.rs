//! Lowering of `uw_core` on hand-built ASTs (the parser is T-132's job), one test per CORE-v0 §4 construct.
//!
//! The atlas here is a tiny inline one: the tests must not depend on a kit dump. The property vocabulary
//! is the committed `kit-meta/core/properties.toml`, read through [`CoreData::builtin`].

use std::fs;

use uw_atlas::Atlas;
use uw_core::ast::{
    AssignOp, Block, Call, Expr, File, Ident, Item, Path, Span, Stmt, TextPart, VarDecl,
};
use uw_core::lower::{CoreData, lower};
use uw_core::{Arg, Lowered, Op};

/// A minimal atlas: the ops the lowering can reach and the few constants the tests mention.
const ATLAS: &str = r#"{
  "schema": 1,
  "ops": [
    {"name": "assign", "code": 1, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "val_add", "code": 2, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "val_sub", "code": 3, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "val_mul", "code": 4, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "val_div", "code": 5, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "val_clamp", "code": 6, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "store_add", "code": 7, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "store_sub", "code": 8, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "store_mul", "code": 9, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "store_div", "code": 10, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": true},
    {"name": "eq", "code": 11, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "neq", "code": 12, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "lt", "code": 13, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "le", "code": 14, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "gt", "code": 15, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "ge", "code": 16, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "try_begin", "code": 17, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "else_try", "code": 18, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "try_end", "code": 19, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "try_for_range", "code": 20, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "store_script_param", "code": 21, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": false},
    {"name": "get_player_agent_no", "code": 22, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": false},
    {"name": "agent_get_slot", "code": 23, "line": null, "signature": "", "lhs": true, "can_fail": false, "global_lhs": false},
    {"name": "agent_set_slot", "code": 24, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "agent_set_speed_modifier", "code": 25, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false},
    {"name": "key_is_down", "code": 26, "line": null, "signature": "", "lhs": false, "can_fail": true, "global_lhs": false},
    {"name": "troop_set_name", "code": 27, "line": null, "signature": "", "lhs": false, "can_fail": false, "global_lhs": false}
  ],
  "hooks": ["game_start"],
  "ranges": [],
  "constants": [
    {"name": "slot_agent_health", "value": 3, "module": "module_constants.py", "line": null},
    {"name": "slot_agent_speed", "value": 4, "module": "module_constants.py", "line": null},
    {"name": "slot_agent_morale", "value": 5, "module": "module_constants.py", "line": null},
    {"name": "trp_player", "value": 0, "module": "module_troops.py", "line": null},
    {"name": "key_left_alt", "value": 56, "module": "module_constants.py", "line": null}
  ],
  "tables": {
    "troops": {
      "module": "module_troops.py", "var": "troops", "id_file": null, "id_prefix": "trp", "ref_tag": "trp",
      "entries": [{"index": 0, "name": "player", "id": "trp_player", "dup": null, "line": null, "value": "trp_player"}]
    }
  }
}"#;

fn atlas() -> Atlas {
    Atlas::from_str(ATLAS).expect("the inline atlas is valid")
}

fn data() -> CoreData {
    CoreData::builtin().expect("the committed properties.toml is valid")
}

/// Lowers ASTs that are expected to be correct.
fn lower_all(files: &[File]) -> Lowered {
    match lower(files, &atlas(), &data()) {
        Ok(lowered) => lowered,
        Err(errors) => panic!("lowering failed: {errors:?}"),
    }
}

/// The operations appended to `game_start`.
fn game_start(lowered: &Lowered) -> &[Op] {
    lowered
        .script_appends
        .iter()
        .find(|(script, _)| script == "game_start")
        .map(|(_, ops)| ops.as_slice())
        .unwrap_or_else(|| panic!("no game_start append in {lowered:?}"))
}

fn find<'a>(ops: &'a [Op], name: &str) -> &'a Op {
    ops.iter()
        .find(|op| op.name == name)
        .unwrap_or_else(|| panic!("no '{name}' in {ops:?}"))
}

// ---- builders (no parser yet: T-132 writes it) --------------------------------------------------

fn span(line: u32) -> Span {
    Span { start: 0, end: 0, line }
}

fn ident(text: &str, line: u32) -> Ident {
    Ident { text: text.to_owned(), span: span(line) }
}

fn path(parts: &[&str], line: u32) -> Path {
    Path { parts: parts.iter().map(|part| ident(part, line)).collect(), span: span(line) }
}

/// A source number in thousandths (`5` → 5000).
fn milli(value: i64, line: u32) -> Expr {
    Expr::Number { milli: value, span: span(line) }
}

fn text(value: &str, line: u32) -> Expr {
    Expr::Text { parts: vec![TextPart::Literal(value.to_owned())], span: span(line) }
}

fn file(feature: &str, items: Vec<Item>) -> File {
    File { feature: feature.to_owned(), items }
}

fn var(name: &str, init: Option<Expr>, range: Option<(i64, i64)>, line: u32) -> Item {
    field(None, name, init, range, line)
}

fn field(kind: Option<&str>, name: &str, init: Option<Expr>, range: Option<(i64, i64)>, line: u32) -> Item {
    Item::Var(VarDecl {
        kind: kind.map(|kind| ident(kind, line)),
        name: ident(name, line),
        init,
        range: range.map(|(lo, hi)| (milli(lo, line), milli(hi, line))),
        span: span(line),
    })
}

fn assign(place: &[&str], op: AssignOp, value: Expr, line: u32) -> Stmt {
    Stmt::Assign { place: path(place, line), op, value, span: span(line) }
}

fn on_start(body: Block) -> Item {
    Item::On {
        event: path(&["game", "start"], 1),
        bound: None,
        when: None,
        body,
        span: span(1),
    }
}

// ---- (a) feature variables at game start --------------------------------------------------------

#[test]
fn a_feature_variable_starts_in_game_start_and_adds() {
    // `var x = 5` + `on game.start { x += 2 }`
    let files = vec![file(
        "main",
        vec![
            var("x", Some(milli(5000, 2)), None, 2),
            on_start(vec![assign(&["x"], AssignOp::Add, milli(2000, 3), 3)]),
        ],
    )];
    let lowered = lower_all(&files);
    let ops = game_start(&lowered);

    let start = find(ops, "assign");
    assert_eq!(
        start.args,
        vec![Arg::Global("$uw_main_x".to_owned()), Arg::Int(5000)],
        "`var x = 5` gives the global its start value at game start"
    );
    let add = find(ops, "val_add");
    assert_eq!(add.args, vec![Arg::Global("$uw_main_x".to_owned()), Arg::Int(2000)]);
    assert!(lowered.scripts.is_empty(), "a game.start body is not a new script");
}

// ---- (b) object fields: a kit slot per kind, and every write clamps ------------------------------

#[test]
fn an_agent_field_gets_a_slot_above_the_kit_slots_and_clamps_its_writes() {
    // `var agent.stamina = 100 in 0..100` + `on game.start { player.agent.stamina += 2 }`
    let files = vec![file(
        "main",
        vec![
            field(Some("agent"), "stamina", Some(milli(100_000, 2)), Some((0, 100_000)), 2),
            on_start(vec![assign(
                &["player", "agent", "stamina"],
                AssignOp::Add,
                milli(2000, 3),
                3,
            )]),
        ],
    )];
    let lowered = lower_all(&files);
    let atlas = atlas();
    let ops = game_start(&lowered);

    let set = find(ops, "agent_set_slot");
    let Arg::Int(slot) = set.args[1] else {
        panic!("the field slot must be a number: {:?}", set.args);
    };
    let highest = atlas
        .constants
        .iter()
        .filter(|constant| constant.name.starts_with("slot_agent_"))
        .filter_map(|constant| constant.value.as_i64())
        .max()
        .expect("the atlas has slot_agent_* constants");
    assert!(slot > highest, "slot {slot} must be above every slot_agent_* ({highest})");

    let clamp = find(ops, "val_clamp");
    assert_eq!(
        clamp.args[1..],
        [Arg::Int(0), Arg::Int(100_001)],
        "`in 0..100` clamps to 0..=100; val_clamp's upper bound is exclusive"
    );

    // The object of the write is the player's agent, fetched once in front of the read.
    let agent = find(ops, "get_player_agent_no");
    assert_eq!(set.args[0], agent.args[0], "the slot write uses the agent the object gave");
    let read = find(ops, "agent_get_slot");
    assert_eq!(read.args[1], agent.args[0]);
    assert_eq!(read.args[2], Arg::Int(slot));
}

// ---- (c) property writes and reads come from the vocabulary -------------------------------------

#[test]
fn a_player_name_write_comes_from_the_vocabulary() {
    let files = vec![file(
        "main",
        vec![on_start(vec![assign(&["player", "name"], AssignOp::Set, text("Alex", 3), 3)])],
    )];
    let lowered = lower_all(&files);
    let ops = game_start(&lowered);

    assert_eq!(ops.len(), 1, "{ops:?}");
    assert_eq!(ops[0].name, "troop_set_name");
    assert_eq!(ops[0].args, vec![Arg::Name("trp_player".to_owned()), Arg::Text("@Alex".to_owned())]);
}

#[test]
fn a_speed_write_passes_the_object_and_the_percent() {
    // `player.agent.speed = 150%`: the object is `player.agent`, the value a percent (as written).
    let files = vec![file(
        "main",
        vec![on_start(vec![assign(
            &["player", "agent", "speed"],
            AssignOp::Set,
            Expr::Percent { value: 150, span: span(3) },
            3,
        )])],
    )];
    let lowered = lower_all(&files);
    let ops = game_start(&lowered);

    assert_eq!(ops.len(), 2, "{ops:?}");
    assert_eq!(ops[0].name, "get_player_agent_no");
    assert_eq!(ops[1].name, "agent_set_speed_modifier");
    assert_eq!(ops[1].args, vec![ops[0].args[0].clone(), Arg::Int(150)]);
}

// ---- (d) an unknown property is an error with file:line -----------------------------------------

#[test]
fn an_unknown_property_is_an_error_with_file_and_line() {
    let files = vec![file(
        "main",
        vec![on_start(vec![assign(&["player", "shoe_size"], AssignOp::Set, milli(40_000, 4), 4)])],
    )];
    let errors = match lower(&files, &atlas(), &data()) {
        Ok(lowered) => panic!("`player.shoe_size` is not in the vocabulary: {lowered:?}"),
        Err(errors) => errors,
    };
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].file, "main");
    assert_eq!(errors[0].line, 4);
    assert!(
        errors[0].to_string().starts_with("main:4: "),
        "the message carries file:line: {}",
        errors[0]
    );
}

// ---- conditions: `if key.held(alt)` --------------------------------------------------------------

#[test]
fn a_condition_property_gates_an_if_block() {
    let files = vec![file(
        "main",
        vec![on_start(vec![Stmt::If {
            cond: Expr::Call(Call {
                callee: path(&["key", "held"], 2),
                args: vec![Expr::Path(path(&["alt"], 2))],
                span: span(2),
            }),
            then: vec![assign(&["player", "name"], AssignOp::Set, text("Uw", 3), 3)],
            otherwise: None,
            span: span(2),
        }])],
    )];
    let lowered = lower_all(&files);
    let ops = game_start(&lowered);

    assert_eq!(ops[0].name, "try_begin");
    let key = find(ops, "key_is_down");
    assert_eq!(
        key.args,
        vec![Arg::Name("key_left_alt".to_owned())],
        "`alt` is the alias of `left_alt`, and the key constant comes from the atlas"
    );
    assert_eq!(find(ops, "troop_set_name").args[1], Arg::Text("@Uw".to_owned()));
    assert_eq!(ops.last().map(|op| op.name.as_str()), Some("try_end"));
}

// ---- fixed point: `unit = "int"` divides by 1000 at the boundary ---------------------------------

#[test]
fn an_int_unit_divides_by_a_thousand_at_the_boundary() {
    let data = CoreData::from_toml(
        r#"
[[property]]
path = "player.level"
unit = "int"
set = [{ op = "assign", args = ["$uw_main_level", "{value}"] }]
"#,
        "test.toml",
    )
    .expect("the test vocabulary is valid");

    // A constant is folded before it reaches the operation.
    let files = vec![file(
        "main",
        vec![on_start(vec![assign(&["player", "level"], AssignOp::Set, milli(12_000, 3), 3)])],
    )];
    let lowered = lower(&files, &atlas(), &data).expect("lowered");
    let ops = game_start(&lowered);
    assert_eq!(ops.len(), 1, "{ops:?}");
    assert_eq!(ops[0].args, vec![Arg::Global("$uw_main_level".to_owned()), Arg::Int(12)]);

    // A value that is only known while the game runs is divided by 1000.
    let files = vec![file(
        "main",
        vec![
            var("v", Some(milli(15_000, 2)), None, 2),
            on_start(vec![assign(&["player", "level"], AssignOp::Set, Expr::Path(path(&["v"], 3)), 3)]),
        ],
    )];
    let lowered = lower(&files, &atlas(), &data).expect("lowered");
    let ops = game_start(&lowered);
    let divide = find(ops, "store_div");
    assert_eq!(
        divide.args[1..],
        [Arg::Global("$uw_main_v".to_owned()), Arg::Int(1000)],
        "the engine counts whole units, UW numbers are thousandths"
    );
    let store = ops.last().expect("the store");
    assert_eq!(store.args[0], Arg::Global("$uw_main_level".to_owned()));
    assert_eq!(store.args[1], divide.args[0], "the divided value is what gets stored");
}

// ---- the vocabulary is data, not code ------------------------------------------------------------

#[test]
fn no_property_name_is_written_in_the_lowering() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("lower");
    let mut sources = 0;
    let mut source = String::new();
    for entry in fs::read_dir(&dir).expect("the lowering directory exists") {
        let path = entry.expect("a directory entry").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            sources += 1;
            source.push_str(&fs::read_to_string(&path).expect("the lowering source is readable"));
        }
    }
    assert!(sources > 0, "no Rust source under {}", dir.display());
    for name in ["speed_modifier", "troop_set_name", "get_player_agent_no", "key_is_down", "trp_player"] {
        assert!(
            !source.contains(name),
            "'{name}' belongs to kit-meta/core/properties.toml, not to the lowering"
        );
    }
}
