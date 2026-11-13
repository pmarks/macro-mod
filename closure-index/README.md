# An unannotated closure parameter used as an index is typed `usize`

```
$ cargo run -q
cde

$ relax run closure_index
src/main.rs:7:45: E0308: mismatched types: expected `&[u8]`, found `&u8`
src/main.rs:8:26: E0308: mismatched types: expected `usize`, found `Range<usize>`
```

`|range| … &stamp[range] …` is called with a `Range<usize>`. rustc leaves the
parameter's type open until the call fixes it. Relax settles on `usize` (the
`Index` impl that returns one element) while checking the body, and then
rejects the call.
