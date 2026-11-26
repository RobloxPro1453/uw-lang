//! Staging copy of the Module System kit and the kit's own build steps.
//!
//! The kit folder is never written. [`stage`] makes a fresh copy in a staging folder (bytecode
//! caches left out, optionally without the kit's `ID_*.py`, optionally with an overlay copied on
//! top), points `export_dir` at `<staging>/out/`, and [`run_steps`] runs the `process_*` steps one
//! by one with Python 2.7 (`-B -E -s`) in the order `build_module.bat` lists them. The bat itself
//! is never called.
//!
//! This is the Rust port of `py/kitrun.py` (Faz 0 reference); `kit_hash` and the output hashes
//! must stay identical to it, because `kit-meta/native.baseline.json` was written by it.
//!
//! Safety: staging folders are deleted and rebuilt on every run, so [`stage`] only deletes a
//! folder that is empty or carries the [`STAGING_MARKER`] file it wrote itself, and refuses any
//! staging folder that is inside the kit or contains it.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Python 2.7 used by the kit on the build machine.
pub const DEFAULT_PYTHON27: &str = r"C:\Python27\python.exe";

/// File written into every staging folder; only folders carrying it are ever deleted.
pub const STAGING_MARKER: &str = ".uw-staging";

/// Words that mark a kit output line as a problem (case-insensitive). Kept broad on purpose: the
/// baseline records them all, and Native prints none.
const PROBLEM_WORDS: [&str; 7] = [
    "error",
    "warning",
    "exception",
    "traceback",
    "unable",
    "not found",
    "invalid",
];

/// What to put into a staging folder.
#[derive(Debug, Clone)]
pub struct StageOptions {
    /// The kit folder (read only).
    pub kit: PathBuf,
    /// The staging folder; deleted and rebuilt.
    pub staging: PathBuf,
    /// Leave the kit's `ID_*.py` files out (build staging: UW writes its own).
    pub no_ids: bool,
    /// Folder whose files are copied over the kit copy (wrappers, `native_module_*.py` …).
    pub overlay: Option<PathBuf>,
}

/// One `process_*` step's result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepResult {
    pub step: String,
    /// Process exit code; `-1` when the process was killed by a signal.
    pub exit: i32,
    pub seconds: f64,
    pub stdout: String,
    pub stderr: String,
    /// Output lines that contain a problem word.
    pub problems: Vec<String>,
}

/// An output file of the kit build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputFile {
    pub bytes: u64,
    pub sha256: String,
}

/// Everything a kit run produced; written as `<staging>/report.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub kit_hash: String,
    pub no_ids: bool,
    pub overlay: Option<String>,
    pub steps: Vec<StepResult>,
    pub failed_steps: Vec<String>,
    pub total_seconds: f64,
    pub outputs: BTreeMap<String, OutputFile>,
}

/// Errors of staging and running the kit.
#[derive(Debug)]
pub enum Error {
    Io { path: PathBuf, source: io::Error },
    /// The staging folder is inside the kit, or the kit is inside it.
    StagingOverlapsKit { kit: PathBuf, staging: PathBuf },
    /// The staging folder exists, is not empty and was not made by `stage`.
    NotAStagingFolder(PathBuf),
    /// `build_module.bat` lists no `process_*.py` step.
    NoSteps(PathBuf),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io { path, source } => write!(f, "{}: {source}", path.display()),
            Error::StagingOverlapsKit { kit, staging } => write!(
                f,
                "staging folder {} overlaps the kit folder {}; the kit is never written",
                staging.display(),
                kit.display()
            ),
            Error::NotAStagingFolder(p) => write!(
                f,
                "{} exists and is not a uw staging folder (no {STAGING_MARKER}); refusing to delete it",
                p.display()
            ),
            Error::NoSteps(p) => write!(f, "{} lists no process_*.py step", p.display()),
            Error::Json(e) => write!(f, "report JSON: {e}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

fn io_err(path: &Path) -> impl FnOnce(io::Error) -> Error + '_ {
    move |source| Error::Io { path: path.to_path_buf(), source }
}

/// The kit's files, relative and `/`-separated, without `__pycache__` folders and `*.pyc` files.
/// Order: a folder's files (sorted), then its sub-folders (sorted), depth first — the same order
/// as `py/kitrun.py`, which `kit_hash` depends on.
pub fn kit_files(kit: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    walk(kit, "", &mut out)?;
    Ok(out)
}

fn walk(dir: &Path, rel: &str, out: &mut Vec<String>) -> Result<()> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for entry in fs::read_dir(dir).map_err(io_err(dir))? {
        let entry = entry.map_err(io_err(dir))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let kind = entry.file_type().map_err(io_err(&entry.path()))?;
        if kind.is_dir() {
            if name != "__pycache__" {
                dirs.push(name);
            }
        } else if !name.ends_with(".pyc") {
            files.push(name);
        }
    }
    files.sort();
    dirs.sort();
    for f in files {
        out.push(format!("{rel}{f}"));
    }
    for d in dirs {
        walk(&dir.join(&d), &format!("{rel}{d}/"), out)?;
    }
    Ok(())
}

