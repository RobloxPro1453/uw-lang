//! Names and shapes: every name in a parsed UW mod is looked up in the Native atlas (nothing is
//! guessed), and the mod becomes a list of checked patches for `uw_lower`.
//!
//! Faz 1 meaning (KARARNAMELER, Sözdizimi 2026-11-15):
//! - `override menu <m> { on enter { ... } }` replaces menu `<m>`'s on-enter operations
//!   (module_game_menus.py:25, field 5 = operations block).
//! - `on game.<hook> { ... }` appends to the engine hook script `game_<hook>`
//!   (module_scripts.py:22, field 2 = operation block).
//! - a statement is a Native operation from header_operations.py; a name argument is a Native
//!   reference (`p_town_6`, written into the operation as a string the kit resolves) or a kit
//!   constant (`reg0`, `tf_male`, imported from the header that defines it).

use std::collections::BTreeMap;

use uw_atlas::Atlas;
use uw_diag::{suggest, Diagnostic, SourceFile, Span};
use uw_syntax::{Arg, File, Item, Stmt};
#[allow(unused_imports)]
use uw_syntax::Call;

pub const E0001_UNKNOWN_NAME: &str = uw_diag::E0001_UNKNOWN_NAME;
pub const E0201_NOT_YET: &str = "E0201";
pub const E0202_DUPLICATE: &str = "E0202";
pub const E0203_TOO_FEW: &str = "E0203";
pub const E0204_TEXT: &str = "E0204";
pub const E0205_MOD_NAME: &str = "E0205";

/// A parsed source file.
pub struct Source {
    pub file: SourceFile,
    pub tree: File,
}

/// An error in source `file` (index into the sources given to [`check`]).
#[derive(Debug)]
pub struct Error {
    pub file: usize,
    pub diag: Diagnostic,
}

/// Values and operations are defined in `uw_lower` (it turns the checked tree into them).
pub use uw_lower::{Op, Value};

/// What a patch does to its entry. `path` (on [`Patch`]) is the field it works on; `ops` is the
/// lowered operation block where the action has one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Replace the operation block at `path` with `ops`.
    ReplaceBody,
    /// Append `ops` to the operation block at `path`.
    Append,
    /// Replace the text field at `path` (`text "…"`, menu field 2).
    ReplaceText(String),
    /// Add a new option to the menu's option list (`option …`).
    InsertOption { option: NewOption, place: OptionPlace },
    /// Change parts of an existing option (`change option …`); `None` keeps the Native part.
    ChangeOption { id: String, text: Option<String>, condition: Option<Vec<Op>>, consequence: Option<Vec<Op>> },
    /// Remove an existing option (`remove option …`).
    RemoveOption(String),
    /// A new menu appended to the list (`new menu_x …`); `entry` holds its bare name.
    NewMenu { text: String, enter: Vec<Op>, options: Vec<NewOption> },
}

/// A menu option written in UW: kit layout module_game_menus.py:28 (id, condition, text,
/// consequence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOption {
    /// Without `mno_` (the kit adds it).
    pub id: String,
    pub condition: Vec<Op>,
    pub text: String,
    pub consequence: Vec<Op>,
    pub origin: String,
}

/// Where a new option goes: the end of the list, or after/before an option id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptionPlace {
    End,
    After(String),
    Before(String),
}

/// One checked change to a Native entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patch {
    /// Python module of the table (`module_game_menus`).
    pub module: String,
    /// List variable in it (`game_menus`).
    pub var: String,
    /// What `uw_runtime.find` takes: the entry's name, `name#n` when the name repeats.
    pub entry: String,
    /// Field path to the operation block inside the entry.
    pub path: Vec<usize>,
    pub action: Action,
    pub ops: Vec<Op>,
    pub origin: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checked {
    pub name: String,
    pub patches: Vec<Patch>,
}

/// Field of the operation block: module_game_menus.py:25 (menus, 5th field),
/// module_scripts.py:22 (scripts, 2nd field).
const MENU_ENTER_FIELD: usize = 4;
const SCRIPT_BODY_FIELD: usize = 1;

fn diag(code: &'static str, message: String, span: Span, help: Option<String>) -> Diagnostic {
    Diagnostic { code, message, span, notes: Vec::new(), help }
}

