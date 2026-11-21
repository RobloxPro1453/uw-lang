//! T-132: the core parser reads the Alex example (main.uw, features/*.uw) into the syntax tree and reports
//! `file:line` errors.

use uw_core::ast::{self, AssignOp, BinOp, Expr, Item, RateUnit, Stmt, TextPart};
use uw_core::parse;

fn read(rel: &str) -> String {
    std::fs::read_to_string(format!("{}/tests/alex/{rel}", env!("CARGO_MANIFEST_DIR")))
        .unwrap_or_else(|e| panic!("cannot read tests/alex/{rel}: {e}"))
}

fn parse_alex(rel: &str, feature: &str) -> ast::File {
    let text = read(rel);
    match parse(feature, rel, &text) {
        Ok(file) => file,
        Err(errors) => {
            let shown: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
            panic!("{rel} must parse, got {shown:?}")
        }
    }
}

/// Every error of a file that must not parse, as `file:line: message`.
fn errors_of(file: &str, text: &str) -> Vec<String> {
    match parse("test", file, text) {
        Ok(got) => panic!("{file} must not parse, got {got:?}"),
        Err(errors) => errors.iter().map(|e| e.to_string()).collect(),
    }
}

fn path_text(path: &ast::Path) -> String {
    path.parts.iter().map(|p| p.text.as_str()).collect::<Vec<_>>().join(".")
}

fn number(expr: &Expr) -> i64 {
    match expr {
        Expr::Number { milli, .. } => *milli,
        other => panic!("expected a number, got {other:?}"),
    }
}

fn identical(ident: &ast::Ident) -> &str {
    &ident.text
}

fn as_path(expr: &Expr) -> &ast::Path {
    match expr {
        Expr::Path(path) => path,
        other => panic!("expected a path, got {other:?}"),
    }
}

#[test]
fn main_uw_parses_game_start() {
    let file = parse_alex("main.uw", "main");
    assert_eq!(file.feature, "main");
    let [Item::On { event, bound, when, body, .. }] = file.items.as_slice() else {
        panic!("main.uw is one `on` item, got {:?}", file.items);
    };
    assert_eq!(path_text(event), "game.start");
    assert!(bound.is_none() && when.is_none(), "no bound object and no condition");
    assert_eq!(body.len(), 4, "game.start has four statements, got {body:?}");

    let Stmt::Call(disable) = &body[0] else { panic!("first statement is disable(…), got {:?}", body[0]) };
    assert_eq!(path_text(&disable.callee), "disable");
    let [Expr::Path(target)] = disable.args.as_slice() else {
        panic!("disable takes the native id, got {:?}", disable.args);
    };
    assert_eq!(path_text(target), "native.character_creation");

    let Stmt::Assign { place, op, value, .. } = &body[1] else {
        panic!("second statement is an assignment, got {:?}", body[1]);
    };
    assert_eq!(path_text(place), "player.name");
    assert_eq!(*op, AssignOp::Set);
    let Expr::Text { parts, .. } = value else { panic!("player.name = \"Alex\", got {value:?}") };
    let [TextPart::Literal(text)] = parts.as_slice() else { panic!("plain text, got {parts:?}") };
    assert_eq!(text, "Alex");

    let Stmt::Assign { place, value, .. } = &body[2] else {
        panic!("third statement is an assignment, got {:?}", body[2]);
    };
    assert_eq!(path_text(place), "player.face");
    let Expr::Call(face) = value else { panic!("face(1), got {value:?}") };
    assert_eq!(path_text(&face.callee), "face");
    assert_eq!(face.args.len(), 1);
    assert_eq!(number(&face.args[0]), 1000, "1 is 1000 thousandths");

    let Stmt::Call(teleport) = &body[3] else {
        panic!("fourth statement is player.teleport(…), got {:?}", body[3]);
    };
    assert_eq!(path_text(&teleport.callee), "player.teleport");
    let [Expr::Path(scene)] = teleport.args.as_slice() else {
        panic!("teleport takes the scene path, got {:?}", teleport.args);
    };
    assert_eq!(path_text(scene), "town.praven.streets");
}

