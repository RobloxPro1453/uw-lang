//! `uw build <mod folder>`: read the .uw files, check them against the atlas, write the wrappers,
//! run the kit in a staging copy and install the result as `<Warband>/Modules/<mod>`.
//!
//! Safety (data loss is the worst error, AGENTS rule 1): the kit folder is never written; a
//! `Modules/<mod>` folder is replaced only when it carries the `.uw-mod` marker this command
//! writes; Native and every other module are only read.

use std::fs;
use std::path::{Path, PathBuf};

use uw_diag::{Diagnostic, SourceFile};

/// Marker file in an installed mod folder; without it `uw build` never deletes the folder.
pub const MOD_MARKER: &str = ".uw-mod";
const OVERLAY_MARKER: &str = ".uw-overlay";

pub struct Options {
    pub dir: PathBuf,
    pub atlas: Option<PathBuf>,
    pub kit: Option<PathBuf>,
    pub python: Option<PathBuf>,
    pub modules: Option<PathBuf>,
    pub install: bool,
    /// Every menu prints its id in game (`uw: menu-x`, `_` shows as a space in game).
    pub debug: bool,
}

/// One source error, with the file and the diagnostic span `uw check --json` needs.
pub struct Problem {
    pub path: PathBuf,
    pub file: SourceFile,
    pub diag: Diagnostic,
}

/// Why parse+check stopped: a setup message, or source errors (parse errors are kept apart because
/// `uw build` ends them with its own "fix the errors" line).
pub enum CheckFailure {
    Message(String),
    Parse(Vec<Problem>),
    Errors(Vec<Problem>),
}

/// The parse+check result for a mod folder; no kit, no emit.
pub struct Checked {
    pub sources: Vec<uw_check::Source>,
    pub model: crate::model::Model,
    pub checked: uw_check::Checked,
}

/// The folder's plain absolute path and its `.uw` files (sorted; `names.uw` is game names, not a
/// source file).
pub fn folder(dir: &Path) -> Result<(PathBuf, Vec<PathBuf>), String> {
    let dir = dir.canonicalize().map_err(|e| format!("{}: {e}", dir.display()))?;
    // Windows canonical paths start with \\?\ ; plain paths read better and work the same here.
    let dir = match dir.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        Some(plain) => PathBuf::from(plain),
        None => dir,
    };
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "uw"))
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some(uw_decomp::names::FILE))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err(format!("no .uw files in {}", dir.display()));
    }
    Ok((dir, paths))
}

/// Steps 1-2 of `uw build` (parse, check), callable without the kit.
pub fn check_sources(dir: &Path, paths: &[PathBuf], atlas: Option<PathBuf>) -> Result<Checked, CheckFailure> {
    // 1. Parse: UW's own words (`lib/*.uw`) first, so every mod sees them (T-079), then the mod's
    // files.
    let mut entries: Vec<(PathBuf, bool)> = lib_files(dir).map_err(CheckFailure::Message)?.into_iter().map(|path| (path, true)).collect();
    entries.extend(paths.iter().cloned().map(|path| (path, false)));
    let sources = parse_sources_collect(&entries)?;

    // 2. Check against the atlas, with game names (P5b): the mod's own `names.uw`, otherwise the
    // names `uw new` derives from the kit's display names.
    let mut model = crate::load(atlas.or_else(|| find_up(dir, crate::ATLAS_REL))).map_err(CheckFailure::Message)?;
    install_game_names(&mut model.atlas, dir).map_err(CheckFailure::Message)?;
    // The record schemas (`kit-meta/records`): `troop … like …` writes its fields through the
    // troop layout, so a mod without them is told that instead of a guess (T-092).
    let schemas = match crate::roundtrip::find_schemas_from(dir) {
        Some(schemas) => crate::roundtrip::load_schemas(&schemas).map_err(CheckFailure::Message)?,
        None => Vec::new(),
    };
    let fallback = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "uw_mod".into());
    match uw_check::check_with_schemas(&model.atlas, &sources, &fallback, &schemas) {
        Ok(checked) => Ok(Checked { sources, model, checked }),
        Err(errors) => Err(CheckFailure::Errors(
            errors
                .into_iter()
                .map(|e| {
                    let source = &sources[e.file];
                    Problem { path: entries[e.file].0.clone(), file: source.file.clone(), diag: e.diag }
                })
                .collect(),
        )),
    }
}

