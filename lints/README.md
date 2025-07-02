## How to Write a Lint
**bpflint** uses [`tree-sitter`][tree-sitter-docs] to drive code
analysis. A lint is basically a `tree-sitter` Query. **bpflint** uses
such queries to find matches on source code and then reports those
alongside location information and some other meta data.

To better understand queries and the language that they are written in,
please refer to the official chapter covering them:
https://tree-sitter.github.io/tree-sitter/using-parsers/queries/index.html

A lint is a regular query, with the added requirement that it contains a
`message` [directive][tree-sitter-directives] that explains to the user
why the pattern being matched on is problematic. For an example please
refer to the [`probe-read` lint][probe-read-message].

A good introduction that to how a query interfaces with the underlying
language grammar can be found in the ["Code Navigation Systems"
chapter][tree-sitter-code-nav].

In our case, the underlying grammar is that of the C language, available
[here][tree-sitter-bpf-c-grammar].

[tree-sitter-docs]: https://tree-sitter.github.io/tree-sitter/
[tree-sitter-directives]: https://tree-sitter.github.io/tree-sitter/using-parsers/queries/3-predicates-and-directives.html#directives
[tree-sitter-code-nav]: https://tree-sitter.github.io/tree-sitter/4-code-navigation.html
[tree-sitter-bpf-c-grammar]: https://github.com/d-e-s-o/tree-sitter-bpf-c/blob/main/grammar.js
[probe-read-message]: https://github.com/d-e-s-o/bpflint/blob/b8716d24fb133de0371152705bd33a1c56f51bfe/lints/probe-read.scm#L8
