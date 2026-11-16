//! Native records as UW record text (docs/meclis/YON2-KARAR.md §2): every atlas entry is printed
//! as `<kind> <id> { <field> <value> … }` and parsed back to the identical atlas value.
//!
//! Round 1 is RAW: values are written exactly as the atlas holds them (no flag, reference or
//! packed-integer decoding). Field names and their order come from `kit-meta/records/<table>.toml`,
//! whose `source` points at the kit's layout comment for each field.
//!
//! A field may be typed (`ref:<table>`, `flags:<prefix>`, `bits`, `list:<type>`); its value then
//! prints readably and reads back to the identical atlas value, or stays raw when that check fails.

use std::path::Path;

use serde::Deserialize;
use serde_json::Value as Json;
use uw_atlas::{Atlas, Entry};

use crate::{items, RawArg, RawOp};

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
    /// Named values of one typed field: `{a 1, b 2}` (attributes, flags with a `rest`).
    Struct(Vec<(String, Value)>),
    /// An operation block, as the compiler handed it back; only [`parse_with_blocks`] builds one.
    Block(Json),
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
        Value::Struct(pairs) => {
            let mut map = serde_json::Map::new();
            for (name, value) in pairs {
                map.insert(name.clone(), to_json(value));
            }
            Json::Object(map)
        }
        Value::Block(json) => json.clone(),
    }
}

/// Writes an operation block back as atlas JSON; the inverse of [`crate::parse_ops`] up to the
/// spellings that function normalises (list statements, one-element tuple operands).
pub fn ops_to_json(ops: &[RawOp]) -> Json {
    let statements = ops
        .iter()
        .map(|op| {
            let mut parts = Vec::with_capacity(1 + op.args.len());
            parts.push(Json::from(op.code));
            parts.extend(op.args.iter().map(|arg| match arg {
                RawArg::Int(n) => Json::from(*n),
                RawArg::Str(s) => Json::String(s.clone()),
            }));
            Json::Array(parts)
        })
        .collect();
    let mut map = serde_json::Map::new();
    map.insert("list".into(), Json::Array(statements));
    Json::Object(map)
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
    /// Field type: `raw`, `ref:<table>`, `flags:<prefix>[,<prefix>…]`, `bits` or `list:<type>`.
    pub ty: String,
    /// Trailing fields may be absent from a record.
    #[serde(default)]
    pub optional: bool,
    /// Where the kit says this: `<file>:<line>`.
    pub source: String,
    /// `flags:` enum groups: `value & mask` names one constant with `prefix`.
    #[serde(default)]
    pub enums: Vec<FlagEnum>,
    /// `flags:` the constant whose value is 0 (`tf_male`); printed instead of an empty list.
    #[serde(default)]
    pub zero: Option<String>,
    /// `bits:` the packed parts, in printing order.
    #[serde(default)]
    pub parts: Vec<BitPart>,
}

/// One enum group of a `flags:` field.
#[derive(Debug, Clone, Deserialize)]
pub struct FlagEnum {
    /// Bit mask, as a TOML integer or a kit constant name.
    pub mask: toml::Value,
    /// Constant-name prefix of the values in this group.
    pub prefix: String,
}

/// One part of a `bits:` field.
#[derive(Debug, Clone, Deserialize)]
pub struct BitPart {
    pub name: String,
    pub shift: u32,
    pub width: u32,
    /// Where the kit says this: `<file>:<line>`.
    pub source: String,
    /// Optional inner type for the extracted value (for example `ref:factions`).
    #[serde(default)]
    pub ty: Option<String>,
}

/// A field type, checked at schema load and used by the printer and the parser.
#[derive(Debug, Clone)]
enum Ty {
    Raw,
    Ref(String),
    Flags(Vec<String>),
    Bits,
    List(Box<Ty>),
}

/// Reads a field type string; `field` supplies the parts (`bits`) and flags settings.
fn parse_ty(text: &str, field: &Field) -> Result<Ty, String> {
    if text == "raw" {
        return Ok(Ty::Raw);
    }
    if text == "bits" {
        if field.parts.is_empty() {
            return Err("`bits` needs [[fields.parts]] entries".into());
        }
        return Ok(Ty::Bits);
    }
    if let Some(table) = text.strip_prefix("ref:") {
        if table.is_empty() {
            return Err("`ref:` needs a table name".into());
        }
        return Ok(Ty::Ref(table.to_string()));
    }
    if let Some(list) = text.strip_prefix("flags:") {
        let prefixes: Vec<String> =
            list.split(',').map(str::trim).filter(|p| !p.is_empty()).map(str::to_string).collect();
        if prefixes.is_empty() {
            return Err("`flags:` needs at least one constant prefix".into());
        }
        return Ok(Ty::Flags(prefixes));
    }
    if let Some(inner) = text.strip_prefix("list:") {
        return Ok(Ty::List(Box::new(parse_ty(inner, field)?)));
    }
    Err(format!(
        "type `{text}` is not supported (allowed: `raw`, `ref:<table>`, `flags:<prefix>`, `bits`, `list:<type>`)"
    ))
}

/// The field's type; schemas are checked by [`Schema::from_toml`], so an unknown type is `raw`.

/// The field layout of one atlas table (`kit-meta/records/<table>.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    pub table: String,
    /// Record word (`faction`, `troop`, …).
    pub kind: String,
    /// Prefix stripped from the atlas id (`fac_`).
    pub id_prefix: String,
    /// The kit keeps each record as a Python list (`{"list": …}` in the atlas), not a tuple.
    /// A table fact (troops, items, dialogs, animations), so the record text does not repeat it.
    #[serde(default)]
    pub list: bool,
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

    /// Parses and checks schema text: field order, unique names and the supported field types.
    pub fn from_toml(text: &str) -> Result<Schema, String> {
        let schema: Schema = toml::from_str(text).map_err(|e| format!("schema: {e}"))?;
        for (i, field) in schema.fields.iter().enumerate() {
            parse_ty(&field.ty, field).map_err(|e| format!("field `{}`: {e}", field.name))?;
            for (j, part) in field.parts.iter().enumerate() {
                if part.width == 0 {
                    return Err(format!("field `{}`: part `{}` has width 0", field.name, part.name));
                }
                if field.parts[..j].iter().any(|p| p.name == part.name) {
                    return Err(format!("field `{}`: part `{}` appears twice", field.name, part.name));
                }
                if let Some(ty) = &part.ty {
                    parse_ty(ty, field)
                        .map_err(|e| format!("field `{}`: part `{}`: {e}", field.name, part.name))?;
                }
            }
            for group in &field.enums {
                if group.prefix.is_empty() {
                    return Err(format!("field `{}`: enum prefix is empty", field.name));
                }
            }
            if schema.fields[..i].iter().any(|f| f.name == field.name) {
                return Err(format!("field `{}` appears twice", field.name));
            }
        }
        Ok(schema)
    }
}

