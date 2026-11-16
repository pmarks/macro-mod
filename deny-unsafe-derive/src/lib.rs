//! `deny(unsafe_code)` must not fire on the `unsafe impl` that an external derive writes.
#![deny(unsafe_code)]

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct P {
    pub x: f32,
}

#[test]
fn bytes() {
    assert_eq!(bytemuck::bytes_of(&P { x: 1.0 }).len(), 4);
}
