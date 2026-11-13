//! A closure `|s| s` coerced to `fn(&mut S) -> &mut dyn Named` while `S` is
//! still unknown: it is fixed only by a later statement. rustc defers the
//! unsizing coercion of the closure's return; Relax lowers the return as a thin
//! pointer and then fails at its first call. Shape from egui_kittest 0.37
//! builder.rs:307 (`get_app: |state| state`).
trait Named {
    fn name(&self) -> String;
}

struct Thing(u32);

impl Named for Thing {
    fn name(&self) -> String {
        format!("thing {}", self.0)
    }
}

struct Kind<S> {
    get: fn(&mut S) -> &mut dyn Named,
}

fn main() {
    let kind = Kind { get: |s| s }; // S unknown here
    let mut t = Thing(7); //            and fixed here
    println!("{}", (kind.get)(&mut t).name());
}
