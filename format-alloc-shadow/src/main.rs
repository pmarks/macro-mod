//! A block-local `extern crate alloc;` must shadow a module named `alloc`
//! that the crate imports. proptest's `prop_assert!` expands to exactly this:
//! `{ extern crate alloc; let message = alloc::format!(..); .. }`.
mod gpu {
    pub mod alloc {}
}
#[allow(unused_imports)]
use gpu::alloc;

fn main() {
    extern crate alloc;
    // a type path through the local `alloc` resolves under Relax ...
    let v: alloc::vec::Vec<u8> = alloc::vec::Vec::from([1, 2]);
    // ... a macro path does not
    let m = alloc::format!("{v:?}");
    println!("{m}");
}
