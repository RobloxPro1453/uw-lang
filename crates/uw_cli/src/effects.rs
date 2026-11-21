//! What an `owner` change really does at game start: the Native chain it enters and the later steps
//! that can overwrite it (T-117). The chain is data, in one place, so a new one is easy to add;
//! today only the center-ownership chain exists.

use uw_decomp::project::OwnerChange;

/// The center-ownership chain, as `(text, evidence)` pairs: the line to print after the two-space
/// indent, and the Native evidence it comes from. The last line names the scripts themselves, so the
/// hidden steps are visible by name (`give_center_to_lord` and friends).
const CENTER_CHAIN: &[(&str, &str)] = &[
    (
        "etki: game_start'ta merkez krallığa verilir",
        "(module_scripts.py:486-497)",
    ),
    (
        "dikkat: aynı blokta lord ataması da çalışır",
        "(module_scripts.py:567-596); lordu değişmezse merkez eski krallığa dönebilir",
    ),
    (
        "dikkat: lordu olmayan merkezlere kendi krallığından lord atanır",
        "(module_scripts.py:598)",
    ),
    (
        "dikkat: özgün krallık slotu o anki krallıktan yazılır",
        "(module_scripts.py:605)",
    ),
    (
        "zincir:",
        "give_center_to_faction_aux → give_center_to_lord → assign_lords_to_empty_centers → slot_center_original_faction",
    ),
];

/// The lines to print under an `owner` change line: what the change does at game start and the later
/// steps that can overwrite it. Every line starts with two spaces.
pub fn lines(_change: &OwnerChange) -> Vec<String> {
    CENTER_CHAIN
        .iter()
        .map(|(text, evidence)| format!("  {text} {evidence}"))
        .collect()
}
