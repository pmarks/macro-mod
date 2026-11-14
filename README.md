# Relax repros

Each directory is a standalone cargo package (it has its own `[workspace]`).
Run one from inside its directory with `cargo run -q` and `relax run <package>`,
and compare the output. Each directory's README shows both outputs and the
real-world crate that hit the bug.

Checked on macOS (Apple silicon), at the Relax commit in the Status column.
Relax was built with `cargo build --release -p relax-runner --bin relax`.

| Directory | Package | Bug | Status |
|---|---|---|---|
| [`macro-sep`](macro-sep) | `macro_sep` | A `$t:ty` fragment rejects a bare trait object with `+` bounds | open on `442f13bc` |
| [`macro-host-cascade`](macro-host-cascade) | `macro_host_cascade` | One failing macro crate breaks every other crate's proc macros | open on `442f13bc` |
| [`closure-index`](closure-index) | `closure_index` | An unannotated closure parameter used as an index is typed `usize` | open on `442f13bc` |
| [`closure-unsize`](closure-unsize) | `closure_unsize` | A closure `\|s\| s` coerced to `fn(&mut S) -> &mut dyn T`: internal compiler error | open on `442f13bc` |
| [`closure-unsize-early`](closure-unsize-early) | `closure_unsize_early` | The same coercion with `S` fixed by a later statement: false E0277 | open on `442f13bc` |
| [`macro-mod`](macro-mod) | `macro_mod` | A macro call fails inside a module declared by a macro | fixed on `442f13bc` |
| [`boxed-async-recursion`](boxed-async-recursion) | `boxed_async_recursion` | Async recursion through a `Box`: the inner future is sized from the outer's layout so far (heap corruption) | open on `96c0c865` |
| [`boxed-async-chain`](boxed-async-chain) | `boxed_async_chain` | The same, three fns deep, where only code needs the size: passes today; a guard for a fix | passes on `96c0c865` |
| [`boxed-async-chain-temp`](boxed-async-chain-temp) | `boxed_async_chain_temp` | A temporary dead before an await stays in the frame: a size cycle rustc does not have | open on `96c0c865` |
