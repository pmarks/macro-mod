# Relax: macro call fails inside a module declared by a macro

`src/a.rs` declares `mod inner;` *through a macro* (`macros::make_mod!(inner)`).
The file it loads, `src/a/inner.rs`, then calls another macro by path
(`use crate::macros; macros::make_ty!(T);`).

```
$ cargo run -q
T

$ relax run macro_mod
src/a/inner.rs:2:1: error: cannot find macro `make_ty` in this scope
src/a.rs:3:16: E0432: unresolved import `inner::T`
src/main.rs:8:28: E0433: failed to resolve: use of undeclared type `a::T`
```

The first error is the real one; the other two follow from it. Replacing
`macros::make_mod!(inner);` in `a.rs` with a plain `mod inner;` makes Relax
accept it. So a file loaded through a macro-expanded `mod` item doesn't get
its `use` imports, or path-based macro resolution, in scope.

Tested with Relax `cf7dc3fb` on macOS (Apple silicon). Built with
`cargo build --release -p relax-runner --bin relax`.

Real-world case: `zarrs` 0.23.14, where `vlen_v2_macros::vlen_v2_codec!` is
used inside modules created by `vlen_v2_module!`. Every crate that depends on
zarrs fails to build under Relax.
