use lak::l3::gemm::gemm::sgemm;
use lak::types::{MatMut, MatRef, Transpose};

#[inline(never)]
fn sgemm_direct(
    alpha: f32,
    beta: f32,
    a: &[f32],
    b: &[f32],
    c: &mut [f32],
    n: usize,
) {
    let a = MatRef::new(a, (n, n));
    let b = MatRef::new(b, (n, n));
    let c = MatMut::new(c, (n, n));

    sgemm(
        Transpose::NoTranspose,
        Transpose::NoTranspose,
        alpha,
        beta,
        a,
        b,
        c,
    );
}

fn main() {
    let n = 512;

    let a = vec![1.0f32; n * n];
    let b = vec![1.0f32; n * n];
    let mut c = vec![0.5f32; n * n];

    sgemm_direct(
        0.1,
        0.1,
        &a,
        &b,
        &mut c,
        n,
    );

    std::hint::black_box(c);
}