/// sha256 of a file, lowercase hex.
pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path).map_err(io_err(path))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = file.read(&mut buf).map_err(io_err(path))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex(&hasher.finalize()))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Hash of every kit file except `module_info.py` (it only holds the user's export path).
pub fn kit_hash(kit: &Path) -> Result<String> {
    let mut hasher = Sha256::new();
    for rel in kit_files(kit)? {
        if rel == "module_info.py" {
            continue;
        }
        let file_hash = sha256_file(&kit.join(&rel))?;
        hasher.update(rel.as_bytes());
        hasher.update(b"\0");
        hasher.update(file_hash.as_bytes());
        hasher.update(b"\n");
    }
    Ok(hex(&hasher.finalize()))
}

/// The `process_*.py` steps of `build_module.bat`, in order (`python.exe <step>` lines).
pub fn steps_from_bat(kit: &Path) -> Result<Vec<String>> {
    let path = kit.join("build_module.bat");
    let bytes = fs::read(&path).map_err(io_err(&path))?;
    let steps = parse_bat_steps(&latin1(&bytes));
    if steps.is_empty() {
        return Err(Error::NoSteps(path));
    }
    Ok(steps)
}

/// Finds every `python.exe <ws> process_<word>.py` (case-insensitive), as `py/kitrun.py` does.
fn parse_bat_steps(text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase(); // ASCII-only change: byte offsets stay the same
    let bytes = text.as_bytes();
    let mut steps = Vec::new();
    let mut from = 0;
    while let Some(found) = lower[from..].find("python.exe") {
        let mut i = from + found + "python.exe".len();
        from = i;
        let ws_start = i;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i == ws_start || !lower[i..].starts_with("process_") {
            continue;
        }
        let name_start = i;
        i += "process_".len();
        let word_start = i;
        while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        if i > word_start && lower[i..].starts_with(".py") {
            steps.push(text[name_start..i + 3].to_string());
            from = i + 3;
        }
    }
    steps
}

fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

fn absolute(p: &Path) -> Result<PathBuf> {
    std::path::absolute(p).map_err(io_err(p))
}

/// Normalised path text for overlap checks: `/`, no trailing `/`, lowercase on Windows.
fn path_key(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    let s = s.trim_end_matches('/').to_string();
    if cfg!(windows) { s.to_lowercase() } else { s }
}

fn inside(child: &str, parent: &str) -> bool {
    child == parent || child.starts_with(&format!("{parent}/"))
}

/// Deletes `staging` only when it is empty or carries [`STAGING_MARKER`].
fn clear_staging(staging: &Path) -> Result<()> {
    if !staging.exists() {
        return Ok(());
    }
    let marker = staging.join(STAGING_MARKER).is_file();
    let empty = fs::read_dir(staging).map_err(io_err(staging))?.next().is_none();
    if !marker && !empty {
        return Err(Error::NotAStagingFolder(staging.to_path_buf()));
    }
    fs::remove_dir_all(staging).map_err(io_err(staging))
}

fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent).map_err(io_err(parent))?;
    }
    fs::copy(src, dst).map_err(io_err(src))?;
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> Result<()> {
    for entry in fs::read_dir(src).map_err(io_err(src))? {
        let entry = entry.map_err(io_err(src))?;
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if entry.file_type().map_err(io_err(&path))?.is_dir() {
            copy_tree(&path, &target)?;
        } else {
            copy_file(&path, &target)?;
        }
    }
    Ok(())
}

/// Builds a fresh staging copy and returns its `out/` folder (the kit's `export_dir`).
pub fn stage(opts: &StageOptions) -> Result<PathBuf> {
    let kit = absolute(&opts.kit)?;
    let staging = absolute(&opts.staging)?;
    let (k, s) = (path_key(&kit), path_key(&staging));
    if inside(&s, &k) || inside(&k, &s) {
        return Err(Error::StagingOverlapsKit { kit, staging });
    }
    clear_staging(&staging)?;
    fs::create_dir_all(&staging).map_err(io_err(&staging))?;
    fs::write(staging.join(STAGING_MARKER), b"made by uw_kitrun; deleted on every build\n")
        .map_err(io_err(&staging))?;

    for rel in kit_files(&kit)? {
        let name = rel.rsplit('/').next().unwrap_or(&rel);
        if opts.no_ids && name.starts_with("ID_") && name.ends_with(".py") {
            continue;
        }
        copy_file(&kit.join(&rel), &staging.join(&rel))?;
    }
    if let Some(overlay) = &opts.overlay {
        copy_tree(overlay, &staging)?;
    }

    let out_dir = staging.join("out");
    fs::create_dir_all(&out_dir).map_err(io_err(&out_dir))?;
    let export = format!("{}/", out_dir.to_string_lossy().replace('\\', "/"));
    let info = staging.join("module_info.py");
    fs::write(&info, format!("# written by uw kitrun\nexport_dir = \"{export}\"\n"))
        .map_err(io_err(&info))?;
    Ok(out_dir)
}

