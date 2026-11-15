//! Native records as UW record text (docs/meclis/YON2-KARAR.md §2): every atlas entry is printed
//! as `<kind> <id> { <field> <value> … }` and parsed back to the identical atlas value.
//!
//! Round 1 is RAW: values are written exactly as the atlas holds them (no flag, reference or
//! packed-integer decoding). Field names and their order come from `kit-meta/records/<table>.toml`,
//! whose `source` points at the kit's layout comment for each field.

use std::path::Path;

use serde::Deserialize;
use serde_json::Value as Json;
use uw_atlas::{Atlas, Entry};

/// One value of a record, as the atlas holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Decimal integer, arbitrary size; may be negative.
    Int(String),
    /// Decimal float, as the atlas wrote it (`0.9`, `1e-05`).
    Float(String),
    /// Text; printed quoted and escaped.
    Str(String),
    /// Bare identifier (`itm_x`, `fac_y`), printed as it is. The JSON literals `null`, `true` and
    /// `false` are carried here too, because the atlas value shape has no other case for them; a
    /// JSON string with the same text stays [`Value::Str`] and is printed quoted.
    Ident(String),
    /// Python list (atlas `{"list": […]}`).
    List(Vec<Value>),
    /// Python tuple (atlas JSON array).
    Tuple(Vec<Value>),
}

/// Reads an atlas value. `{"list": …}` is a Python list, a JSON array is a Python tuple.
pub fn from_json(v: &Json) -> Value {
    match v {
        Json::Null => Value::Ident("null".into()),
        Json::Bool(b) => Value::Ident(if *b { "true" } else { "false" }.into()),
        Json::Number(n) => {
            // `arbitrary_precision` keeps the digits the atlas wrote; only the shape decides.
            let text = n.to_string();
            if text.contains(['.', 'e', 'E']) {
                Value::Float(text)
            } else {
                Value::Int(text)
            }
        }
        Json::String(s) => {
            if ident_like(s) {
                Value::Ident(s.clone())
            } else {
                Value::Str(s.clone())
            }
        }
        Json::Array(items) => Value::Tuple(items.iter().map(from_json).collect()),
        Json::Object(map) => match map.get("list") {
            Some(Json::Array(items)) => Value::List(items.iter().map(from_json).collect()),
            // The atlas only writes `{"list": …}`; keep any other object visible as text so a
            // record never turns silently exact.
            _ => Value::Str(v.to_string()),
        },
    }
}

/// Writes a [`Value`] back as atlas JSON; the inverse of [`from_json`].
pub fn to_json(v: &Value) -> Json {
    match v {
        Value::Int(text) | Value::Float(text) => {
            serde_json::from_str(text).expect("Value::Int / Value::Float hold a decimal number")
        }
        Value::Str(s) => Json::String(s.clone()),
        Value::Ident(s) => match s.as_str() {
            "null" => Json::Null,
            "true" => Json::Bool(true),
            "false" => Json::Bool(false),
            _ => Json::String(s.clone()),
        },
        Value::List(items) => {
            let mut map = serde_json::Map::new();
            map.insert("list".into(), Json::Array(items.iter().map(to_json).collect()));
            Json::Object(map)
        }
        Value::Tuple(items) => Json::Array(items.iter().map(to_json).collect()),
    }
}

/// A string that prints bare and reads back unchanged: `[a-z_][a-z0-9_]*`, not a JSON literal.
fn ident_like(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c == '_' => {}
        _ => return false,
    }
    !matches!(s, "null" | "true" | "false") && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// One field of a record.
#[derive(Debug, Clone, Deserialize)]
pub struct Field {
    pub name: String,
    /// Field type; round 1 allows only `"raw"`.
    pub ty: String,
    /// Trailing fields may be absent from a record.
    #[serde(default)]
    pub optional: bool,
    /// Where the kit says this: `<file>:<line>`.
    pub source: String,
}

/// The field layout of one atlas table (`kit-meta/records/<table>.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    pub table: String,
    /// Record word (`faction`, `troop`, …).
    pub kind: String,
    /// Prefix stripped from the atlas id (`fac_`).
    pub id_prefix: String,
    pub fields: Vec<Field>,
}

