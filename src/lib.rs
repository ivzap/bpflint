//! A linter for BPF C code.

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod redefine;

mod lint;
mod report;

// TODO: Perhaps it's better to re-implement these types to decouple us
//       from `tree-sitter` and to have more control over details.
pub use tree_sitter::Point;
pub use tree_sitter::Range;

pub use crate::lint::LintMatch;
pub use crate::lint::lint;
pub use crate::report::report_terminal;


#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::path::PathBuf;

    use anyhow::Context as _;
    use anyhow::Error;

    use wasm_bindgen::prelude::wasm_bindgen;

    use super::*;

    /// Lint source code `code` representing a file at `path` and
    /// produce a report, end-to-end.
    ///
    /// This function exists mostly because exposing something like our
    /// `LintMatch` type across WASM's ABI is a major PITA and our
    /// interactive service only cares about the end-to-end workflow
    /// anyway.
    #[wasm_bindgen]
    pub fn lint_html(code: Vec<u8>, path: String) -> Result<String, String> {
        fn lint_impl(code: Vec<u8>, path: PathBuf) -> Result<String, Error> {
            let mut report = Vec::new();
            let matches = lint(&code)?;
            for m in matches {
                let () = report_terminal(&m, &code, &path, &mut report)?;
            }
            let report =
                String::from_utf8(report).context("generated report contains invalid UTF-8")?;
            Ok(report)
        }

        lint_impl(code, PathBuf::from(path)).map_err(|err| format!("{err:?}"))
    }
}