/// Runs the build, printing progress. `Err` is the message for the user.
pub fn run(opts: Options) -> Result<(), String> {
    let (dir, paths) = folder(&opts.dir)?;
    let Checked { sources, model, checked } = match check_sources(&dir, &paths, opts.atlas.clone()) {
        Ok(checked) => checked,
        Err(CheckFailure::Message(message)) => return Err(message),
        Err(CheckFailure::Parse(problems)) => {
            for p in &problems {
                eprintln!("{}\n", p.diag.render(&p.file));
            }
            return Err("fix the errors above and run `uw build` again".into());
        }
        Err(CheckFailure::Errors(problems)) => {
            for p in &problems {
                eprintln!("{}\n", p.diag.render(&p.file));
            }
            return Err(format!(
                "{} {} found; nothing was built",
                problems.len(),
                if problems.len() == 1 { "error" } else { "errors" }
            ));
        }
    };
    if !checked.name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || checked.name.is_empty() {
        return Err(format!("mod name '{}' may only use letters, digits and _", checked.name));
    }
    println!("checked   {} file(s), {} change(s) to Native", sources.len(), checked.patches.len());
    let report = impact_report(&model, &checked);
    if !report.is_empty() {
        print!("{report}");
    }
    build_checked(&dir, &checked, opts)
}

/// Steps 3-5 of `uw build` for checked patches, whatever made them (a `.uw` mod folder, or a core
/// language project, `crate::core_build`): emit the overlay, run the kit in staging, install.
pub fn build_checked(dir: &Path, checked: &uw_check::Checked, opts: Options) -> Result<(), String> {
    let dir = dir.to_path_buf();
    // 3. Emit the overlay.
    let kit = opts
        .kit
        .or_else(|| find_kit(&dir))
        .or_else(|| std::env::current_dir().ok().and_then(|d| find_kit(&d)))
        .ok_or("Module System kit not found; pass --kit <folder>")?;
    let root = find_up(&dir, crate::ATLAS_REL)
        .and_then(|atlas| atlas.parent()?.parent()?.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| dir.clone());
    let work = root.join("target").join("uw-build").join(&checked.name);
    let overlay = work.join("overlay");
    reset_dir(&overlay, OVERLAY_MARKER)?;
    let emitted = uw_emit::emit(checked, opts.debug);
    if opts.debug {
        println!("debug     every menu will print its id in game (uw: menu-...)");
    }
    for file in &emitted.files {
        write(&overlay.join(&file.name), &file.text)?;
    }
    for module in &emitted.native_modules {
        let from = kit.join(format!("{module}.py"));
        fs::copy(&from, overlay.join(format!("native_{module}.py"))).map_err(|e| format!("{}: {e}", from.display()))?;
    }
    println!("generated {}", overlay.display());

    // 4. Run the kit.
    let python = opts.python.unwrap_or_else(|| PathBuf::from(uw_kitrun::DEFAULT_PYTHON27));
    if !python.is_file() {
        return Err(format!("Python 2.7 not found at {} (the kit needs it); pass --python <python.exe>", python.display()));
    }
    println!("building  with the kit (about 30 seconds)...");
    let stage = uw_kitrun::StageOptions { kit, staging: work.join("staging"), no_ids: false, overlay: Some(overlay) };
    let report = uw_kitrun::build(&stage, &python).map_err(|e| e.to_string())?;
    if !report.failed_steps.is_empty() {
        for step in report.steps.iter().filter(|s| s.exit != 0) {
            eprintln!("kit step {} failed:", step.step);
            let tail: Vec<&str> = step.stderr.lines().rev().take(12).collect();
            for line in tail.into_iter().rev() {
                eprintln!("    {line}");
            }
        }
        return Err("the kit rejected the generated files (see above); this is a uw bug if `uw build` printed no error before".into());
    }
    let problems: Vec<&String> = report.steps.iter().flat_map(|s| &s.problems).collect();
    for line in &problems {
        eprintln!("kit warning: {line}");
    }
    let out = work.join("staging").join("out");
    println!("built     {} files in {:.0} s", report.outputs.len(), report.total_seconds);

    // 5. Install.
    if !opts.install {
        println!("done      output in {} (not installed: --no-install)", out.display());
        return Ok(());
    }
    let modules = opts.modules.or_else(find_modules).ok_or(
        "Warband's Modules folder not found; pass --modules <folder> or --no-install",
    )?;
    let dest = install(&out, &modules, &checked.name)?;
    println!("installed {}", dest.display());
    println!("done      pick '{}' in the Warband launcher", checked.name);
    Ok(())
}

