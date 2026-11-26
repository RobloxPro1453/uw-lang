//! Checked patches -> the files put over the kit copy: one thin wrapper `module_X.py` per changed
//! module (Native's list from `native_module_X.py`, changed by `uw_runtime`), plus `uw_runtime.py`.
//! Pure: no I/O. The caller copies the kit's `module_X.py` to `native_module_X.py` next to them.
//!
//! Faz 1 note: there are no new entries yet, so `uw_lower` / `uw_plan` have nothing to add and the
//! patches come straight from `uw_check` (docs/ARCHITECTURE.md).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use uw_check::{Action, Checked, NewDialogLine, NewOption, NewTrigger, Op, OptionPlace, Patch, Value};

/// The runtime the wrappers import (py/uw_runtime.py, embedded).
pub const UW_RUNTIME: &str = include_str!("../../../py/uw_runtime.py");

/// One file to write into the overlay folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutFile {
    pub name: String,
    pub text: String,
}

/// Everything to place over the kit copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emitted {
    pub files: Vec<OutFile>,
    /// Kit modules to copy unchanged as `native_<module>.py` (`module_scripts`, ...).
    pub native_modules: Vec<String>,
}

fn py_str(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn value(v: &Value) -> String {
    match v {
        Value::Ref(r) => py_str(r),
        Value::Const { name, .. } => name.clone(),
        Value::Int(n) => n.to_string(),
        Value::Text(t) => py_str(t),
    }
}

/// One operation as a Python expression; the kit flag prefixes (`neg|eq`) stay as written.
fn op_stmt(op: &Op) -> String {
    if op.args.is_empty() {
        op.name.clone()
    } else {
        let args: Vec<String> = op.args.iter().map(value).collect();
        format!("({}, {})", op.name, args.join(", "))
    }
}

/// An operation list literal: `[`, one op per line with its `# origin`, `]` at `indent`.
fn op_list(ops: &[Op], indent: usize) -> String {
    if ops.is_empty() {
        return "[]".to_string();
    }
    let mut out = String::from("[\n");
    for op in ops {
        let _ = writeln!(out, "{:width$}{},  # {}", "", op_stmt(op), op.origin, width = indent + 4);
    }
    let _ = write!(out, "{:width$}]", "", width = indent);
    out
}

/// An option tuple `(ID, [COND], TEXT, [CONS])` after an already-open `(`: fields one per line at
/// `base + 4`, closing `)` at `base` (no trailing comma).
fn write_option(out: &mut String, option: &NewOption, base: usize) {
    let field = " ".repeat(base + 4);
    let close = " ".repeat(base);
    let _ = writeln!(out);
    let _ = writeln!(out, "{field}{},", py_str(&option.id));
    let _ = writeln!(out, "{field}{},", op_list(&option.condition, base + 4));
    let _ = writeln!(out, "{field}{},", py_str(&option.text));
    let _ = writeln!(out, "{field}{},", op_list(&option.consequence, base + 4));
    let _ = write!(out, "{close})");
}

/// Debug builds: a line after the patches that makes every menu print its own id when it opens
/// (`uw: menu-start-phase-2`), so the modder sees in game which menu to override. The game shows
/// `_` as a space, so the id is written with `-`; `display_log_message` keeps the line in the log
/// window too (plain `display_message` scrolled away before it could be read).
const DEBUG_MENUS: &str = "
# uw --debug: every menu shows its id when it opens (game shows _ as space, so - is used)
game_menus = _uw.tag_all(game_menus, 4, lambda e: [(display_log_message, \"@uw: menu-\" + e[0].replace(\"_\", \"-\"), 0xFFFF00)])
";

/// A dialog line `[SPEAKER, STATE, [CONDITION], TEXT, NEXT, [CONSEQUENCE]]` after an already-open
/// `[`: one field per line at `base + 4`, closing `]` at `base` (no trailing comma, T-102).
fn write_dialog_line(out: &mut String, line: &NewDialogLine, base: usize) {
    let field = " ".repeat(base + 4);
    let close = " ".repeat(base);
    let _ = writeln!(out);
    let _ = writeln!(out, "{field}{},", line.speaker);
    let _ = writeln!(out, "{field}{},", py_str(&line.state));
    let _ = writeln!(out, "{field}{},", op_list(&line.condition, base + 4));
    let _ = writeln!(out, "{field}{},", py_str(&line.text));
    let _ = writeln!(out, "{field}{},", py_str(&line.next));
    let _ = writeln!(out, "{field}{},", op_list(&line.consequence, base + 4));
    let _ = write!(out, "{close}]");
}

/// A generated trigger tuple: `(head, [conditions], [body])`, or `(head, [body])` without conditions;
/// blocks at `base + 4`, closing `)` at `base`.
fn write_trigger(out: &mut String, trigger: &NewTrigger, base: usize) {
    let field = " ".repeat(base + 4);
    let _ = writeln!(out, "(");
    let _ = writeln!(out, "{field}{},", trigger.head);
    if let Some(conditions) = &trigger.conditions {
        let _ = writeln!(out, "{field}{},", op_list(conditions, base + 4));
    }
    let _ = writeln!(out, "{field}{},", op_list(&trigger.body, base + 4));
    let _ = write!(out, "{})", " ".repeat(base));
}

fn wrapper(name: &str, module: &str, patches: &[&Patch], debug: bool) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# -*- coding: utf-8 -*-");
    let _ = writeln!(out, "# Generated by uw for mod '{name}'. Do not edit: change the .uw files and run `uw build`.");
    let _ = writeln!(out, "# Native's own list comes from native_{module}.py, an unchanged copy of the kit file.");
    let _ = writeln!(out, "from native_{module} import *");
    out.push_str(&section(module, patches, debug));
    out
}

/// The imports and patch statements of one module, without the file header and the native import:
/// the part [`wrapper`] writes into a standalone overlay module, and a whole-table project build
/// appends to its own `module_<t>.py` (T-123).
pub fn section(module: &str, patches: &[&Patch], debug: bool) -> String {
    let mut headers: BTreeSet<&str> = BTreeSet::new();
    // Every operation a patch writes: its own block, and the condition / consequence blocks a new
    // dialog line brings (T-102).
    let mut ops: Vec<&Op> = patches.iter().flat_map(|patch| patch.ops.iter()).collect();
    for patch in patches {
        match &patch.action {
            Action::NewDialog { lines, .. } => {
                for line in lines {
                    ops.extend(line.condition.iter().chain(&line.consequence));
                }
            }
            Action::AppendTriggers { triggers } | Action::NewPresentation { triggers, .. } => {
                for trigger in triggers {
                    ops.extend(trigger.conditions.iter().flatten().chain(&trigger.body));
                }
            }
            _ => {}
        }
    }
    for op in ops {
        for arg in &op.args {
            if let Value::Const { module, .. } = arg {
                headers.insert(module);
            }
        }
    }
    headers.remove("header_common");
    headers.remove("header_operations");

    let mut out = String::new();
    let _ = writeln!(out, "from header_common import *");
    let _ = writeln!(out, "from header_operations import *");
    for header in &headers {
        let _ = writeln!(out, "from {header} import *");
    }
    let _ = writeln!(out, "import uw_runtime as _uw");
    for patch in patches {
        let path = match patch.path.as_slice() {
            [one] => one.to_string(),
            many => format!("({},)", many.iter().map(usize::to_string).collect::<Vec<_>>().join(", ")),
        };
        let _ = writeln!(out);
        let _ = writeln!(out, "# {}", patch.origin);
        match &patch.action {
            Action::ReplaceBody | Action::Append => {
                let func = if matches!(&patch.action, Action::ReplaceBody) { "replace_body" } else { "append_ops" };
                let _ = writeln!(out, "{var} = _uw.{func}({var}, {entry}, {path}, [", var = patch.var, entry = py_str(&patch.entry));
                for op in &patch.ops {
                    let _ = writeln!(out, "    {},  # {}", op_stmt(op), op.origin);
                }
                let _ = writeln!(out, "])");
            }
            Action::ReplaceText(text) => {
                let _ = writeln!(
                    out,
                    "{var} = _uw.set_text({var}, {entry}, {path}, {text})",
                    var = patch.var,
                    entry = py_str(&patch.entry),
                    text = py_str(text)
                );
            }
            Action::InsertOption { option, place } => {
                let _ = write!(out, "{var} = _uw.add_option({var}, {entry}, {path}, (", var = patch.var, entry = py_str(&patch.entry));
                write_option(&mut out, option, 0);
                let (after, before) = match place {
                    OptionPlace::End => ("None".to_string(), "None".to_string()),
                    OptionPlace::After(id) => (py_str(id), "None".to_string()),
                    OptionPlace::Before(id) => ("None".to_string(), py_str(id)),
                };
                let _ = writeln!(out, ", after={after}, before={before})");
            }
            Action::ChangeOption { id, text, condition, consequence } => {
                let _ = write!(
                    out,
                    "{var} = _uw.change_option({var}, {entry}, {path}, {id}",
                    var = patch.var,
                    entry = py_str(&patch.entry),
                    id = py_str(id)
                );
                let text = text.as_deref().map_or_else(|| "None".to_string(), py_str);
                let condition = condition.as_deref().map_or_else(|| "None".to_string(), |ops| op_list(ops, 0));
                let consequence = consequence.as_deref().map_or_else(|| "None".to_string(), |ops| op_list(ops, 0));
                let _ = writeln!(out, ", text={text}, condition={condition}, consequence={consequence})");
            }
            Action::RemoveOption(id) => {
                let _ = writeln!(
                    out,
                    "{var} = _uw.remove_option({var}, {entry}, {path}, {id})",
                    var = patch.var,
                    entry = py_str(&patch.entry),
                    id = py_str(id)
                );
            }
            Action::NewMenu { text, enter, options } => {
                let _ = writeln!(out, "{var} = _uw.add_entry({var}, (", var = patch.var);
                let _ = writeln!(out, "    {},", py_str(&patch.entry));
                let _ = writeln!(out, "    0,");
                let _ = writeln!(out, "    {},", py_str(text));
                let _ = writeln!(out, "    \"none\",");
                let _ = writeln!(out, "    {},", op_list(enter, 4));
                if options.is_empty() {
                    let _ = writeln!(out, "    [],");
                } else {
                    let _ = writeln!(out, "    [");
                    for option in options {
                        let _ = write!(out, "        (");
                        write_option(&mut out, option, 8);
                        let _ = writeln!(out, ",");
                    }
                    let _ = writeln!(out, "    ],");
                }
                let _ = writeln!(out, "))");
            }
            Action::NewSimpleTrigger { interval, comment } => {
                if let Some(comment) = comment {
                    let _ = writeln!(out, "# {comment}");
                }
                let _ = writeln!(out, "{var} = _uw.append_entry({var}, (", var = patch.var);
                let _ = writeln!(out, "    {interval},");
                let _ = writeln!(out, "    {},", op_list(&patch.ops, 4));
                let _ = writeln!(out, "))");
            }
            // New dialog lines (`dialog …`, T-102): whole records of the `dialogs` list, put in
            // front of Native's lines of the same state. The kit scans the table top to bottom and
            // the first matching NPC line wins (module_dialogs.py:16-18), so a line written after
            // Native's own would never be reached. The
            // voice-over field is left out: a shorter line means `NO_VOICEOVER`
            // (process_dialogs.py:180-183).
            Action::NewDialog { state, lines } => {
                let _ = writeln!(out, "{var} = _uw.insert_before_state({var}, {state}, [", var = patch.var, state = py_str(state));
                for line in lines {
                    let _ = write!(out, "    [");
                    write_dialog_line(&mut out, line, 4);
                    let _ = writeln!(out, ",");
                }
                let _ = writeln!(out, "])");
            }
            // A new record (troop, town, …): one whole record at the end of its list, so Native's
            // own records keep their numbers. `value` is the record as Python source (T-092,
            // T-099); the module, the list and its entry name come from the item's own table.
            Action::NewRecord { value } => {
                let _ = writeln!(out, "{var} = _uw.add_entry({var}, {value})", var = patch.var);
            }
            // An existing record replaced in place (`override troop <id> { … }`, T-118): the entry
            // keeps its place in the list, so nothing that points at it by number shifts.
            // The core language's generated entries (uw_core, D4): a script, triggers added to an
            // existing entry, a presentation.
            Action::NewScript => {
                let _ = writeln!(
                    out,
                    "{var} = _uw.add_entry({var}, ({entry}, {body}))",
                    var = patch.var,
                    entry = py_str(&patch.entry),
                    body = op_list(&patch.ops, 0)
                );
            }
            Action::AppendTriggers { triggers } => {
                let _ = writeln!(out, "{var} = _uw.append_ops({var}, {entry}, {path}, [", var = patch.var, entry = py_str(&patch.entry));
                for trigger in triggers {
                    let _ = write!(out, "    ");
                    write_trigger(&mut out, trigger, 4);
                    let _ = writeln!(out, ",");
                }
                let _ = writeln!(out, "])");
            }
            Action::NewPresentation { flags, mesh, triggers } => {
                let _ = writeln!(out, "{var} = _uw.add_entry({var}, (", var = patch.var);
                let _ = writeln!(out, "    {},", py_str(&patch.entry));
                let _ = writeln!(out, "    {flags},");
                let _ = writeln!(out, "    {mesh},");
                let _ = writeln!(out, "    [");
                for trigger in triggers {
                    let _ = write!(out, "        ");
                    write_trigger(&mut out, trigger, 8);
                    let _ = writeln!(out, ",");
                }
                let _ = writeln!(out, "    ],");
                let _ = writeln!(out, "))");
            }
            Action::ReplaceRecord { value } => {
                let _ = writeln!(
                    out,
                    "{var} = _uw.replace_entry({var}, {entry}, {value})",
                    var = patch.var,
                    entry = py_str(&patch.entry),
                    value = value
                );
            }
        }
    }
    if debug && module == "module_game_menus" {
        out.push_str(DEBUG_MENUS);
    }
    out
}

/// `debug`: also tag every menu with its id (see [`DEBUG_MENUS`]); the menus wrapper is written
/// even when the mod does not change a menu.
pub fn emit(checked: &Checked, debug: bool) -> Emitted {
    // Module -> its patches; sorted, so the output is deterministic.
    let mut by_module: BTreeMap<&str, Vec<&Patch>> = BTreeMap::new();
    for patch in &checked.patches {
        by_module.entry(patch.module.as_str()).or_default().push(patch);
    }
    if debug {
        by_module.entry("module_game_menus").or_default();
    }
    let mut files = Vec::new();
    for (module, patches) in &by_module {
        files.push(OutFile { name: format!("{module}.py"), text: wrapper(&checked.name, module, patches, debug) });
    }
    files.push(OutFile { name: "uw_runtime.py".into(), text: UW_RUNTIME.to_owned() });
    Emitted { files, native_modules: by_module.keys().map(|m| (*m).to_owned()).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uw_check::Op;

    fn op(name: &str, args: Vec<Value>, origin: &str) -> Op {
        Op { name: name.into(), args, origin: origin.into() }
    }

    fn menu_patch(entry: &str, path: Vec<usize>, action: Action, origin: &str) -> Patch {
        Patch {
            module: "module_game_menus".into(),
            var: "game_menus".into(),
            entry: entry.into(),
            path,
            action,
            ops: vec![],
            origin: origin.into(),
        }
    }

    /// The one wrapper `emit` writes for a single menu patch.
    fn emitted(patch: Patch) -> String {
        emit(&Checked { name: "m".into(), patches: vec![patch] }, false).files[0].text.clone()
    }

    /// T-118: a record override is one `replace_entry` call in the table's wrapper.
    #[test]
    fn a_record_override_replaces_the_entry_in_place() {
        let patch = Patch {
            module: "module_troops".into(),
            var: "troops".into(),
            entry: "trp_swadian_recruit".into(),
            path: vec![],
            action: Action::ReplaceRecord { value: "[\"trp_swadian_recruit\", \"Recruit\"]".into() },
            ops: vec![],
            origin: "start.uw:1".into(),
        };
        let text = emitted(patch);
        assert!(
            text.contains("troops = _uw.replace_entry(troops, \"trp_swadian_recruit\", [\"trp_swadian_recruit\", \"Recruit\"])"),
            "{text}"
        );
        assert!(text.contains("# start.uw:1"), "{text}");
    }

    #[test]
    fn praven_wrappers_match_the_hand_written_d4_shape() {
        let checked = Checked {
            name: "praven_start".into(),
            patches: vec![
                Patch {
                    module: "module_game_menus".into(),
                    var: "game_menus".into(),
                    entry: "start_phase_2".into(),
                    path: vec![4],
                    action: Action::ReplaceBody,
                    ops: vec![
                        Op {
                            name: "party_relocate_near_party".into(),
                            args: vec![Value::Ref("p_main_party".into()), Value::Ref("p_town_6".into()), Value::Int(3)],
                            origin: "start.uw:4".into(),
                        },
                        Op { name: "change_screen_map".into(), args: vec![], origin: "start.uw:5".into() },
                    ],
                    origin: "start.uw:2".into(),
                },
                Patch {
                    module: "module_scripts".into(),
                    var: "scripts".into(),
                    entry: "game_start".into(),
                    path: vec![1],
                    action: Action::Append,
                    ops: vec![Op {
                        name: "troop_set_type".into(),
                        args: vec![
                            Value::Ref("trp_player".into()),
                            Value::Const { name: "tf_male".into(), module: "header_troops".into() },
                        ],
                        origin: "start.uw:9".into(),
                    }, Op {
                        name: "display_message".into(),
                        args: vec![Value::Text("@say \"hi\"".into())],
                        origin: "start.uw:10".into(),
                    }],
                    origin: "start.uw:8".into(),
                },
            ],
        };
        let out = emit(&checked, false);
        assert_eq!(out.native_modules, ["module_game_menus", "module_scripts"]);
        let names: Vec<&str> = out.files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["module_game_menus.py", "module_scripts.py", "uw_runtime.py"]);
        assert_eq!(
            out.files[0].text,
            "# -*- coding: utf-8 -*-\n\
             # Generated by uw for mod 'praven_start'. Do not edit: change the .uw files and run `uw build`.\n\
             # Native's own list comes from native_module_game_menus.py, an unchanged copy of the kit file.\n\
             from native_module_game_menus import *\n\
             from header_common import *\n\
             from header_operations import *\n\
             import uw_runtime as _uw\n\
             \n\
             # start.uw:2\n\
             game_menus = _uw.replace_body(game_menus, \"start_phase_2\", 4, [\n    \
             (party_relocate_near_party, \"p_main_party\", \"p_town_6\", 3),  # start.uw:4\n    \
             change_screen_map,  # start.uw:5\n\
             ])\n"
        );
        let scripts = &out.files[1].text;
        assert!(scripts.contains("from header_troops import *\n"), "{scripts}");
        assert!(scripts.contains("scripts = _uw.append_ops(scripts, \"game_start\", 1, [\n"), "{scripts}");
        assert!(scripts.contains("(troop_set_type, \"trp_player\", tf_male),  # start.uw:9"), "{scripts}");
        assert!(scripts.contains("(display_message, \"@say \\\"hi\\\"\"),"), "{scripts}");
        assert!(out.files[2].text.contains("def replace_body("));
        assert!(!out.files[0].text.contains("tag_all"));
    }

    #[test]
    fn debug_tags_menus_even_without_menu_patches() {
        let checked = Checked { name: "m".into(), patches: vec![] };
        let out = emit(&checked, true);
        assert_eq!(out.native_modules, ["module_game_menus"]);
        let menus = &out.files[0].text;
        assert!(menus.contains("game_menus = _uw.tag_all(game_menus, 4, lambda e: [(display_log_message, \"@uw: menu-\" + e[0].replace(\"_\", \"-\"), 0xFFFF00)])"), "{menus}");
        assert!(emit(&checked, false).files.iter().all(|f| f.name != "module_game_menus.py"));
    }

    #[test]
    fn replace_text_emits_set_text() {
        let text = emitted(menu_patch("start_phase_2", vec![2], Action::ReplaceText("New text".into()), "x.uw:3"));
        let expected = [
            "# x.uw:3",
            "game_menus = _uw.set_text(game_menus, \"start_phase_2\", 2, \"New text\")",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }

    #[test]
    fn insert_option_emits_add_option() {
        let option = NewOption {
            id: "uw_skip".into(),
            condition: vec![op("eq", vec![Value::Ref("$x".into()), Value::Int(1)], "x.uw:5")],
            text: "Skip".into(),
            consequence: vec![op("jump_to_menu", vec![Value::Ref("mnu_start_game_0".into())], "x.uw:6")],
            origin: "x.uw:4".into(),
        };
        let text = emitted(menu_patch(
            "start_phase_2",
            vec![5],
            Action::InsertOption { option, place: OptionPlace::End },
            "x.uw:3",
        ));
        let expected = [
            "# x.uw:3",
            "game_menus = _uw.add_option(game_menus, \"start_phase_2\", 5, (",
            "    \"uw_skip\",",
            "    [",
            "        (eq, \"$x\", 1),  # x.uw:5",
            "    ],",
            "    \"Skip\",",
            "    [",
            "        (jump_to_menu, \"mnu_start_game_0\"),  # x.uw:6",
            "    ],",
            "), after=None, before=None)",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }

    #[test]
    fn insert_option_places_use_the_matching_keyword() {
        let option = |id: &str| NewOption {
            id: id.into(),
            condition: vec![],
            text: "T".into(),
            consequence: vec![],
            origin: "x.uw:4".into(),
        };
        let after = emitted(menu_patch(
            "m",
            vec![5],
            Action::InsertOption { option: option("a"), place: OptionPlace::After("a".into()) },
            "x.uw:3",
        ));
        assert!(after.ends_with("), after=\"a\", before=None)\n"), "{after}");
        let before = emitted(menu_patch(
            "m",
            vec![5],
            Action::InsertOption { option: option("b"), place: OptionPlace::Before("b".into()) },
            "x.uw:3",
        ));
        assert!(before.ends_with("), after=None, before=\"b\")\n"), "{before}");
    }

    #[test]
    fn change_option_emits_change_option() {
        let text = emitted(menu_patch(
            "start_phase_2",
            vec![5],
            Action::ChangeOption {
                id: "uw_skip".into(),
                text: Some("Skip".into()),
                condition: Some(vec![op("neg|eq", vec![Value::Ref("$x".into()), Value::Int(1)], "x.uw:5")]),
                consequence: Some(vec![op("jump_to_menu", vec![Value::Ref("mnu_start_game_0".into())], "x.uw:6")]),
            },
            "x.uw:3",
        ));
        let expected = [
            "# x.uw:3",
            "game_menus = _uw.change_option(game_menus, \"start_phase_2\", 5, \"uw_skip\", text=\"Skip\", condition=[",
            "    (neg|eq, \"$x\", 1),  # x.uw:5",
            "], consequence=[",
            "    (jump_to_menu, \"mnu_start_game_0\"),  # x.uw:6",
            "])",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }

    #[test]
    fn change_option_none_keeps_the_native_part() {
        let text = emitted(menu_patch(
            "start_phase_2",
            vec![5],
            Action::ChangeOption { id: "uw_skip".into(), text: None, condition: None, consequence: None },
            "x.uw:3",
        ));
        let expected = [
            "# x.uw:3",
            "game_menus = _uw.change_option(game_menus, \"start_phase_2\", 5, \"uw_skip\", text=None, condition=None, consequence=None)",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }

    #[test]
    fn remove_option_emits_remove_option() {
        let text = emitted(menu_patch("start_phase_2", vec![5], Action::RemoveOption("uw_skip".into()), "x.uw:3"));
        let expected = [
            "# x.uw:3",
            "game_menus = _uw.remove_option(game_menus, \"start_phase_2\", 5, \"uw_skip\")",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }

    #[test]
    fn new_menu_emits_add_entry() {
        let text = emitted(menu_patch(
            "uw_ask",
            vec![],
            Action::NewMenu {
                text: "Where from?".into(),
                enter: vec![op("change_screen_map", vec![], "x.uw:9")],
                options: vec![NewOption {
                    id: "north".into(),
                    condition: vec![],
                    text: "From the north".into(),
                    consequence: vec![op("jump_to_menu", vec![Value::Ref("mnu_start_phase_2".into())], "x.uw:11")],
                    origin: "x.uw:10".into(),
                }],
            },
            "x.uw:8",
        ));
        let expected = [
            "# x.uw:8",
            "game_menus = _uw.add_entry(game_menus, (",
            "    \"uw_ask\",",
            "    0,",
            "    \"Where from?\",",
            "    \"none\",",
            "    [",
            "        change_screen_map,  # x.uw:9",
            "    ],",
            "    [",
            "        (",
            "            \"north\",",
            "            [],",
            "            \"From the north\",",
            "            [",
            "                (jump_to_menu, \"mnu_start_phase_2\"),  # x.uw:11",
            "            ],",
            "        ),",
            "    ],",
            "))",
        ]
        .join("\n")
            + "\n";
        assert!(text.ends_with(expected.as_str()), "{text}");
    }
}
