mod common; 
use common::{
    bench_rng, 
    bytes_count_f32, 
    flops_count,
    MATRIX_SIZES_L3 as MATRIX_SIZES
}; 

use blas_src as _; 
use cblas_sys::{ 
    cblas_sgemm, 
};

use lak::helpers::make_vec_random;
use lak::types::{MatMut, MatRef, Transpose}; 

use lak::l3::gemm::gemm::sgemm as gemm; 

use divan::counter::{BytesCount, ItemsCount};
use divan::{black_box, Bencher};


fn main() {
    divan::main();
}


// gemm \\


#[divan::bench(args = MATRIX_SIZES)]
fn lak_sgemm_nn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f32> = make_vec_random(n * n, rng);
    let bbuf: Vec<f32> = make_vec_random(n * n, rng);

    let c_init: Vec<f32> = make_vec_random(n * n, rng);
    let mut cbuf = c_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(2.0, 2, 1.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            cbuf.copy_from_slice(&c_init);

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
fn blas_sgemm_nn(bencher: Bencher, n: usize) {
    let alpha = 0.1;
    let beta  = 0.1;

    let rng = &mut bench_rng(0);

    let abuf: Vec<f32> = make_vec_random(n * n, rng);
    let bbuf: Vec<f32> = make_vec_random(n * n, rng);

    let c_init: Vec<f32> = make_vec_random(n * n, rng);
    let mut cbuf = c_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(2.0, 2, 1.0, n as f32) as u64))
        .counter(ItemsCount::new(flops_count(2, 3, 0, n) as u64))
        .bench_local(|| {
            cbuf.copy_from_slice(&c_init);

            unsafe {
                cblas_sgemm(
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
