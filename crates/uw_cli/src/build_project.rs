//! P3b (docs/meclis/YON2-KARAR.md §3.3): build a whole-table project with the kit and, when asked,
//! compare every output `.txt` with the Native baseline (`kit-meta/native.baseline.json`, sha256 per
//! file).
//!
//! `uw build-project --native-check`: the 27 modules are written from the atlas values themselves.
//! This proves the module writer (`uw_decomp::project::pymodule`) before a project folder feeds it.
//! `uw build-project <dir>`: the same modules from the project folder (`project::load`), installed
//! with `uw build`'s safety rules.
//! The kit folder is never written (staging copy, `uw_kitrun`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value as Json;
use uw_decomp::project::pymodule::module_text;
use uw_decomp::records::{self, Schema};

const OVERLAY_MARKER: &str = ".uw-overlay";
const USAGE: &str = "usage:
  uw build-project <dir> [--compare-native] [--no-install]
      write the project folder's whole tables, build them with the kit and install the mod;
      --compare-native: say how many output files are byte-identical to Native
  uw build-project --native-check
      the same, but the modules come from the atlas values themselves (no project folder)";

/// The module files of a whole-table build: `module_<t>.py` text per atlas table, records in order.
pub fn modules(atlas: &uw_atlas::Atlas, records: &BTreeMap<String, Vec<Json>>) -> Vec<(String, String)> {
    atlas
        .tables
        .iter()
        .filter_map(|(name, table)| {
            let values = records.get(name)?;
            // The kit's own module comes along as `native_<module>.py`: its imports and helpers
            // (`find_troop`, `level_bits` …) stay visible to `process_*`; only the list is replaced.
            let text = format!("from native_{} import *
{}", table.module, module_text(&table.var, values));
            Some((format!("{}.py", table.module), text))
        })
        .collect()
}

/// A finished kit run and where the files it wrote are.
pub struct Built {
    pub report: uw_kitrun::Report,
    /// `<staging>/out`, the files the kit wrote (what `install` copies).
    pub out: PathBuf,
}

/// Writes `modules` (and the kit's own `native_<module>.py` next to them) into an overlay under
/// `<root>/target/uw-build/<name>` and runs the kit on it.
pub fn build(
    root: &Path,
    kit: &Path,
    python: &Path,
    name: &str,
    modules: &[(String, String)],
) -> Result<Built, String> {
    let work = root.join("target").join("uw-build").join(name);
    let overlay = work.join("overlay");
    crate::build::reset_dir(&overlay, OVERLAY_MARKER)?;
    for (file, text) in modules {
        let path = overlay.join(file);
        std::fs::write(&path, text.as_bytes()).map_err(|e| format!("{}: {e}", path.display()))?;
        let native = kit.join(file);
        let copy = overlay.join(format!("native_{file}"));
        std::fs::copy(&native, &copy).map_err(|e| format!("{}: {e}", native.display()))?;
    }
    let stage = uw_kitrun::StageOptions {
        kit: kit.to_path_buf(),
        staging: work.join("staging"),
        no_ids: false,
        overlay: Some(overlay),
    };
    let report = uw_kitrun::build(&stage, python).map_err(|e| e.to_string())?;
    let out = work.join("staging").join("out");
    Ok(Built { report, out })
}

/// The kit's failed steps, one block each (its last stderr lines).
pub fn failure_lines(report: &uw_kitrun::Report) -> Vec<String> {
    let mut lines = Vec::new();
    for step in report.steps.iter().filter(|s| s.exit != 0) {
        lines.push(format!("kit step {} failed:", step.step));
        lines.extend(step.stderr.lines().rev().take(8).collect::<Vec<_>>().into_iter().rev().map(|l| format!("    {l}")));
    }
    lines
}

/// Compares the kit's outputs with Native: the differing files' lines, how many outputs are
/// byte-identical and how many the baseline holds.
pub fn compare(root: &Path, report: &uw_kitrun::Report) -> Result<(Vec<String>, usize, usize), String> {
    let baseline_path = root.join("kit-meta").join("native.baseline.json");
    let baseline: Json = serde_json::from_str(
        &std::fs::read_to_string(&baseline_path).map_err(|e| format!("{}: {e}", baseline_path.display()))?,
    )
    .map_err(|e| format!("{}: {e}", baseline_path.display()))?;
    let want = baseline["outputs"].as_object().ok_or("baseline has no outputs")?;
    let mut same = 0;
    let mut lines = Vec::new();
    for (file, info) in want {
        match report.outputs.get(file) {
            Some(got) if Some(got.sha256.as_str()) == info["sha256"].as_str() => same += 1,
            Some(got) => lines.push(format!("differs  {file} ({} bytes, Native {})", got.bytes, info["bytes"])),
            None => lines.push(format!("missing  {file}")),
        }
    }
    for file in report.outputs.keys().filter(|f| !want.contains_key(*f)) {
        lines.push(format!("extra    {file}"));
    }
    Ok((lines, same, want.len()))
}

/// `outputs N/32 byte-identical to Native` (and the time the kit took).
pub fn outputs_line(same: usize, want: usize, seconds: f64) -> String {
    format!("outputs  {same}/{want} byte-identical to Native ({seconds:.0} s)")
}

/// Runs the kit on `modules` (overlay) and compares the outputs with the baseline. Returns the lines of
/// the report and whether every output is byte-identical.
pub fn build_and_compare(
    root: &Path,
    kit: &Path,
    python: &Path,
    name: &str,
    modules: &[(String, String)],
) -> Result<(Vec<String>, bool), String> {
    let built = build(root, kit, python, name, modules)?;
    let report = &built.report;
    let mut lines = failure_lines(report);
    let (mut differences, same, want) = compare(root, report)?;
    lines.append(&mut differences);
    let ok = report.failed_steps.is_empty() && same == want && report.outputs.len() == want;
    lines.insert(0, outputs_line(same, want, report.total_seconds));
    Ok((lines, ok))
}

/// What the command line asked for.
#[derive(Debug, PartialEq)]
pub struct Args {
    /// The project folder (`--native-check` has none).
    pub dir: Option<PathBuf>,
    /// Build the atlas values themselves, no project folder.
    pub native_check: bool,
    /// Compare every output with Native.
    pub compare_native: bool,
    /// Install the result as `<Warband>/Modules/<project folder name>`.
    pub install: bool,
}

pub fn parse_args(args: &[String]) -> Result<Args, String> {
    let mut parsed = Args { dir: None, native_check: false, compare_native: false, install: true };
    for arg in args {
        match arg.as_str() {
            "--native-check" => parsed.native_check = true,
            "--compare-native" => parsed.compare_native = true,
            "--no-install" => parsed.install = false,
            other if other.starts_with('-') => return Err(format!("unknown argument `{other}`\n{USAGE}")),
            other if parsed.dir.is_none() => parsed.dir = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument `{other}`\n{USAGE}")),
        }
    }
    if parsed.native_check && parsed.dir.is_some() {
        return Err(format!("--native-check builds the atlas values themselves; no project folder is needed\n{USAGE}"));
    }
    if !parsed.native_check && parsed.dir.is_none() {
        return Err(format!("which project folder should I build?\n{USAGE}"));
    }
    Ok(parsed)
}

pub fn cmd(args: &[String]) -> ExitCode {
    let args = match parse_args(args) {
        Ok(args) => args,
        Err(message) => return crate::fail(&message),
    };
    if args.native_check {
        return native_check();
    }
    let Some(dir) = args.dir.as_deref() else {
        return crate::fail(&format!("which project folder should I build?\n{USAGE}"));
    };
    build_project(dir, &args)
}

/// `uw build-project --native-check`: the modules are the atlas values themselves.
fn native_check() -> ExitCode {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let model = match crate::load(None) {
        Ok(model) => model,
        Err(message) => return crate::fail(&message),
    };
    let Some(kit) = crate::build::find_kit(&root) else {
        return crate::fail("Module System kit not found");
    };
    let python = PathBuf::from(uw_kitrun::DEFAULT_PYTHON27);
    let records: BTreeMap<String, Vec<Json>> = model
        .atlas
        .tables
        .iter()
        .map(|(name, table)| (name.clone(), table.entries.iter().map(|e| e.value.clone()).collect()))
        .collect();
    let modules = modules(&model.atlas, &records);
    println!("modules  {} written from the atlas", modules.len());
    match build_and_compare(&root, &kit, &python, "_native_check", &modules) {
        Ok((lines, ok)) => {
            for line in lines {
                println!("{line}");
            }
            if ok { ExitCode::SUCCESS } else { ExitCode::FAILURE }
        }
        Err(message) => crate::fail(&message),
    }
}

/// `uw build-project <dir>`: the same build, from the project folder.
fn build_project(dir: &Path, args: &Args) -> ExitCode {
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let (atlas, schemas) = match crate::new::atlas_and_schemas(None) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    let mut compile = |text: &str| crate::roundtrip::compile(&atlas, text).map(|ops| records::ops_to_json(&ops));
    let mut parse = |schema: &Schema, text: &str| records::parse_with_blocks(schema, text, &mut compile);
    let records = match uw_decomp::project::load::load(&atlas, &schemas, dir, &mut parse) {
        Ok(records) => records,
        Err(errors) => {
            for line in &errors {
                eprintln!("  {line}");
            }
            return crate::fail(&format!(
                "{} problem{} in {}; nothing was built",
                errors.len(),
                if errors.len() == 1 { "" } else { "s" },
                dir.display()
            ));
        }
    };
    // The mod name is the project folder's name, checked like `uw build` checks it.
    let Some(name) = dir.file_name().map(|n| n.to_string_lossy().into_owned()) else {
        return crate::fail(&format!("{} has no folder name to use as the mod name", dir.display()));
    };
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return crate::fail(&format!("mod name '{name}' may only use letters, digits and _"));
    }
    let modules = modules(&atlas, &records);
    println!("modules  {} written from {}", modules.len(), dir.display());
    let Some(kit) = crate::build::find_kit(dir).or_else(|| crate::build::find_kit(&root)) else {
        return crate::fail("Module System kit not found");
    };
    let python = PathBuf::from(uw_kitrun::DEFAULT_PYTHON27);
    let built = match build(&root, &kit, &python, &name, &modules) {
        Ok(built) => built,
        Err(message) => return crate::fail(&message),
    };
    for line in failure_lines(&built.report) {
        eprintln!("{line}");
    }
    for problem in built.report.steps.iter().flat_map(|s| &s.problems) {
        eprintln!("kit warning: {problem}");
    }
    println!("built     {} files in {:.0} s", built.report.outputs.len(), built.report.total_seconds);
    let mut ok = built.report.failed_steps.is_empty();
    if args.compare_native {
        match compare(&root, &built.report) {
            Ok((lines, same, want)) => {
                println!("{}", outputs_line(same, want, built.report.total_seconds));
                for line in lines {
                    println!("{line}");
                }
                ok = ok && same == want && built.report.outputs.len() == want;
            }
            Err(message) => return crate::fail(&message),
        }
    }
    if !ok {
        return crate::fail("the build did not pass (see above); nothing was installed");
    }
    if !args.install {
        println!("done      output in {} (not installed: --no-install)", built.out.display());
        return ExitCode::SUCCESS;
    }
    let Some(modules_dir) = crate::build::find_modules() else {
        return crate::fail("Warband's Modules folder not found; pass --no-install");
    };
    match crate::build::install(&built.out, &modules_dir, &name) {
        Ok(dest) => {
            println!("installed {}", dest.display());
            println!("done      pick '{}' in the Warband launcher", name);
            ExitCode::SUCCESS
        }
        Err(message) => crate::fail(&message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two tables; the module texts and the mod name come from these.
    const TINY_ATLAS: &str = r#"{
      "schema": 1, "kit_hash": "t057", "ops": [], "hooks": [], "ranges": [],
      "tables": {
        "factions": {"module":"module_factions","var":"factions","id_file":"ID_factions","id_prefix":"fac_",
          "entries": [
            {"index":0,"name":"no_faction","id":"fac_no_faction","dup":null,"line":1,
              "value":["no_faction","No Faction",0,10,[],[]]}
          ]},
        "items": {"module":"module_items","var":"items","id_file":"ID_items","id_prefix":"itm_",
          "entries": [
            {"index":0,"name":"sword","id":"itm_sword","dup":null,"line":2,
              "value":["sword","Sword",0,0,0]}
          ]}
      }
    }"#;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    /// The atlas' own values per table, like `--native-check` builds them.
    fn atlas_records(atlas: &uw_atlas::Atlas) -> BTreeMap<String, Vec<Json>> {
        atlas
            .tables
            .iter()
            .map(|(name, table)| (name.clone(), table.entries.iter().map(|e| e.value.clone()).collect()))
            .collect()
    }

    #[test]
    fn arguments_take_a_folder_and_the_flags() {
        let parsed = parse_args(&args(&["modum"])).unwrap();
        assert_eq!(parsed.dir, Some(PathBuf::from("modum")));
        assert!(parsed.install && !parsed.compare_native && !parsed.native_check, "{parsed:?}");

        let parsed = parse_args(&args(&["modum", "--compare-native", "--no-install"])).unwrap();
        assert_eq!(parsed.dir, Some(PathBuf::from("modum")));
        assert!(parsed.compare_native && !parsed.install, "{parsed:?}");

        // `--native-check` keeps working and needs no folder.
        let parsed = parse_args(&args(&["--native-check"])).unwrap();
        assert!(parsed.native_check && parsed.dir.is_none() && parsed.install, "{parsed:?}");
    }

    #[test]
    fn arguments_reject_unknown_flags_and_missing_folder() {
        assert!(parse_args(&args(&[])).is_err());
        let err = parse_args(&args(&["--what"])).unwrap_err();
        assert!(err.contains("unknown argument `--what`"), "{err}");
        let err = parse_args(&args(&["one", "two"])).unwrap_err();
        assert!(err.contains("unexpected argument `two`"), "{err}");
        let err = parse_args(&args(&["--native-check", "modum"])).unwrap_err();
        assert!(err.contains("no project folder is needed"), "{err}");
    }

    #[test]
    fn module_texts_import_the_native_module_and_name_their_table() {
        let atlas = uw_atlas::Atlas::from_str(TINY_ATLAS).unwrap();
        let modules = modules(&atlas, &atlas_records(&atlas));
        let mut files: Vec<&str> = modules.iter().map(|(file, _)| file.as_str()).collect();
        files.sort();
        assert_eq!(files, ["module_factions.py", "module_items.py"]);
        for (file, text) in &modules {
            let module = file.strip_suffix(".py").unwrap();
            assert!(text.starts_with(&format!("from native_{module} import *\n")), "{text}");
            let table = atlas.tables.values().find(|t| t.module == module).unwrap();
            assert!(text.contains(&format!("\n{} = [\n", table.var)), "{text}");
        }
        let factions = &modules.iter().find(|(file, _)| file == "module_factions.py").unwrap().1;
        assert!(factions.contains(r#"("no_faction", "No Faction", 0, 10, (), ())"#), "{factions}");
    }

    #[test]
    fn a_table_without_records_gets_no_module() {
        let atlas = uw_atlas::Atlas::from_str(TINY_ATLAS).unwrap();
        let mut records = atlas_records(&atlas);
        records.remove("items");
        let modules = modules(&atlas, &records);
        assert_eq!(modules.len(), 1, "{modules:?}");
        assert_eq!(modules[0].0, "module_factions.py");
    }

    #[test]
    fn compare_counts_identical_outputs_and_lists_the_others() {
        let root = std::env::temp_dir().join(format!("uw-build-project-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("kit-meta")).unwrap();
        std::fs::write(
            root.join("kit-meta").join("native.baseline.json"),
            r#"{"outputs": {"menus.txt": {"bytes": 5, "sha256": "aa"}, "scripts.txt": {"bytes": 3, "sha256": "bb"}}}"#,
        )
        .unwrap();
        let mut outputs = BTreeMap::new();
        outputs.insert("menus.txt".to_owned(), uw_kitrun::OutputFile { bytes: 5, sha256: "aa".into() });
        outputs.insert("scripts.txt".to_owned(), uw_kitrun::OutputFile { bytes: 3, sha256: "cc".into() });
        outputs.insert("extra.txt".to_owned(), uw_kitrun::OutputFile { bytes: 1, sha256: "dd".into() });
        let report = uw_kitrun::Report {
            kit_hash: "t057".into(),
            no_ids: false,
            overlay: None,
            steps: Vec::new(),
            failed_steps: Vec::new(),
            total_seconds: 1.0,
            outputs,
        };
        let (lines, same, want) = compare(&root, &report).unwrap();
        assert_eq!((same, want), (1, 2));
        assert!(lines.iter().any(|l| l.contains("differs  scripts.txt")), "{lines:?}");
        assert!(lines.iter().any(|l| l.contains("extra    extra.txt")), "{lines:?}");
        assert_eq!(outputs_line(same, want, 1.0), "outputs  1/2 byte-identical to Native (1 s)");
        let _ = std::fs::remove_dir_all(&root);
    }
}