fn problem_lines(stdout: &str, stderr: &str) -> Vec<String> {
    format!("{stdout}\n{stderr}")
        .lines()
        .filter(|line| {
            let l = line.to_lowercase();
            PROBLEM_WORDS.iter().any(|w| l.contains(w))
        })
        .map(str::to_string)
        .collect()
}

/// Runs each step as `<python> -B -E -s <step>` in `staging`, with every `PYTHON*` environment
/// variable removed. A failing step does not stop the run; the report lists it.
pub fn run_steps(staging: &Path, steps: &[String], python: &Path) -> Result<Vec<StepResult>> {
    let mut results = Vec::with_capacity(steps.len());
    for step in steps {
        let mut cmd = Command::new(python);
        cmd.args(["-B", "-E", "-s", step]).current_dir(staging);
        for (key, _) in std::env::vars_os() {
            if key.to_string_lossy().to_uppercase().starts_with("PYTHON") {
                cmd.env_remove(&key);
            }
        }
        let t0 = Instant::now();
        let out = cmd.output().map_err(io_err(python))?;
        let seconds = (t0.elapsed().as_secs_f64() * 1000.0).round() / 1000.0;
        let stdout = latin1(&out.stdout).replace("\r\n", "\n");
        let stderr = latin1(&out.stderr).replace("\r\n", "\n");
        results.push(StepResult {
            step: step.clone(),
            exit: out.status.code().unwrap_or(-1),
            seconds,
            problems: problem_lines(&stdout, &stderr),
            stdout,
            stderr,
        });
    }
    Ok(results)
}

/// Every file under `out_dir` with its size and sha256, keyed by `/`-separated relative path.
pub fn outputs(out_dir: &Path) -> Result<BTreeMap<String, OutputFile>> {
    let mut map = BTreeMap::new();
    for rel in all_files(out_dir, "")? {
        let path = out_dir.join(&rel);
        let bytes = fs::metadata(&path).map_err(io_err(&path))?.len();
        map.insert(rel, OutputFile { bytes, sha256: sha256_file(&path)? });
    }
    Ok(map)
}

fn all_files(dir: &Path, rel: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).map_err(io_err(dir))? {
        let entry = entry.map_err(io_err(dir))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if entry.file_type().map_err(io_err(&entry.path()))?.is_dir() {
            out.extend(all_files(&entry.path(), &format!("{rel}{name}/"))?);
        } else {
            out.push(format!("{rel}{name}"));
        }
    }
    Ok(out)
}

/// Stage, run every step, collect outputs, write `<staging>/report.json`.
pub fn build(opts: &StageOptions, python: &Path) -> Result<Report> {
    let t0 = Instant::now();
    let steps = steps_from_bat(&opts.kit)?;
    let out_dir = stage(opts)?;
    let staging = absolute(&opts.staging)?;
    let results = run_steps(&staging, &steps, python)?;
    let report = Report {
        kit_hash: kit_hash(&opts.kit)?,
        no_ids: opts.no_ids,
        overlay: opts.overlay.as_ref().map(|p| p.to_string_lossy().into_owned()),
        failed_steps: results.iter().filter(|r| r.exit != 0).map(|r| r.step.clone()).collect(),
        steps: results,
        total_seconds: (t0.elapsed().as_secs_f64() * 1000.0).round() / 1000.0,
        outputs: outputs(&out_dir)?,
    };
    let path = staging.join("report.json");
    let json = serde_json::to_string_pretty(&report).map_err(Error::Json)?;
    fs::write(&path, json).map_err(io_err(&path))?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bat_steps_keep_order_and_case_and_skip_other_lines() {
        let bat = "@echo off\r\nC:\\Python27\\python.exe process_init.py\r\nrem python.exe notes.py\r\n\
                   C:\\Python27\\PYTHON.EXE   Process_Items.py\r\npython.exe process_.py\r\n\
                   python.exeprocess_x.py\r\nc:\\python27\\python.exe process_postfx.py\r\n@del *.pyc\r\n";
        assert_eq!(
            parse_bat_steps(bat),
            ["process_init.py", "Process_Items.py", "process_postfx.py"]
        );
    }

    #[test]
    fn problem_lines_match_words_case_insensitively() {
        let p = problem_lines("Exporting...\nWARNING: x\n", "Traceback (most recent call last):\nok");
        assert_eq!(p, ["WARNING: x", "Traceback (most recent call last):"]);
        assert!(problem_lines("Exporting strings...\n", "").is_empty());
    }

    #[test]
    fn overlap_check_is_by_path_component() {
        assert!(inside("c:/kit/staging", "c:/kit"));
        assert!(inside("c:/kit", "c:/kit"));
        assert!(!inside("c:/kit2/staging", "c:/kit"));
    }
}
