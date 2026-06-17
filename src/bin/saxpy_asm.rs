use lak::l1::axpy;
use lak::types::{VecMut, VecRef};

#[inline(never)]
fn saxpy(alpha: f32, x: &[f32], y: &mut [f32]) {
    let x = VecRef::new(x);
    let y = VecMut::new(y);

    axpy(alpha, x, y);
}

fn main() {
    let alpha = 2.0f32;
    let x = vec![1.0f32; 1024];
    let mut y = vec![3.0f32; 1024];

    saxpy(alpha, &x, &mut y);

    std::hint::black_box(y);
}
