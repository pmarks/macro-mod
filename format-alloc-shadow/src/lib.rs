//! `format!` inside a `proptest!` test, in a module that glob-imports a
//! local module named `alloc`.
mod gpu {
    pub mod alloc {
        pub fn bytes(n: usize) -> Vec<u8> {
            vec![0; n]
        }
    }
}

#[allow(unused_imports)]
use gpu::alloc;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn formats(n in 0usize..100) {
            let s = format!("{n}");
            prop_assert!(!s.is_empty());
        }
    }
}