/// Smallest number of values `op` needs according to its header comment
/// (`(name,<a>,<b>,[c])`: 2), when the comment has that form.
pub fn min_args(op: &uw_atlas::Op) -> Option<usize> {
    let sig = &op.signature;
    let head = format!("({}", op.name);
    let start = sig.find(&head)?;
    let rest = &sig[start + head.len()..];
    let rest = rest.trim_start();
    if !(rest.starts_with(',') || rest.starts_with(')')) {
        return None;
    }
    let close = rest.find(')')?;
    let inner = &rest[..close];
    let mut depth = 0i32;
    let mut required = 0;
    for c in inner.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            '<' if depth == 0 => required += 1,
            _ => {}
        }
    }
    Some(required)
}

struct Ctx<'a> {
    atlas: &'a Atlas,
    errors: Vec<Error>,
    file: usize,
    source: &'a SourceFile,
}

impl Ctx<'_> {
    fn error(&mut self, d: Diagnostic) {
        self.errors.push(Error { file: self.file, diag: d });
    }

    fn origin(&self, span: Span) -> String {
        let (line, _) = self.source.line_col(span.start);
        format!("{}:{line}", self.source.name)
    }

    /// Checks a block and lowers it to the operations the kit writes.
    fn ops(&mut self, stmts: &[Stmt]) -> Vec<Op> {
        let tree = self.stmts(stmts);
        uw_lower::lower(&tree)
    }

    /// Checks a block into the tree `uw_lower` lowers.
    fn stmts(&mut self, stmts: &[Stmt]) -> Vec<uw_lower::Stmt> {
        let mut out = Vec::new();
        for stmt in stmts {
            match stmt {
                Stmt::Call(call) => {
                    if let Some(op) = self.call(call) {
                        out.push(uw_lower::Stmt::Op(op));
                    }
                }
                // Checking assignments and `if` is task T-017.
                Stmt::Assign { span, .. } | Stmt::If { span, .. } => self.error(diag(
                    E0201_NOT_YET,
                    "assignments and `if` are parsed but not checked yet".into(),
                    *span,
                    None,
                )),
            }
        }
        out
    }

    /// Checks one operation call: the operation exists, has enough values, every value resolves.
    fn call(&mut self, stmt: &Call) -> Option<Op> {
        {
            let Some(op) = self.atlas.op(&stmt.op.text) else {
                let help = suggest(&stmt.op.text, self.atlas.ops.iter().map(|o| o.name.as_str()))
                    .map(|s| format!("did you mean '{s}'? (`uw browse` lists every operation)"));
                self.error(diag(
                    E0001_UNKNOWN_NAME,
                    format!("'{}' is not a Native operation", stmt.op.text),
                    stmt.op.span,
                    help,
                ));
                return None;
            };
            if let Some(min) = min_args(op) {
                if stmt.args.len() < min {
                    let mut d = diag(
                        E0203_TOO_FEW,
                        format!("'{}' needs at least {min} values, got {}", op.name, stmt.args.len()),
                        stmt.span,
                        Some(format!("Native: {}", op.signature.trim())),
                    );
                    if let Some(line) = op.line {
                        d.notes.push(format!("header_operations.py:{line}"));
                    }
                    self.error(d);
                }
            }
            let mut args = Vec::new();
            for arg in &stmt.args {
                match arg {
                    Arg::Int(n, _) => args.push(Value::Int(*n)),
                    Arg::Global(g) | Arg::Local(g) => args.push(Value::Ref(g.text.clone())),
                    Arg::Str(text, span) => {
                        if !text.is_ascii() {
                            self.error(diag(
                                E0204_TEXT,
                                "text with non-ASCII letters is not supported yet".into(),
                                *span,
                                Some("Turkish letters wait for game experiment D5; use plain letters for now".into()),
                            ));
                        }
                        args.push(Value::Text(text.clone()));
                    }
                    Arg::Name(name) => {
                        if self.atlas.resolve_ref(&name.text).is_some() {
                            args.push(Value::Ref(name.text.clone()));
                        } else if let Some(c) = self.atlas.constant(&name.text) {
                            args.push(Value::Const { name: c.name.clone(), module: c.module.clone() });
                        } else {
                            let refs: Vec<String> = self
                                .atlas
                                .tables
                                .values()
                                .filter_map(|t| {
                                    let tag = t.ref_tag.as_deref()?;
                                    Some(t.entries.iter().filter_map(move |e| {
                                        e.name.as_deref().map(|n| format!("{tag}_{}", n.to_lowercase()))
                                    }))
                                })
                                .flatten()
                                .collect();
                            let candidates = refs
                                .iter()
                                .map(String::as_str)
                                .chain(self.atlas.constants.iter().map(|c| c.name.as_str()));
                            let help = suggest(&name.text, candidates).map(|s| format!("did you mean '{s}'?"));
                            self.error(diag(
                                E0001_UNKNOWN_NAME,
                                format!("'{}' is not a Native name or kit constant", name.text),
                                name.span,
                                help.or_else(|| Some("`uw browse` finds Native names by part of the name or in-game text".into())),
                            ));
                        }
                    }
                }
            }
            Some(Op { name: op.name.clone(), args, origin: self.origin(stmt.span) })
        }
    }
}

