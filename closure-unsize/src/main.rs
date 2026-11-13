//! Shape of egui_kittest 0.37 builder.rs:307: closures `|s| s` and
//! `|s| Box::new(s)` coerced to fn pointers returning `&mut dyn Named` and
//! `Box<dyn Named>`, in a struct boxed into an enum variant, with `S`
//! inferred from another argument.
trait Named {
    fn name(&self) -> String;
}

struct Thing(u32);

impl Named for Thing {
    fn name(&self) -> String {
        format!("thing {}", self.0)
    }
}

struct KindEframe<S> {
    get: fn(&mut S) -> &mut dyn Named,
    take: fn(S) -> Box<dyn Named>,
}

enum Kind<S> {
    Eframe(Box<KindEframe<S>>),
    #[allow(dead_code)]
    Other,
}

fn from_builder<S>(kind: Kind<S>, state: S) -> (Kind<S>, S) {
    (kind, state)
}

fn build<S: Named + 'static>(make: impl FnOnce() -> S) -> (Kind<S>, S) {
    let state = make();
    let kind = Kind::Eframe(Box::new(KindEframe {
        get: |s| s,
        take: |s| Box::new(s),
    }));
    from_builder(kind, state)
}

fn main() {
    let (kind, mut state) = build(|| Thing(7));
    let Kind::Eframe(k) = kind else { unreachable!() };
    println!("{} / {}", (k.get)(&mut state).name(), (k.take)(state).name());
}
