//! P3b: a whole Native table written back as a Python 2 module the
//! kit's `process_*.py` imports (`from module_<t> import *`). Values come in the atlas JSON form:
//! JSON array = tuple, `{"list": …}` = list, strings are the kit's byte strings decoded as latin-1
//! (py/atlas_dump.py:73-79), floats are Python 2 `repr` text, integers keep every digit.

use serde_json::Value as Json;

/// `<var> = [ … ]`: the module text for one table, records in the given order.
pub fn module_text(var: &str, records: &[Json]) -> String {
    let mut out = String::from("# -*- coding: latin-1 -*-\n# written by uw (project build, P3b); do not edit\n");
    out.push_str(var);
    out.push_str(" = [\n");
    for record in records {
        out.push_str("  ");
        write(record, &mut out);
        out.push_str(",\n");
    }
    out.push_str("]\n");
    out
}

/// One value as Python 2 source.
pub fn write(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("None"),
        Json::Bool(true) => out.push_str("True"),
        Json::Bool(false) => out.push_str("False"),
        // Integers: the digits as they are (arbitrary_precision keeps them). Floats: Python's repr
        // text reads back to the same double.
        Json::Number(n) => out.push_str(&n.to_string()),
        Json::String(s) => write_str(s, out),
        Json::Array(items) => seq('(', ')', items, out),
        Json::Object(map) => match map.get("list") {
            Some(Json::Array(items)) if map.len() == 1 => seq('[', ']', items, out),
            // The atlas has no other object; keep it visible rather than guessing.
            _ => out.push_str(&format!("__uw_unknown_value__({})", value)),
        },
    }
}

fn seq(open: char, close: char, items: &[Json], out: &mut String) {
    out.push(open);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write(item, out);
    }
    if open == '(' && items.len() == 1 {
        out.push(',');
    }
    out.push(close);
}

/// A Python 2 byte-string literal: printable ASCII as is, everything else `\xHH` (latin-1 byte).
fn write_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            ' '..='~' => out.push(c),
            c if (c as u32) <= 0xFF => out.push_str(&format!("\\x{:02x}", c as u32)),
            // Not a latin-1 byte: the atlas came from a Python `unicode` value. Visible failure.
            c => out.push_str(&format!("\\u{{uw: not latin-1 U+{:04X}}}", c as u32)),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_print_as_python_2() {
        let v: Json = serde_json::from_str(
            r#"["a\"b\\c", "é\n", 1606938044258990275541962092341162602522202993782792835301376, -0.25,
               {"list": [1, [2]]}, [3], null, true]"#,
        )
        .unwrap();
        let mut out = String::new();
        write(&v, &mut out);
        assert_eq!(
            out,
            r#"("a\"b\\c", "\xe9\x0a", 1606938044258990275541962092341162602522202993782792835301376, -0.25, [1, (2,)], (3,), None, True)"#
        );
    }

    #[test]
    fn module_text_names_the_variable() {
        let text = module_text("factions", &[serde_json::json!(["no_faction", "No Faction"])]);
        assert!(text.contains("factions = [\n  (\"no_faction\", \"No Faction\"),\n]\n"), "{text}");
    }
}
