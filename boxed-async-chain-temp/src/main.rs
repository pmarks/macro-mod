//! Async recursion broken by a box, through a chain of three async fns:
//! `top` awaits a `mid` future inline, `mid` awaits a `leaf` future inline, and
//! `leaf` boxes a `top` future. Legal Rust: the box breaks the cycle, and only
//! `leaf`'s allocation needs `top`'s size. Variant: `leaf` also makes a temporary
//! `top` future that is dead before its await.
use std::hint::black_box;
use std::mem::size_of_val;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Records the size of the last allocation, to see what `Box::pin` asked for.
struct Recording;
static LAST: AtomicUsize = AtomicUsize::new(0);
/// size_of_val of a `top` future, taken in main
static WANT: AtomicUsize = AtomicUsize::new(0);
unsafe impl std::alloc::GlobalAlloc for Recording {
    unsafe fn alloc(&self, l: std::alloc::Layout) -> *mut u8 {
        LAST.store(l.size(), Ordering::Relaxed);
        unsafe { std::alloc::System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: std::alloc::Layout) {
        unsafe { std::alloc::System.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Recording = Recording;

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
async fn top(n: u32) -> u64 {
    let mut buf = [0u64; 512];
    for (i, x) in buf.iter_mut().enumerate() {
        *x = i as u64 ^ n as u64;
    }
    YieldOnce(false).await;
    let deeper = mid(n).await;
    black_box(&mut buf);
    buf.iter().fold(deeper, |a, &b| a.wrapping_mul(31).wrapping_add(b))
}

async fn mid(n: u32) -> u64 {
    leaf(n).await.wrapping_add(1)
}

async fn leaf(n: u32) -> u64 {
    if n == 0 {
        return 7;
    }
    let boxed = Box::pin(top(n - 1));
    let allocated = LAST.load(Ordering::Relaxed);
    // the box must hold a whole `top` future
    // a temporary top future, dead before the await (rustc does not keep it in leaf's frame)
    let want = size_of_val(&top(0));
    assert_eq!(allocated, want, "Box::pin(top(..)) allocated {allocated} B for a {want} B future");
    boxed.await
}

fn main() {
    let t = top(3);
    WANT.store(size_of_val(&t), Ordering::Relaxed);
    println!("size_of_val: top {}, mid {}, leaf {}", size_of_val(&t), size_of_val(&mid(3)), size_of_val(&leaf(3)));
    let v = pollster::block_on(t);
    println!("top(3) = {v:016x}");
}
