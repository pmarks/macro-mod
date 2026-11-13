# Relax repros

Each directory is a standalone cargo package (it has its own `[workspace]`).
Run one from inside its directory with `cargo run -q` and `relax run <package>`,
and compare the output. Each directory's README shows both outputs and the
real-world crate that hit the bug.

Checked with Relax `442f13bc` on macOS (Apple silicon). Relax was built with
`cargo build --release -p relax-runner --bin relax`.

| Directory | Package | Bug | Status on `442f13bc` |
|---|---|---|---|
| [`macro-sep`](macro-sep) | `macro_sep` | A `$t:ty` fragment rejects a bare trait object with `+` bounds | open |
| [`macro-host-cascade`](macro-host-cascade) | `macro_host_cascade` | One failing macro crate breaks every other crate's proc macros | open |
| [`closure-index`](closure-index) | `closure_index` | An unannotated closure parameter used as an index is typed `usize` | open |
| [`closure-unsize`](closure-unsize) | `closure_unsize` | A closure `\|s\| s` coerced to `fn(&mut S) -> &mut dyn T`: internal compiler error | open |
| [`closure-unsize-early`](closure-unsize-early) | `closure_unsize_early` | The same coercion with `S` fixed by a later statement: false E0277 | open |
| [`macro-mod`](macro-mod) | `macro_mod` | A macro call fails inside a module declared by a macro | fixed |
