//! The document map (`docs/HARITA.md`) is the single entry point to the project's documents: every
//! document under `docs/` must be listed there (by its path or by a folder row ending in `/`), and every
//! path the map names must exist. A new document nobody listed, or a listed one that was moved or
//! deleted, fails the build — so no report is silently forgotten (owner, 2026-11-22).

use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Every backticked path in the map that starts with `docs/` (and the few outside it).
fn listed(map: &str) -> Vec<String> {
    map.split('`')
        .skip(1)
        .step_by(2)
        .filter(|s| (s.starts_with("docs/") || s.starts_with(".evrim/") || s.starts_with("kit-meta/") || *s == "CHANGELOG.md" || *s == "AGENTS.md") && !s.contains(' '))
        .map(str::to_owned)
        .collect()
}

fn collect(dir: &Path, out: &mut Vec<String>, root: &Path) {
    for entry in std::fs::read_dir(dir).expect("readable docs folder").flatten() {
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
        // Archive, task log and the viewer program are covered by their folder rows only.
        if rel.starts_with("docs/arsiv/") || rel.starts_with("docs/tasks/") || rel.starts_with("docs/graph_viewer/program") {
            continue;
        }
        if path.is_dir() {
            collect(&path, out, root);
        } else {
            out.push(rel);
        }
    }
}

#[test]
fn every_document_is_on_the_map_and_every_map_entry_exists() {
    let root = root();
    let map = std::fs::read_to_string(root.join("docs/HARITA.md")).expect("docs/HARITA.md exists");
    let entries = listed(&map);
    assert!(entries.len() > 20, "the map lists the documents: {entries:?}");

    let missing: Vec<&String> = entries.iter().filter(|e| !root.join(e.trim_end_matches('/')).exists()).collect();
    assert!(missing.is_empty(), "docs/HARITA.md names paths that do not exist (moved? update the map): {missing:?}");

    let mut docs = Vec::new();
    collect(&root.join("docs"), &mut docs, &root);
    let covered = |doc: &str| {
        doc == "docs/HARITA.md"
            || entries.iter().any(|e| e == doc || (e.ends_with('/') && doc.starts_with(e.as_str())))
    };
    let orphans: Vec<&String> = docs.iter().filter(|d| !covered(d)).collect();
    assert!(
        orphans.is_empty(),
        "documents not on docs/HARITA.md (add a row, or move them to docs/arsiv/): {orphans:?}"
    );
}
