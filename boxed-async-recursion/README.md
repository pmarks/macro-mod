# Async recursion through a `Box`: the inner future is sized too small

```
$ cargo run -q
size_of_val(outer) = 4120, size_of_val(inner) = 4128
outer(3) = 553b8499c39e4003

$ relax run boxed_async_recursion
size_of_val(outer) = 4136, size_of_val(inner) = 16
relax:   segfault at pc in JIT boxed_async_recursion::<YieldOnce as std::future::Future>::poll +0x94
```

`outer` does `Box::pin(inner(n - 1)).await`, and `inner` awaits `outer(n)`
inline. The box breaks the cycle, so this is legal Rust, and `inner`'s future
must be at least as big as `outer`'s.

Relax lowers `outer`'s poll function to find its frame (saved locals).
`Box::pin` needs `inner`'s size, so `inner`'s poll function is lowered next,
while `outer` is still in `coro_busy`. `inner` holds an `outer` future, and
`Unit::layout_of` answers with `outer`'s layout so far (its state without
saved locals). `inner`'s frame is recorded from that and never corrected: the
`coro_selfref` re-lowering only re-lowers `outer`. `Box::pin` then allocates
16 bytes, and `outer`'s 4 KB state is written into them.

In a larger program, which coroutine is lowered first depends on what runs
first. So the same future type gets different sizes in different runs, and a
warm cache links code built under both: heap and stack corruption in code
that never recurses at the point of the crash.
