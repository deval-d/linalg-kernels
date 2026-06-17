use lak::l2::gemv;
use lak::types::{MatRef, Transpose, VecMut, VecRef};

#[inline(never)]
fn sgemv_n(
    alpha: f32,
    beta: f32,
    a: &[f32],
    x: &[f32],
    y: &mut [f32],
    m: usize,
    n: usize,
) {
    let a = MatRef::new(a, (m, n));
    let x = VecRef::new(x);
    let y = VecMut::new(y);

    gemv(
        Transpose::NoTranspose,
        alpha,
        beta,
        a,
        x,
        y,
    );
}

fn main() {
    let m = 1024;
    let n = 1024;

    let alpha = 2.0f32;
    let beta = 3.0f32;

    let a = vec![1.0f32; m * n];
    let x = vec![2.0f32; n];
    let mut y = vec![3.0f32; m];

    sgemv_n(alpha, beta, &a, &x, &mut y, m, n);

    std::hint::black_box(y);
}
