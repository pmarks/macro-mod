//! faer's `Mat::as_ref`.
fn main() {
    let m = faer::Mat::<f64>::from_fn(2, 2, |i, j| (i * 2 + j) as f64);
    let r = m.as_ref();
    println!("{}", r[(1, 0)]);
}
