//! Operation blocks of the `triggers` table(s). Filled in by its own task (docs/tasks/T-02x).

use crate::Block;
use uw_atlas::Atlas;

/// Every operation block of this table, in table order.
pub fn blocks(_atlas: &Atlas) -> Vec<Block> {
    Vec::new()
}