/// The id a record is printed with: the atlas id without the schema's prefix. Tables whose records
/// have no id in the kit (triggers, simple triggers, skins) use the record's index.
fn record_id(schema: &Schema, entry: &Entry) -> String {
    match entry.id.as_deref().or(entry.name.as_deref()) {
        Some(id) if !id.is_empty() => id.strip_prefix(&schema.id_prefix).unwrap_or(id).to_owned(),
        _ => entry.index.to_string(),
    }
}

/// A bare id reads back as one `word`; anything else (dialog `a:b`, string `x_=_y`) is quoted.
fn is_word(id: &str) -> bool {
    !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-' | '+'))
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
/// This raw form does not decode field types; [`print_with_blocks`] does, given the atlas.
pub fn print(schema: &Schema, entry: &Entry) -> String {
    print_record(None, schema, entry, &[], &mut |_| unreachable!("no block paths were given"))
}

/// Prints one atlas entry like [`print`], but every value at a `paths` position prints as an
/// operation block: `ops {`, the block text `print_block` returns indented four more spaces, `}`.
///
/// A path counts list/tuple positions from the record value root, with `{"list": …}` wrappers not
/// counted, the same walk [`crate::extract`] uses. Values at no path keep their raw text.
pub fn print_with_blocks(
    atlas: &Atlas,
    schema: &Schema,
    entry: &Entry,
    paths: &[Vec<usize>],
    print_block: &mut dyn FnMut(&Json) -> String,
) -> String {
    print_record(Some(atlas), schema, entry, paths, print_block)
}

fn print_record(
    atlas: Option<&Atlas>,
    schema: &Schema,
    entry: &Entry,
    paths: &[Vec<usize>],
    print_block: &mut dyn FnMut(&Json) -> String,
) -> String {
    let id = record_id(schema, entry);
    let mut out = format!("{} ", schema.kind);
    if is_word(&id) {
        out.push_str(&id);
    } else {
        write_str(&id, &mut out);
    }
    out.push_str(" {\n");
    let fields = record_values(&entry.value).len();
    let seq = items(&entry.value);
    for i in 0..fields {
        let name = match schema.fields.get(i) {
            Some(field) => field.name.clone(),
            None => format!("field_{}", i + 1),
        };
        out.push_str("  ");
        out.push_str(&name);
        out.push(' ');
        let value = seq.and_then(|items| items.get(i)).unwrap_or(&entry.value);
        let under_block = paths.iter().any(|path| path.first() == Some(&i));
        let typed = match (atlas, schema.fields.get(i)) {
            (Some(atlas), Some(field)) if !under_block && field.ty != "raw" => {
                typed_value(atlas, field, &from_json(value))
            }
            _ => None,
        };
        match typed {
            Some(readable) => write_value(&readable, &mut out),
            None => write_json_value(value, &mut vec![i], paths, print_block, 2, &mut out),
        }
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

/// Writes one atlas value, printing the value itself as an `ops { … }` block when `path` names
/// one of `paths` and recursing into lists and tuples otherwise.
fn write_json_value(
    value: &Json,
    path: &mut Vec<usize>,
    paths: &[Vec<usize>],
    print_block: &mut dyn FnMut(&Json) -> String,
    indent: usize,
    out: &mut String,
) {
    if paths.iter().any(|block| block == path) {
        write_block(&print_block(value), indent, out);
        return;
    }
    let (open, close, items) = match value {
        Json::Array(items) => ('(', ')', Some(items)),
        Json::Object(map) => match map.get("list").and_then(Json::as_array) {
            Some(items) => ('[', ']', Some(items)),
            None => (' ', ' ', None),
        },
        _ => (' ', ' ', None),
    };
    let Some(items) = items else {
        write_value(&from_json(value), out);
        return;
    };
    out.push(open);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        path.push(i);
        write_json_value(item, path, paths, print_block, indent + 2, out);
        path.pop();
    }
    // A one-element tuple is `(a,)`, like [`write_seq`].
    if open == '(' && items.len() == 1 {
        out.push(',');
    }
    out.push(close);
}

/// `ops {`, the block text one level deeper, `}`.
fn write_block(text: &str, indent: usize, out: &mut String) {
    out.push_str("ops {\n");
    for line in text.lines() {
        if !line.is_empty() {
            out.push_str(&" ".repeat(indent + 4));
            out.push_str(line);
        }
        out.push('\n');
    }
    out.push_str(&" ".repeat(indent));
    out.push('}');
}

fn write_value(value: &Value, out: &mut String) {
    match value {
        Value::Int(text) | Value::Float(text) | Value::Ident(text) => out.push_str(text),
        Value::Str(text) => write_str(text, out),
        Value::List(items) => write_seq('[', ']', items, out),
        Value::Tuple(items) => write_seq('(', ')', items, out),
        Value::Struct(pairs) => write_struct(pairs, out),
        // Only `parse_with_blocks` builds one; printing takes blocks from the atlas by path
        // before it converts a value, so this is not reached by `print`.
        Value::Block(json) => out.push_str(&json.to_string()),
    }
}

fn write_struct(pairs: &[(String, Value)], out: &mut String) {
    out.push('{');
    for (i, (name, value)) in pairs.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        out.push_str(name);
        out.push(' ');
        write_value(value, out);
    }
    out.push('}');
}

/// A one-pair `rest` struct prints bare inside a list: `rest 123`, not `{rest 123}`.
fn is_rest_pair(value: &Value) -> Option<&Value> {
    match value {
        Value::Struct(pairs) if pairs.len() == 1 && pairs[0].0 == "rest" => Some(&pairs[0].1),
        _ => None,
    }
}

fn write_seq(open: char, close: char, items: &[Value], out: &mut String) {
    out.push(open);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        match is_rest_pair(item) {
            Some(rest) => {
                out.push_str("rest ");
                write_value(rest, out);
            }
            None => write_value(item, out),
        }
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

/// Parses record text back into `(id, atlas value)`. Errors name the line. This raw form does not
/// decode field types; [`parse_with_blocks`] does, given the atlas.
pub fn parse(schema: &Schema, text: &str) -> Result<(String, Json), String> {
    parse_record(None, schema, text, None)
}

/// Parses record text written by [`print_with_blocks`]: every `ops { … }` block is cut out
/// (strings and nested braces aware) and `compile` turns its inner text back into atlas JSON.
pub fn parse_with_blocks(
    atlas: &Atlas,
    schema: &Schema,
    text: &str,
    compile: &mut dyn FnMut(&str) -> Result<Json, String>,
) -> Result<(String, Json), String> {
    parse_record(Some(atlas), schema, text, Some(compile))
}

fn parse_record<'t, 'c>(
    atlas: Option<&Atlas>,
    schema: &Schema,
    text: &'t str,
    compile: Option<&'c mut dyn FnMut(&str) -> Result<Json, String>>,
) -> Result<(String, Json), String> {
    let mut p = Parser::new(text, compile);
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
        let index = match schema.fields.iter().position(|f| f.name == name) {
            Some(index) => index,
            None => match name.strip_prefix("field_").and_then(|n| n.parse::<usize>().ok()) {
                Some(n) if n >= 1 => n - 1,
                _ => return Err(format!("line {field_line}: unknown field `{name}`")),
            },
        };
        let mut value = p.value()?;
        if let (Some(atlas), Some(field)) = (atlas, schema.fields.get(index)) {
            if field.ty != "raw" {
                if let Some(encoded) = encode_field(atlas, field, &value) {
                    value = encoded;
                }
            }
        }
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
    let items: Vec<Value> = values.into_iter().flatten().collect();
    let record = if schema.list { Value::List(items) } else { Value::Tuple(items) };
    Ok((id, to_json(&record)))
}

/// One printed text left at most per failing record kind; enough to see what broke.
const FIRST_FAILURES: usize = 5;

/// How one table's records did, in the style of the operation round-trip measure.
#[derive(Debug, Clone, Default)]
pub struct TableReport {
    pub records: usize,
    pub exact: usize,
    pub failed: usize,
    /// Schema fields with a readable type (`ref:`, `flags:`, `bits`, `list:`).
    pub typed_fields: usize,
    /// Schema fields in all.
    pub fields: usize,
    /// Values printed raw: raw fields, block-path fields and typed fallbacks.
    pub raw_rest: usize,
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
    report.fields = schema.fields.len();
    report.typed_fields = schema.fields.iter().filter(|field| field.ty != "raw").count();
    let Some(table) = atlas.tables.get(&schema.table) else {
        report.failed += 1;
        report.first_failures.push(format!("no table `{}` in the atlas", schema.table));
        return report;
    };
    for entry in &table.entries {
        report.records += 1;
        let label = record_id(schema, entry);
        report.raw_rest += raw_values(atlas, schema, entry, &[]);
        let text = print_record(Some(atlas), schema, entry, &[], &mut |_| unreachable!("no blocks"));
        match parse_record(Some(atlas), schema, &text, None) {
            Ok((id, value)) if id == label && value == entry.value => report.exact += 1,
            Ok(_) => report.fail(format!("{label}: print/parse changed the record value")),
            Err(e) => report.fail(format!("{label}: {e}")),
        }
    }
    report
}

/// How many values of `entry` print raw: raw fields, block-path fields and typed fields whose
/// decode did not encode back exactly.
fn raw_values(atlas: &Atlas, schema: &Schema, entry: &Entry, paths: &[Vec<usize>]) -> usize {
    let seq = items(&entry.value);
    let mut count = 0;
    for i in 0..record_values(&entry.value).len() {
        let value = seq.and_then(|items| items.get(i)).unwrap_or(&entry.value);
        let typed = match schema.fields.get(i) {
            Some(field) if field.ty != "raw" && !paths.iter().any(|path| path.first() == Some(&i)) => {
                typed_value(atlas, field, &from_json(value)).is_some()
            }
            _ => false,
        };
        if !typed {
            count += 1;
        }
    }
    count
}

/// Like [`roundtrip_table`], but every operation block at a `blocks` path prints as `ops { … }`
/// through `print_block` and reads back through `compile`. `blocks` may hold other tables'
/// blocks; only those whose table matches the schema are used.
pub fn roundtrip_table_with_blocks(
    atlas: &Atlas,
    schema: &Schema,
    blocks: &[crate::Block],
    print_block: &mut dyn FnMut(&Json) -> String,
    compile: &mut dyn FnMut(&str) -> Result<Json, String>,
) -> TableReport {
    let mut report = TableReport {
        fields: schema.fields.len(),
        typed_fields: schema.fields.iter().filter(|field| field.ty != "raw").count(),
        ..TableReport::default()
    };
    let Some(table) = atlas.tables.get(&schema.table) else {
        report.failed += 1;
        report.first_failures.push(format!("no table `{}` in the atlas", schema.table));
        return report;
    };
    let mut by_entry: std::collections::BTreeMap<usize, Vec<Vec<usize>>> = Default::default();
    for block in blocks.iter().filter(|block| block.table == schema.table) {
        by_entry.entry(block.index).or_default().push(block.path.clone());
    }
    for entry in &table.entries {
        report.records += 1;
        let label = record_id(schema, entry);
        let paths = by_entry.get(&entry.index).map(Vec::as_slice).unwrap_or(&[]);
        report.raw_rest += raw_values(atlas, schema, entry, paths);
        let text = print_with_blocks(atlas, schema, entry, paths, &mut *print_block);
        match parse_with_blocks(atlas, schema, &text, &mut *compile) {
            Ok((id, value)) if id == label => {
                match first_difference(&entry.value, &value, &mut Vec::new(), paths) {
                    None => report.exact += 1,
                    Some(diff) => {
                        report.fail(format!("{label}: print/parse changed the record value: {diff}"));
                    }
                }
            }
            Ok(_) => report.fail(format!("{label}: print/parse read another record id")),
            Err(e) => report.fail(format!("{label}: {e}")),
        }
    }
    report
}

/// `None` when `got` is the same record as `want`, comparing operation blocks at `paths` as
/// operations (the measure `roundtrip_table_with_blocks` uses); otherwise the first difference.
pub fn record_difference(want: &Json, got: &Json, paths: &[Vec<usize>]) -> Option<String> {
    first_difference(want, got, &mut Vec::new(), paths)
}

/// The first value `got` changed from `want`, described for a report; `None` when they are the
/// same record. At a block path the operations must match (the atlas spellings `parse_ops`
/// normalises are compared through it); elsewhere the value and the Python list-versus-tuple
/// shape must be identical.
fn first_difference(
    want: &Json,
    got: &Json,
    path: &mut Vec<usize>,
    paths: &[Vec<usize>],
) -> Option<String> {
    if paths.iter().any(|block| block == path) {
        let (want, got) = (crate::parse_ops(want), crate::parse_ops(got));
        return (want != got).then(|| format!("{path:?}: operations {want:?} became {got:?}"));
    }
    match (want, got) {
        (Json::Array(want_items), Json::Array(got_items)) => {
            first_item_difference(want_items, got_items, path, paths)
        }
        (Json::Object(want_map), Json::Object(got_map)) => match (want_map.get("list"), got_map.get("list")) {
            (Some(Json::Array(want_items)), Some(Json::Array(got_items)))
                if want_map.len() == 1 && got_map.len() == 1 =>
            {
                first_item_difference(want_items, got_items, path, paths)
            }
            _ => (want != got).then(|| format!("{path:?}: {} became {}", preview(want), preview(got))),
        },
        _ => (want != got).then(|| format!("{path:?}: {} became {}", preview(want), preview(got))),
    }
}

fn first_item_difference(
    want: &[Json],
    got: &[Json],
    path: &mut Vec<usize>,
    paths: &[Vec<usize>],
) -> Option<String> {
    if want.len() != got.len() {
        return Some(format!("{path:?}: {} values became {}", want.len(), got.len()));
    }
    for (i, (want, got)) in want.iter().zip(got).enumerate() {
        path.push(i);
        let found = first_difference(want, got, path, paths);
        path.pop();
        if found.is_some() {
            return found;
        }
    }
    None
}

/// JSON text for a failure line, cut short so a huge block does not flood the report.
fn preview(value: &Json) -> String {
    let text = value.to_string();
    if text.chars().count() <= 160 {
        text
    } else {
        format!("{}…", text.chars().take(160).collect::<String>())
    }
}

/// Character reader that keeps the line number, so errors can name it.
struct Parser<'t, 'c> {
    chars: std::str::Chars<'t>,
    peeked: Option<char>,
    line: usize,
    /// Reads an `ops { … }` body back; `None` in the plain [`parse`], which has no blocks.
    compile: Option<&'c mut dyn FnMut(&str) -> Result<Json, String>>,
}

impl<'t, 'c> Parser<'t, 'c> {
    fn new(text: &'t str, compile: Option<&'c mut dyn FnMut(&str) -> Result<Json, String>>) -> Self {
        Parser { chars: text.chars(), peeked: None, line: 1, compile }
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

    /// Whitespace only; a pair inside a list is `rest 123` without a comma.
    fn skip_spaces(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
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
            Some('{') => {
                self.bump();
                Ok(Value::Struct(self.pairs()?))
            }
            Some(_) => {
                let word = self.word();
                if word.is_empty() {
                    return Err(format!("line {line}: expected a value"));
                }
                if word == "ops" && self.compile.is_some() {
                    self.skip_gaps();
                    if self.peek() == Some('{') {
                        self.bump();
                        let inner = self.block_text()?;
                        let compile = self.compile.as_mut().expect("checked just above");
                        return compile(&inner).map(Value::Block);
                    }
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
                Some(_) => {
                    let value = self.value()?;
                    // A flag `rest` is one pair, written `rest 123` without a comma.
                    if value == Value::Ident("rest".into()) {
                        self.skip_spaces();
                        if !matches!(self.peek(), None | Some(',') | Some(';')) && self.peek() != Some(close) {
                            let rest = self.value()?;
                            items.push(Value::Struct(vec![("rest".into(), rest)]));
                            continue;
                        }
                    }
                    items.push(value);
                }
            }
        }
    }

    /// The pairs of `{name value, …}`; the opening brace is already read.
    fn pairs(&mut self) -> Result<Vec<(String, Value)>, String> {
        let mut pairs = Vec::new();
        loop {
            self.skip_gaps();
            match self.peek() {
                None => return Err(format!("line {}: expected `}}`", self.line)),
                Some('}') => {
                    self.bump();
                    return Ok(pairs);
                }
                Some(_) => {}
            }
            let line = self.line;
            let name = self.word();
            if name.is_empty() {
                return Err(format!("line {line}: expected a field name"));
            }
            self.skip_gaps();
            if self.peek() == Some('}') {
                return Err(format!("line {line}: field `{name}` has no value"));
            }
            let value = self.value()?;
            pairs.push((name, value));
        }
    }

    /// The body of `ops { … }`; the opening brace is already read. Stops at the matching `}`,
    /// ignoring braces inside strings, and returns the text between the braces.
    fn block_text(&mut self) -> Result<String, String> {
        let open_line = self.line;
        let mut text = String::new();
        let mut depth = 0usize;
        let mut quoted = false;
        loop {
            match self.bump() {
                None => return Err(format!("line {open_line}: unterminated `ops {{` block")),
                Some(c) if quoted => {
                    text.push(c);
                    if c == '\\' {
                        // The escaped character is part of the string; keep it verbatim.
                        match self.bump() {
                            Some(escaped) => text.push(escaped),
                            None => {
                                return Err(format!("line {open_line}: unterminated `ops {{` block"));
                            }
                        }
                    } else if c == '"' {
                        quoted = false;
                    }
                }
                Some('"') => {
                    quoted = true;
                    text.push('"');
                }
                Some('{') => {
                    depth += 1;
                    text.push('{');
                }
                Some('}') if depth == 0 => return Ok(text),
                Some('}') => {
                    depth -= 1;
                    text.push('}');
                }
                Some(c) => text.push(c),
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

/// A readable value for a typed field, checked both ways: `None` (the field prints raw) when the
/// readable form does not encode back to exactly the atlas value.
fn typed_value(atlas: &Atlas, field: &Field, original: &Value) -> Option<Value> {
    let ty = parse_ty(&field.ty, field).ok()?;
    let readable = decode_ty(atlas, &ty, field, original)?;
    let back = encode_ty(atlas, &ty, field, &readable)?;
    (back == *original).then_some(readable)
}

/// The atlas value a printed readable field reads back to; `None` keeps the parsed value as it is.
fn encode_field(atlas: &Atlas, field: &Field, value: &Value) -> Option<Value> {
    let ty = parse_ty(&field.ty, field).ok()?;
    encode_ty(atlas, &ty, field, value)
}

fn decode_ty(atlas: &Atlas, ty: &Ty, field: &Field, value: &Value) -> Option<Value> {
    match ty {
        Ty::Raw => Some(value.clone()),
        Ty::Ref(table) => decode_ref(atlas, table, value),
        Ty::Flags(prefixes) => decode_flags(atlas, field, prefixes, value),
        Ty::Bits => decode_bits(atlas, field, value),
        Ty::List(inner) => match value {
            Value::List(items) => Some(Value::List(decode_items(atlas, inner, field, items)?)),
            Value::Tuple(items) => Some(Value::Tuple(decode_items(atlas, inner, field, items)?)),
            _ => None,
        },
    }
}

fn encode_ty(atlas: &Atlas, ty: &Ty, field: &Field, value: &Value) -> Option<Value> {
    match ty {
        Ty::Raw => Some(value.clone()),
        Ty::Ref(table) => encode_ref(atlas, table, value),
        Ty::Flags(prefixes) => encode_flags(atlas, field, prefixes, value),
        Ty::Bits => encode_bits(atlas, field, value),
        Ty::List(inner) => match value {
            Value::List(items) => Some(Value::List(encode_items(atlas, inner, field, items)?)),
            Value::Tuple(items) => Some(Value::Tuple(encode_items(atlas, inner, field, items)?)),
            _ => None,
        },
    }
}

fn decode_items(atlas: &Atlas, ty: &Ty, field: &Field, items: &[Value]) -> Option<Vec<Value>> {
    items.iter().map(|item| decode_ty(atlas, ty, field, item)).collect()
}

fn encode_items(atlas: &Atlas, ty: &Ty, field: &Field, items: &[Value]) -> Option<Vec<Value>> {
    items.iter().map(|item| encode_ty(atlas, ty, field, item)).collect()
}

/// A table index to the atlas id of that entry (`3` -> `fac_kingdom_1`).
fn decode_ref(atlas: &Atlas, table: &str, value: &Value) -> Option<Value> {
    let Value::Int(text) = value else { return None };
    let index: usize = text.parse().ok()?;
    let entries = &atlas.tables.get(table)?.entries;
    let entry = entries.iter().find(|entry| entry.index == index)?;
    let id = entry.id.as_deref()?;
    if id.is_empty() || !is_word(id) {
        return None;
    }
    // The project's game name when it has one (`fac_kingdom_1` -> `swadia`, P5b).
    Some(Value::Ident(atlas.game_name(id).unwrap_or(id).to_string()))
}

/// An atlas id (or a project's game name for one) of that table back to its index.
fn encode_ref(atlas: &Atlas, table: &str, value: &Value) -> Option<Value> {
    let Value::Ident(id) = value else { return None };
    let id = atlas.reference_for(id).unwrap_or(id.as_str());
    let entries = &atlas.tables.get(table)?.entries;
    let entry = entries.iter().find(|entry| entry.id.as_deref() == Some(id))?;
    Some(Value::Int(entry.index.to_string()))
}

/// The integer a JSON constant holds, as a big unsigned integer.
fn json_big(value: &Json) -> Option<Big> {
    match value {
        Json::Number(number) => Big::parse(&number.to_string()),
        _ => None,
    }
}

/// The value of a kit constant by name (the last definition wins, like the atlas lookup).
fn constant_big(atlas: &Atlas, name: &str) -> Option<Big> {
    json_big(&atlas.constant(name)?.value)
}

/// The mask of one `flags` enum group: a TOML integer or a kit constant name.
fn enum_mask(atlas: &Atlas, group: &FlagEnum) -> Option<Big> {
    match &group.mask {
        toml::Value::Integer(value) if *value >= 0 => Some(Big::from_u64(*value as u64)),
        toml::Value::String(name) => constant_big(atlas, name),
        _ => None,
    }
}

fn decode_flags(atlas: &Atlas, field: &Field, prefixes: &[String], value: &Value) -> Option<Value> {
    let Value::Int(text) = value else { return None };
    let number = Big::parse(text)?;
    if number.is_zero() {
        if let Some(name) = &field.zero {
            if constant_big(atlas, name).is_some_and(|value| value.is_zero()) {
                return Some(Value::List(vec![Value::Ident(name.clone())]));
            }
        }
        return Some(Value::List(Vec::new()));
    }
    let mut remaining = number;
    let mut names: Vec<Value> = Vec::new();
    for group in &field.enums {
        let mask = enum_mask(atlas, group)?;
        let masked = remaining.and(&mask);
        if masked.is_zero() {
            continue;
        }
        let mut found: Option<&str> = None;
        let mut count = 0;
        for constant in &atlas.constants {
            if !constant.name.starts_with(&group.prefix) {
                continue;
            }
            let Some(constant_value) = json_big(&constant.value) else { continue };
            // Constants with bits outside the mask (tf_hero under troop_type_mask) do not count.
            if constant_value.and(&mask) != constant_value || constant_value != masked {
                continue;
            }
            count += 1;
            found = Some(&constant.name);
        }
        if count == 1 {
            names.push(Value::Ident(found.expect("one match").to_string()));
            remaining = remaining.sub(&masked)?;
        }
    }
    let mut singles: Vec<(Big, String)> = Vec::new();
    for constant in &atlas.constants {
        if !prefixes.iter().any(|prefix| constant.name.starts_with(prefix)) {
            continue;
        }
        let Some(constant_value) = json_big(&constant.value) else { continue };
        if constant_value.single_bit().is_none() || singles.iter().any(|(known, _)| *known == constant_value) {
            continue;
        }
        singles.push((constant_value, constant.name.clone()));
    }
    singles.sort_by(|a, b| a.0.cmp(&b.0));
    for (bit, name) in singles {
        if remaining.and(&bit) == bit {
            remaining = remaining.sub(&bit)?;
            names.push(Value::Ident(name));
        }
    }
    if !remaining.is_zero() {
        names.push(Value::Struct(vec![("rest".into(), Value::Int(remaining.to_decimal()))]));
    }
    Some(Value::List(names))
}

fn encode_flags(atlas: &Atlas, _field: &Field, _prefixes: &[String], value: &Value) -> Option<Value> {
    match value {
        // A raw integer (an unrecognised shape) stays raw.
        Value::Int(_) => None,
        Value::Ident(name) => Some(Value::Int(constant_big(atlas, name)?.to_decimal())),
        Value::List(items) => {
            let mut total = Big::zero();
            for item in items {
                match item {
                    Value::Ident(name) => {
                        total = total.or(&constant_big(atlas, name)?);
                    }
                    Value::Struct(pairs) if pairs.len() == 1 && pairs[0].0 == "rest" => {
                        let Value::Int(text) = &pairs[0].1 else { return None };
                        total = total.add(&Big::parse(text)?);
                    }
                    _ => return None,
                }
            }
            Some(Value::Int(total.to_decimal()))
        }
        _ => None,
    }
}

fn decode_bits(atlas: &Atlas, field: &Field, value: &Value) -> Option<Value> {
    let Value::Int(text) = value else { return None };
    let number = Big::parse(text)?;
    let mut covered = Big::zero();
    let mut pairs = Vec::new();
    for part in &field.parts {
        let mask = Big::one().shl(part.width).sub(&Big::one())?;
        let raw = number.shr(part.shift).and(&mask);
        let readable = match &part.ty {
            Some(text) => {
                let ty = parse_ty(text, field).ok()?;
                decode_ty(atlas, &ty, field, &Value::Int(raw.to_decimal()))?
            }
            None => Value::Int(raw.to_decimal()),
        };
        pairs.push((part.name.clone(), readable));
        covered = covered.or(&mask.shl(part.shift));
    }
    let rest = number.sub(&number.and(&covered))?;
    if !rest.is_zero() {
        pairs.push(("rest".into(), Value::Int(rest.to_decimal())));
    }
    Some(Value::Struct(pairs))
}

fn encode_bits(atlas: &Atlas, field: &Field, value: &Value) -> Option<Value> {
    let Value::Struct(pairs) = value else { return None };
    let mut seen = vec![false; field.parts.len()];
    let mut total = Big::zero();
    for (name, pair_value) in pairs {
        if name == "rest" {
            let Value::Int(text) = pair_value else { return None };
            total = total.add(&Big::parse(text)?);
            continue;
        }
        let index = field.parts.iter().position(|part| &part.name == name)?;
        if seen[index] {
            return None;
        }
        seen[index] = true;
        let part = &field.parts[index];
        let raw = match &part.ty {
            Some(text) => {
                let ty = parse_ty(text, field).ok()?;
                encode_ty(atlas, &ty, field, pair_value)?
            }
            None => pair_value.clone(),
        };
        let Value::Int(text) = raw else { return None };
        let part_value = Big::parse(&text)?;
        if !part_value.shr(part.width).is_zero() {
            return None;
        }
        total = total.or(&part_value.shl(part.shift));
    }
    if seen.iter().any(|seen| !seen) {
        return None;
    }
    Some(Value::Int(total.to_decimal()))
}

/// Unsigned arbitrary-precision integer for bit masks and shifts: 32-bit limbs, little-endian.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Big(Vec<u32>);

impl Big {
    fn zero() -> Big {
        Big(Vec::new())
    }

    fn one() -> Big {
        Big(vec![1])
    }

    fn from_u64(value: u64) -> Big {
        let mut limbs = vec![value as u32];
        let high = (value >> 32) as u32;
        if high != 0 {
            limbs.push(high);
        }
        Big::normalise(limbs)
    }

    fn normalise(mut limbs: Vec<u32>) -> Big {
        while limbs.last() == Some(&0) {
            limbs.pop();
        }
        Big(limbs)
    }

    fn parse(text: &str) -> Option<Big> {
        if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        let mut number = Big::zero();
        for byte in text.bytes() {
            number = number.mul_small(10).add_small((byte - b'0') as u32);
        }
        Some(number)
    }

    fn to_decimal(&self) -> String {
        if self.0.is_empty() {
            return "0".into();
        }
        let mut limbs = self.0.clone();
        let mut chunks = Vec::new();
        while !limbs.is_empty() {
            chunks.push(divide_small(&mut limbs, 1_000_000_000));
        }
        let mut out = chunks.pop().unwrap_or(0).to_string();
        for chunk in chunks.iter().rev() {
            out.push_str(&format!("{chunk:09}"));
        }
        out
    }

    fn mul_small(&self, factor: u32) -> Big {
        let mut out = Vec::with_capacity(self.0.len() + 1);
        let mut carry = 0u64;
        for &limb in &self.0 {
            let value = limb as u64 * factor as u64 + carry;
            out.push(value as u32);
            carry = value >> 32;
        }
        if carry != 0 {
            out.push(carry as u32);
        }
        Big::normalise(out)
    }

    fn add_small(&self, add: u32) -> Big {
        let mut out = self.0.clone();
        let mut carry = add as u64;
        let mut i = 0;
        while carry != 0 {
            if i == out.len() {
                out.push(0);
            }
            let value = out[i] as u64 + carry;
            out[i] = value as u32;
            carry = value >> 32;
            i += 1;
        }
        Big::normalise(out)
    }

    fn add(&self, other: &Big) -> Big {
        let len = self.0.len().max(other.0.len());
        let mut out = Vec::with_capacity(len + 1);
        let mut carry = 0u64;
        for i in 0..len {
            let value = self.0.get(i).copied().unwrap_or(0) as u64
                + other.0.get(i).copied().unwrap_or(0) as u64
                + carry;
            out.push(value as u32);
            carry = value >> 32;
        }
        if carry != 0 {
            out.push(carry as u32);
        }
        Big::normalise(out)
    }

    /// `self - other`; `None` when `other` is bigger (bit masks never need negative values).
    fn sub(&self, other: &Big) -> Option<Big> {
        if self.cmp(other) == std::cmp::Ordering::Less {
            return None;
        }
        let mut out = Vec::with_capacity(self.0.len());
        let mut borrow = 0i64;
        for i in 0..self.0.len() {
            let mut value = self.0[i] as i64 - borrow - other.0.get(i).copied().unwrap_or(0) as i64;
            if value < 0 {
                value += 1i64 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out.push(value as u32);
        }
        Some(Big::normalise(out))
    }

    fn and(&self, other: &Big) -> Big {
        let len = self.0.len().min(other.0.len());
        Big::normalise((0..len).map(|i| self.0[i] & other.0[i]).collect())
    }

    fn or(&self, other: &Big) -> Big {
        let len = self.0.len().max(other.0.len());
        Big::normalise(
            (0..len)
                .map(|i| self.0.get(i).copied().unwrap_or(0) | other.0.get(i).copied().unwrap_or(0))
                .collect(),
        )
    }

    fn shl(&self, bits: u32) -> Big {
        if self.0.is_empty() {
            return Big::zero();
        }
        let words = (bits / 32) as usize;
        let rem = bits % 32;
        let mut out = vec![0u32; words];
        if rem == 0 {
            out.extend_from_slice(&self.0);
        } else {
            let mut carry = 0u32;
            for &limb in &self.0 {
                out.push((limb << rem) | carry);
                carry = limb >> (32 - rem);
            }
            if carry != 0 {
                out.push(carry);
            }
        }
        Big::normalise(out)
    }

    fn shr(&self, bits: u32) -> Big {
        let words = (bits / 32) as usize;
        let rem = bits % 32;
        if words >= self.0.len() {
            return Big::zero();
        }
        let source = &self.0[words..];
        let mut out = Vec::with_capacity(source.len());
        if rem == 0 {
            out.extend_from_slice(source);
        } else {
            for (i, &limb) in source.iter().enumerate() {
                let low = limb >> rem;
                let high = source.get(i + 1).map(|next| next << (32 - rem)).unwrap_or(0);
                out.push(low | high);
            }
        }
        Big::normalise(out)
    }

    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }

    fn cmp(&self, other: &Big) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Equal => {
                for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
                    match a.cmp(b) {
                        Ordering::Equal => {}
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }

    /// The bit position when the value is exactly one bit; `None` for zero and multi-bit values.
    fn single_bit(&self) -> Option<u32> {
        let mut found = None;
        for (i, &limb) in self.0.iter().enumerate() {
            let mut rest = limb;
            while rest != 0 {
                if found.is_some() {
                    return None;
                }
                found = Some(i as u32 * 32 + rest.trailing_zeros());
                rest &= rest - 1;
            }
        }
        found
    }
}

/// One digit of a base-1e9 decomposition; strips the leading zero limbs of `limbs`.
fn divide_small(limbs: &mut Vec<u32>, divisor: u32) -> u32 {
    let mut remainder = 0u64;
    for i in (0..limbs.len()).rev() {
        let current = (remainder << 32) | limbs[i] as u64;
        limbs[i] = (current / divisor as u64) as u32;
        remainder = current % divisor as u64;
    }
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
    remainder as u32
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

    /// List-shaped records, records without an id, and ids that are not one bare word (dialog `a:b`,
    /// string `x_=_y`) all come back exact; the bugs the T-031..T-042 lanes reported.
    #[test]
    fn list_records_unnamed_records_and_odd_ids_round_trip() {
        let atlas = Atlas::from_str(r#"{
          "schema": 1, "kit_hash": "records", "ops": [], "hooks": [], "ranges": [],
          "tables": { "dialogs": {"module":"module_dialogs","var":"dialogs","id_file":"","id_prefix":"","entries":[
            {"index":0,"name":null,"id":"dlga_start:close_window","dup":null,"line":1,"value":{"list":[1,"start"]}},
            {"index":1,"name":null,"id":"str_a_=_b","dup":null,"line":2,"value":{"list":[2,"b"]}},
            {"index":2,"name":null,"id":null,"dup":null,"line":3,"value":{"list":[3,"c"]}}
          ]}}
        }"#).expect("atlas");
        let mut schema = Schema::from_toml(
            "table = \"dialogs\"\nkind = \"dialog\"\nid_prefix = \"\"\nlist = true\n\
             [[fields]]\nname = \"a\"\nty = \"raw\"\nsource = \"x:1\"\n\
             [[fields]]\nname = \"b\"\nty = \"raw\"\nsource = \"x:2\"\n",
        )
        .expect("schema");
        let entries = &atlas.tables["dialogs"].entries;
        assert!(print(&schema, &entries[0]).starts_with("dialog \"dlga_start:close_window\" {"));
        assert!(print(&schema, &entries[2]).starts_with("dialog 2 {"));
        let report = roundtrip_table(&atlas, &schema);
        assert_eq!((report.exact, report.failed), (3, 0), "{:?}", report.first_failures);
        // Without `list = true` the shape is lost and every record fails.
        schema.list = false;
        assert_eq!(roundtrip_table(&atlas, &schema).exact, 0);
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

    /// One troop whose fields exercise every readable type, plus the constants they name.
    const TYPED: &str = r#"{
      "schema": 1, "kit_hash": "typed", "ops": [], "hooks": [], "ranges": [],
      "constants": [
        {"name":"tf_male","value":0,"module":"header_troops","line":10},
        {"name":"tf_female","value":1,"module":"header_troops","line":11},
        {"name":"tf_undead","value":2,"module":"header_troops","line":12},
        {"name":"troop_type_mask","value":15,"module":"header_troops","line":13},
        {"name":"tf_hero","value":16,"module":"header_troops","line":14},
        {"name":"tf_mounted","value":1024,"module":"header_troops","line":19}
      ],
      "tables": {
        "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_","entries":[
          {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,"value":[0]},
          {"index":1,"name":"kingdom_1","id":"fac_kingdom_1","dup":null,"line":2,"value":[0]}
        ]},
        "troops": {"module":"module_troops","var":"troops","id_file":"ID_troops","id_prefix":"trp_","entries":[
          {"index":0,"name":"knight","id":"trp_knight","dup":null,"line":1,
           "value":{"list":["Knight","Knights",1041,0,0,1,{"list":[]},107374184469,{"list":[0,1]}]}}
        ]}
      }
    }"#;

    fn typed_schema() -> Schema {
        let raw = |name: &str, source: &str| {
            format!("[[fields]]\nname = \"{name}\"\nty = \"raw\"\nsource = \"{source}\"\n")
        };
        let text = format!(
            "table = \"troops\"\nkind = \"troop\"\nid_prefix = \"trp_\"\nlist = true\n\
             {}\
             {}\
             [[fields]]\nname = \"flags\"\nty = \"flags:tf_\"\nenums = [{{ mask = \"troop_type_mask\", prefix = \"tf_\" }}]\nzero = \"tf_male\"\nsource = \"header_troops.py:9\"\n\
             {} {} \
             [[fields]]\nname = \"faction\"\nty = \"ref:factions\"\nsource = \"module_troops.py:19\"\n\
             {}\
             [[fields]]\nname = \"attributes\"\nty = \"bits\"\nsource = \"module_troops.py:21\"\n\
             [[fields.parts]]\nname = \"str\"\nshift = 0\nwidth = 8\nsource = \"header_troops.py:90\"\n\
             [[fields.parts]]\nname = \"agi\"\nshift = 8\nwidth = 8\nsource = \"header_troops.py:119\"\n\
             [[fields.parts]]\nname = \"level\"\nshift = 32\nwidth = 8\nsource = \"header_troops.py:208\"\n\
             [[fields]]\nname = \"allies\"\nty = \"list:ref:factions\"\nsource = \"module_troops.py:20\"\n",
            raw("name", "module_troops.py:14"),
            raw("plural_name", "module_troops.py:15"),
            raw("scene", "module_troops.py:17"),
            raw("reserved", "module_troops.py:18"),
            raw("inventory", "module_troops.py:20"),
        );
        Schema::from_toml(&text).expect("typed schema loads")
    }

    fn typed_atlas() -> Atlas {
        Atlas::from_str(TYPED).expect("typed atlas loads")
    }

    #[test]
    fn typed_fields_print_readably_and_read_back_to_the_atlas_value() {
        let atlas = typed_atlas();
        let schema = typed_schema();
        let entry = &atlas.tables["troops"].entries[0];
        let text = print_with_blocks(&atlas, &schema, entry, &[], &mut |_| unreachable!("no blocks"));
        assert_eq!(
            text,
            concat!(
                "troop knight {\n",
                "  name \"Knight\"\n",
                "  plural_name \"Knights\"\n",
                "  flags [tf_female, tf_hero, tf_mounted]\n",
                "  scene 0\n",
                "  reserved 0\n",
                "  faction fac_kingdom_1\n",
                "  inventory []\n",
                "  attributes {str 21, agi 8, level 25}\n",
                "  allies [fac_no_faction, fac_kingdom_1]\n",
                "}\n",
            )
        );
        let mut compile = |text: &str| serde_json::from_str(text).map_err(|error: serde_json::Error| error.to_string());
        let (id, value) = parse_with_blocks(&atlas, &schema, &text, &mut compile).expect("typed text parses");
        assert_eq!(id, "knight");
        assert_eq!(value, entry.value);
        let report = roundtrip_table(&atlas, &schema);
        assert_eq!((report.records, report.exact, report.failed), (1, 1, 0), "{:?}", report.first_failures);
        assert_eq!((report.typed_fields, report.fields, report.raw_rest), (4, 9, 5));
    }

    #[test]
    fn unknown_bits_go_to_rest_and_a_bad_ref_stays_an_int() {
        let atlas = typed_atlas();
        let schema = typed_schema();
        let field = |name: &str| schema.fields.iter().find(|field| field.name == name).expect("field");

        // flags: the whole packed value, with bits no kit constant names.
        let flags = (1u64 << 40) + 1041;
        let readable = typed_value(&atlas, field("flags"), &Value::Int(flags.to_string())).expect("typed flags");
        assert_eq!(
            readable,
            Value::List(vec![
                Value::Ident("tf_female".into()),
                Value::Ident("tf_hero".into()),
                Value::Ident("tf_mounted".into()),
                Value::Struct(vec![("rest".into(), Value::Int((1u64 << 40).to_string()))]),
            ])
        );
        assert_eq!(
            encode_field(&atlas, field("flags"), &readable),
            Some(Value::Int(flags.to_string()))
        );
        // Zero is the kit's zero constant.
        assert_eq!(
            typed_value(&atlas, field("flags"), &Value::Int("0".into())),
            Some(Value::List(vec![Value::Ident("tf_male".into())]))
        );

        // bits: one value outside every part lands in `rest <n>`.
        let attributes = 107374184469u64 + (1u64 << 40);
        let readable = typed_value(&atlas, field("attributes"), &Value::Int(attributes.to_string())).expect("bits");
        let Value::Struct(pairs) = &readable else { panic!("bits is a struct: {readable:?}") };
        assert_eq!(pairs.last(), Some(&("rest".to_string(), Value::Int((1u64 << 40).to_string()))));
        assert_eq!(
            encode_field(&atlas, field("attributes"), &readable),
            Some(Value::Int(attributes.to_string()))
        );

        // A mask written as a TOML integer means the same as a kit constant name.
        let group = FlagEnum { mask: toml::Value::Integer(15), prefix: "tf_".into() };
        assert_eq!(enum_mask(&atlas, &group), Some(Big::from_u64(15)));

        // ref: a negative, too big or non-integer value stays a plain integer.
        for raw in ["-1", "99", "not_an_int"] {
            assert_eq!(typed_value(&atlas, field("faction"), &Value::Int(raw.into())), None, "{raw}");
            assert_eq!(encode_field(&atlas, field("faction"), &Value::Int(raw.into())), None, "{raw}");
        }
        assert_eq!(
            typed_value(&atlas, field("faction"), &Value::Int("1".into())),
            Some(Value::Ident("fac_kingdom_1".into()))
        );

        // list keeps its shape: a tuple reads back as a tuple.
        let tuple = Value::Tuple(vec![Value::Int("0".into()), Value::Int("1".into())]);
        assert_eq!(
            typed_value(&atlas, field("allies"), &tuple),
            Some(Value::Tuple(vec![
                Value::Ident("fac_no_faction".into()),
                Value::Ident("fac_kingdom_1".into()),
            ]))
        );
    }

    #[test]
    fn a_struct_value_prints_inline_and_parses_back() {
        let value = Value::Struct(vec![
            ("a".into(), Value::Int("1".into())),
            ("b".into(), Value::Int("2".into())),
        ]);
        let mut out = String::new();
        write_value(&value, &mut out);
        assert_eq!(out, "{a 1, b 2}");
        let mut parser = Parser::new(&out, None);
        assert_eq!(parser.value().expect("struct parses"), value);
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
        // Since P5 (T-062) some fields are typed; every field still cites the kit.
        assert!(schema.fields.iter().all(|f| f.source.contains(".py:")));
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

    /// One menu with a top-level enter block and an option with an empty condition block and a
    /// consequence block.
    const MENUS: &str = r#"{
      "schema": 1, "kit_hash": "records", "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","entries":[
          {"index":0,"name":"start","id":"menu_start","dup":null,"line":10,
           "value":["start",0,"Start","none",{"list":[[2052],[1106,"@hi"]]},{"list":[
             ["go",{"list":[]},"Go",{"list":[[2060,"mnu_town"]]}]
           ]}]}
        ]}
      }
    }"#;

    fn menus_schema() -> Schema {
        Schema::from_toml(
            "table = \"menus\"\nkind = \"menu\"\nid_prefix = \"menu_\"\n\
             [[fields]]\nname = \"id\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n\
             [[fields]]\nname = \"flags\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n\
             [[fields]]\nname = \"text\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n\
             [[fields]]\nname = \"mesh\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n\
             [[fields]]\nname = \"operations\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n\
             [[fields]]\nname = \"options\"\nty = \"raw\"\nsource = \"module_game_menus.py:20\"\n",
        )
        .expect("menus schema loads")
    }

    /// A fake printer/compiler pair: a block's atlas JSON prints as compact JSON (braces and
    /// quoted text included) and reads back through `serde_json`.
    #[test]
    fn blocks_print_as_ops_and_parse_back() {
        let atlas = Atlas::from_str(MENUS).expect("tiny atlas loads");
        let schema = menus_schema();
        let entry = &atlas.tables["menus"].entries[0];
        let paths = vec![vec![4], vec![5, 0, 3]];
        let mut print_block = |block: &Json| block.to_string();
        let text = print_with_blocks(&atlas, &schema, entry, &paths, &mut print_block);
        assert!(text.contains("  operations ops {"), "{text}");
        assert!(text.contains("  options [(go, [], \"Go\", ops {"), "{text}");
        assert_eq!(text.matches("ops {").count(), 2, "{text}");

        let mut compile = |text: &str| serde_json::from_str(text).map_err(|e| e.to_string());
        let (id, value) = parse_with_blocks(&atlas, &schema, &text, &mut compile).expect("parses");
        assert_eq!(id, "start");
        assert_eq!(value, entry.value);

        // The same pair through `extract::all` (which also finds the empty condition block).
        let mut print_block = |block: &Json| block.to_string();
        let mut compile = |text: &str| serde_json::from_str(text).map_err(|e| e.to_string());
        let report = roundtrip_table_with_blocks(
            &atlas,
            &schema,
            &crate::extract::all(&atlas),
            &mut print_block,
            &mut compile,
        );
        assert_eq!((report.records, report.exact, report.failed), (1, 1, 0), "{:?}", report.first_failures);
    }

    /// The block body ends at its matching `}`, not at a brace inside a nested `if` or a string.
    #[test]
    fn an_ops_body_ends_at_its_matching_brace_and_ignores_strings() {
        let schema = Schema::from_toml(
            "table = \"menus\"\nkind = \"menu\"\nid_prefix = \"menu_\"\n\
             [[fields]]\nname = \"operations\"\nty = \"raw\"\nsource = \"x:1\"\n",
        )
        .expect("schema loads");
        let mut seen = String::new();
        let mut compile = |text: &str| {
            seen = text.to_string();
            Ok(serde_json::json!({"list": [[4]]}))
        };
        let text = "menu start {\n  operations ops {\n    if x { y() }\n    text \"} { \\\" }\"\n  }\n}\n";
        let atlas = Atlas::from_str(MENUS).expect("tiny atlas loads");
        let (id, value) = parse_with_blocks(&atlas, &schema, text, &mut compile).expect("parses");
        assert_eq!(id, "start");
        assert!(seen.contains("if x { y() }"), "{seen}");
        assert!(seen.trim_end().ends_with("text \"} { \\\" }\""), "{seen}");
        assert_eq!(value, serde_json::json!([{"list": [[4]]}]));
    }

    /// `ops_to_json` is read back by `parse_ops` as the same operations.
    #[test]
    fn ops_to_json_is_read_back_by_parse_ops() {
        let block: Json =
            serde_json::from_str(r#"{"list":[[31,"$x",3],4,[2147483679,":a","trp_player"]]}"#).expect("json");
        let ops = crate::parse_ops(&block).expect("a block");
        assert_eq!(crate::parse_ops(&ops_to_json(&ops)), Some(ops.clone()));
        // A plain-array block spelling: the normalised operations survive the trip.
        let array: Json = serde_json::from_str(r#"[[4],[31,1,0]]"#).expect("json");
        let ops = crate::parse_ops(&array).expect("a block");
        assert_eq!(crate::parse_ops(&ops_to_json(&ops)), Some(ops));
    }

    #[test]
    fn real_atlas_blocks_survive_ops_to_json() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/uw-staging/atlas.json");
        if !std::path::Path::new(path).exists() {
            eprintln!("skipped: no atlas at {path}");
            return;
        }
        let atlas = Atlas::from_path(path).expect("full atlas loads");
        let blocks = crate::extract::all(&atlas);
        assert!(!blocks.is_empty());
        for block in &blocks {
            assert_eq!(
                crate::parse_ops(&ops_to_json(&block.ops)),
                Some(block.ops.clone()),
                "{}",
                block.label
            );
        }
    }
}
