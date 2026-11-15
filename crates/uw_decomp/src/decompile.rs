//! Native operation block -> UW text (docs/tasks/T-020).

use crate::RawOp;
use uw_atlas::Atlas;

/// How much structure the printer may recover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Clean `try_begin/else_try/try_end` -> `if/else`, flags -> `or`/`not`, `assign` -> `=`.
    Structured,
    /// One UW call per operation, flags written as `neg|op(...)`; must always round-trip.
    Raw,
}

/// UW statement text for `ops` (no surrounding braces), one statement per line.
pub fn decompile(_atlas: &Atlas, _ops: &[RawOp], _mode: Mode) -> String {
    String::new()
}