#[test]
fn stamina_uw_field_while_and_ui() {
    let file = parse_alex("features/stamina.uw", "stamina");
    assert_eq!(file.feature, "stamina");
    assert_eq!(file.items.len(), 3, "var, while in, ui — got {:?}", file.items);

    // `var agent.stamina = 100 in 0..100`: a field on the agent kind, clamped into 0..100.
    let Item::Var(decl) = &file.items[0] else { panic!("first item is the var, got {:?}", file.items[0]) };
    assert_eq!(decl.kind.as_ref().map(identical), Some("agent"));
    assert_eq!(identical(&decl.name), "stamina");
    assert_eq!(number(decl.init.as_ref().expect("initial value")), 100_000);
    let (lo, hi) = decl.range.as_ref().expect("range 0..100");
    assert_eq!(number(lo), 0);
    assert_eq!(number(hi), 100_000, "100 is 100000 thousandths");

    let Item::WhileIn { context, body, .. } = &file.items[1] else {
        panic!("second item is `while in mission`, got {:?}", file.items[1])
    };
    assert_eq!(path_text(context), "mission");
    let [Stmt::Assign { place, op, value, .. }] = body.as_slice() else {
        panic!("one assignment in the loop, got {body:?}")
    };
    assert_eq!(path_text(place), "player.agent.stamina");
    assert_eq!(*op, AssignOp::Add);
    let Expr::Rate { milli, unit, .. } = value else { panic!("15 per second, got {value:?}") };
    assert_eq!((*milli, *unit), (15_000, RateUnit::Second));

    // `ui.bar stamina_bar { … }`: six properties, one color and one tuple.
    let Item::Ui { part, name, props, .. } = &file.items[2] else {
        panic!("third item is the ui bar, got {:?}", file.items[2])
    };
    assert_eq!(identical(part), "bar");
    assert_eq!(identical(name), "stamina_bar");
    assert_eq!(props.len(), 6, "six properties, got {props:?}");
    let prop = |want: &str| -> &Expr {
        props
            .iter()
            .find(|(key, _)| key.text == want)
            .map(|(_, value)| value)
            .unwrap_or_else(|| panic!("property {want} is missing, got {props:?}"))
    };
    assert_eq!(number(prop("max")), 100_000);
    match prop("color") {
        Expr::Color { rgb, .. } => assert_eq!(*rgb, 0x3F_A3_4D),
        other => panic!("color = #3FA34D, got {other:?}"),
    }
    match prop("size") {
        Expr::Tuple { items, .. } => {
            assert_eq!(items.len(), 2, "size is (width, height)");
            assert_eq!((number(&items[0]), number(&items[1])), (220_000, 14_000));
        }
        other => panic!("size = (220, 14), got {other:?}"),
    }
    assert_eq!(path_text(as_path(prop("value"))), "player.agent.stamina");
}

#[test]
fn sprint_uw_constants_while_and_otherwise() {
    let file = parse_alex("features/sprint.uw", "sprint");
    assert_eq!(file.feature, "sprint");
    assert_eq!(file.items.len(), 3, "const, const, while — got {:?}", file.items);

    let Item::Const { name, value, .. } = &file.items[0] else {
        panic!("SPEED is a const, got {:?}", file.items[0])
    };
    assert_eq!(identical(name), "SPEED");
    match value {
        Expr::Percent { value, .. } => assert_eq!(*value, 150, "150% keeps the number"),
        other => panic!("150%, got {other:?}"),
    }

    let Item::Const { name, value, .. } = &file.items[1] else {
        panic!("COST is a const, got {:?}", file.items[1])
    };
    assert_eq!(identical(name), "COST");
    let Expr::Rate { milli, unit, .. } = value else { panic!("35 per second, got {value:?}") };
    assert_eq!((*milli, *unit), (35_000, RateUnit::Second));

    // `while key.held(alt) and player.agent.stamina > 0 { … } otherwise { … }`
    let Item::While { cond, body, otherwise, .. } = &file.items[2] else {
        panic!("third item is the while, got {:?}", file.items[2])
    };
    let Expr::Binary { op, left, right, .. } = cond else { panic!("`and` condition, got {cond:?}") };
    assert_eq!(*op, BinOp::And);
    let Expr::Call(call) = &**left else { panic!("key.held(alt), got {left:?}") };
    assert_eq!(path_text(&call.callee), "key.held");
    let [Expr::Path(key)] = call.args.as_slice() else { panic!("key.held(alt), got {:?}", call.args) };
    assert_eq!(path_text(key), "alt");
    let Expr::Binary { op, left, right, .. } = &**right else { panic!("a comparison, got {right:?}") };
    assert_eq!(*op, BinOp::Gt);
    assert_eq!(path_text(as_path(left)), "player.agent.stamina");
    assert_eq!(number(right), 0);

    assert_eq!(body.len(), 2, "two statements in the loop, got {body:?}");
    let Some(otherwise) = otherwise else { panic!("sprint.uw has an `otherwise` block") };
    let [Stmt::Assign { place, value, .. }] = otherwise.as_slice() else {
        panic!("one statement in `otherwise`, got {otherwise:?}")
    };
    assert_eq!(path_text(place), "player.agent.speed");
    match value {
        Expr::Percent { value, .. } => assert_eq!(*value, 100),
        other => panic!("100%, got {other:?}"),
    }
}

#[test]
fn text_keeps_brace_expressions_as_parts() {
    let file = parse("test", "t.uw", "const LABEL = \"Stam {player.agent.stamina}/100\"\n")
        .expect("a text with one {expr} parses");
    let [Item::Const { value, .. }] = file.items.as_slice() else { panic!("one const, got {:?}", file.items) };
    let Expr::Text { parts, .. } = value else { panic!("a text, got {value:?}") };
    assert_eq!(parts.len(), 3, "literal, expression, literal — got {parts:?}");
    assert!(matches!(&parts[0], TextPart::Literal(text) if text == "Stam "));
    match &parts[1] {
        TextPart::Expr(expr) => assert_eq!(path_text(as_path(expr)), "player.agent.stamina"),
        other => panic!("the {{…}} part is an expression, got {other:?}"),
    }
    assert!(matches!(&parts[2], TextPart::Literal(text) if text == "/100"));
}

