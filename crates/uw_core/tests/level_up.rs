//! Regression and capability tests for fixed-point `*`/`/`, user `fn` calls, and parameterized kit ops.

use uw_core::lower::kit::{kit_ops, kit_properties};
use uw_core::{Arg, Op};

mod common;
use common::{lower_err, lower_ok};

fn start_ops(source: &str) -> Vec<Op> {
    let lowered = lower_ok(&[("main", source)]);
    lowered
        .script_appends
        .into_iter()
        .find(|(name, _)| name == "game_start")
        .map(|(_, ops)| ops)
        .unwrap_or_default()
}

#[test]
fn fixed_point_mul_and_div_preserve_thousandths_scale() {
    let ops = start_ops(
        "var x = 2 * 3\n\
         var y = 0.5 * 4\n\
         var z = 6 / 2\n\
         var half = 3 / 2\n\
         on game.start {\n\
           x *= 2.5\n\
           z /= 0.5\n\
         }\n",
    );
    // Constant folding keeps 1000x scale: 6 -> 6000, 2 -> 2000, 3 -> 3000, 1.5 -> 1500.
    assert_eq!(ops[0].args, vec![Arg::Global("$uw_main_x".into()), Arg::Int(6000)]);
    assert_eq!(ops[1].args, vec![Arg::Global("$uw_main_y".into()), Arg::Int(2000)]);
    assert_eq!(ops[2].args, vec![Arg::Global("$uw_main_z".into()), Arg::Int(3000)]);
    assert_eq!(ops[3].args, vec![Arg::Global("$uw_main_half".into()), Arg::Int(1500)]);

    // Compound *= divides by 1000 after multiplying; /= multiplies by 1000 before dividing.
    assert_eq!(ops[4].name, "val_mul");
    assert_eq!(ops[4].args, vec![Arg::Global("$uw_main_x".into()), Arg::Int(2500)]);
    assert_eq!(ops[5].name, "val_div");
    assert_eq!(ops[5].args, vec![Arg::Global("$uw_main_x".into()), Arg::Int(1000)]);

    assert_eq!(ops[6].name, "val_mul");
    assert_eq!(ops[6].args, vec![Arg::Global("$uw_main_z".into()), Arg::Int(1000)]);
    assert_eq!(ops[7].name, "val_div");
    assert_eq!(ops[7].args, vec![Arg::Global("$uw_main_z".into()), Arg::Int(500)]);
}

#[test]
fn user_functions_can_be_called_within_and_across_features() {
    let lowered = lower_ok(&[
        (
            "math_util",
            "fn double_plus(a, b) {\n  return a * 2 + b\n}\n\
             fn do_nothing(a) {\n  let t = a\n}\n",
        ),
        (
            "main",
            "var total = 0\n\
             fn local_helper(v) {\n  return math_util.double_plus(v, 5)\n}\n\
             on game.start {\n\
               math_util.do_nothing(1)\n\
               total = local_helper(10)\n\
             }\n",
        ),
    ]);
    assert!(lowered.scripts.iter().any(|(name, _)| name == "uw_math_util_double_plus"));
    assert!(lowered.scripts.iter().any(|(name, _)| name == "uw_main_local_helper"));

    let start = lowered
        .script_appends
        .iter()
        .find(|(name, _)| name == "game_start")
        .map(|(_, ops)| ops)
        .expect("game_start");
    let calls: Vec<&Op> = start.iter().filter(|op| op.name == "call_script").collect();
    assert_eq!(calls.len(), 2, "{start:?}");
    assert_eq!(calls[0].args, vec![Arg::Name("script_uw_math_util_do_nothing".into()), Arg::Int(1000)]);
    assert_eq!(calls[1].args, vec![Arg::Name("script_uw_main_local_helper".into()), Arg::Int(10_000)]);

    let err = lower_err(&[(
        "main",
        "fn no_ret() {\n  let x = 1\n}\non game.start {\n  let y = no_ret()\n}\n",
    )]);
    assert!(err.contains("returns no value"), "{err}");

    let err16 = lower_err(&[(
        "main",
        "fn too_many(a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16) {\n  return a1\n}\n",
    )]);
    assert!(err16.contains("at most 15"), "{err16}");
}

#[test]
fn kit_vocabulary_reaches_over_800_operations_with_parameterized_getters_and_conditions() {
    let props = kit_properties(&kit_ops());
    let reachable: std::collections::BTreeSet<&str> = props
        .iter()
        .flat_map(|p| p.get.iter().chain(&p.set).chain(&p.cond).chain(&p.run))
        .map(|s| s.op.as_str())
        .collect();
    assert!(
        reachable.len() >= 800,
        "expected >= 800 reachable kit ops, got {}",
        reachable.len()
    );
}
