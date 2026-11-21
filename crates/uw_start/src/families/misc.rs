//! Operations with no effect on the starting world: text, messages, notes, achievements,
//! sounds. They run without changing any record cell (see families/mod.rs for the rules).

use serde_json::Value as Json;

use crate::{Flow, Machine, Val};

pub fn exec(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    let silent = name.starts_with("str_")
        || name.starts_with("display_")
        || name.starts_with("add_") && (name.contains("_note") || name.contains("tableau"))
        || name.ends_with("_set_note_available")
        || name.starts_with("talk_info_")
        || matches!(name, "unlock_achievement" | "play_sound" | "set_trigger_result");
    if !silent {
        return None;
    }
    // `str_store_*` write string registers only; nothing in the world changes.
    let _ = (m, args, Val::default());
    Some(Flow::Done)
}