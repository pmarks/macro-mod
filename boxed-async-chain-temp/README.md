# A temporary that is dead before an await stays in the frame

```
$ cargo run -q
size_of_val: top 4152, mid 40, leaf 32
top(3) = 1d44d5476316400b

$ relax run boxed_async_chain_temp
size_of_val: top 4344, mid 216, leaf 208
top(3) = 1d44d5476316400b
```

[`boxed-async-chain`](../boxed-async-chain), plus `size_of_val(&top(0))` in
`leaf`: a temporary `top` future that is dropped before `leaf`'s await. rustc
leaves it out of `leaf`'s frame (32 bytes). Relax keeps it across the await,
so for Relax `leaf` holds a `top` inline and `top` (through `mid`) holds a
`leaf` inline: a size cycle that rustc does not have.

Today this runs, but only because `leaf` is lowered while `top` is in
`coro_busy` and gets `top`'s layout so far (the
[`boxed-async-recursion`](../boxed-async-recursion) bug), which hides the
cycle. With a prototype fix that gives `leaf` `top`'s real size, `mid` grows
to 4552 and `leaf` to 4544, and the program segfaults: the cycle should be an
error (or, better, the temporary should not be saved).

More generally, Relax frames hold several copies of one future (a future, its
moved copy, its pinned copy), so they are far bigger than rustc's, and dead
values are kept across awaits.
