# Async recursion through a `Box`, three fns deep (passes: a regression guard)

```
$ cargo run -q
size_of_val: top 4152, mid 40, leaf 32
top(3) = 1d44d5476316400b

$ relax run boxed_async_chain
size_of_val: top 4336, mid 208, leaf 200
top(3) = 1d44d5476316400b
```

`top` awaits `mid` inline, `mid` awaits `leaf` inline, and `leaf` boxes a
`top` future. Here only `leaf`'s *code* needs `top`'s size (for the
allocation); no frame holds the coroutine being lowered. Relax gets this right
today: a recording global allocator checks that `Box::pin` asks for a whole
`top` future.

It is here for a fix of [`boxed-async-recursion`](../boxed-async-recursion).
A fix that throws away the frames of every coroutine lowered against a layout
so far (here `mid` and `leaf`) gives `top` a frame built from partial sizes.
Re-lowering `top` then moves its frame, and Relax reports a false E0733.
