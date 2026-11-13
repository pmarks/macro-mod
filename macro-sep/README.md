# A `$t:ty` fragment rejects a bare trait object with `+` bounds

```
$ cargo run -q
std::fmt::Display + Send
FnOnce(u8) -> u8 + 'static

$ relax run macro_sep
relax: load error macro_sep: src/main.rs:9:20: no rule of `t!` matches these arguments
```

The same types written with `dyn` match, and so does `FnOnce(u8) -> u8`
without the `+ …`. Only a bare (edition-2021) trait object with `+` bounds
fails to match `$t:ty`.

Real-world case: `indexed_db_futures_macros_internal` 1.0.0,
`src/generic_bounds.rs:52`. Its `make_opts!` matches
`::core::ops::FnOnce(..) -> crate::Result<()> + 'static` as a `ty`. Because of
[`../macro-host-cascade`](../macro-host-cascade), that also stops wgpu, eframe
and anything else whose proc macros run in the same host.