/// What the mod makes unreachable in Native. A replaced menu body that leaves the menu
/// (`change_screen_*`, `jump_to_menu`) means the menu's own options never show, so all of that
/// menu's Native edges are cut; whatever the new code itself mentions still counts as reached.
pub fn impact_report(model: &crate::model::Model, checked: &uw_check::Checked) -> String {
    use crate::model::Target;
    let mut cut = Vec::new();
    let mut new_refs = Vec::new();
    for patch in &checked.patches {
        for op in &patch.ops {
            for arg in &op.args {
                if let uw_check::Value::Ref(text) = arg {
                    if let Some((table, entry)) = model.atlas.resolve_ref(text) {
                        new_refs.push(Target::Entry { table: table.to_owned(), index: entry.index });
                    }
                }
            }
        }
        let leaves = patch.ops.iter().any(|op| op.name.starts_with("change_screen_") || op.name == "jump_to_menu");
        if patch.module == "module_game_menus" && patch.action == uw_check::Action::ReplaceBody && leaves {
            let base = patch.entry.split('#').next().unwrap_or(&patch.entry);
            if let Some((table, entry)) = model.atlas.resolve_ref(&format!("mnu_{base}")) {
                cut.push(Target::Entry { table: table.to_owned(), index: entry.index });
            }
        }
    }
    let mut lost = crate::impact::unreachable_after(model, &cut);
    // Still reached through the new code.
    let mut seen = std::collections::BTreeSet::new();
    let mut queue = new_refs;
    while let Some(node) = queue.pop() {
        if seen.insert(node.clone()) {
            for (next, _) in model.outgoing(&node) {
                if matches!(next, Target::Entry { .. }) {
                    queue.push(next.clone());
                }
            }
        }
    }
    lost.retain(|t| !seen.contains(t));
    crate::impact::render(model, &lost)
}

fn write(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// The game names the check accepts (P5b): the mod folder's own `names.uw` when it has one, otherwise
/// the names [`uw_decomp::names::derive`] reads from the kit's display names. A bad `names.uw` is an
/// error, never a silent fallback to the kit references.
fn install_game_names(atlas: &mut uw_atlas::Atlas, dir: &Path) -> Result<(), String> {
    if dir.join(uw_decomp::names::FILE).is_file() {
        crate::new::install_names(atlas, dir)?;
    } else {
        let names = uw_decomp::names::derive(atlas);
        atlas.set_game_names(&uw_decomp::names::pairs(&names));
    }
    Ok(())
}

/// Empties `dir` for reuse; refuses a non-empty folder without `marker`.
pub(crate) fn reset_dir(dir: &Path, marker: &str) -> Result<(), String> {
    if dir.exists() {
        let empty = fs::read_dir(dir).map_err(|e| e.to_string())?.next().is_none();
        if !empty && !dir.join(marker).is_file() {
            return Err(format!("{} exists and was not made by uw; refusing to delete it", dir.display()));
        }
        fs::remove_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    write(&dir.join(marker), "made by uw; deleted and rebuilt on every `uw build`\n")
}

pub(crate) fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| format!("{}: {e}", to.display()))?;
    for entry in fs::read_dir(from).map_err(|e| format!("{}: {e}", from.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let target = to.join(entry.file_name());
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), &target).map_err(|e| format!("{}: {e}", entry.path().display()))?;
        }
    }
    Ok(())
}

