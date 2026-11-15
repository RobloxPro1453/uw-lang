//! P3b (docs/meclis/YON2-KARAR.md §3.3): build a whole-table project with the kit and compare every
//! output `.txt` with the Native baseline (`kit-meta/native.baseline.json`, sha256 per file).
//!
//! `uw build-project --native-check`: the 27 modules are written from the atlas values themselves.
//! This proves the module writer (`uw_decomp::project::pymodule`) before a project folder feeds it.
//! The kit folder is never written (staging copy, `uw_kitrun`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::Value as Json;
use uw_decomp::project::pymodule::module_text;

const OVERLAY_MARKER: &str = ".uw-overlay";

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

/// Runs the kit on `modules` (overlay) and compares the outputs with the baseline. Returns the lines of
/// the report and whether every output is byte-identical.
pub fn build_and_compare(
    root: &Path,
    kit: &Path,
    python: &Path,
    name: &str,
    modules: &[(String, String)],
) -> Result<(Vec<String>, bool), String> {
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
    let mut lines = Vec::new();
    for step in report.steps.iter().filter(|s| s.exit != 0) {
        lines.push(format!("kit step {} failed:", step.step));
        lines.extend(step.stderr.lines().rev().take(8).collect::<Vec<_>>().into_iter().rev().map(|l| format!("    {l}")));
    }
    let baseline_path = root.join("kit-meta").join("native.baseline.json");
    let baseline: Json = serde_json::from_str(
        &std::fs::read_to_string(&baseline_path).map_err(|e| format!("{}: {e}", baseline_path.display()))?,
    )
    .map_err(|e| format!("{}: {e}", baseline_path.display()))?;
    let want = baseline["outputs"].as_object().ok_or("baseline has no outputs")?;
    let mut same = 0;
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
    let ok = report.failed_steps.is_empty() && same == want.len() && report.outputs.len() == want.len();
    lines.insert(0, format!("outputs  {same}/{} byte-identical to Native ({:.0} s)", want.len(), report.total_seconds));
    Ok((lines, ok))
}

pub fn cmd(args: &[String]) -> ExitCode {
    if args.first().map(String::as_str) != Some("--native-check") {
        return crate::fail("usage: uw build-project --native-check");
    }
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
