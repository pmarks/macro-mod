# faer's `Mat::as_ref`: internal compiler error "non-concrete region in LUB"

```
$ cargo run -q
2

$ RELAX_ALLOW_C=1 relax run faer_as_ref
relax: main panic: compile error in faer::mat::matref::generic::Mat::as_ref:
  internal compiler error in faer::mat::matref::generic::Mat::as_ref:
  non-concrete region in LUB (at libs/relax/types/src/infer/free_regions.rs:73)
```

`faer::Mat::<f64>::from_fn(..).as_ref()` with faer `=0.24.4`, the current
release. Relax hits the error the first time it compiles `Mat::as_ref`.

`RELAX_ALLOW_C=1` is there only because faer's dependencies build C (`psm`).

The same run also prints two errors from pulp 0.22.3 that don't stop it:
`E0425: cannot find function vmaxnmvq_f64` and `vminnmvq_f64`, in
`pulp/src/aarch64.rs`. Those are two NEON intrinsics missing from
`core::arch::aarch64`.