/// `<modules>/<name>` = a copy of `<modules>/Native` with the built files over it.
pub fn install(out: &Path, modules: &Path, name: &str) -> Result<PathBuf, String> {
    let native = modules.join("Native");
    if !native.join("module.ini").is_file() {
        return Err(format!("{} is not a Warband Native module folder", native.display()));
    }
    if name.eq_ignore_ascii_case("native") {
        return Err("a mod cannot be called Native".into());
    }
    let dest = modules.join(name);
    if dest.exists() {
        if !dest.join(MOD_MARKER).is_file() {
            return Err(format!(
                "{} already exists and was not made by uw; refusing to replace it (rename your mod or move that folder)",
                dest.display()
            ));
        }
        fs::remove_dir_all(&dest).map_err(|e| format!("{}: {e}", dest.display()))?;
    }
    copy_tree(&native, &dest)?;
    write(&dest.join(MOD_MARKER), "made by uw build from Native + this mod; `uw build` replaces it\n")?;
    copy_tree(out, &dest)?;
    Ok(dest)
}

/// Parses `(path, from_lib)` pairs in order. A lib file may hold only `fn` items (T-079): it is
/// UW's own word library, not a mod.
pub(crate) fn parse_sources(entries: &[(PathBuf, bool)]) -> Result<Vec<uw_check::Source>, String> {
    match parse_sources_collect(entries) {
        Ok(sources) => Ok(sources),
        Err(CheckFailure::Message(message)) => Err(message),
        Err(CheckFailure::Parse(problems)) | Err(CheckFailure::Errors(problems)) => {
            for p in &problems {
                eprintln!("{}\n", p.diag.render(&p.file));
            }
            Err("fix the errors above and run `uw build` again".into())
        }
    }
}

/// [`parse_sources`] without the printing: `Err` is what the caller should show.
fn parse_sources_collect(entries: &[(PathBuf, bool)]) -> Result<Vec<uw_check::Source>, CheckFailure> {
    let mut sources = Vec::new();
    let mut problems = Vec::new();
    for (path, from_lib) in entries {
        let text = fs::read_to_string(path).map_err(|e| CheckFailure::Message(format!("{}: {e}", path.display())))?;
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = SourceFile::new(name, text);
        match uw_syntax::parse(&file.text) {
            Ok(tree) => {
                if *from_lib
                    && (tree.module.is_some()
                        || tree.items.iter().any(|item| !matches!(item, uw_syntax::Item::Fn { .. })))
                {
                    return Err(CheckFailure::Message(format!("{}: a lib file may only hold `fn` items", path.display())));
                }
                sources.push(uw_check::Source { file, tree });
            }
            Err(diag) => problems.push(Problem { path: path.clone(), file, diag }),
        }
    }
    if problems.is_empty() {
        Ok(sources)
    } else {
        Err(CheckFailure::Parse(problems))
    }
}

/// Every `.uw` file of the nearest `lib` folder (UW's own words), sorted.
pub(crate) fn lib_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let Some(lib) = find_lib(dir) else { return Ok(Vec::new()) };
    let mut paths: Vec<PathBuf> = fs::read_dir(&lib)
        .map_err(|e| format!("{}: {e}", lib.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "uw"))
        .collect();
    paths.sort();
    Ok(paths)
}

/// The `lib` folder of this or a parent folder, found like the atlas.
fn find_lib(dir: &Path) -> Option<PathBuf> {
    let mut starts = vec![dir.to_path_buf()];
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    starts
        .iter()
        .flat_map(|s| s.ancestors().map(|a| a.join("lib")).collect::<Vec<_>>())
        .find(|p| p.is_dir())
}

pub(crate) fn find_up(dir: &Path, rel: &str) -> Option<PathBuf> {
    let mut starts = vec![dir.to_path_buf()];
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    starts
        .iter()
        .flat_map(|s| s.ancestors().map(|a| a.join(rel)).collect::<Vec<_>>())
        .find(|p| p.is_file())
}

/// A `Module_system*` folder with `build_module.bat` next to `dir` or one of its parents.
pub(crate) fn find_kit(dir: &Path) -> Option<PathBuf> {
    for ancestor in dir.ancestors() {
        let Ok(entries) = fs::read_dir(ancestor) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.starts_with("module_system") && path.join("build_module.bat").is_file() {
                return Some(path);
            }
        }
    }
    None
}

