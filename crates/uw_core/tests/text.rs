//! D5 text with values: `"Gold: {gold}"` (CORE-v0 §2). Numbers go through the compiler's own
//! registers as whole units; text values are inlined or referenced by their string register.

use uw_core::{Arg, Lowered, Op};

mod common;
use common::{lower_err, lower_ok};

fn script<'a>(lowered: &'a Lowered, name: &str) -> &'a [Op] {
    let found: Vec<_> = lowered.scripts.iter().filter(|(script, _)| script.ends_with(name)).collect();
    assert_eq!(found.len(), 1, "script {name} once in {:?}", lowered.scripts.iter().map(|s| &s.0).collect::<Vec<_>>());
    &found[0].1
}

fn texts(ops: &[Op]) -> Vec<String> {
    ops.iter()
        .flat_map(|op| op.args.iter())
        .filter_map(|arg| if let Arg::Text(text) = arg { Some(text.clone()) } else { None })
        .collect()
}

#[test]
fn number_goes_through_a_hidden_register_as_whole_units() {
    let lowered = lower_ok(&[("a", "var gold = 0\nfn show() {\n  troop.player.name = \"Gold: {gold} coins\"\n}\n")]);
    let ops = script(&lowered, "show");
    assert!(texts(ops).contains(&"@Gold: {reg52} coins".to_owned()), "{ops:?}");
    let assign = ops.iter().position(|op| op.name == "assign" && op.args.first() == Some(&Arg::Reg(52)));
    let store = ops.iter().position(|op| op.name == "str_store_string");
    assert!(assign.is_some() && store.is_some() && assign < store, "register filled before the text is used: {ops:?}");
    // thousandths → whole units before the engine shows the number
    assert!(ops.iter().any(|op| op.name == "store_div" && op.args.last() == Some(&Arg::Int(1000))), "{ops:?}");
}

#[test]
fn constants_fold_and_several_numbers_take_their_own_registers() {
    let lowered = lower_ok(&[(
        "a",
        "var a = 1\nvar b = 2\nfn show() {\n  troop.player.name = \"{a} + {b} = {a + b}, seven {7}\"\n}\n",
    )]);
    let ops = script(&lowered, "show");
    assert!(texts(ops).contains(&"@{reg52} + {reg53} = {reg54}, seven {reg55}".to_owned()), "{ops:?}");
    assert!(ops.iter().any(|op| op.name == "assign" && op.args == vec![Arg::Reg(55), Arg::Int(7)]), "7 folds to whole units");
}

#[test]
fn nested_text_is_inlined() {
    let lowered = lower_ok(&[("a", "const WHO = \"Alex\"\nfn show() {\n  troop.player.name = \"Sir {WHO}\"\n}\n")]);
    assert!(texts(script(&lowered, "show")).contains(&"@Sir Alex".to_owned()));
}

#[test]
fn too_many_numbers_is_an_error_not_a_collision() {
    let err = lower_err(&[(
        "a",
        "var n = 1\nfn show() {\n  troop.player.name = \"{n}{n}{n}{n}{n}{n}{n}{n}{n}\"\n}\n",
    )]);
    assert!(err.contains("at most 8 numbers"), "{err}");
}

#[test]
fn registers_rotate_so_two_texts_never_share_one() {
    let lowered = lower_ok(&[(
        "a",
        "var a = 1\nvar b = 2\nfn show() {\n  troop.player.name = \"{a}\"\n  troop.player.name = \"{b}\"\n}\n",
    )]);
    let found = texts(script(&lowered, "show"));
    assert!(found.contains(&"@{reg52}".to_owned()) && found.contains(&"@{reg53}".to_owned()), "{found:?}");
}
