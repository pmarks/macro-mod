# `format!` in a `proptest!` test fails when a local module named `alloc` is in scope

```
$ cargo test -q
running 1 test
.
test result: ok. 1 passed; 0 failed; ...

$ relax test format_alloc_shadow
src/lib.rs:19:5: internal: macro `format!` is not supported yet
error: could not compile `format_alloc_shadow` (test) due to 1 previous error
```

The crate has a local module `gpu::alloc` and imports it at the crate root
(`use gpu::alloc;`). A test module glob-imports the root (`use super::*;`) and
calls `format!` inside proptest's `proptest!` macro.

`format!` should resolve to std's macro whatever local names are in scope.
Under Relax, the local `alloc` seems to shadow the `alloc` crate that
`format!` comes from. Each of these makes the test pass under Relax:

- deleting `use gpu::alloc;`
- calling `format!` from an ordinary function in a child module that has the
  same imports, with no `proptest!` (`relax run` prints the right output)
