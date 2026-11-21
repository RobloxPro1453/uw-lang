//! `uw easy`: generate the easy vocabulary (`kit-meta/easy/ops.json`, `manual.toml`) from the kit.
//!
//! The rules live in [`uw_easy`]; this only finds the atlas (or the kit header when the atlas
//! cannot be dumped in this lane) and writes the two data files.

use std::path::PathBuf;
use std::process::ExitCode;

use uw_easy::ops::Report;

pub fn cmd(args: &[String]) -> ExitCode {
    let mut atlas = None;
    let mut header = None;
    let mut out = PathBuf::from("kit-meta/easy");
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--atlas" => match it.next() {
                Some(path) => atlas = Some(PathBuf::from(path)),
                None => return crate::fail("--atlas needs a path"),
            },
            "--header" => match it.next() {
                Some(path) => header = Some(PathBuf::from(path)),
                None => return crate::fail("--header needs a path"),
            },
            "--out" => match it.next() {
                Some(path) => out = PathBuf::from(path),
                None => return crate::fail("--out needs a folder"),
            },
            other => return crate::fail(&format!("easy: unexpected argument {other}")),
        }
    }
    let atlas_file = match (atlas, &header) {
        (Some(path), _) => Some(path),
        (None, Some(_)) => None,
        (None, None) => match crate::atlas_path(None) {
            Ok(path) => Some(PathBuf::from(path)),
            Err(message) => return crate::fail(&message),
        },
    };
    let atlas = match (&atlas_file, &header) {
        (Some(path), _) => match uw_atlas::Atlas::from_path(path) {
            Ok(atlas) => atlas,
            Err(err) => return crate::fail(&format!("{}: {err}", path.display())),
        },
        (None, Some(path)) => match uw_easy::ops::atlas_from_header(path) {
            Ok(atlas) => atlas,
            Err(err) => return crate::fail(&format!("{}: {err}", path.display())),
        },
        (None, None) => unreachable!("an atlas path or a header is always set here"),
    };
    let report = Report::generate(&atlas);
    if let Err(err) = report.write(&out) {
        return crate::fail(&format!("{}: {err}", out.display()));
    }
    // Constants and slots (constants.json, slots.json) need the full atlas, not the header.
    if let Some(path) = &atlas_file {
        match uw_easy::generate(path, out.join("prefixes.toml"), &out) {
            Ok(_) => println!("easy: constants.json, slots.json -> {}", out.display()),
            Err(err) => return crate::fail(&format!("constants/slots: {err}")),
        }
    }
    println!(
        "easy: {} auto, {} manual, {} name clashes -> {}",
        report.auto(),
        report.manual_count(),
        report.clashes.len(),
        out.display()
    );
    ExitCode::SUCCESS
}
