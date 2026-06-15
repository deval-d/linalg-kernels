mod common;

use common::{
    bench_rng,
    bytes_count_f64,
    flops_count,
    MATRIX_SIZES_L3 as MATRIX_SIZES,
};

#[cfg(any(feature = "accelerate", feature = "openblas"))]
use blas_src as _;
use cblas_sys::cblas_dgemm;

use faer::{Accum, Mat, Par};
use faer::linalg::matmul::matmul as faer_matmul;

use lak::helpers::make_vec_random;
use lak::types::{MatMut, MatRef, Transpose};

use lak::l3::gemm::gemm::dgemm as gemm;

use divan::counter::{BytesCount, ItemsCount};
use divan::{black_box, Bencher};


fn main() {
    divan::main();
}


// gemm NN \\


#[divan::bench(args = MATRIX_SIZES)]
fn lak_dgemm_nn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            let a = MatRef::new(&abuf, (n, n));
            let b = MatRef::new(&bbuf, (n, n));
            let c = MatMut::new(&mut cbuf, (n, n));

            gemm(
                Transpose::NoTranspose,
                Transpose::NoTranspose,
                alpha,
                beta,
                a,
                b,
                c,
            );

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn blas_dgemm_nn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dgemm(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    n as i32,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    bbuf.as_ptr(),
                    n as i32,
                    beta,
                    cbuf.as_mut_ptr(),
                    n as i32,
                );
            }

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn faer_dgemm_nn(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let a = Mat::from_fn(n, n, |i, j| abuf[i + j * n]);
    let b = Mat::from_fn(n, n, |i, j| bbuf[i + j * n]);

    let mut c = Mat::<f64>::zeros(n, n);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            faer_matmul(
                &mut c,
                Accum::Replace, // roughly matches beta = 0.0
                                // does not read pre-existing acc
                &a,
                &b,
                alpha,
                Par::Seq,
            );

            black_box(&c);
        });
}


// gemm NT \\


#[divan::bench(args = MATRIX_SIZES)]
fn lak_dgemm_nt(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            let a = MatRef::new(&abuf, (n, n));
            let b = MatRef::new(&bbuf, (n, n));
            let c = MatMut::new(&mut cbuf, (n, n));

            gemm(
                Transpose::NoTranspose,
                Transpose::Transpose,
                alpha,
                beta,
                a,
                b,
                c,
            );

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn blas_dgemm_nt(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dgemm(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans,
                    n as i32,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    bbuf.as_ptr(),
                    n as i32,
                    beta,
                    cbuf.as_mut_ptr(),
                    n as i32,
                );
            }

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn faer_dgemm_nt(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let a = Mat::from_fn(n, n, |i, j| abuf[i + j * n]);
    let b = Mat::from_fn(n, n, |i, j| bbuf[i + j * n]);

    let mut c = Mat::<f64>::zeros(n, n);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            faer_matmul(
                &mut c,
                Accum::Replace, // roughly matches beta = 0.0
                                // does not read pre-existing acc
                &a,
                b.transpose(),
                alpha,
                Par::Seq,
            );

            black_box(&c);
        });
}


// gemm TN \\


#[divan::bench(args = MATRIX_SIZES)]
fn lak_dgemm_tn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            let a = MatRef::new(&abuf, (n, n));
            let b = MatRef::new(&bbuf, (n, n));
            let c = MatMut::new(&mut cbuf, (n, n));

            gemm(
                Transpose::Transpose,
                Transpose::NoTranspose,
                alpha,
                beta,
                a,
                b,
                c,
            );

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn blas_dgemm_tn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dgemm(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    n as i32,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    bbuf.as_ptr(),
                    n as i32,
                    beta,
                    cbuf.as_mut_ptr(),
                    n as i32,
                );
            }

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn faer_dgemm_tn(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let a = Mat::from_fn(n, n, |i, j| abuf[i + j * n]);
    let b = Mat::from_fn(n, n, |i, j| bbuf[i + j * n]);

    let mut c = Mat::<f64>::zeros(n, n);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            faer_matmul(
                &mut c,
                Accum::Replace, // roughly matches beta = 0.0
                                // does not read pre-existing acc
                a.transpose(),
                &b,
                alpha,
                Par::Seq,
            );

            black_box(&c);
        });
}


// gemm TT \\


#[divan::bench(args = MATRIX_SIZES)]
fn lak_dgemm_tt(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            let a = MatRef::new(&abuf, (n, n));
            let b = MatRef::new(&bbuf, (n, n));
            let c = MatMut::new(&mut cbuf, (n, n));

            gemm(
                Transpose::Transpose,
                Transpose::Transpose,
                alpha,
                beta,
                a,
                b,
                c,
            );

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn blas_dgemm_tt(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.0;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let mut cbuf: Vec<f64> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dgemm(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans,
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans,
                    n as i32,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    bbuf.as_ptr(),
                    n as i32,
                    beta,
                    cbuf.as_mut_ptr(),
                    n as i32,
                );
            }

            black_box(&cbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn faer_dgemm_tt(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f64> = make_vec_random(n * n, rng);
    let bbuf: Vec<f64> = make_vec_random(n * n, rng);

    let a = Mat::from_fn(n, n, |i, j| abuf[i + j * n]);
    let b = Mat::from_fn(n, n, |i, j| bbuf[i + j * n]);

    let mut c = Mat::<f64>::zeros(n, n);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 2, 0.0, n as f64) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            faer_matmul(
                &mut c,
                Accum::Replace, // roughly matches beta = 0.0
                                // does not read pre-existing acc
                a.transpose(),
                b.transpose(),
                alpha,
                Par::Seq,
            );

            black_box(&c);
        });
}
