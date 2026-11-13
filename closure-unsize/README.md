# A closure `|s| s` coerced to `fn(&mut S) -> &mut dyn T`: internal compiler error

```
$ cargo run -q
thing 7 / thing 7

$ relax run closure_unsize
relax: main panic: compile error in {closure#26}@2: src/main.rs:35:18: internal: value of
  `&mut dyn closure_unsize::Named` has 1 parts, expected 2 -- Relax bug, report it
```

The struct holds `get: fn(&mut S) -> &mut dyn Named`. It's built with
`get: |s| s` inside `Box::new(..)` inside an enum variant, and `S` is fixed
only later, by another argument. The closure's return needs an unsizing
coercion (`&mut S` to the fat `&mut dyn Named`). Relax lowers the return as a
thin pointer, and the JIT fails at the first call.

Real-world case: egui_kittest 0.37, `src/builder.rs:307`
(`get_app: |state| state`, `take_app: |state| Box::new(state)`). It is reached
by every `Harness::builder().build_eframe(..)`.

See [`../closure-unsize-early`](../closure-unsize-early) for the same coercion
failing at type check instead.
