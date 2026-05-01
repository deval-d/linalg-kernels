mod common; 
use common::{bench_rng, bytes_count_f32, flops_count, MATRIX_SIZES}; 

use blas_src as _; 
use cblas_sys::{ 
    cblas_sgemv, 
};

use lak::types::{VecRef, VecMut, MatRef}; 
use lak::helpers::make_vec_random;

use lak::l2::gemv::gemv_n; 

use divan::counter::{BytesCount, ItemsCount}; 
use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_sgemv_n(bencher: Bencher, n: usize) { 
    let alpha = 0.1; 
    let beta  = 0.1; 

    let rng = &mut bench_rng(0); 
    let xbuf: Vec<f32> = make_vec_random(n, rng); 
    let mut ybuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng); 

    let a = MatRef::new(&abuf, (n, n)); 
    let x = VecRef::new(&xbuf); 
    let mut y = VecMut::new(&mut ybuf);

    bencher
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| { 
            gemv_n(alpha, beta, a, x, y.reborrow())
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_sgemv_n(bencher: Bencher, n: usize) { 
    let alpha = 0.1; 
    let beta  = 0.1; 

    let rng = &mut bench_rng(0); 
    let xbuf: Vec<f32> = make_vec_random(n, rng); 
    let mut ybuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng); 

    bencher 
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| { 
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
                    1 as i32,
                    beta, 
                    ybuf.as_mut_ptr(),
                    1 as i32, 
                )
            }
        }); 
}
