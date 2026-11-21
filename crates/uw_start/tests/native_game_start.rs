//! Runs Native's real `game_start` when the built atlas is present (target/uw-staging/atlas.json);
//! skipped with a note otherwise. Prints the snapshot size and the unmodelled operations, which is
//! the work list of the operation families.

use std::path::PathBuf;

use uw_start::{Flow, Machine};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn schemas() -> Vec<uw_decomp::records::Schema> {
    let dir = root().join("kit-meta/records");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).expect("kit-meta/records") {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "toml") {
            out.push(uw_decomp::records::Schema::from_path(&path).expect("schema"));
        }
    }
    out
}

#[test]
#[ignore = "slow until T-126; run: cargo test --release -p uw_start -- --ignored --nocapture"]
fn native_game_start_runs_to_the_end_and_is_deterministic() {
    let path = root().join("target/uw-staging/atlas.json");
    let Ok(atlas) = uw_atlas::Atlas::from_path(&path) else {
        eprintln!("skipped: no built atlas at {}", path.display());
        return;
    };
    let run = || {
        let mut m = Machine::new(&atlas, schemas(), 1);
        let flow = m.run_script("game_start", vec![]).expect("game_start runs");
        (flow, m.cells.clone(), m.report.clone(), m.trace.len())
    };
    let (flow, cells, report, writes) = run();
    assert_eq!(flow, Flow::Done);
    let (_, again, _, _) = run();
    assert_eq!(cells, again, "same seed, same snapshot");
    let certain = cells.values().filter(|v| v.certain()).count();
    let unknown: usize = report.unknown_ops.values().sum();
    eprintln!(
        "game_start: {} steps, {} writes, {} cells ({} certain), {} scripts, {} unmodelled op runs in {} kinds",
        report.steps,
        writes,
        cells.len(),
        certain,
        report.scripts_called.len(),
        unknown,
        report.unknown_ops.len()
    );
    let mut top: Vec<_> = report.unknown_ops.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1));
    for (name, n) in top.iter().take(40) {
        eprintln!("  unmodelled {n:>6}  {name}");
    }
    assert!(writes > 1000, "Native's start writes thousands of slots");
}
