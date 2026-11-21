//! T-117: the effect lines printed under an `owner` change (`src/effects.rs`). `uw` is a binary
//! crate, so an integration test cannot `use` its modules; the module is pulled in by path instead.

#[path = "../src/effects.rs"]
mod effects;

use uw_decomp::project::OwnerChange;

fn change() -> OwnerChange {
    OwnerChange {
        party: "p_town_1".into(),
        from: "fac_kingdom_1".into(),
        to: "fac_kingdom_4".into(),
    }
}

#[test]
fn the_center_chain_names_the_lord_step_and_the_empty_center_step() {
    let lines = effects::lines(&change());
    assert!(lines.len() >= 4, "at least 4 lines, got {}", lines.len());
    let text = lines.join("\n");
    assert!(
        text.contains("give_center_to_lord"),
        "the lord step must be named:\n{text}"
    );
    assert!(
        text.contains("module_scripts.py:598"),
        "the empty-center step must be named:\n{text}"
    );
}

#[test]
fn every_line_starts_with_two_spaces() {
    for line in effects::lines(&change()) {
        assert!(line.starts_with("  "), "not indented by two spaces: {line:?}");
        assert!(!line.starts_with("   "), "indented by more than two spaces: {line:?}");
    }
}
