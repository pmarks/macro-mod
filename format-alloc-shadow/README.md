# A block-local `extern crate alloc;` doesn't shadow an imported `alloc` for macro paths

```
$ cargo run -q
[1, 2]

$ relax run format_alloc_shadow
src/main.rs:15:13: macro `format!` is not supported yet
error: could not compile `format_alloc_shadow` due to 1 previous error
```

The crate imports a module of its own named `alloc` (`use gpu::alloc;`). A
function body declares `extern crate alloc;` and calls `alloc::format!`. The
block-local item should shadow the import, as it does under rustc.

Under Relax, the type path `alloc::vec::Vec` resolves through the local
`extern crate`, but the macro path `alloc::format!` doesn't (`alloc::vec!`
fails the same way). Without `use gpu::alloc;` it works.

proptest hits this: `prop_assert!` expands to
`{ extern crate alloc; let message = alloc::format!(..); .. }`. So any
`prop_assert!` in a module that glob-imports a parent with an `alloc` module
in scope fails to compile.
