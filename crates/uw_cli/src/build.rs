//! `uw build <mod folder>`: read the .uw files, check them against the atlas, write the wrappers,
//! run the kit in a staging copy and install the result as `<Warband>/Modules/<mod>`.
//!
//! Safety (data loss is the worst error, AGENTS rule 1): the kit folder is never written; a
//! `Modules/<mod>` folder is replaced only when it carries the `.uw-mod` marker this command
//! writes; Native and every other module are only read.

use std::fs;
use std::path::{Path, PathBuf};

use uw_diag::SourceFile;

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
}

/// Runs the build, printing progress. `Err` is the message for the user.
pub fn run(opts: Options) -> Result<(), String> {
    let dir = opts.dir.canonicalize().map_err(|e| format!("{}: {e}", opts.dir.display()))?;
    // Windows canonical paths start with \\?\ ; plain paths read better and work the same here.
    let dir = match dir.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        Some(plain) => PathBuf::from(plain),
        None => dir,
    };
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "uw"))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err(format!("no .uw files in {}", dir.display()));
    }

    // 1. Parse.
    let mut sources = Vec::new();
    let mut failed = false;
    for path in &paths {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let file = SourceFile::new(name, text);
        match uw_syntax::parse(&file.text) {
            Ok(tree) => sources.push(uw_check::Source { file, tree }),
            Err(d) => {
                eprintln!("{}\n", d.render(&file));
                failed = true;
            }
        }
    }
    if failed {
        return Err("fix the errors above and run `uw build` again".into());
    }

    // 2. Check against the atlas.
    let model = crate::load(opts.atlas.or_else(|| find_up(&dir, crate::ATLAS_REL)))?;
    let fallback = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "uw_mod".into());
    let checked = match uw_check::check(&model.atlas, &sources, &fallback) {
        Ok(checked) => checked,
        Err(errors) => {
            for e in &errors {
                eprintln!("{}\n", e.diag.render(&sources[e.file].file));
            }
            return Err(format!(
                "{} {} found; nothing was built",
                errors.len(),
                if errors.len() == 1 { "error" } else { "errors" }
            ));
        }
    };
    if !checked.name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') || checked.name.is_empty() {
        return Err(format!("mod name '{}' may only use letters, digits and _", checked.name));
    }
    println!("checked   {} file(s), {} change(s) to Native", sources.len(), checked.patches.len());

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
    let emitted = uw_emit::emit(&checked);
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

fn write(path: &Path, text: &str) -> Result<(), String> {
    fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))
}

/// Empties `dir` for reuse; refuses a non-empty folder without `marker`.
fn reset_dir(dir: &Path, marker: &str) -> Result<(), String> {
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

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
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

fn find_up(dir: &Path, rel: &str) -> Option<PathBuf> {
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
fn find_kit(dir: &Path) -> Option<PathBuf> {
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
fn find_modules() -> Option<PathBuf> {
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
