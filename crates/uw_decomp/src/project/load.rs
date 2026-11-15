//! P3b: a `uw new` project folder read back into whole tables, in `uw.lock` order. Filled by T-056.

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value as Json;
use uw_atlas::Atlas;

use crate::records::Schema;

/// Every table's record values, in the order the kit will number them: the `uw.lock` order first,
/// then records that are new in the project (appended, never inserted). `parse` reads one record's
/// text (records::parse_with_blocks). Errors are one line each, with `file:` and the record id.
pub fn load(
    atlas: &Atlas,
    schemas: &[Schema],
    dir: &Path,
    parse: &mut dyn FnMut(&Schema, &str) -> Result<(String, Json), String>,
) -> Result<BTreeMap<String, Vec<Json>>, Vec<String>> {
    let _ = (atlas, schemas, dir, parse);
    Err(vec!["project::load is not written yet (T-056)".into()])
}