/// `start_phase_2`, `menu_start_phase_2`, `mnu_start_phase_2`, `camp#2` -> (entry ref for the
/// runtime, label) of a menu.
fn find_menu(atlas: &Atlas, text: &str) -> Option<String> {
    let (base, occ) = match text.split_once('#') {
        Some((b, n)) => (b, Some(n)),
        None => (text, None),
    };
    let bare = base
        .strip_prefix("menu_")
        .or_else(|| base.strip_prefix("mnu_"))
        .unwrap_or(base)
        .to_lowercase();
    let table = atlas.tables.get("menus")?;
    let hits: Vec<&uw_atlas::Entry> = table
        .entries
        .iter()
        .filter(|e| e.name.as_deref().is_some_and(|n| n.to_lowercase() == bare))
        .collect();
    let entry = match occ {
        Some(n) => *hits.get(n.parse::<usize>().ok()?.checked_sub(1)?)?,
        None if hits.len() == 1 => hits[0],
        None => return None,
    };
    let name = entry.name.clone()?;
    Some(match entry.dup {
        Some(n) => format!("{name}#{n}"),
        None => name,
    })
}

/// Checks every source; on success the patches in source order.
pub fn check(atlas: &Atlas, sources: &[Source], fallback_name: &str) -> Result<Checked, Vec<Error>> {
    let mut errors = Vec::new();
    let mut name: Option<String> = None;
    let mut patches = Vec::new();
    let mut overridden: BTreeMap<String, String> = BTreeMap::new();

    for (index, source) in sources.iter().enumerate() {
        let mut ctx = Ctx { atlas, errors: Vec::new(), file: index, source: &source.file };
        if let Some(m) = &source.tree.module {
            match &name {
                Some(existing) if existing != &m.text => ctx.error(diag(
                    E0205_MOD_NAME,
                    format!("this mod is already named '{existing}' in another file"),
                    m.span,
                    Some("name the mod in one file only".into()),
                )),
                _ => name = Some(m.text.clone()),
            }
        }
        for item in &source.tree.items {
            match item {
                Item::Override { kind, name: target, handlers, members: _, span } => {
                    // `override menu_x` (kind from the prefix) and menu members: task T-017.
                    let Some(kind) = kind else {
                        ctx.error(diag(E0201_NOT_YET, "`override <id>` is parsed but not checked yet".into(), target.span, None));
                        continue;
                    };
                    if kind.text != "menu" {
                        ctx.error(diag(
                            E0201_NOT_YET,
                            format!("`override {}` is not in UW yet; Faz 1 has `override menu`", kind.text),
                            kind.span,
                            None,
                        ));
                        continue;
                    }
                    let Some(entry) = find_menu(atlas, &target.text) else {
                        let menus = atlas.tables.get("menus");
                        let names: Vec<String> = menus
                            .map(|t| t.entries.iter().filter_map(|e| e.name.clone()).collect())
                            .unwrap_or_default();
                        let help = suggest(&target.text, names.iter().map(String::as_str))
                            .map(|s| format!("did you mean '{s}'?"))
                            .or_else(|| Some("`uw browse menu_` lists the Native menus".into()));
                        ctx.error(diag(
                            E0001_UNKNOWN_NAME,
                            format!("'{}' is not a Native menu (or its name repeats; add #1, #2)", target.text),
                            target.span,
                            help,
                        ));
                        continue;
                    };
                    let origin = ctx.origin(*span);
                    if let Some(first) = overridden.insert(entry.clone(), origin.clone()) {
                        ctx.error(diag(
                            E0202_DUPLICATE,
                            format!("menu '{entry}' is already overridden at {first}"),
                            target.span,
                            Some("put both changes in one `override` block".into()),
                        ));
                        continue;
                    }
                    for handler in handlers {
                        if handler.event.text != "enter" {
                            let help = suggest(&handler.event.text, ["enter"]).map(|s| format!("did you mean '{s}'?"));
                            ctx.error(diag(
                                E0201_NOT_YET,
                                format!("`on {}` is not in UW yet; menus have `on enter`", handler.event.text),
                                handler.event.span,
                                help,
                            ));
                            continue;
                        }
                        let ops = ctx.ops(&handler.body.stmts);
                        patches.push(Patch {
                            module: "module_game_menus".into(),
                            var: "game_menus".into(),
                            entry: entry.clone(),
                            path: vec![MENU_ENTER_FIELD],
                            action: Action::ReplaceBody,
                            ops,
                            origin: origin.clone(),
                        });
                    }
                }
                // New records: task T-017.
                Item::New { id, .. } => ctx.error(diag(E0201_NOT_YET, "`new` is parsed but not checked yet".into(), id.span, None)),
                Item::On { path, body, span } => {
                    let words: Vec<&str> = path.iter().map(|p| p.text.as_str()).collect();
                    let hook = match words.as_slice() {
                        ["game", rest] => format!("game_{rest}"),
                        _ => String::new(),
                    };
                    if !atlas.is_hook(&hook) {
                        let hooks: Vec<String> = atlas.hooks.iter().map(|h| h.replacen("game_", "game.", 1)).collect();
                        let asked = words.join(".");
                        let help = suggest(&asked, hooks.iter().map(String::as_str))
                            .map(|s| format!("did you mean 'on {s}'?"))
                            .or_else(|| Some("engine hooks are `on game.<name>`, e.g. `on game.start`".into()));
                        let whole = Span { start: path[0].span.start, end: path[path.len() - 1].span.end };
                        ctx.error(diag(
                            E0001_UNKNOWN_NAME,
                            format!("'{asked}' is not an engine hook"),
                            whole,
                            help,
                        ));
                        continue;
                    }
                    let ops = ctx.ops(&body.stmts);
                    patches.push(Patch {
                        module: "module_scripts".into(),
                        var: "scripts".into(),
                        entry: hook,
                        path: vec![SCRIPT_BODY_FIELD],
                        action: Action::Append,
                        ops,
                        origin: ctx.origin(*span),
                    });
                }
            }
        }
        errors.extend(ctx.errors);
    }
    if errors.is_empty() {
        Ok(Checked { name: name.unwrap_or_else(|| fallback_name.to_owned()), patches })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atlas() -> Atlas {
        let text = r#"{
          "schema": 1, "kit_hash": "t",
          "ops": [
            {"name":"party_relocate_near_party","code":1623,"line":667,"signature":"(party_relocate_near_party,<party_id>,<target_party_id>,<value_spawn_radius>),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"change_screen_map","code":2052,"line":1093,"signature":"","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"display_message","code":1106,"line":482,"signature":"(display_message,<string_id>,[hex_colour_code]),","lhs":false,"can_fail":false,"global_lhs":false},
            {"name":"assign","code":2133,"line":40,"signature":"had to put this here","lhs":true,"can_fail":false,"global_lhs":false}
          ],
          "hooks": ["game_start"],
          "ranges": [],
          "constants": [{"name":"reg0","value":1,"module":"header_common","line":563}],
          "tables": {
            "menus": {"module":"module_game_menus","var":"game_menus","id_file":"ID_menus","id_prefix":"menu_","ref_tag":"mnu",
              "entries": [
                {"index":0,"name":"start_game_0","id":"menu_start_game_0","dup":null,"line":39,"value":[]},
                {"index":1,"name":"start_phase_2","id":"menu_start_phase_2","dup":null,"line":55,"value":[]},
                {"index":2,"name":"camp","id":null,"dup":1,"line":60,"value":[]},
                {"index":3,"name":"camp","id":"menu_camp","dup":2,"line":70,"value":[]}
              ]},
            "parties": {"module":"module_parties","var":"parties","id_file":"ID_parties","id_prefix":"p_","ref_tag":"p",
              "entries": [
                {"index":0,"name":"main_party","id":"p_main_party","dup":null,"line":1,"value":[]},
                {"index":26,"name":"town_6","id":"p_town_6","dup":null,"line":82,"value":[]}
              ]},
            "scripts": {"module":"module_scripts","var":"scripts","id_file":"ID_scripts","id_prefix":"script_","ref_tag":"script",
              "entries": [{"index":0,"name":"game_start","id":"script_game_start","dup":null,"line":31,"value":[]}]}
          }
        }"#;
        Atlas::from_str(text).unwrap()
    }

    fn run(text: &str) -> Result<Checked, Vec<String>> {
        let file = SourceFile::new("start.uw", text);
        let tree = uw_syntax::parse(text).map_err(|d| vec![d.render(&file)])?;
        check(&atlas(), &[Source { file: file.clone(), tree }], "folder").map_err(|errs| {
            errs.iter().map(|e| e.diag.render(&file)).collect()
        })
    }

    #[test]
    fn praven_becomes_two_patches() {
        let checked = run("mod praven_start\noverride menu start_phase_2 {\n on enter {\n  party_relocate_near_party(p_main_party, p_town_6, 3)\n  change_screen_map()\n }\n}\non game.start { assign($g_x, reg0); display_message(\"@hi\") }\n").unwrap();
        assert_eq!(checked.name, "praven_start");
        assert_eq!(checked.patches.len(), 2);
        let menu = &checked.patches[0];
        assert_eq!((menu.module.as_str(), menu.entry.as_str(), menu.path.as_slice()), ("module_game_menus", "start_phase_2", &[4][..]));
        assert_eq!(menu.action, Action::ReplaceBody);
        assert_eq!(menu.ops[0].args, [Value::Ref("p_main_party".into()), Value::Ref("p_town_6".into()), Value::Int(3)]);
        assert_eq!(menu.ops[0].origin, "start.uw:4");
        let script = &checked.patches[1];
        assert_eq!((script.entry.as_str(), &script.action), ("game_start", &Action::Append));
        assert_eq!(script.ops[0].args[1], Value::Const { name: "reg0".into(), module: "header_common".into() });
    }

    #[test]
    fn menu_names_in_every_spelling_and_repeats() {
        for name in ["start_phase_2", "menu_start_phase_2", "mnu_start_phase_2"] {
            let c = run(&format!("override menu {name} {{ on enter {{ change_screen_map() }} }}")).unwrap();
            assert_eq!(c.patches[0].entry, "start_phase_2");
            assert_eq!(c.name, "folder");
        }
        let c = run("override menu camp#2 { on enter { change_screen_map() } }").unwrap();
        assert_eq!(c.patches[0].entry, "camp#2");
        let errs = run("override menu camp { on enter { change_screen_map() } }").unwrap_err();
        assert!(errs[0].contains("add #1, #2"), "{errs:?}");
    }

    #[test]
    fn every_unknown_name_is_reported_with_a_suggestion() {
        let errs = run("override menu start_phase_3 { on enter { change_screan_map()\n party_relocate_near_party(p_main_party, p_town_7, 3) } }\non game.strat { }\n").unwrap_err();
        let all = errs.join("\n");
        assert!(all.contains("'start_phase_3' is not a Native menu"), "{all}");
        assert!(all.contains("did you mean 'start_phase_2'?"), "{all}");
        assert!(all.contains("'game.strat' is not an engine hook"), "{all}");
        assert!(all.contains("did you mean 'on game.start'?"), "{all}");
        let errs = run("override menu start_phase_2 { on enter { change_screan_map()\n party_relocate_near_party(p_main_party, p_town_7, 3) } }").unwrap_err();
        let all = errs.join("\n");
        assert!(all.contains("did you mean 'change_screen_map'?"), "{all}");
        assert!(all.contains("'p_town_7' is not a Native name"), "{all}");
        assert!(all.contains("did you mean 'p_town_6'?"), "{all}");
    }

    #[test]
    fn too_few_values_quote_the_kit() {
        let errs = run("on game.start { party_relocate_near_party(p_main_party, p_town_6) }").unwrap_err();
        assert!(errs[0].contains("needs at least 3 values, got 2"), "{errs:?}");
        assert!(errs[0].contains("header_operations.py:667"), "{errs:?}");
        // Optional [..] values and unparsable comments are not counted.
        assert!(run("on game.start { display_message(\"@x\") }").is_ok());
        assert!(run("on game.start { assign($a, 1) }").is_ok());
    }

    #[test]
    fn duplicates_events_and_text_are_checked() {
        let errs = run("override menu start_phase_2 { on enter { } }\noverride menu mnu_start_phase_2 { on enter { } }").unwrap_err();
        assert!(errs[0].contains("already overridden at start.uw:1"), "{errs:?}");
        let errs = run("override menu start_phase_2 { on entre { } }").unwrap_err();
        assert!(errs[0].contains("did you mean 'enter'?"), "{errs:?}");
        let errs = run("override troop x { on enter { } }").unwrap_err();
        assert!(errs[0].contains("`override troop` is not in UW yet"), "{errs:?}");
        let errs = run("on game.start { display_message(\"@Ağa\") }").unwrap_err();
        assert!(errs[0].contains("non-ASCII"), "{errs:?}");
    }

    #[test]
    fn min_args_reads_header_comments() {
        let a = atlas();
        assert_eq!(min_args(a.op("party_relocate_near_party").unwrap()), Some(3));
        assert_eq!(min_args(a.op("display_message").unwrap()), Some(1));
        assert_eq!(min_args(a.op("change_screen_map").unwrap()), None);
        assert_eq!(min_args(a.op("assign").unwrap()), None);
    }
}
