//! `uw new <dir> --from native` (P3a, docs/meclis/YON2-KARAR.md §3.2): writes every Native record
//! into a project folder as UW text; `uw new --check <dir>` reads such a folder back and counts the
//! records that still match the atlas. The render/parse pair is `records::print` / `records::parse`
//! for now (T-043 swaps them).

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use uw_decomp::project;
use uw_decomp::records::{self, Schema};
use uw_atlas::{Atlas, Entry};

/// The atlas and the record schemas the export/check needs.
pub(crate) fn atlas_and_schemas(atlas_path: Option<PathBuf>) -> Result<(Atlas, Vec<Schema>), String> {
    let path = atlas_path
        .or_else(|| std::env::var_os("UW_ATLAS").map(PathBuf::from))
        .or_else(crate::find_atlas)
        .ok_or_else(|| {
            format!("no atlas found ({} in this or a parent folder)", crate::ATLAS_REL)
        })?;
    let atlas = Atlas::from_path(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let dir = crate::roundtrip::find_schemas()
        .ok_or("no record schemas found (kit-meta/records in this or a parent folder)")?;
    let schemas = crate::roundtrip::load_schemas(&dir)?;
    Ok((atlas, schemas))
}

/// Installs the project's game names (`names.uw`, P5b) into the atlas when the folder has the file.
/// Returns how many names; a bad file is an error (one line per problem).
pub(crate) fn install_names(atlas: &mut Atlas, dir: &Path) -> Result<usize, String> {
    let path = dir.join(uw_decomp::names::FILE);
    if !path.exists() {
        return Ok(0);
    }
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let names = uw_decomp::names::parse(atlas, &text)
        .map_err(|errors| errors.iter().map(|e| format!("{}: {e}", path.display())).collect::<Vec<_>>().join("
"))?;
    atlas.set_game_names(&uw_decomp::names::pairs(&names));
    Ok(names.len())
}

/// Copies Native's `languages/` (all languages, unchanged) into `<dir>/languages/` (T-089).
/// A missing game install is one printed line, not an error.
fn copy_native_languages(dir: &Path) -> Result<Option<String>, String> {
    let Some(modules) = crate::build::find_modules() else {
        return Ok(Some("languages skipped: Warband's Modules folder not found".to_owned()));
    };
    copy_languages(&modules, dir)
}

/// The path-parameterised half of [`copy_native_languages`], so a test can pass a fake install.
fn copy_languages(modules: &Path, dir: &Path) -> Result<Option<String>, String> {
    let from = modules.join("Native").join("languages");
    if !from.is_dir() {
        return Ok(Some(format!("languages skipped: {} not found", from.display())));
    }
    crate::build::copy_tree(&from, &dir.join("languages"))?;
    Ok(None)
}

/// `uw new <dir> --from native [--atlas <path>]`: writes the folder.
fn export_project(dir: &Path, atlas_path: Option<PathBuf>) -> ExitCode {
    let (mut atlas, schemas) = match atlas_and_schemas(atlas_path) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    // Game names first (P5b): every record and block below prints them.
    let names = uw_decomp::names::derive(&atlas);
    atlas.set_game_names(&uw_decomp::names::pairs(&names));
    let atlas = atlas;
    // Operation blocks print as readable `ops { … }` (T-043), the same text `uw roundtrip` measures.
    let blocks = uw_decomp::extract::all(&atlas);
    let paths = crate::roundtrip::block_paths(&blocks);
    let mut render = |schema: &Schema, entry: &Entry| {
        let paths = paths.get(&(schema.table.as_str(), entry.index)).map(Vec::as_slice).unwrap_or(&[]);
        records::print_with_blocks(&atlas, schema, entry, paths, &mut |block| crate::roundtrip::block_text(&atlas, block))
    };
    match project::export(&atlas, &schemas, dir, &mut render) {
        Ok(summary) => {
            let names_path = dir.join(uw_decomp::names::FILE);
            if let Err(e) = std::fs::write(&names_path, uw_decomp::names::print(&names)) {
                return crate::fail(&format!("{}: {e}", names_path.display()));
            }
            match copy_native_languages(dir) {
                Ok(Some(line)) => println!("{line}"),
                Ok(None) => {}
                Err(message) => return crate::fail(&message),
            }
            println!(
                "{} files, {} records, {} game names -> {}",
                summary.files,
                summary.records,
                names.len(),
                dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(message) => crate::fail(&message),
    }
}

/// `uw new --check <dir>`: reads the folder back and says how many records are exact.
fn check_project(dir: &Path, atlas_path: Option<PathBuf>) -> ExitCode {
    let (mut atlas, schemas) = match atlas_and_schemas(atlas_path) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    if let Err(message) = install_names(&mut atlas, dir) {
        return crate::fail(&message);
    }
    let atlas = atlas;
    let mut compile = |text: &str| crate::roundtrip::compile(&atlas, text).map(|ops| records::ops_to_json(&ops));
    let mut parse = |schema: &Schema, text: &str| records::parse_with_blocks(&atlas, schema, text, &mut compile);
    let blocks = uw_decomp::extract::all(&atlas);
    let paths = crate::roundtrip::block_paths(&blocks);
    let mut same = |schema: &Schema, entry: &Entry, value: &serde_json::Value| {
        let paths = paths.get(&(schema.table.as_str(), entry.index)).map(Vec::as_slice).unwrap_or(&[]);
        records::record_difference(&entry.value, value, paths).is_none()
    };
    let summary = match project::verify_with(&atlas, &schemas, dir, &mut parse, &mut same) {
        Ok(summary) => summary,
        Err(message) => return crate::fail(&message),
    };
    println!(
        "{}: {} records, exact {}, failed {}",
        dir.display(),
        summary.records,
        summary.exact,
        summary.failed
    );
    for failure in &summary.first_failures {
        println!("  {failure}");
    }
    if summary.failed == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub fn cmd(args: &[String]) -> ExitCode {
    let mut from: Option<String> = None;
    let mut check: Option<PathBuf> = None;
    let mut atlas_path: Option<PathBuf> = None;
    let mut dir: Option<PathBuf> = None;
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--from" => match it.next() {
                Some(kind) => from = Some(kind.clone()),
                None => return crate::fail("--from needs a source (native)"),
            },
            "--check" => match it.next() {
                Some(path) => check = Some(PathBuf::from(path)),
                None => return crate::fail("--check needs a project folder"),
            },
            "--atlas" => match it.next() {
                Some(path) => atlas_path = Some(PathBuf::from(path)),
                None => return crate::fail("--atlas needs a path"),
            },
            other if other.starts_with('-') => {
                return crate::fail(&format!("unknown argument `{other}`"))
            }
            other if dir.is_none() => dir = Some(PathBuf::from(other)),
            other => return crate::fail(&format!("unexpected argument `{other}`")),
        }
    }
    if let Some(check) = check {
        if dir.is_some() {
            return crate::fail("--check takes the folder it checks; no other folder is needed");
        }
        return check_project(&check, atlas_path);
    }
    let Some(dir) = dir else {
        return crate::fail("which folder should I write?\n  uw new <dir> --from native [--atlas <atlas.json>]");
    };
    match from.as_deref() {
        Some("native") => export_project(&dir, atlas_path),
        Some(other) => crate::fail(&format!("`--from {other}` is not supported; only `--from native`")),
        None => crate::fail("from where? `uw new <dir> --from native`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("uw-new-languages-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_projects_languages_are_a_copy_of_the_games() {
        let root = scratch("copy");
        let modules = root.join("Modules");
        let tr = modules.join("Native").join("languages").join("tr");
        std::fs::create_dir_all(&tr).unwrap();
        std::fs::write(tr.join("parties.csv"), "p_town_6|Praven Şehri\n").unwrap();
        let project = root.join("proje");
        std::fs::create_dir_all(&project).unwrap();

        assert_eq!(copy_languages(&modules, &project), Ok(None));
        let copied = project.join("languages").join("tr").join("parties.csv");
        assert_eq!(std::fs::read_to_string(&copied).unwrap(), "p_town_6|Praven Şehri\n");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_missing_game_install_is_one_skipped_line_not_an_error() {
        let root = scratch("missing");
        let project = root.join("proje");
        std::fs::create_dir_all(&project).unwrap();

        let line = copy_languages(&root.join("Modules"), &project).unwrap().unwrap();
        assert!(line.starts_with("languages skipped:"), "{line}");
        assert!(!project.join("languages").exists());
        let _ = std::fs::remove_dir_all(&root);
    }
}
