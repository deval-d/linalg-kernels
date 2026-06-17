mod common; 
use common::{bench_rng, bytes_count_f32, flops_count, MATRIX_SIZES_L2 as MATRIX_SIZES}; 

#[cfg(any(feature = "accelerate", feature = "openblas"))]
use blas_src as _; 
use cblas_sys::{ 
    cblas_sgemv, cblas_sger, cblas_strmv, cblas_strsv, 
};

use lak::helpers::{make_behaved_mat_dd_f32, make_vec_random};
use lak::types::{MatMut, MatRef, Transpose, Triangular, VecMut, VecRef};

use lak::l2::gemv;
use lak::l2::ger;
use lak::l2::trmv;
use lak::l2::trsv;

use divan::counter::{BytesCount, ItemsCount};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}


// gemv \\ 


#[divan::bench(args = MATRIX_SIZES)]
fn lak_sgemv_n(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.0, 2, 3.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            // for n = 2048 this is only 8 KB, while A is 16 MB
            // does not meaningfully change the gemv timing.
            ybuf.copy_from_slice(&y_init);

            let a = MatRef::new(&abuf, (n, n));
            let x = VecRef::new(&xbuf);
            let y = VecMut::new(&mut ybuf);

            gemv(
                Transpose::NoTranspose,
                alpha,
                beta,
                a,
                x,
                y,
            );

            black_box(&ybuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_sgemv_n(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.0, 2, 3.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            ybuf.copy_from_slice(&y_init);

            unsafe {
                cblas_sgemv(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                    beta,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }

            black_box(&ybuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_sgemv_t(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.0, 2, 3.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            // for n = 2048 this is only 8 KB, while a is 16 MB
            // does not meaningfully change the gemv timing.
            ybuf.copy_from_slice(&y_init);

            let a = MatRef::new(&abuf, (n, n));
            let x = VecRef::new(&xbuf);
            let y = VecMut::new(&mut ybuf);

            gemv(
                Transpose::Transpose, 
                alpha,
                beta,
                a,
                x,
                y,
            );

            black_box(&ybuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_sgemv_t(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.0, 2, 3.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            ybuf.copy_from_slice(&y_init);

            unsafe {
                cblas_sgemv(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans,
                    n as i32,
                    n as i32,
                    alpha,
                    abuf.as_ptr(),
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                    beta,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }

            black_box(&ybuf);
        });
}


// ger \\ 


#[divan::bench(args = MATRIX_SIZES)]
fn blas_sger(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(1);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let ybuf: Vec<f32> = make_vec_random(n, rng);

    let a_init: Vec<f32> = make_vec_random(n * n, rng);
    let mut abuf = a_init.clone(); 

    bencher
        .counter(BytesCount::new(bytes_count_f32(2.0, 2, 2.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 0, n) as u64))
        .bench_local(|| {
            abuf.copy_from_slice(&a_init);

            unsafe {
                cblas_sger(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    n as i32,
                    n as i32,
                    alpha,
                    xbuf.as_ptr(),
                    1,
                    ybuf.as_ptr(),
                    1,
                    abuf.as_mut_ptr(),
                    n as i32,
                );
            }

            black_box(&ybuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_sger(bencher: Bencher, n: usize) {
    let alpha = 0.1;

    let rng = &mut bench_rng(1);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let ybuf: Vec<f32> = make_vec_random(n, rng);

    let a_init: Vec<f32> = make_vec_random(n * n, rng);
    let mut abuf = a_init.clone(); 

    bencher
        .counter(BytesCount::new(bytes_count_f32(2.0, 2, 2.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 0, n) as u64))
        .bench_local(|| {
            abuf.copy_from_slice(&a_init);

            let a = MatMut::new(&mut abuf, (n, n)); 
            let x = VecRef::new(&xbuf); 
            let y = VecRef::new(&ybuf);

            ger(alpha, a, x, y); 

            black_box(&ybuf);
        });
}


// trmv \\ 


#[divan::bench(args = MATRIX_SIZES)]
fn lak_strmv_ln(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trmv(Triangular::Lower, Transpose::NoTranspose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strmv_ln(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strmv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasLower, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_strmv_un(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trmv(Triangular::Upper, Transpose::NoTranspose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strmv_un(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strmv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasUpper, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_strmv_lt(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trmv(Triangular::Lower, Transpose::Transpose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strmv_lt(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strmv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasLower, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_strmv_ut(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trmv(Triangular::Upper, Transpose::Transpose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strmv_ut(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(2);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 0.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strmv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasUpper, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);

        }); 
}


// trsv \\ 


#[divan::bench(args = MATRIX_SIZES)]
fn lak_strsv_ln(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trsv(Triangular::Lower, Transpose::NoTranspose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strsv_ln(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strsv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasLower, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_strsv_un(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trsv(Triangular::Upper, Transpose::NoTranspose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strsv_un(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strsv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasUpper, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_strsv_lt(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trsv(Triangular::Lower, Transpose::Transpose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strsv_lt(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strsv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasLower, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}


#[divan::bench(args = MATRIX_SIZES)]
fn lak_strsv_ut(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            let a = MatRef::new(&abuf, (n, n)); 
            let x = VecMut::new(&mut xbuf); 

            trsv(Triangular::Upper, Transpose::Transpose, a, x); 

            black_box(&xbuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_strsv_ut(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);

    let mut xbuf: Vec<f32> = make_vec_random(n, rng);
    let xbuf_init = xbuf.clone();
    let abuf: Vec<f32> = make_behaved_mat_dd_f32(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1.5, 2, 1.5, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(1, 2, 0, n) as u64))
        .bench_local(|| {
            xbuf.copy_from_slice(&xbuf_init);

            unsafe { 
                cblas_strsv( 
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_UPLO::CblasUpper, 
                    cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
                    cblas_sys::CBLAS_DIAG::CblasNonUnit, 
                    n as i32, 
                    abuf.as_ptr(), 
                    n as i32, 
                    xbuf.as_mut_ptr(), 
                    1 as i32,
                ) 

            }

            black_box(&xbuf);
        });
}