#[test]
fn unclosed_block_is_reported_at_its_opening_brace() {
    let errs = errors_of("open.uw", "on game.start {\n    disable(native.character_creation)\n");
    assert_eq!(errs.len(), 1, "one error for the unclosed block, got {errs:?}");
    assert!(errs[0].starts_with("open.uw:1:"), "the error names the opening `{{`: {errs:?}");
    assert!(errs[0].contains("never closed"), "got {errs:?}");
}

#[test]
fn otherwise_without_while_is_an_error() {
    let errs = errors_of("stray.uw", "otherwise {\n    player.agent.speed = 100%\n}\n");
    assert!(errs.iter().any(|e| e.starts_with("stray.uw:1:") && e.contains("otherwise")), "got {errs:?}");
}

#[test]
fn unknown_character_is_an_error_on_its_line() {
    let errs = errors_of("bad.uw", "on game.start {\n    player.name = @\n}\n");
    assert!(
        errs.iter().any(|e| e.starts_with("bad.uw:2:") && e.contains("unknown character '@'")),
        "got {errs:?}"
    );
}

#[test]
fn a_stray_closing_brace_is_an_error_and_not_a_hang() {
    let errs = errors_of("brace.uw", "}\nconst A = 1\n");
    assert!(errs.iter().any(|e| e.starts_with("brace.uw:1:")), "got {errs:?}");
}

#[test]
fn several_errors_are_reported_from_one_file() {
    // Two broken items: the parser skips to the next item instead of stopping at the first error.
    let errs = errors_of("two.uw", "const A =\nconst B =\nconst C = 1\n");
    let lines: Vec<&str> = errs.iter().filter_map(|e| e.split(':').nth(1)).collect();
    assert_eq!(lines, ["1", "2"], "one error per broken const, got {errs:?}");
}

#[test]
fn fn_if_for_let_return_and_on_bound_parse() {
    let text = "\
fn speed_of(agent) {
    let bonus = 1 + 2 * 3
    if bonus > 5 {
        return bonus
    } else {
        return
    }
    for item in player.items where item.owned {
        player.give(item, -1)
    }
}
on mission.update(agent) when not agent.dead {
}
";
    let file = parse("test", "forms.uw", text).expect("the statement forms parse");
    assert_eq!(file.items.len(), 2, "fn and on, got {:?}", file.items);

    let Item::Fn { name, params, body, .. } = &file.items[0] else { panic!("fn, got {:?}", file.items[0]) };
    assert_eq!(identical(name), "speed_of");
    assert_eq!(params.iter().map(identical).collect::<Vec<_>>(), ["agent"]);
    assert_eq!(body.len(), 3, "let, if, for — got {body:?}");

    let Stmt::Let { name, value, .. } = &body[0] else { panic!("let, got {:?}", body[0]) };
    assert_eq!(identical(name), "bonus");
    match value {
        Expr::Binary { op: BinOp::Add, right, .. } => match &**right {
            Expr::Binary { op: BinOp::Mul, .. } => {}
            other => panic!("2 * 3 binds before +, got {other:?}"),
        },
        other => panic!("1 + 2 * 3, got {other:?}"),
    }

    let Stmt::If { cond, then, otherwise, .. } = &body[1] else { panic!("if, got {:?}", body[1]) };
    assert!(matches!(cond, Expr::Binary { op: BinOp::Gt, .. }), "bonus > 5, got {cond:?}");
    assert!(matches!(then.as_slice(), [Stmt::Return { value: Some(_), .. }]), "return bonus, got {then:?}");
    let otherwise = otherwise.as_ref().expect("an else block");
    assert!(matches!(otherwise.as_slice(), [Stmt::Return { value: None, .. }]), "bare return, got {otherwise:?}");

    let Stmt::For { var, iter, filter, body, .. } = &body[2] else { panic!("for, got {:?}", body[2]) };
    assert_eq!(identical(var), "item");
    assert_eq!(path_text(as_path(iter)), "player.items");
    assert!(filter.is_some(), "`where item.owned`, got {filter:?}");
    let [Stmt::Call(give)] = body.as_slice() else { panic!("one call in the loop, got {body:?}") };
    assert_eq!(path_text(&give.callee), "player.give");
    assert!(
        matches!(give.args.get(1), Some(Expr::Unary { neg: true, not: false, .. })),
        "-1 is a negated number, got {:?}",
        give.args
    );

    let Item::On { event, bound, when, body, .. } = &file.items[1] else {
        panic!("on, got {:?}", file.items[1])
    };
    assert_eq!(path_text(event), "mission.update");
    assert_eq!(bound.as_ref().map(identical), Some("agent"));
    assert!(
        matches!(when, Some(Expr::Unary { not: true, neg: false, .. })),
        "`when not agent.dead`, got {when:?}"
    );
    assert!(body.is_empty(), "an empty block, got {body:?}");
}
