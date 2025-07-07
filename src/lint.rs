use anyhow::Context as _;
use anyhow::Result;

use tracing::warn;

use tree_sitter::Parser;
use tree_sitter::Query;
use tree_sitter::QueryCursor;
use tree_sitter::StreamingIterator as _;
use tree_sitter::Tree;
use tree_sitter_bpf_c::LANGUAGE;

use crate::Point;
use crate::Range;


mod lints {
    include!(concat!(env!("OUT_DIR"), "/lints.rs"));
}

impl From<tree_sitter::Point> for Point {
    fn from(other: tree_sitter::Point) -> Self {
        let tree_sitter::Point { row, column } = other;
        Self { row, col: column }
    }
}

impl From<tree_sitter::Range> for Range {
    fn from(other: tree_sitter::Range) -> Self {
        let tree_sitter::Range {
            start_byte,
            end_byte,
            start_point,
            end_point,
        } = other;
        Self {
            bytes: start_byte..end_byte,
            start_point: Point::from(start_point),
            end_point: Point::from(end_point),
        }
    }
}


/// Meta data about a lint.
#[derive(Clone, Debug)]
pub struct LintMeta {
    /// The lint's name.
    pub name: String,
    /// The struct is non-exhaustive and open to extension.
    #[doc(hidden)]
    pub _non_exhaustive: (),
}


/// Retrieve the list of lints shipped with the library.
pub fn builtin_lints() -> impl ExactSizeIterator<Item = LintMeta> + DoubleEndedIterator {
    lints::LINTS.iter().map(|(name, _code)| LintMeta {
        name: name.to_string(),
        _non_exhaustive: (),
    })
}


/// Details about a lint match.
#[derive(Clone, Debug)]
pub struct LintMatch {
    /// The name of the lint that matched.
    pub lint_name: String,
    /// The lint's message.
    pub message: String,
    /// The code range that triggered the lint.
    pub range: Range,
}


fn lint_impl(tree: &Tree, code: &[u8], lint_src: &str, lint_name: &str) -> Result<Vec<LintMatch>> {
    let query =
        Query::new(&LANGUAGE.into(), lint_src).with_context(|| "failed to compile lint query")?;
    let mut query_cursor = QueryCursor::new();
    let mut results = Vec::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), code);
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let settings = query.property_settings(m.pattern_index);
            let setting = settings
                .iter()
                .find(|prop| &*prop.key == "message")
                .with_context(|| format!("{lint_name}: failed to find `message` property"))?;

            let r#match = LintMatch {
                lint_name: lint_name.to_string(),
                message: setting
                    .value
                    .as_ref()
                    .with_context(|| format!("{lint_name}: `message` property has no value set"))?
                    .to_string(),
                range: Range::from(capture.node.range()),
            };
            let () = results.push(r#match);
        }
    }

    if query_cursor.did_exceed_match_limit() {
        warn!("query exceeded maximum number of in-progress captures");
    }
    Ok(results)
}

fn lint_multi(code: &[u8], lints: &[(&str, &str)]) -> Result<Vec<LintMatch>> {
    let mut parser = Parser::new();
    let () = parser
        .set_language(&LANGUAGE.into())
        .context("failed to load C parser")?;
    let tree = parser
        .parse(code, None)
        .context("failed to provided source code")?;
    let mut results = Vec::new();
    for (lint_name, lint_src) in lints {
        let matches = lint_impl(&tree, code, lint_src, lint_name)?;
        let () = results.extend(matches);
    }
    Ok(results)
}

/// Lint code using the default set of lints.
///
/// - `code` is the source code in question, for example as read from a
///   file
pub fn lint(code: &[u8]) -> Result<Vec<LintMatch>> {
    lint_multi(code, &lints::LINTS)
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::Point;


    /// Check that the `builtin_lints()` function reports sensible
    /// results.
    #[test]
    fn built_in_lint_listing() {
        assert!(builtin_lints().any(|lint| lint.name == "probe-read"));
    }

    /// Check that a missing `message` property is being flagged
    /// appropriately.
    #[test]
    fn missing_message_property() {
        let code = r#"
test_fn(/* doesn't matter */);
"#;
        let lint = r#"
(call_expression
    function: (identifier) @function (#eq? @function "test_fn")
)
        "#;
        let err = lint_multi(code.as_bytes(), &[("test_fn", lint)]).unwrap_err();
        assert_eq!(
            err.to_string(),
            "test_fn: failed to find `message` property",
            "{err}"
        );
    }

    /// Check that some basic linting works as expected.
    #[test]
    fn basic_linting() {
        let code = r#"
SEC("tp_btf/sched_switch")
int handle__sched_switch(u64 *ctx)
{
    struct task_struct *prev = (struct task_struct *)ctx[1];
    struct event event = {0};
    bpf_probe_read(event.comm, TASK_COMM_LEN, prev->comm);
    return 0;
}
"#;

        let matches = lint(code.as_bytes()).unwrap();
        assert_eq!(matches.len(), 1);

        let LintMatch {
            lint_name,
            message,
            range,
        } = &matches[0];
        assert_eq!(lint_name, "probe-read");
        assert!(
            message.starts_with("bpf_probe_read() is deprecated"),
            "{message}"
        );
        assert_eq!(&code[range.bytes.clone()], "bpf_probe_read");
        assert_eq!(range.start_point, Point { row: 6, col: 4 });
        assert_eq!(range.end_point, Point { row: 6, col: 18 });
    }
}
