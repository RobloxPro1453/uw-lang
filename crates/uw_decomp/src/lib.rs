//! Native -> UW decompiler and its round-trip measure (docs/meclis/DEVIR.md, kaldıraç planı).
//!
//! An operation block is read from the atlas JSON as [`RawOp`]s (`extract`), printed as UW text
//! (`decompile`), parsed and checked back (`uw_syntax::parse_block`, `uw_check::check_block`) and
//! turned into [`RawOp`]s again ([`to_raw`]). The block round-trips when both lists are equal.

pub mod decompile;
pub mod names;
pub mod extract;
pub mod project;
pub mod records;
pub mod roundtrip;

use serde_json::Value as Json;
use uw_atlas::Atlas;

/// One operand as the atlas holds it: an integer or a string (`"$g"`, `":l"`, `"p_town_6"`, `"@text"`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawArg {
    Int(i64),
    Str(String),
}

/// One operation: the code with its `neg` / `this_or_next` flag bits, and its operands.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawOp {
    pub code: u64,
    pub args: Vec<RawArg>,
}

/// One operation block of a Native entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Atlas table name (`"menus"`).
    pub table: String,
    /// Index of the entry in that table.
    pub index: usize,
    /// Field path from the entry's value to the block (`{"list": …}` wrappers are not counted).
    pub path: Vec<usize>,
    /// Human label for reports: `menus/start_phase_2/option town_1/consequence`.
    pub label: String,
    pub ops: Vec<RawOp>,
}

/// Unwraps the atlas' `{"list": [...]}` form; plain arrays (tuples) are returned as they are.
pub fn items(v: &Json) -> Option<&Vec<Json>> {
    match v {
        Json::Object(map) => map.get("list")?.as_array(),
        Json::Array(items) => Some(items),
        _ => None,
    }
}

/// Reads an operation block: a list whose statements are a bare code (`4`) or `[code, args…]`.
/// `None` when anything else is inside (a float operand, a nested list), so callers can tell a
/// field that is not an operation block.
///
/// Two Native spellings are read as what the kit writes to the .txt (process_operations.py
/// `save_statement`), since the round-trip measure compares compiled output:
/// - a statement written as a Python list `[op, …]` instead of a tuple (atlas `{"list": […]}`);
/// - an operand that is a one-element tuple (`ACHIEVEMENT_X = 56,` in module_constants.py gives
///   `(56,)`): `"%d " % (56,)` writes `56`, so it is `Int(56)`.
pub fn parse_ops(v: &Json) -> Option<Vec<RawOp>> {
    let mut out = Vec::new();
    for stmt in items(v)? {
        match stmt {
            Json::Number(n) => out.push(RawOp { code: n.as_u64()?, args: Vec::new() }),
            Json::Array(_) | Json::Object(_) => {
                let (head, rest) = items(stmt)?.split_first()?;
                let code = head.as_u64()?;
                let args = rest.iter().map(operand).collect::<Option<Vec<_>>>()?;
                out.push(RawOp { code, args });
            }
            _ => return None,
        }
    }
    Some(out)
}

fn operand(a: &Json) -> Option<RawArg> {
    match a {
        Json::Number(n) => n.as_i64().map(RawArg::Int),
        Json::String(s) => Some(RawArg::Str(s.clone())),
        Json::Array(_) | Json::Object(_) => match items(a)?.as_slice() {
            [one] => one.as_i64().map(RawArg::Int),
            _ => None,
        },
        _ => None,
    }
}

/// The comparison aliases header_operations.py defines as expressions (lines 444-446:
/// `lt = neg | ge`, `neq = neg | eq`, `le = neg | gt`), which the atlas does not list as ops.
fn alias(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "lt" => &["neg", "ge"],
        "neq" => &["neg", "eq"],
        "le" => &["neg", "gt"],
        _ => return None,
    })
}

/// Turns lowered operations back into codes and operands. Op names may carry flag prefixes
/// (`neg|this_or_next|eq`); each part is looked up in the atlas and the codes are OR-ed.
pub fn to_raw(atlas: &Atlas, ops: &[uw_lower::Op]) -> Result<Vec<RawOp>, String> {
    ops.iter()
        .map(|op| {
            let mut code = 0;
            for part in op.name.split('|').flat_map(|part| alias(part).unwrap_or(&[part][..]).iter().copied().collect::<Vec<_>>()) {
                code |= atlas.op(part).ok_or_else(|| format!("unknown operation `{part}`"))?.code;
            }
            let args = op
                .args
                .iter()
                .map(|v| match v {
                    uw_lower::Value::Int(i) => Ok(RawArg::Int(*i)),
                    uw_lower::Value::Ref(s) | uw_lower::Value::Text(s) => Ok(RawArg::Str(s.clone())),
                    uw_lower::Value::Const { name, .. } => atlas
                        .constant(name)
                        .and_then(|c| c.value.as_i64())
                        .map(RawArg::Int)
                        .ok_or_else(|| format!("constant `{name}` has no integer value")),
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(RawOp { code, args })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ops_reads_bare_codes_and_statements() {
        let v: Json = serde_json::from_str(r#"{"list": [4, [31, "$x", 3], [2147483679, ":a", "trp_player"], 3]}"#).unwrap();
        let ops = parse_ops(&v).unwrap();
        assert_eq!(ops.len(), 4);
        assert_eq!(ops[0], RawOp { code: 4, args: vec![] });
        assert_eq!(ops[2].code, 2147483679);
        assert_eq!(ops[2].args, vec![RawArg::Str(":a".into()), RawArg::Str("trp_player".into())]);
    }

    #[test]
    fn parse_ops_reads_list_statements_and_one_tuples_as_the_kit_writes_them() {
        let v: Json = serde_json::from_str(r#"{"list": [{"list": [2048]}, [372, [56]], [370, ":s", {"list": [57]}, 0]]}"#).unwrap();
        let ops = parse_ops(&v).unwrap();
        assert_eq!(ops[0], RawOp { code: 2048, args: vec![] });
        assert_eq!(ops[1], RawOp { code: 372, args: vec![RawArg::Int(56)] });
        assert_eq!(ops[2].args, vec![RawArg::Str(":s".into()), RawArg::Int(57), RawArg::Int(0)]);
        let two: Json = serde_json::from_str(r#"{"list": [[372, [56, 57]]]}"#).unwrap();
        assert!(parse_ops(&two).is_none());
    }

    #[test]
    fn parse_ops_rejects_non_blocks() {
        let floats: Json = serde_json::from_str(r#"{"list": [[31, 1.5]]}"#).unwrap();
        assert!(parse_ops(&floats).is_none());
        let text: Json = serde_json::from_str(r#""hello""#).unwrap();
        assert!(parse_ops(&text).is_none());
    }
}
