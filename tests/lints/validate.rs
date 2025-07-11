use std::env;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;

use bpflint::LintMeta;
use bpflint::builtin_lints;


/// Check that `builtin_lints()` reports all lints we expect it to.
// Note that there is some overlap with logic from `build.rs` here, but
// we don't really have a good way to share it.
#[test]
fn validate_builtin_lints() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lint_dir = Path::new(&manifest_dir).join("lints");

    let mut lints = Vec::new();
    for result in read_dir(&lint_dir).unwrap() {
        let entry = result.unwrap();
        if let Some(lint_name) = entry.file_name().to_str().unwrap().strip_suffix(".scm") {
            let () = lints.push(lint_name.to_string());
        }
    }
    let () = lints.sort();

    let mut expected = builtin_lints()
        .map(|LintMeta { name, .. }| name)
        .collect::<Vec<_>>();
    let () = expected.sort();

    assert_eq!(lints, expected)
}

fn is_lower_ascii_slug(s: &str) -> bool {
    // Must be non‑empty and neither start nor end with a hyphen
    if s.is_empty() || s.starts_with('-') || s.ends_with('-') {
        return false;
    }

    // Validate every character
    s.chars().all(|c| c.is_ascii_lowercase() || c == '-')
}

mod slug {
    use super::*;

    #[test]
    fn valid() {
        assert!(is_lower_ascii_slug("hello"));
        assert!(is_lower_ascii_slug("hello-world"));
        assert!(is_lower_ascii_slug("a-b-c"));
    }

    #[test]
    fn invalid_chars() {
        assert!(!is_lower_ascii_slug("Hello"));
        assert!(!is_lower_ascii_slug("abc_def"));
        assert!(!is_lower_ascii_slug("naïve"));
    }

    #[test]
    fn hyphen_rules() {
        assert!(!is_lower_ascii_slug("-hello"));
        assert!(!is_lower_ascii_slug("hello-"));
        assert!(!is_lower_ascii_slug("-"));
    }

    #[test]
    fn empty() {
        assert!(!is_lower_ascii_slug(""));
    }
}


/// Check that our lint names comply with repository policy.
#[test]
fn builtin_lint_names() {
    for LintMeta { name, .. } in builtin_lints() {
        assert!(
            is_lower_ascii_slug(&name),
            "lint `{name}` contains invalid characters (allowed: [a-z] and `-`)"
        );
    }
}
