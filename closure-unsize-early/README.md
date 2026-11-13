# A closure `|s| s` coerced to `fn(&mut S) -> &mut dyn T`, with `S` fixed later: false E0277

```
$ cargo run -q
thing 7

$ relax run closure_unsize_early
src/main.rs:23:16: E0277: the size for values of type `dyn closure_unsize_early::Named` cannot be known at compilation time
```

`let kind = Kind { get: |s| s };` leaves `S` open, and the next statement fixes
it to `Thing`. rustc defers the coercion of the closure's return. Relax unifies
the closure's input with its return type, infers `S = dyn Named`, and then
reports that `S` is unsized.

This is the shorter form of [`../closure-unsize`](../closure-unsize), which
gets past type checking and fails in the JIT instead.
