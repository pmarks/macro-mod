# One failing macro crate breaks every other crate's proc macros

The program uses only `document_features!`. Its other dependency,
`indexed_db_futures_macros_internal` 1.0.0, is never invoked. That crate fails
to build under Relax (see [`../macro-sep`](../macro-sep)), and the failure
surfaces as an error inside the unrelated `document_features!` call.

```
$ cargo run -q
ok

$ relax run macro_host_cascade
relax: load error macro_host_cascade: src/main.rs:1:10: expected a literal, found `compile_error ! ("`document_features!`:
  proc-macro host: macro crate indexed_db_futures_macros_internal: indexed_db_futures_macros_internal-1.0.0/src/generic_bounds.rs:52:1:
  no rule of `make_opts!` matches these arguments")`
relax: 1 crate did not load (the load errors above), so the run stops before `main`
```

A macro crate that fails to load should break only that crate's own macros.
In a real app (QPlot), this one unused crate stopped wgpu_core, wgpu,
egui_wgpu, eframe and poll_promise from loading, and broke every zerocopy and
thiserror derive, so the run stopped before `main`.

Only crates.io dependencies are used. A path dependency is checked as one of
the app's own crates, which reports the matcher error directly instead.