impl Schema {
    /// Reads one `kit-meta/records/<table>.toml`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Schema, String> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("{}: {e}", path.display()))?;
        Schema::from_toml(&text)
    }

    /// Parses and checks schema text (round 1: every field is `raw`).
    pub fn from_toml(text: &str) -> Result<Schema, String> {
        let schema: Schema = toml::from_str(text).map_err(|e| format!("schema: {e}"))?;
        for (i, field) in schema.fields.iter().enumerate() {
            if field.ty != "raw" {
                return Err(format!(
                    "field `{}`: type `{}` is not supported (round 1 allows only `raw`)",
                    field.name, field.ty
                ));
            }
            if schema.fields[..i].iter().any(|f| f.name == field.name) {
                return Err(format!("field `{}` appears twice", field.name));
            }
        }
        Ok(schema)
    }
}

/// The id a record is printed with: the atlas id without the schema's prefix.
fn record_id(schema: &Schema, entry: &Entry) -> String {
    let id = entry.id.as_deref().or(entry.name.as_deref()).unwrap_or("");
    id.strip_prefix(&schema.id_prefix).unwrap_or(id).to_owned()
}

/// The values of one record: the fields of its tuple, or the elements of its list.
fn record_values(value: &Json) -> Vec<Value> {
    match from_json(value) {
        Value::Tuple(values) | Value::List(values) => values,
        other => vec![other],
    }
}

/// Prints one atlas entry as record text: `<kind> <id> {`, one `name value` line per field, `}`.
///
/// Values past the schema's fields print as `field_<n>` (`n` is the 1-based position in the record).
pub fn print(schema: &Schema, entry: &Entry) -> String {
    let mut out = format!("{} {} {{\n", schema.kind, record_id(schema, entry));
    for (i, value) in record_values(&entry.value).iter().enumerate() {
        let name = match schema.fields.get(i) {
            Some(field) => field.name.clone(),
            None => format!("field_{}", i + 1),
        };
        out.push_str("  ");
        out.push_str(&name);
        out.push(' ');
        write_value(value, &mut out);
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

fn write_value(value: &Value, out: &mut String) {
    match value {
        Value::Int(text) | Value::Float(text) | Value::Ident(text) => out.push_str(text),
        Value::Str(text) => write_str(text, out),
        Value::List(items) => write_seq('[', ']', items, out),
        Value::Tuple(items) => write_seq('(', ')', items, out),
    }
}

fn write_seq(open: char, close: char, items: &[Value], out: &mut String) {
    out.push(open);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_value(item, out);
    }
    // A one-element tuple is `(a,)`: a bare `(a)` would read as a parenthesised value.
    if open == '(' && items.len() == 1 {
        out.push(',');
    }
    out.push(close);
}

fn write_str(text: &str, out: &mut String) {
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            _ => out.push(c),
        }
    }
    out.push('"');
}

