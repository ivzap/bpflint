bpflinter
=========

- [Changelog](CHANGELOG.md)

**bpflinter** is a command line based linter for BPF C code. It is
powered by the [**bpflint**][bpflint] library.

Usage
-----

The program requires Rust toolchain to build. Use `cargo build` to build
it or download a pre-built, statically linked binary from the latest
[`cli-*` release][cli-releases].

To lint a `*.bpf.c` file, just provide it's path as argument. E.g.,
```
$ bpflinter ../examples/task_longrun.bpf.c
# Or using cargo:
$ cargo run -- ../examples/task_longrun.bpf.c
warning: [probe-read] bpf_probe_read() is deprecated and replaced by bpf_probe_user() and bpf_probe_kernel(); refer to bpf-helpers(7)
  --> ../examples/task_longrun.bpf.c:43:24
   |
43 |                         bpf_probe_read(event.comm, TASK_COMM_LEN, prev->comm);
   |                         ^^^^^^^^^^^^^^
   |
warning: [probe-read] bpf_probe_read() is deprecated and replaced by bpf_probe_user() and bpf_probe_kernel(); refer to bpf-helpers(7)
  --> ../examples/task_longrun.bpf.c:44:24
   |
44 |                         bpf_probe_read(event.bt, sizeof(t->bt), t->bt);
   |                         ^^^^^^^^^^^^^^
   |
```

[cli-releases]: https://github.com/d-e-s-o/bpflint/releases
[bpflint]: https://github.com/d-e-s-o/bpflint
