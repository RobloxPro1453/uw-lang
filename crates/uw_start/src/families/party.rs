//! party operations for the start dry run (see families/mod.rs for the rules).

use serde_json::Value as Json;

use crate::{Flow, Machine};

pub fn exec(_m: &mut Machine, _name: &str, _args: &[Json]) -> Option<Flow> {
    None
}