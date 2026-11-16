# `deny(unsafe_code)` fires on the `unsafe impl` an external derive writes

```
$ cargo test -q
running 1 test
.
test result: ok. 1 passed; 0 failed; ...

$ relax test deny_unsafe_derive
src/lib.rs:4:1: error: implementation of an `unsafe` trait [unsafe_code]
error: could not compile `deny_unsafe_derive` (test) due to 1 previous error
```

`relax check deny_unsafe_derive` reports the same error.

The crate has `#![deny(unsafe_code)]` and derives `bytemuck::Pod` and
`bytemuck::Zeroable`. Those derives expand to `unsafe impl`. rustc doesn't
apply `unsafe_code` to tokens that come from an external proc macro: the
lint skips spans from external macro expansions. Relax reports the error at
the derive.

This is common in practice: crates that deny `unsafe_code` and derive
bytemuck's traits for GPU vertex structs.
