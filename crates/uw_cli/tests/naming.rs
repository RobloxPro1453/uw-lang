//! T-048: the decompiler prints register, position and slot names instead of the numbers the kit
//! names, and the text reads back to the numbers it replaced. Single operations go through the
//! compiler here; the whole real atlas is measured with the `uw roundtrip` binary.

use std::path::Path;
use std::process::Command;

use uw_atlas::Atlas;
use uw_decomp::decompile::{decompile, Mode};
use uw_decomp::{to_raw, RawArg, RawOp};
use uw_diag::SourceFile;

/// A small atlas with the kit's register and position basics and the named operations.
const ATLAS: &str = r#"{
  "schema": 1, "kit_hash": "t048",
  "ops": [
    {"name":"assign","code":2133,"line":1191,"signature":"had to put this here so that it can be called from conditions.","lhs":true,"can_fail":false,"global_lhs":false},
    {"name":"troop_set_slot","code":500,"line":239,"signature":"(troop_set_slot,<troop_id>,<slot_no>,<value>),","lhs":false,"can_fail":false,"global_lhs":false},
    {"name":"init_position","code":701,"line":302,"signature":"(init_position,<position_no>),","lhs":false,"can_fail":false,"global_lhs":false}
  ],
  "hooks": [],
  "ranges": [],
  "constants": [
    {"name":"op_num_value_bits","value":56,"module":"header_common","line":357},
    {"name":"tag_register","value":1,"module":"header_common","line":359},
    {"name":"opmask_register","value":72057594037927936,"module":"header_common","line":387},
    {"name":"reg0","value":72057594037927936,"module":"header_common","line":563},
    {"name":"pos0","value":0,"module":"header_common","line":497},
    {"name":"pos1","value":1,"module":"header_common","line":498},
    {"name":"slot_troop_x","value":7,"module":"module_constants","line":15}
  ],
  "tables": {}
}"#;

fn text(s: &str) -> RawArg {
    RawArg::Str(s.to_owned())
}

fn op(code: u64, args: Vec<RawArg>) -> RawOp {
    RawOp { code, args }
}

/// Prints `ops` and compiles the text back the way a saved `.uw` block is read (parse + check +
/// lower), asserting it is the operation it started from.
fn printed_and_read_back(atlas: &Atlas, ops: &[RawOp]) -> String {
    let text = decompile(atlas, ops, Mode::Raw);
    let file = SourceFile::new("block.uw", &text);
    let stmts = uw_syntax::parse_block(&text).expect("printed block parses");
    let lowered = uw_check::check_block(atlas, &file, &stmts).expect("printed block checks");
    assert_eq!(to_raw(atlas, &lowered).expect("printed block lowers"), ops, "{text}");
    text
}

#[test]
fn names_read_back_to_the_numbers_they_replace() {
    let atlas = Atlas::from_str(ATLAS).expect("fixture atlas");
    let ops = vec![
        op(2133, vec![RawArg::Int(72057594037927936), text("$g_multiplayer_team_1_faction")]),
        op(701, vec![RawArg::Int(1)]),
        op(500, vec![RawArg::Int(0), RawArg::Int(7), RawArg::Int(1)]),
    ];
    assert_eq!(
        printed_and_read_back(&atlas, &ops),
        "assign(reg0, $g_multiplayer_team_1_faction)\ninit_position(pos1)\ntroop_set_slot(0, slot_troop_x, 1)"
    );
}

/// `n` when `word` is exactly `prefix` followed by ASCII digits (`reg0`, `pos12`).
fn numbered<'a>(word: &'a str, prefix: &str) -> Option<&'a str> {
    let rest = word.strip_prefix(prefix)?;
    (!rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit())).then_some(rest)
}

/// How many `regN`, `posN` and `slot_*` operands the block texts written under `dir` carry.
fn names_written(dir: &Path) -> [usize; 3] {
    let mut named = [0usize; 3];
    for entry in std::fs::read_dir(dir).expect("output folder") {
        let path = entry.expect("folder entry").path();
        if path.extension().is_none_or(|ext| ext != "uw") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("block text");
        for word in text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
            if numbered(word, "reg").is_some() {
                named[0] += 1;
            } else if numbered(word, "pos").is_some() {
                named[1] += 1;
            } else if word.starts_with("slot_") {
                named[2] += 1;
            }
        }
    }
    named
}

#[test]
fn real_atlas_round_trip_still_exact_and_names_operands() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
    if !Path::new(path).exists() {
        eprintln!("skipped: no atlas at {path}");
        return;
    }
    let dir = std::env::temp_dir().join(format!("uw-naming-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("roundtrip")
        .args(["--atlas", path])
        .arg("--out")
        .arg(&dir)
        .output()
        .expect("uw runs");
    let stdout = String::from_utf8(out.stdout).expect("utf-8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf-8 stderr");
    assert!(out.status.success(), "{stdout}\n{stderr}");

    // The operation blocks' TOTAL line, before the records one.
    let total = stdout.lines().find(|line| line.starts_with("TOTAL")).expect("a TOTAL line");
    let words: Vec<&str> = total.split_whitespace().collect();
    assert_eq!(words[1], "9113", "{stdout}");
    let failed = words.iter().position(|word| *word == "failed").expect("a failed column");
    assert_eq!(words[failed + 1], "0", "{total}");

    let named = names_written(&dir);
    std::fs::remove_dir_all(&dir).ok();
    // Measured on the real atlas: 4303 register, 5678 position and 5213 slot operands are named (5559 before the kit reader read the call form under the assignment, 2026-11-22).
    assert_eq!(named, [4303, 5678, 5213], "{total}");
}
