//! T-054: the structured printer writes loops as `for`, guards as `require` and unclean tries as
//! `attempt`, and each printed block reads back to the operations it came from. The real atlas is
//! measured with the `uw roundtrip` binary (see `roundtrip.rs`).

use uw_atlas::Atlas;
use uw_decomp::decompile::{decompile, Mode};
use uw_decomp::{to_raw, RawArg, RawOp};
use uw_diag::SourceFile;

/// A small atlas with the block operations and the two loop kinds the printed text needs.
const ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t054",
  "ops": [
    {"name":"neg","code":2147483648,"line":440,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"this_or_next","code":1073741824,"line":441,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"try_end","code":3,"line":16,"signature":"(try_end),","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"try_begin","code":4,"line":17,"signature":"(try_begin),","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"else_try","code":5,"line":18,"signature":"(else_try),","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"try_for_range","code":6,"line":19,"signature":"(try_for_range,<destination>,<lower_bound>,<upper_bound>),","lhs":true,"can_fail":false,"global_lhs":false},
    {"name":"try_for_parties","code":11,"line":26,"signature":"(try_for_parties,<destination>),","lhs":true,"can_fail":false,"global_lhs":false},
    {"name":"eq","code":31,"line":31,"signature":"(eq,<value>,<value>),","lhs":false,"can_fail":true,"global_lhs":false},
    {"name":"party_is_active","code":132,"line":94,"signature":"(party_is_active,<party_id>),","lhs":false,"can_fail":true,"global_lhs":false},
    {"name":"change_screen_map","code":2052,"line":1093,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"assign","code":2133,"line":40,"signature":"","lhs":true,"can_fail":false,"global_lhs":false}
  ],
  "hooks": [],
  "ranges": [],
  "constants": [],
  "tables": {}
}"#;

const THIS_OR_NEXT_EQ: u64 = 1073741824 | 31;

fn atlas() -> Atlas {
    Atlas::from_str(ATLAS).expect("fixture atlas")
}

fn op(code: u64, args: Vec<RawArg>) -> RawOp {
    RawOp { code, args }
}

fn int(n: i64) -> RawArg {
    RawArg::Int(n)
}

fn text(s: &str) -> RawArg {
    RawArg::Str(s.to_owned())
}

/// Prints `ops` structured and compiles the text back the way a saved `.uw` block is read
/// (parse + check + lower), asserting it is the operation it started from.
fn printed_and_read_back(ops: &[RawOp]) -> String {
    let atlas = atlas();
    let text = decompile(&atlas, ops, Mode::Structured);
    let file = SourceFile::new("block.uw", &text);
    let stmts = uw_syntax::parse_block(&text).expect("printed block parses");
    let lowered = uw_check::check_block(&atlas, &file, &stmts).expect("printed block checks");
    assert_eq!(to_raw(&atlas, &lowered).expect("printed block lowers"), ops, "{text}");
    text
}

#[test]
fn a_loop_prints_as_for_and_reads_back() {
    let ops = vec![op(6, vec![text(":i"), int(0), int(3)]), op(2052, vec![]), op(3, vec![])];
    assert_eq!(printed_and_read_back(&ops), "for :i in range(0, 3) {\n    change_screen_map()\n}");
}

#[test]
fn a_guard_inside_a_loop_prints_as_require_and_reads_back() {
    let ops = vec![op(11, vec![text(":p")]), op(132, vec![text(":p")]), op(2052, vec![]), op(3, vec![])];
    assert_eq!(
        printed_and_read_back(&ops),
        "for :p in parties() {\n    require party_is_active(:p)\n    change_screen_map()\n}"
    );
}

#[test]
fn a_guard_prints_as_require_and_reads_back() {
    let ops = vec![op(2052, vec![]), op(31, vec![text("$g_x"), int(1)]), op(2052, vec![])];
    assert_eq!(
        printed_and_read_back(&ops),
        "change_screen_map()\nrequire $g_x == 1\nchange_screen_map()"
    );
}

#[test]
fn an_or_chain_guard_prints_as_one_require_and_reads_back() {
    let ops = vec![
        op(31, vec![text("$g_x"), int(1)]),
        op(THIS_OR_NEXT_EQ, vec![text(":a"), int(2)]),
        op(31, vec![text(":a"), int(3)]),
        op(2052, vec![]),
    ];
    assert_eq!(
        printed_and_read_back(&ops),
        "require $g_x == 1 and (:a == 2 or :a == 3)\nchange_screen_map()"
    );
}

#[test]
fn an_unclean_try_prints_as_attempt_and_reads_back() {
    let ops = vec![
        op(4, vec![]),
        op(2133, vec![text("$g_x"), int(1)]),
        op(2052, vec![]),
        op(5, vec![]),
        op(132, vec![text(":p")]),
        op(2052, vec![]),
        op(3, vec![]),
    ];
    assert_eq!(
        printed_and_read_back(&ops),
        concat!(
            "attempt {\n",
            "    $g_x = 1\n",
            "    change_screen_map()\n",
            "} else attempt {\n",
            "    require party_is_active(:p)\n",
            "    change_screen_map()\n",
            "}"
        )
    );
}

#[test]
fn an_attempt_arm_without_require_prints_as_else_and_reads_back() {
    let ops = vec![
        op(4, vec![]),
        op(2133, vec![text("$g_x"), int(1)]),
        op(2052, vec![]),
        op(5, vec![]),
        op(2052, vec![]),
        op(3, vec![]),
    ];
    assert_eq!(
        printed_and_read_back(&ops),
        concat!(
            "attempt {\n",
            "    $g_x = 1\n",
            "    change_screen_map()\n",
            "} else {\n",
            "    change_screen_map()\n",
            "}"
        )
    );
}

#[test]
fn a_loop_with_an_else_try_stays_raw_and_still_reads_back() {
    let ops = vec![op(6, vec![text(":i"), int(0), int(3)]), op(5, vec![]), op(2052, vec![]), op(3, vec![])];
    let printed = decompile(&atlas(), &ops, Mode::Structured);
    assert!(!printed.contains("for "), "{printed}");
    let file = SourceFile::new("block.uw", &printed);
    let stmts = uw_syntax::parse_block(&printed).expect("raw block parses");
    let lowered = uw_check::check_block(&atlas(), &file, &stmts).expect("raw block checks");
    assert_eq!(to_raw(&atlas(), &lowered).expect("raw block lowers"), ops, "{printed}");
}
