//! Operation families. Each file models one group of Native operations for the start dry run and
//! returns `None` for operations it does not own. Order matters only for speed.
//!
//! A family must:
//! - read operands with `m.get`, store with `m.set`, read record cells with `m.read`, write them
//!   with `m.write` (so the trace sees every write);
//! - build results with `Val::derived(n, &[inputs])` so random/unknown/deps carry over;
//! - never guess: an operation whose effect it cannot compute is left to `m.unknown_op`.

use serde_json::Value as Json;

use crate::{Flow, Machine};

pub mod faction;
pub mod misc;
pub mod party;
pub mod troop;

pub fn exec(m: &mut Machine, name: &str, args: &[Json]) -> Option<Flow> {
    party::exec(m, name, args)
        .or_else(|| troop::exec(m, name, args))
        .or_else(|| faction::exec(m, name, args))
        .or_else(|| misc::exec(m, name, args))
}