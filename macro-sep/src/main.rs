//! A `$t:ty` fragment holding a bare trait object with `+` bounds (valid in
//! edition 2021 as a macro fragment; `indexed_db_futures_macros_internal`
//! 1.0.0 passes `::core::ops::FnOnce(..) -> crate::Result<()> + 'static`).
macro_rules! t {
    ($t:ty) => { stringify!($t) };
}

fn main() {
    println!("{}", t!(std::fmt::Display + Send));
    println!("{}", t!(FnOnce(u8) -> u8 + 'static));
}
