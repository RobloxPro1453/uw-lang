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
fn atlas_and_schemas(atlas_path: Option<PathBuf>) -> Result<(Atlas, Vec<Schema>), String> {
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

/// `uw new <dir> --from native [--atlas <path>]`: writes the folder.
fn export_project(dir: &Path, atlas_path: Option<PathBuf>) -> ExitCode {
    let (atlas, schemas) = match atlas_and_schemas(atlas_path) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    // Operation blocks print as readable `ops { … }` (T-043), the same text `uw roundtrip` measures.
    let blocks = uw_decomp::extract::all(&atlas);
    let paths = crate::roundtrip::block_paths(&blocks);
    let mut render = |schema: &Schema, entry: &Entry| {
        let paths = paths.get(&(schema.table.as_str(), entry.index)).map(Vec::as_slice).unwrap_or(&[]);
        records::print_with_blocks(schema, entry, paths, &mut |block| crate::roundtrip::block_text(&atlas, block))
    };
    match project::export(&atlas, &schemas, dir, &mut render) {
        Ok(summary) => {
            println!(
                "{} files, {} records -> {} (not compiled yet: P3b)",
                summary.files,
                summary.records,
                dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(message) => crate::fail(&message),
    }
}

/// `uw new --check <dir>`: reads the folder back and says how many records are exact.
fn check_project(dir: &Path, atlas_path: Option<PathBuf>) -> ExitCode {
    let (atlas, schemas) = match atlas_and_schemas(atlas_path) {
        Ok(found) => found,
        Err(message) => return crate::fail(&message),
    };
    let mut compile = |text: &str| crate::roundtrip::compile(&atlas, text).map(|ops| records::ops_to_json(&ops));
    let mut parse = |schema: &Schema, text: &str| records::parse_with_blocks(schema, text, &mut compile);
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