/// Parses record text back into `(id, atlas value)`. Errors name the line.
pub fn parse(schema: &Schema, text: &str) -> Result<(String, Json), String> {
    let mut p = Parser::new(text);
    p.skip_gaps();
    let kind_line = p.line;
    let kind = p.word();
    if kind != schema.kind {
        return Err(format!(
            "line {kind_line}: expected the record kind `{}`, found `{kind}`",
            schema.kind
        ));
    }
    p.skip_gaps();
    let id_line = p.line;
    let id = if p.peek() == Some('"') {
        p.string(id_line)?
    } else {
        p.word()
    };
    if id.is_empty() {
        return Err(format!("line {id_line}: expected the record id"));
    }
    p.skip_gaps();
    if p.peek() != Some('{') {
        return Err(format!("line {}: expected `{{` after `{} {id}`", p.line, schema.kind));
    }
    p.bump();

    let mut values: Vec<Option<Value>> = vec![None; schema.fields.len()];
    let close_line;
    loop {
        p.skip_gaps();
        match p.peek() {
            None => return Err(format!("line {}: expected `}}`", p.line)),
            Some('}') => {
                close_line = p.line;
                p.bump();
                break;
            }
            Some(_) => {}
        }
        let field_line = p.line;
        let name = p.word();
        if name.is_empty() {
            return Err(format!("line {field_line}: expected a field name"));
        }
        p.skip_gaps();
        if p.peek() == Some('}') {
            return Err(format!("line {field_line}: field `{name}` has no value"));
        }
        let value = p.value()?;
        let index = match schema.fields.iter().position(|f| f.name == name) {
            Some(index) => index,
            None => match name.strip_prefix("field_").and_then(|n| n.parse::<usize>().ok()) {
                Some(n) if n >= 1 => n - 1,
                _ => return Err(format!("line {field_line}: unknown field `{name}`")),
            },
        };
        if index < values.len() && values[index].is_some() {
            return Err(format!("line {field_line}: field `{name}` appears twice"));
        }
        if index >= values.len() {
            values.resize(index + 1, None);
        }
        values[index] = Some(value);
    }

    for (i, slot) in values.iter().enumerate() {
        if slot.is_none() {
            let optional = schema.fields.get(i).is_some_and(|f| f.optional);
            if !optional {
                let name = schema.fields.get(i).map(|f| f.name.as_str()).unwrap_or("?");
                return Err(format!("line {close_line}: missing required field `{name}`"));
            }
        }
    }
    // Absent optional trailing fields are simply not part of the value.
    while matches!(values.last(), Some(None)) {
        values.pop();
    }
    Ok((id, Json::Array(values.into_iter().flatten().map(|v| to_json(&v)).collect())))
}

/// One printed text left at most per failing record kind; enough to see what broke.
const FIRST_FAILURES: usize = 5;

/// How one table's records did, in the style of the operation round-trip measure.
#[derive(Debug, Clone, Default)]
pub struct TableReport {
    pub records: usize,
    pub exact: usize,
    pub failed: usize,
    /// The first few failures, with the record id.
    pub first_failures: Vec<String>,
}

impl TableReport {
    fn fail(&mut self, message: String) {
        self.failed += 1;
        if self.first_failures.len() < FIRST_FAILURES {
            self.first_failures.push(message);
        }
    }
}

/// Prints and parses every record of the schema's table and counts the exact round-trips.
pub fn roundtrip_table(atlas: &Atlas, schema: &Schema) -> TableReport {
    let mut report = TableReport::default();
    let Some(table) = atlas.tables.get(&schema.table) else {
        report.failed += 1;
        report.first_failures.push(format!("no table `{}` in the atlas", schema.table));
        return report;
    };
    for entry in &table.entries {
        report.records += 1;
        let label = record_id(schema, entry);
        let text = print(schema, entry);
        match parse(schema, &text) {
            Ok((id, value)) if id == label && value == entry.value => report.exact += 1,
            Ok(_) => report.fail(format!("{label}: print/parse changed the record value")),
            Err(e) => report.fail(format!("{label}: {e}")),
        }
    }
    report
}

