//! `uw build` for a core language project (D4; docs/dil/CORE-v0.md §1): `main.uw` + `features/*.uw`.
//!
//! The core lowering (`uw_core`) gives operations and generated entries; here they become the same
//! `uw_check` patches the old front end makes, so emitting, the kit run in staging and the install are
//! the very same code (`build::build_checked`): the kit folder is never written, an installed folder is
//! replaced only when it carries uw's marker, Native's own entries stay as they are.
//!
//! `slots.lock` (the slot number of every object field, saved games depend on it) is read from the
//! project folder and written back only when the lowering succeeded and something was added.

use std::fs;
use std::path::{Path, PathBuf};

use uw_atlas::Atlas;
use uw_check::{Action, Checked, NewTrigger, Patch, Value};
use uw_core::lower::{lower_locked, time, CoreData};
use uw_core::slots::SlotLock;
use uw_core::{Arg, Lowered, MissionTarget, TriggerWhen};

use crate::build;

/// The slot lock's file name in the project folder.
pub const LOCK_FILE: &str = "slots.lock";
/// The trigger list of a mission template (module_mission_templates.py: id, flags, type, text,
/// groups, triggers).
const TEMPLATE_TRIGGERS_FIELD: usize = 5;
/// The enter-operations field of a menu record (module_game_menus.py header comment, field 5 counted from 1).
const MENU_ENTER_FIELD: usize = 4;
/// Mission templates whose id starts with this are multiplayer: the dispatcher stays out of them
/// until the language says what runs on the server and what on a client.
const MULTIPLAYER_PREFIX: &str = "multiplayer_";

/// True when `dir` is a core project: it has a `main.uw` that is not an old-syntax mod file (those
/// start with a `mod <name>` line).
pub fn is_core_project(dir: &Path) -> bool {
    match fs::read_to_string(dir.join("main.uw")) {
        Ok(text) => !text.lines().any(|line| line.trim_start().starts_with("mod ")),
        Err(_) => false,
    }
}

/// The project's files as (feature, shown path, text): `main.uw` first, then `features/*.uw` sorted.
pub fn read_project(dir: &Path) -> Result<Vec<(String, String, String)>, String> {
    let read = |path: &Path| fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()));
    let mut files = vec![("main".to_owned(), "main.uw".to_owned(), read(&dir.join("main.uw"))?)];
    let features = dir.join("features");
    if features.is_dir() {
        let mut paths: Vec<PathBuf> = fs::read_dir(&features)
            .map_err(|e| format!("{}: {e}", features.display()))?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "uw"))
            .collect();
        paths.sort();
        for path in paths {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_owned();
            files.push((stem.clone(), format!("features/{stem}.uw"), read(&path)?));
        }
    }
    Ok(files)
}

/// The mod's name from its folder: lower case, anything but a letter or digit becomes `_`.
pub fn mod_name(dir: &Path) -> String {
    let raw = dir.file_name().and_then(|n| n.to_str()).unwrap_or("uw_mod");
    raw.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' }).collect()
}

