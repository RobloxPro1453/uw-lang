//! Where each record goes in a `uw new` project (P3a). Filled by T-045.

use uw_atlas::{Atlas, Entry};

use crate::records::Schema;

/// Relative path (forward slashes, ends in `.uw`) of the file that holds this record. Many records may
/// share one path; the exporter puts them in one file in atlas order.
pub fn path_for(atlas: &Atlas, schema: &Schema, entry: &Entry) -> String {
    let _ = (atlas, entry);
    format!("{}/{}.uw", schema.table, schema.table)
}
