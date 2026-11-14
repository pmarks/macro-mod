//! Async recursion broken by a box, through a second async fn:
//! `outer` boxes an `inner` future, and `inner` awaits an `outer` future inline.
//! Legal Rust (the box breaks the cycle), so `size_of_val(&inner(..))` must be
//! at least `size_of_val(&outer(..))`.
use std::hint::black_box;
use std::mem::size_of_val;

struct YieldOnce(bool);
impl std::future::Future for YieldOnce {
    type Output = ();
    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        if self.0 {
            return std::task::Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

/// A big frame: `buf` lives across both awaits.
async fn outer(n: u32) -> u64 {
    let mut buf = [0u64; 512];
    for (i, x) in buf.iter_mut().enumerate() {
        *x = i as u64 ^ n as u64;
    }
    YieldOnce(false).await;
    let deeper = if n > 0 { Box::pin(inner(n - 1)).await } else { 0 };
    black_box(&mut buf);
    buf.iter().fold(deeper, |a, &b| a.wrapping_mul(31).wrapping_add(b))
}

/// Holds an `outer` future inline across its await.
async fn inner(n: u32) -> u64 {
    outer(n).await.wrapping_add(1)
}

fn main() {
    let a = outer(3);
    let b = inner(3);
    let (sa, sb) = (size_of_val(&a), size_of_val(&b));
    println!("size_of_val(outer) = {sa}, size_of_val(inner) = {sb}");
    let v = pollster::block_on(a);
    drop(b);
    println!("outer(3) = {v:016x}");
    assert!(sb >= sa, "inner's future ({sb} B) holds an outer future ({sa} B) inline");
}
