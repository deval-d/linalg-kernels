mod common; 

use common::{bench_rng, bytes_count_f32, flops_count, MATRIX_SIZES}; 

use blas_src as _; 
use cblas_sys::{ 
    cblas_sgemv, cblas_sger, 
};

use lak::helpers::make_vec_random;
use lak::l2::gemv::gemv;
use lak::l2::ger::ger;
use lak::types::{MatMut, MatRef, Transpose, VecMut, VecRef};

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
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
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
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
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
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
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
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
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
        .counter(BytesCount::new(bytes_count_f32(2, 2, 2, n) as u64))
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
        .counter(BytesCount::new(bytes_count_f32(2, 2, 2, n) as u64))
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



