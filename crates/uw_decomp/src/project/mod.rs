//! `uw new <dir> --from native` (P3a, docs/meclis/YON2-KARAR.md §3.2): writes every Native record as
//! UW record text into a project folder, and reads such a folder back to prove nothing was lost.
//! Export only: the folder is not compiled yet (that is P3b).

pub mod layout;

use std::path::Path;

use uw_atlas::{Atlas, Entry};

use crate::records::Schema;

/// What one export wrote.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExportSummary {
    pub files: usize,
    pub records: usize,
}

/// Writes every entry of every schema's table under `out`, at `layout::path_for`. Records that share a
/// path go into one file, in atlas order, separated by a blank line. `render` turns one entry into its
/// record text (records::print today; the block-aware printer after T-043). Filled by T-044.
pub fn export(
    atlas: &Atlas,
    schemas: &[Schema],
    out: &Path,
    render: &mut dyn FnMut(&Schema, &Entry) -> String,
) -> Result<ExportSummary, String> {
    let _ = (atlas, schemas, out, render);
    Err("project::export is not written yet (T-044)".into())
}