/// Parses and lowers the project. Errors come back as printable `path:line: message` lines.
pub fn lower_project(dir: &Path, atlas: &Atlas, lock: &SlotLock) -> Result<Lowered, Vec<String>> {
    let files = read_project(dir).map_err(|e| vec![e])?;
    let shown = |feature: &str| -> String {
        files
            .iter()
            .find(|(f, _, _)| f == feature)
            .map_or_else(|| feature.to_owned(), |(_, path, _)| path.clone())
    };
    let mut parsed = Vec::new();
    let mut errors = Vec::new();
    for (feature, path, text) in &files {
        match uw_core::parse(feature, path, text) {
            Ok(file) => parsed.push(file),
            Err(errs) => errors.extend(errs.into_iter().map(|e| format!("{}:{}: {}", path, e.line, e.message))),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let data = CoreData::builtin().map_err(|e| vec![e.to_string()])?;
    lower_locked(&parsed, atlas, &data, lock).map_err(|errs| {
        errs.into_iter().map(|e| format!("{}:{}: {}", shown(&e.file), e.line, e.message)).collect()
    })
}

/// One core operation as a patch operation. A name the atlas has as a constant is written bare (its
/// header is imported); any other name is a table reference the kit resolves (`"prsnt_uw_hud"`).
fn op(atlas: &Atlas, op: &uw_core::Op) -> uw_check::Op {
    let mut name = op.name.clone();
    if op.this_or_next {
        name = format!("this_or_next|{name}");
    }
    if op.neg {
        name = format!("neg|{name}");
    }
    let args = op
        .args
        .iter()
        .map(|arg| match arg {
            Arg::Int(n) => Value::Int(*n),
            Arg::Local(s) | Arg::Global(s) => Value::Ref(s.clone()),
            Arg::Reg(n) => Value::Const { name: format!("reg{n}"), module: "header_common".into() },
            Arg::Name(n) => match atlas.constant(n) {
                Some(constant) => Value::Const { name: n.clone(), module: constant.module.trim_end_matches(".py").to_owned() },
                None => Value::Ref(n.clone()),
            },
            Arg::Text(t) => Value::Text(t.clone()),
        })
        .collect();
    uw_check::Op { name, args, origin: "uw".into() }
}

fn ops(atlas: &Atlas, list: &[uw_core::Op]) -> Vec<uw_check::Op> {
    list.iter().map(|o| op(atlas, o)).collect()
}

/// A float the way the kit writes it (`1`, `0.5`, never `1.0e0`).
fn number(value: f64) -> String {
    if value.fract() == 0.0 { format!("{}", value as i64) } else { format!("{value}") }
}

/// The single-player mission templates (the dispatcher's targets), as `uw_runtime.find` names them.
pub fn player_templates(atlas: &Atlas) -> Vec<String> {
    let Some(table) = atlas.tables.get("mission_templates") else { return Vec::new() };
    table
        .entries
        .iter()
        .filter_map(|entry| {
            let name = entry.name.as_deref()?;
            if name.starts_with(MULTIPLAYER_PREFIX) {
                return None;
            }
            Some(match entry.dup {
                Some(n) => format!("{name}#{n}"),
                None => name.to_owned(),
            })
        })
        .collect()
}

/// Everything the lowering made, as patches for the emitter.
pub fn to_patches(name: &str, lowered: &Lowered, atlas: &Atlas) -> Result<Checked, String> {
    if !lowered.overrides.is_empty() {
        return Err("`disable`/`override` of Native entries is not built yet (D4b)".into());
    }
    let mut patches = Vec::new();
    let patch = |module: &str, var: &str, entry: &str, path: Vec<usize>, action: Action, body: Vec<uw_check::Op>| Patch {
        module: module.into(),
        var: var.into(),
        entry: entry.into(),
        path,
        action,
        ops: body,
        origin: "uw core".into(),
    };
    for (script, body) in &lowered.script_appends {
        patches.push(patch("module_scripts", "scripts", script, vec![1], Action::Append, ops(atlas, body)));
    }
    for (menu, body) in &lowered.menu_appends {
        // A menu record is (id, flags, text, mesh, enter operations, options): field 4 runs when it opens.
        patches.push(patch("module_game_menus", "game_menus", menu, vec![MENU_ENTER_FIELD], Action::Append, ops(atlas, body)));
    }
    for (script, body) in &lowered.scripts {
        patches.push(patch("module_scripts", "scripts", script, vec![], Action::NewScript, ops(atlas, body)));
    }
    for trigger in &lowered.simple_triggers {
        let action = Action::NewSimpleTrigger {
            interval: number(trigger.interval),
            comment: Some("uw: game-clock dispatcher (every `every`/`after` on game time)".into()),
        };
        patches.push(patch("module_simple_triggers", "simple_triggers", "", vec![], action, ops(atlas, &trigger.body)));
    }
    for presentation in &lowered.presentations {
        let flags = if presentation.flags.is_empty() { "0".to_owned() } else { presentation.flags.join("|") };
        let mesh = presentation.mesh.as_ref().map_or_else(|| "0".to_owned(), |m| format!("\"{m}\""));
        let triggers = vec![
            NewTrigger { head: "ti_on_presentation_load".into(), conditions: None, body: ops(atlas, &presentation.on_load) },
            NewTrigger { head: "ti_on_presentation_run".into(), conditions: None, body: ops(atlas, &presentation.on_run) },
        ];
        let action = Action::NewPresentation { flags, mesh, triggers };
        patches.push(patch("module_presentations", "presentations", &presentation.name, vec![], action, Vec::new()));
    }
    // Mission triggers, grouped by the templates they go to.
    let mut with_player = Vec::new();
    for trigger in &lowered.mission_triggers {
        let head = match &trigger.when {
            TriggerWhen::Interval(seconds) => format!("{}, 0, 0", number(*seconds)),
            TriggerWhen::Event(event) => format!("{event}, 0, 0"),
        };
        let built = NewTrigger { head, conditions: Some(ops(atlas, &trigger.conditions)), body: ops(atlas, &trigger.body) };
        match &trigger.target {
            MissionTarget::WithPlayer => with_player.push(built),
            MissionTarget::Named(names) => {
                for template in names {
                    let action = Action::AppendTriggers { triggers: vec![built.clone()] };
                    patches.push(patch("module_mission_templates", "mission_templates", template, vec![TEMPLATE_TRIGGERS_FIELD], action, Vec::new()));
                }
            }
        }
    }
    if !with_player.is_empty() {
        let templates = player_templates(atlas);
        if templates.is_empty() {
            return Err("the atlas lists no mission templates; rebuild it (docs/DEVELOPMENT.md, Atlas)".into());
        }
        for template in templates {
            let action = Action::AppendTriggers { triggers: with_player.clone() };
            patches.push(patch(
                "module_mission_templates",
                "mission_templates",
                &template,
                vec![TEMPLATE_TRIGGERS_FIELD],
                action,
                Vec::new(),
            ));
        }
    }
    Ok(Checked { name: name.to_owned(), patches })
}

/// `uw build <core project>`.
pub fn run(opts: build::Options) -> Result<(), String> {
    let dir = opts.dir.canonicalize().map_err(|e| format!("{}: {e}", opts.dir.display()))?;
    let dir = match dir.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        Some(plain) => PathBuf::from(plain),
        None => dir,
    };
    let name = mod_name(&dir);
    let atlas_path = crate::atlas_path(opts.atlas.clone())?;
    let atlas = Atlas::from_path(&atlas_path).map_err(|e| format!("{}: {e}", atlas_path.display()))?;

    let lock_path = dir.join(LOCK_FILE);
    let old_lock_text = fs::read_to_string(&lock_path).ok();
    let lock = match &old_lock_text {
        Some(text) => SlotLock::parse(text)?,
        None => SlotLock::new(),
    };
    let lowered = match lower_project(&dir, &atlas, &lock) {
        Ok(lowered) => lowered,
        Err(errors) => {
            for error in &errors {
                eprintln!("{error}");
            }
            return Err(format!(
                "{} {} found; nothing was built",
                errors.len(),
                if errors.len() == 1 { "error" } else { "errors" }
            ));
        }
    };
    for warning in &lowered.warnings {
        eprintln!("warning: {warning}");
    }
    let files = read_project(&dir)?.len();
    println!("checked   {files} file(s) of core project '{name}'");

    // The lock only grows (uw_core::slots); write it only when this build added a field.
    if lowered.slot_lock != lock || (old_lock_text.is_none() && lowered.slot_lock.fields().next().is_some()) {
        fs::write(&lock_path, lowered.slot_lock.to_text()).map_err(|e| format!("{}: {e}", lock_path.display()))?;
        println!("wrote     {LOCK_FILE} (commit it: saved games depend on these numbers)");
    }
    print!("{}", time::report(&lowered));

    let checked = to_patches(&name, &lowered, &atlas)?;
    build::build_checked(&dir, &checked, opts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uw_core::{MissionTrigger, Op as CoreOp};

    fn atlas() -> Atlas {
        Atlas::from_str(
            r#"{"schema": 1, "ops": [], "hooks": [], "ranges": [],
                "constants": [{"name": "key_q", "value": 16, "module": "header_triggers", "line": null}],
                "tables": {"mission_templates": {"module": "module_mission_templates", "var": "mission_templates",
                  "id_file": null, "id_prefix": "mt", "ref_tag": "mt", "entries": [
                    {"index": 0, "name": "town_center", "id": "mt_town_center", "dup": null, "line": null, "value": null},
                    {"index": 1, "name": "multiplayer_dm", "id": "mt_multiplayer_dm", "dup": null, "line": null, "value": null},
                    {"index": 2, "name": "arena", "id": null, "dup": 2, "line": null, "value": null}]}}}"#,
        )
        .unwrap()
    }

    fn core_op(name: &str, args: Vec<Arg>, neg: bool) -> CoreOp {
        CoreOp { name: name.into(), args, neg, this_or_next: false }
    }

    #[test]
    fn a_core_project_is_a_main_uw_without_a_mod_line() {
        let root = std::env::temp_dir().join(format!("uw-core-detect-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        assert!(!is_core_project(&root), "no main.uw");
        fs::write(root.join("main.uw"), "mod old_style\n").unwrap();
        assert!(!is_core_project(&root), "old syntax");
        fs::write(root.join("main.uw"), "// the whole game\n").unwrap();
        assert!(is_core_project(&root));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn the_dispatcher_goes_to_every_single_player_template() {
        let mut lowered = Lowered::default();
        lowered.scripts.push(("uw_mission_frame".into(), vec![core_op("key_clicked", vec![Arg::Name("key_q".into())], true)]));
        lowered.mission_triggers.push(MissionTrigger {
            when: TriggerWhen::Interval(0.0),
            conditions: Vec::new(),
            body: vec![core_op("call_script", vec![Arg::Name("script_uw_mission_frame".into())], false)],
            target: MissionTarget::WithPlayer,
        });
        let checked = to_patches("alex", &lowered, &atlas()).unwrap();
        let script = &checked.patches[0];
        assert_eq!(script.action, Action::NewScript);
        assert_eq!(script.ops[0].name, "neg|key_clicked");
        assert_eq!(script.ops[0].args, vec![Value::Const { name: "key_q".into(), module: "header_triggers".into() }]);
        let templates: Vec<&str> = checked.patches[1..].iter().map(|p| p.entry.as_str()).collect();
        assert_eq!(templates, vec!["town_center", "arena#2"], "multiplayer templates are left out");
        let Action::AppendTriggers { triggers } = &checked.patches[1].action else { panic!("{:?}", checked.patches[1]) };
        assert_eq!(triggers[0].head, "0, 0, 0");
        assert_eq!(triggers[0].body[0].args, vec![Value::Ref("script_uw_mission_frame".into())]);
        let text = uw_emit::emit(&checked, false).files.iter().map(|f| f.text.clone()).collect::<String>();
        assert!(text.contains("scripts = _uw.add_entry(scripts, (\"uw_mission_frame\", ["), "{text}");
        assert!(text.contains("from header_triggers import *"), "{text}");
        assert!(text.contains("mission_templates = _uw.append_ops(mission_templates, \"arena#2\", 5, ["), "{text}");
    }
}
