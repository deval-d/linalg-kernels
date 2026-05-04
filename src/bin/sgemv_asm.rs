use lak::l2::gemv::gemv_n;
use lak::types::{MatRef, VecMut, VecRef};

#[inline(never)]
fn sgemv(a: &[f32], x: &[f32], y: &mut [f32], m: usize, n: usize) {
    let alpha = 3.1415926; 
    let beta  = alpha; 
    let a = MatRef::new(a, (m, n));
    let x = VecRef::new(x);
    let y = VecMut::new(y);

    gemv_n(alpha, beta, a, x, y);
}

fn main() {
    let m = 128;
    let n = 128;

    let a = vec![1.0; m * n];
    let x = vec![1.0; n];
    let mut y = vec![0.0; m];

    sgemv(&a, &x, &mut y, m, n);

    std::hint::black_box(y);
}
