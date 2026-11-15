//! One module per Native table that holds operation blocks. Each module only reads the atlas JSON
//! (`Entry::value`) and returns its blocks; it never parses or prints UW.

pub mod dialogs;
pub mod menus;
pub mod mission_templates;
pub mod other;
pub mod presentations;
pub mod scripts;
pub mod triggers;

use crate::Block;
use uw_atlas::Atlas;

/// Every operation block of every table, table by table.
pub fn all(atlas: &Atlas) -> Vec<Block> {
    let mut out = Vec::new();
    out.extend(scripts::blocks(atlas));
    out.extend(menus::blocks(atlas));
    out.extend(dialogs::blocks(atlas));
    out.extend(triggers::blocks(atlas));
    out.extend(mission_templates::blocks(atlas));
    out.extend(presentations::blocks(atlas));
    out.extend(other::blocks(atlas));
    out
}