/// Character reader that keeps the line number, so errors can name it.
struct Parser<'a> {
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
    line: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Parser { chars: text.chars(), peeked: None, line: 1 }
    }

    fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.chars.next();
        }
        self.peeked
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        self.peeked = None;
        if c == Some('\n') {
            self.line += 1;
        }
        c
    }

    /// Whitespace and the record separators (`,` `;`).
    fn skip_gaps(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == ',' || c == ';' {
                self.bump();
            } else {
                break;
            }
        }
    }

    /// A bare token: a field name, a kind, an id, or a number / identifier value.
    fn word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '+') {
                word.push(c);
                self.bump();
            } else {
                break;
            }
        }
        word
    }

    /// A quoted string; the opening quote is next.
    fn string(&mut self, line: usize) -> Result<String, String> {
        self.bump();
        let mut text = String::new();
        loop {
            match self.bump() {
                None => return Err(format!("line {line}: unterminated string")),
                Some('"') => return Ok(text),
                Some('\\') => match self.bump() {
                    Some('"') => text.push('"'),
                    Some('\\') => text.push('\\'),
                    Some('n') => text.push('\n'),
                    Some(c) => return Err(format!("line {}: unknown escape `\\{c}`", self.line)),
                    None => return Err(format!("line {line}: unterminated string")),
                },
                Some(c) => text.push(c),
            }
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        let line = self.line;
        match self.peek() {
            None => Err(format!("line {line}: expected a value")),
            Some('"') => Ok(Value::Str(self.string(line)?)),
            Some('[') => {
                self.bump();
                Ok(Value::List(self.items(']')?))
            }
            Some('(') => {
                self.bump();
                Ok(Value::Tuple(self.items(')')?))
            }
            Some(_) => {
                let word = self.word();
                if word.is_empty() {
                    return Err(format!("line {line}: expected a value"));
                }
                Ok(number_or_ident(&word))
            }
        }
    }

    /// The values of a list `[…]` or tuple `(…)`; the opening bracket is already read.
    fn items(&mut self, close: char) -> Result<Vec<Value>, String> {
        let mut items = Vec::new();
        loop {
            self.skip_gaps();
            match self.peek() {
                None => return Err(format!("line {}: expected `{close}`", self.line)),
                Some(c) if c == close => {
                    self.bump();
                    return Ok(items);
                }
                Some(_) => items.push(self.value()?),
            }
        }
    }
}

/// A bare token is a decimal number when the whole token is one, an identifier otherwise.
fn number_or_ident(word: &str) -> Value {
    match number(word) {
        Some(value) => value,
        None => Value::Ident(word.to_owned()),
    }
}

fn number(word: &str) -> Option<Value> {
    let b = word.as_bytes();
    let mut i = 0;
    if b.first() == Some(&b'-') {
        i += 1;
    }
    let digits = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits {
        return None;
    }
    let mut float = false;
    if b.get(i) == Some(&b'.') {
        i += 1;
        let digits = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == digits {
            return None;
        }
        float = true;
    }
    if matches!(b.get(i), Some(b'e' | b'E')) {
        i += 1;
        if matches!(b.get(i), Some(b'+' | b'-')) {
            i += 1;
        }
        let digits = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == digits {
            return None;
        }
        float = true;
    }
    if i != b.len() {
        return None;
    }
    Some(if float { Value::Float(word.to_owned()) } else { Value::Int(word.to_owned()) })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tiny atlas shaped like the real dump, with every value shape the format has to keep:
    /// a tuple, a list, a negative int, a 2^200 int, a float, a string with a quote, an extra
    /// value past the schema and a one-element tuple.
    const TINY: &str = r#"{
      "schema": 1, "kit_hash": "records",
      "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_","entries":[
          {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":26,
           "value":[-7,1606938044258990275541962092341162602522202993782792835301376,0.5,
                    "He said \"hi\"\nbye",{"list":[1,2]},[3,"in,ner"],"extra",[56]]},
          {"index":1,"name":"forest_bandits","id":"fac_forest_bandits","dup":null,"line":30,
           "value":[0,1,0.0,"plain",{"list":[]}]}
        ]}
      }
    }"#;

    const SCHEMA: &str = r#"
table = "factions"
kind = "faction"
id_prefix = "fac_"

[[fields]]
name = "signed"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "big"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "ratio"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "title"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "members"
ty = "raw"
source = "module_factions.py:7"

