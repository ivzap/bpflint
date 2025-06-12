//! A linter for BPF C code.

mod lint;
mod report;

// TODO: Perhaps it's better to re-implement these types to decouple us
//       from `tree-sitter` and to have more control over details.
pub use tree_sitter::Point;
pub use tree_sitter::Range;

pub use crate::lint::LintMatch;
pub use crate::lint::lint;
pub use crate::report::report_terminal;