/// Warband's Modules folder in the default Steam library, if present.
pub(crate) fn find_modules() -> Option<PathBuf> {
    let candidates = [
        r"C:\Program Files (x86)\Steam\steamapps\common\MountBlade Warband\Modules",
        r"C:\Program Files\Steam\steamapps\common\MountBlade Warband\Modules",
    ];
    candidates.iter().map(PathBuf::from).find(|p| p.join("Native").join("module.ini").is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uw-build-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn fake_modules(root: &Path) -> (PathBuf, PathBuf) {
        let modules = root.join("Modules");
        fs::create_dir_all(modules.join("Native").join("Data")).unwrap();
        fs::write(modules.join("Native").join("module.ini"), "native").unwrap();
        fs::write(modules.join("Native").join("menus.txt"), "native menus").unwrap();
        fs::write(modules.join("Native").join("Data").join("x.txt"), "data").unwrap();
        let out = root.join("out");
        fs::create_dir_all(&out).unwrap();
        fs::write(out.join("menus.txt"), "uw menus").unwrap();
        (modules, out)
    }

    #[test]
    fn install_copies_native_then_overlays_and_marks() {
        let root = temp("install");
        let (modules, out) = fake_modules(&root);
        let dest = install(&out, &modules, "praven_start").unwrap();
        assert_eq!(fs::read_to_string(dest.join("menus.txt")).unwrap(), "uw menus");
        assert_eq!(fs::read_to_string(dest.join("Data").join("x.txt")).unwrap(), "data");
        assert!(dest.join(MOD_MARKER).is_file());
        // Native itself is untouched.
        assert_eq!(fs::read_to_string(modules.join("Native").join("menus.txt")).unwrap(), "native menus");
        // A second build replaces its own folder.
        fs::write(dest.join("stale.txt"), "old").unwrap();
        install(&out, &modules, "praven_start").unwrap();
        assert!(!dest.join("stale.txt").exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn install_refuses_foreign_folders_and_native() {
        let root = temp("refuse");
        let (modules, out) = fake_modules(&root);
        fs::create_dir_all(modules.join("BenimModum")).unwrap();
        fs::write(modules.join("BenimModum").join("precious.txt"), "keep").unwrap();
        let err = install(&out, &modules, "BenimModum").unwrap_err();
        assert!(err.contains("refusing to replace it"), "{err}");
        assert_eq!(fs::read_to_string(modules.join("BenimModum").join("precious.txt")).unwrap(), "keep");
        assert!(install(&out, &modules, "native").is_err());
        assert_eq!(fs::read_to_string(modules.join("Native").join("menus.txt")).unwrap(), "native menus");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn game_names_come_from_the_mods_file_or_the_kits_display_names() {
        let root = temp("names");
        let golden = || {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");
            uw_atlas::Atlas::from_path(path).unwrap()
        };
        // No `names.uw`: the derived Native names are installed ("Praven" -> `praven`).
        let mut atlas = golden();
        install_game_names(&mut atlas, &root).unwrap();
        assert_eq!(atlas.reference_for("praven"), Some("p_town_6"));
        // The mod's own `names.uw` is what the check accepts, not the derived names.
        fs::write(root.join(uw_decomp::names::FILE), "golden_town = p_town_6\n").unwrap();
        let mut atlas = golden();
        install_game_names(&mut atlas, &root).unwrap();
        assert_eq!(atlas.reference_for("golden_town"), Some("p_town_6"));
        assert_eq!(atlas.reference_for("praven"), None);
        // A wrong line stops the build instead of falling back to the kit references.
        fs::write(root.join(uw_decomp::names::FILE), "golden_town = p_town_99\n").unwrap();
        assert!(install_game_names(&mut golden(), &root).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn lib_files_are_parsed_before_the_mod_and_hold_only_fns() {
        let root = temp("lib");
        let lib = root.join("lib");
        fs::create_dir_all(&lib).unwrap();
        fs::write(
            lib.join("core.uw"),
            "fn give_gold(:amount) {\n    troop_add_gold(trp_player, :amount)\n}\n",
        )
        .unwrap();
        let mod_dir = root.join("mod");
        fs::create_dir_all(&mod_dir).unwrap();
        let mod_file = mod_dir.join("mod.uw");
        fs::write(&mod_file, "mod m\n\non game.start {\n    give_gold(100)\n}\n").unwrap();

        let entries = |with_mod: bool| -> Vec<(PathBuf, bool)> {
            let mut entries: Vec<(PathBuf, bool)> =
                lib_files(&mod_dir).unwrap().into_iter().map(|path| (path, true)).collect();
            if with_mod {
                entries.push((mod_file.clone(), false));
            }
            entries
        };
        let sources = parse_sources(&entries(true)).unwrap();
        assert_eq!(
            sources.iter().map(|s| s.file.name.as_str()).collect::<Vec<_>>(),
            ["core.uw", "mod.uw"]
        );
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");
        let atlas = uw_atlas::Atlas::from_path(path).unwrap();
        assert!(uw_check::check(&atlas, &sources, "m").is_ok());

        // A mod fn with a lib fn's name is an error: the lib file came first.
        fs::write(
            &mod_file,
            "mod m\n\nfn give_gold(:amount) {\n    troop_add_gold(trp_player, :amount)\n}\n\non game.start {\n    give_gold(100)\n}\n",
        )
        .unwrap();
        let sources = parse_sources(&entries(true)).unwrap();
        let errors = uw_check::check(&atlas, &sources, "m").unwrap_err();
        assert!(errors[0].diag.message.contains("already defined at core.uw:"), "{}", errors[0].diag.message);

        // A lib file may hold only `fn` items, and the error names the file.
        fs::write(lib.join("bad.uw"), "mod bad\n").unwrap();
        let err = match parse_sources(&entries(false)) {
            Ok(_) => panic!("a lib file with a `mod` item must be an error"),
            Err(err) => err,
        };
        assert!(err.contains("bad.uw") && err.contains("only hold `fn` items"), "{err}");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn reset_dir_only_clears_marked_folders() {
        let root = temp("reset");
        let foreign = root.join("foreign");
        fs::create_dir_all(&foreign).unwrap();
        fs::write(foreign.join("a.txt"), "keep").unwrap();
        assert!(reset_dir(&foreign, OVERLAY_MARKER).is_err());
        assert!(foreign.join("a.txt").is_file());
        let ours = root.join("ours");
        reset_dir(&ours, OVERLAY_MARKER).unwrap();
        fs::write(ours.join("old.py"), "x").unwrap();
        reset_dir(&ours, OVERLAY_MARKER).unwrap();
        assert!(!ours.join("old.py").exists() && ours.join(OVERLAY_MARKER).is_file());
        let _ = fs::remove_dir_all(&root);
    }
}

#[cfg(test)]
mod impact_tests {
    use uw_check::{Action, Checked, Op, Patch, Value};

    fn model() -> crate::model::Model {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_impact.json");
        crate::model::Model::new(uw_atlas::Atlas::from_path(path).unwrap())
    }

    fn menu_patch(ops: Vec<Op>) -> Checked {
        Checked {
            name: "t".into(),
            patches: vec![Patch {
                module: "module_game_menus".into(),
                var: "game_menus".into(),
                entry: "a".into(),
                path: vec![4],
                action: Action::ReplaceBody,
                ops,
                origin: "t.uw:1".into(),
            }],
        }
    }

    fn op(name: &str, args: Vec<Value>) -> Op {
        Op { name: name.into(), args, origin: "t.uw:2".into() }
    }

    #[test]
    fn leaving_a_menu_reports_what_it_no_longer_reaches() {
        let m = model();
        let report = super::impact_report(&m, &menu_patch(vec![op("change_screen_map", vec![])]));
        assert!(report.starts_with("impact: 2 Native entries"), "{report}");
        // The new code jumps to c itself: only b is lost.
        let report = super::impact_report(&m, &menu_patch(vec![op("jump_to_menu", vec![Value::Ref("mnu_c".into())])]));
        assert!(report.starts_with("impact: 1 Native entries"), "{report}");
        assert!(report.contains("menu_b") && !report.contains("menu_c"), "{report}");
        // A body that stays in the menu cuts nothing.
        assert_eq!(super::impact_report(&m, &menu_patch(vec![op("display_message", vec![Value::Text("@x".into())])])), "");
    }
}