[[fields]]
name = "pair"
ty = "raw"
optional = true
source = "module_factions.py:7"
"#;

    fn atlas() -> Atlas {
        Atlas::from_str(TINY).expect("tiny atlas loads")
    }

    fn schema() -> Schema {
        Schema::from_toml(SCHEMA).expect("schema loads")
    }

    #[test]
    fn values_survive_the_json_conversion() {
        let text = r#"[-7, 1606938044258990275541962092341162602522202993782792835301376, 0.5,
                        "He said \"hi\"\nbye", {"list":[1,2]}, [3,"in,ner"], [[56]], null, true, false]"#;
        let value: Json = serde_json::from_str(text).expect("json");
        assert_eq!(to_json(&from_json(&value)), value);
        // A JSON string that looks like a literal stays a string, not null / true / false.
        let strings: Json = serde_json::from_str(r#"["null","true","false"]"#).expect("json");
        assert_eq!(to_json(&from_json(&strings)), strings);
    }

    #[test]
    fn print_writes_every_value_shape() {
        let atlas = atlas();
        let text = print(&schema(), &atlas.tables["factions"].entries[0]);
        assert_eq!(
            text,
            concat!(
                "faction no_faction {\n",
                "  signed -7\n",
                "  big 1606938044258990275541962092341162602522202993782792835301376\n",
                "  ratio 0.5\n",
                "  title \"He said \\\"hi\\\"\\nbye\"\n",
                "  members [1, 2]\n",
                "  pair (3, \"in,ner\")\n",
                "  field_7 extra\n",
                "  field_8 (56,)\n",
                "}\n",
            )
        );
    }

    #[test]
    fn parse_reads_the_printed_text_back() {
        let atlas = atlas();
        let entry = &atlas.tables["factions"].entries[0];
        let (id, value) = parse(&schema(), &print(&schema(), entry)).expect("parses");
        assert_eq!(id, "no_faction");
        assert_eq!(value, entry.value);
    }

    #[test]
    fn roundtrip_counts_two_exact_records() {
        let report = roundtrip_table(&atlas(), &schema());
        assert_eq!((report.records, report.exact, report.failed), (2, 2, 0), "{:?}", report.first_failures);
    }

    #[test]
    fn a_missing_optional_trailing_field_is_allowed_and_a_required_one_is_not() {
        let schema = schema();
        let (id, value) =
            parse(&schema, "faction forest_bandits {\n  signed 0\n  big 1\n  ratio 0.0\n  title \"plain\"\n  members []\n}\n")
                .expect("optional tail may be absent");
        assert_eq!(id, "forest_bandits");
        assert_eq!(value, serde_json::json!([0, 1, 0.0, "plain", {"list": []}]));
        let err = parse(&schema, "faction x {\n  big 1\n  ratio 1.5\n  title \"t\"\n  members []\n  pair (1,)\n}\n")
            .expect_err("a required field cannot be missing");
        assert!(err.contains("line 7") && err.contains("signed"), "{err}");
    }

    #[test]
    fn parse_errors_name_the_line() {
        let schema = schema();
        let err = parse(&schema, "faction x {\n  signed -7\n  big\n}\n").expect_err("no value");
        assert!(err.contains("line 3") && err.contains("`big`"), "{err}");
        let err = parse(&schema, "faction x {\n  nope 1\n}\n").expect_err("unknown field");
        assert!(err.contains("line 2: unknown field `nope`"), "{err}");
        let err = parse(&schema, "troop x {\n}\n").expect_err("wrong kind");
        assert!(err.contains("line 1") && err.contains("`faction`"), "{err}");
    }

    #[test]
    fn schema_rejects_a_type_other_than_raw() {
        let text = SCHEMA.replacen("ty = \"raw\"", "ty = \"int\"", 1);
        let err = Schema::from_toml(&text).expect_err("only raw in round 1");
        assert!(err.contains("`signed`") && err.contains("raw"), "{err}");
    }

    #[test]
    fn pilot_schema_has_the_seven_kit_fields() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/factions.toml");
        let schema = Schema::from_path(path).expect("factions schema loads");
        assert_eq!((schema.table.as_str(), schema.kind.as_str(), schema.id_prefix.as_str()),
                   ("factions", "faction", "fac_"));
        let names: Vec<&str> = schema.fields.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            ["id", "name", "faction_flags", "coherence", "relations", "ranks", "faction_color"]
        );
        assert!(schema.fields.iter().all(|f| f.ty == "raw" && f.source.starts_with("module_factions.py:")));
        assert!(schema.fields.last().expect("color").optional);
    }

    #[test]
    fn real_atlas_factions_round_trip() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let schema = Schema::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/../../kit-meta/records/factions.toml"))
            .expect("factions schema loads");
        let report = roundtrip_table(&atlas, &schema);
        assert_eq!(report.records, 34);
        assert_eq!(report.exact, 34);
        assert_eq!(report.failed, 0, "{:?}", report.first_failures);
    }
}
