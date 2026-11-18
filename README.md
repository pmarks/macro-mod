# Relax repros

Each directory is a standalone cargo package (it has its own `[workspace]`).
Run one from inside its directory with `cargo run -q` and `relax run <package>`
(or `cargo test -q` and `relax test <package>`), and compare the output. Each
directory's README shows both outputs and the real-world crate that hit the bug.

Checked on macOS (Apple silicon), at the Relax commit in the Status column.
Relax was built with `cargo build --release -p relax-runner --bin relax`.

| Directory | Package | Bug | Status |
|---|---|---|---|
| [`macro-sep`](macro-sep) | `macro_sep` | A `$t:ty` fragment rejects a bare trait object with `+` bounds | fixed on `193e0b6c` |
| [`macro-host-cascade`](macro-host-cascade) | `macro_host_cascade` | One failing macro crate breaks every other crate's proc macros | fixed on `193e0b6c` |
| [`closure-index`](closure-index) | `closure_index` | An unannotated closure parameter used as an index is typed `usize` | fixed on `193e0b6c` |
| [`closure-unsize`](closure-unsize) | `closure_unsize` | A closure `\|s\| s` coerced to `fn(&mut S) -> &mut dyn T`: internal compiler error | fixed on `193e0b6c` |
| [`closure-unsize-early`](closure-unsize-early) | `closure_unsize_early` | The same coercion with `S` fixed by a later statement: false E0277 | fixed on `193e0b6c` |
| [`macro-mod`](macro-mod) | `macro_mod` | A macro call fails inside a module declared by a macro | fixed on `193e0b6c` |
| [`boxed-async-recursion`](boxed-async-recursion) | `boxed_async_recursion` | Async recursion through a `Box`: the inner future is sized from the outer's layout so far (heap corruption) | fixed on `193e0b6c` |
| [`boxed-async-chain`](boxed-async-chain) | `boxed_async_chain` | The same, three fns deep, where only code needs the size: passes today; a guard for a fix | passes on `193e0b6c` |
| [`boxed-async-chain-temp`](boxed-async-chain-temp) | `boxed_async_chain_temp` | A temporary dead before an await stays in the frame: a size cycle rustc does not have | fixed on `193e0b6c` |
| [`deny-unsafe-derive`](deny-unsafe-derive) | `deny_unsafe_derive` | `deny(unsafe_code)` fires on the `unsafe impl` that bytemuck's derive writes (`relax test`) | open on `193e0b6c` |
| [`format-alloc-shadow`](format-alloc-shadow) | `format_alloc_shadow` | A block-local `extern crate alloc;` doesn't shadow an imported `alloc` module for macro paths: "macro `format!` is not supported yet" (proptest's `prop_assert!`) | open on `c9f9d129e` |
| [`faer-as-ref`](faer-as-ref) | `faer_as_ref` | faer 0.24.4 `Mat::as_ref`: internal compiler error "non-concrete region in LUB" | open on `193e0b6c` |
