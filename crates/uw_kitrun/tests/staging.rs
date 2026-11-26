//! Staging rules on a fake kit in a temp folder, plus one run against the real kit when the kit
//! and Python 2.7 are on this machine (compared with `kit-meta/native.baseline.json`).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

use uw_kitrun::*;

/// A temp folder removed on drop.
struct Temp(PathBuf);

impl Temp {
    fn new(tag: &str) -> Temp {
        static N: AtomicU32 = AtomicU32::new(0);
        let p = std::env::temp_dir().join(format!(
            "uw_kitrun_{tag}_{}_{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        Temp(p)
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// Fake kit: bat, a module, an ID file, a nested folder, bytecode that must be skipped.
fn fake_kit(root: &Path) -> PathBuf {
    let kit = root.join("kit");
    write(&kit.join("build_module.bat"), "python.exe process_a.py\npython.exe process_b.py\n");
    write(&kit.join("module_x.py"), "x = [1]\n");
    write(&kit.join("ID_x.py"), "x_one = 0\n");
    write(&kit.join("module_info.py"), "export_dir = \"C:/somewhere/\"\n");
    write(&kit.join("sub/data.txt"), "d\n");
    write(&kit.join("module_x.pyc"), "bytecode");
    write(&kit.join("__pycache__/module_x.cpython-312.pyc"), "bytecode");
    kit
}

fn opts(kit: &Path, staging: &Path) -> StageOptions {
    StageOptions { kit: kit.into(), staging: staging.into(), no_ids: false, overlay: None }
}

#[test]
fn kit_files_skip_bytecode_and_list_folders_after_files() {
    let t = Temp::new("files");
    let kit = fake_kit(&t.0);
    assert_eq!(
        kit_files(&kit).unwrap(),
        ["ID_x.py", "build_module.bat", "module_info.py", "module_x.py", "sub/data.txt"]
    );
}

#[test]
fn kit_hash_ignores_module_info_only() {
    let t = Temp::new("hash");
    let kit = fake_kit(&t.0);
    let h0 = kit_hash(&kit).unwrap();
    write(&kit.join("module_info.py"), "export_dir = \"D:/elsewhere/\"\n");
    assert_eq!(kit_hash(&kit).unwrap(), h0);
    write(&kit.join("module_x.py"), "x = [2]\n");
    assert_ne!(kit_hash(&kit).unwrap(), h0);
}

#[test]
fn stage_copies_kit_writes_export_dir_and_leaves_kit_untouched() {
    let t = Temp::new("stage");
    let kit = fake_kit(&t.0);
    let before = kit_hash(&kit).unwrap();
    let info_before = fs::read(kit.join("module_info.py")).unwrap();
    let staging = t.0.join("staging");
    let out = stage(&opts(&kit, &staging)).unwrap();

    assert!(staging.join("ID_x.py").is_file());
    assert!(staging.join("sub/data.txt").is_file());
    assert!(!staging.join("module_x.pyc").exists());
    assert!(!staging.join("__pycache__").exists());
    assert!(out.is_dir() && out.ends_with("out"));
    let info = fs::read_to_string(staging.join("module_info.py")).unwrap();
    let export = format!("{}/", out.to_string_lossy().replace('\\', "/"));
    assert_eq!(info, format!("# written by uw kitrun\nexport_dir = \"{export}\"\n"));
    assert!(!info.contains('\\'));

    assert_eq!(kit_hash(&kit).unwrap(), before);
    assert_eq!(fs::read(kit.join("module_info.py")).unwrap(), info_before);
}

#[test]
fn stage_without_ids_and_with_overlay() {
    let t = Temp::new("overlay");
    let kit = fake_kit(&t.0);
    let overlay = t.0.join("overlay");
    write(&overlay.join("module_x.py"), "from native_module_x import *\n");
    write(&overlay.join("native_module_x.py"), "x = [1]\n");
    let staging = t.0.join("staging");
    let mut o = opts(&kit, &staging);
    o.no_ids = true;
    o.overlay = Some(overlay);
    stage(&o).unwrap();
    assert!(!staging.join("ID_x.py").exists());
    assert_eq!(
        fs::read_to_string(staging.join("module_x.py")).unwrap(),
        "from native_module_x import *\n"
    );
    assert!(staging.join("native_module_x.py").is_file());
}

#[test]
fn restage_replaces_old_staging_content() {
    let t = Temp::new("restage");
    let kit = fake_kit(&t.0);
    let staging = t.0.join("staging");
    stage(&opts(&kit, &staging)).unwrap();
    write(&staging.join("leftover.pyc"), "old");
    write(&staging.join("out/old.txt"), "old");
    stage(&opts(&kit, &staging)).unwrap();
    assert!(!staging.join("leftover.pyc").exists());
    assert!(!staging.join("out/old.txt").exists());
}

#[test]
fn refuses_staging_inside_kit_and_kit_inside_staging() {
    let t = Temp::new("overlap");
    let kit = fake_kit(&t.0);
    let before = kit_files(&kit).unwrap();
    let e = stage(&opts(&kit, &kit.join("staging"))).unwrap_err();
    assert!(matches!(e, Error::StagingOverlapsKit { .. }), "{e}");
    let e = stage(&opts(&kit, &t.0)).unwrap_err();
    assert!(matches!(e, Error::StagingOverlapsKit { .. }), "{e}");
    assert_eq!(kit_files(&kit).unwrap(), before);
}

#[test]
fn refuses_to_delete_a_folder_it_did_not_make() {
    let t = Temp::new("foreign");
    let kit = fake_kit(&t.0);
    let foreign = t.0.join("my_mod");
    write(&foreign.join("precious.txt"), "keep me");
    let e = stage(&opts(&kit, &foreign)).unwrap_err();
    assert!(matches!(e, Error::NotAStagingFolder(_)), "{e}");
    assert_eq!(fs::read_to_string(foreign.join("precious.txt")).unwrap(), "keep me");
}

#[test]
fn outputs_lists_files_with_hashes() {
    let t = Temp::new("outputs");
    write(&t.0.join("b.txt"), "abc");
    write(&t.0.join("a/c.txt"), "");
    let map = outputs(&t.0).unwrap();
    assert_eq!(map.keys().collect::<Vec<_>>(), ["a/c.txt", "b.txt"]);
    assert_eq!(map["b.txt"].bytes, 3);
    assert_eq!(
        map["b.txt"].sha256,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

/// The real kit, built with Rust, must match the baseline written by `py/kitrun.py`.
#[test]
fn real_kit_build_matches_native_baseline() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let kit = root.join("Module_system 1.171");
    let python = Path::new(DEFAULT_PYTHON27);
    if !kit.is_dir() || !python.is_file() {
        eprintln!("skipped: kit or Python 2.7 missing");
        return;
    }
    let baseline: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("kit-meta/native.baseline.json")).unwrap(),
    )
    .unwrap();
    let t = Temp::new("real");
    let staging = t.0.join("staging");
    let report = build(&opts(&kit, &staging), python).unwrap();

    assert!(report.failed_steps.is_empty(), "failed: {:?}", report.failed_steps);
    assert_eq!(report.steps.len(), baseline["steps"].as_array().unwrap().len());
    assert_eq!(report.kit_hash, baseline["kit_hash"].as_str().unwrap());
    assert!(report.steps.iter().all(|s| s.problems.is_empty()));
    let want = baseline["outputs"].as_object().unwrap();
    assert_eq!(report.outputs.len(), want.len());
    for (name, file) in &report.outputs {
        assert_eq!(file.sha256, want[name]["sha256"].as_str().unwrap(), "{name}");
    }
    assert!(staging.join("report.json").is_file());
}
