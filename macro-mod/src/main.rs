mod macros {
    macro_rules! make_mod { ($m:ident) => { mod $m; }; }
    macro_rules! make_ty { ($t:ident) => { pub struct $t; impl $t { pub fn name() -> &'static str { stringify!($t) } } }; }
    pub(crate) use make_mod;
    pub(crate) use make_ty;
}
mod a;
fn main() { println!("{}", a::T::name()); }
