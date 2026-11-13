//! An unannotated closure parameter used as an index: its type comes from the
//! call below (a Range), not from the indexing inside the body.
const FIELD: std::ops::Range<usize> = 2..5;

fn main() {
    let stamp: &[u8; 8] = b"abcdefgh";
    let field = |range| std::str::from_utf8(&stamp[range]).unwrap();
    println!("{}", field(FIELD));
}
