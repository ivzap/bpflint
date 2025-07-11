//! Tests for the `unstable-attach-point` lint.

use std::path::Path;

use bpflint::lint;
use bpflint::report_terminal;

use pretty_assertions::assert_eq;


#[test]
fn basic() {
    let code = r#"
SEC("fentry/do_nanosleep")
int nanosleep(void *ctx) {
}
"#;

    let mut report = Vec::new();
    let () = lint(code.as_bytes())
        .unwrap()
        .into_iter()
        .try_for_each(|m| report_terminal(&m, code.as_bytes(), Path::new("<stdin>"), &mut report))
        .unwrap();
    let report = String::from_utf8(report).unwrap();

    let expected = r#"warning: [unstable-attach-point] kprobe/kretprobe/fentry/fexit are conceptually unstable and prone to changes between kernel versions; consider more stable attach points such as tracepoints or LSM hooks, if available
  --> <stdin>:1:4
  | 
1 | SEC("fentry/do_nanosleep")
  |     ^^^^^^^^^^^^^^^^^^^^^
  | 
"#;
    assert_eq!(report, expected);
}


#[test]
fn basic2() {
    let code = r#"
SEC("kprobe/cap_capable")

int BPF_KPROBE(kprobe__foobar, const struct cred *cred,
               struct user_namespace *targ_ns, int cap, int cap_opt) {
"#;

    let mut report = Vec::new();
    let () = lint(code.as_bytes())
        .unwrap()
        .into_iter()
        .try_for_each(|m| report_terminal(&m, code.as_bytes(), Path::new("<stdin>"), &mut report))
        .unwrap();
    let report = String::from_utf8(report).unwrap();

    let expected = r#"warning: [unstable-attach-point] kprobe/kretprobe/fentry/fexit are conceptually unstable and prone to changes between kernel versions; consider more stable attach points such as tracepoints or LSM hooks, if available
  --> <stdin>:1:4
  | 
1 | SEC("kprobe/cap_capable")
  |     ^^^^^^^^^^^^^^^^^^^^
  | 
"#;
    assert_eq!(report, expected);
}
