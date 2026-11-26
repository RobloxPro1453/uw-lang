//! T-100: `uw check --json` points at a mod folder's error, and a clean folder is zero errors.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/atlas_golden.json");

/// A scratch folder under the system temp dir; the test that makes it removes it again.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("uw-check-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch folder");
    let dir = dir.canonicalize().expect("canonical scratch folder");
    // `uw check` prints plain paths, without Windows' `\\?\` long-path prefix.
    match dir.to_str().and_then(|s| s.strip_prefix(r"\\?\")) {
        Some(plain) => PathBuf::from(plain),
        None => dir,
    }
}

fn check(dir: &Path) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_uw"))
        .arg("check")
        .arg(dir)
        .args(["--json", "--atlas", FIXTURE])
        .output()
        .expect("uw runs");
    (
        out.status.success(),
        String::from_utf8(out.stdout).expect("utf-8 stdout"),
        String::from_utf8(out.stderr).expect("utf-8 stderr"),
    )
}

#[test]
fn json_points_at_the_unknown_name() {
    let dir = scratch("unknown");
    let input = dir.join("input.uw");
    std::fs::write(
        &input,
        "mod broken\n\noverride menu start_phase_2 {\n    on enter {\n        change_screan_map()\n    }\n}\n",
    )
    .expect("input.uw");

    let (ok, out, err) = check(&dir);
    assert!(!ok, "check must fail on an unknown name:\n{out}\n{err}");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "one error and the summary:\n{out}");

    let error: serde_json::Value = serde_json::from_str(lines[0]).expect("first line is JSON");
    assert_eq!(error["file"], input.display().to_string());
    assert_eq!(error["line"], 5);
    assert_eq!(error["column"], 9);
    assert_eq!(error["end_line"], 5);
    assert_eq!(error["end_column"], 26);
    assert_eq!(error["severity"], "error");
    assert_eq!(error["code"], "E0001");
    assert!(
        error["message"].as_str().unwrap_or("").contains("'change_screan_map' is not a Native operation"),
        "{error}"
    );
    assert!(error["help"].as_str().unwrap_or("").contains("change_screen_map"), "{error}");

    let summary: serde_json::Value = serde_json::from_str(lines[1]).expect("last line is JSON");
    assert_eq!(summary["summary"]["errors"], 1);
    assert_eq!(summary["summary"]["warnings"], 0);

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_clean_folder_is_zero_errors_and_exit_0() {
    let dir = scratch("clean");
    std::fs::write(dir.join("input.uw"), "mod clean\n").expect("input.uw");

    let (ok, out, err) = check(&dir);
    assert!(ok, "a clean folder must pass:\n{out}\n{err}");
    let summary: serde_json::Value = serde_json::from_str(out.lines().last().expect("summary line")).expect("JSON");
    assert_eq!(summary["summary"]["errors"], 0);
    assert_eq!(summary["summary"]["warnings"], 0);

    std::fs::remove_dir_all(&dir).ok();
}
