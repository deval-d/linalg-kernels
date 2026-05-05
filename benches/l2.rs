mod common; 
use common::{bench_rng, bytes_count_f32, flops_count, MATRIX_SIZES}; 

use blas_src as _; 
use cblas_sys::{ 
    cblas_sgemv, 
};

use lak::helpers::make_vec_random;
use lak::l2::gemv::gemv_n;
use lak::types::{MatRef, VecMut, VecRef};

use divan::counter::{BytesCount, ItemsCount};
use divan::{black_box, Bencher};

fn main() {
    divan::main();
}

#[divan::bench(args = MATRIX_SIZES)]
fn lak_sgemv_n(bencher: Bencher, n: usize) {
    let alpha = black_box(0.1);
    let beta  = black_box(0.1);

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            // for n = 2048 this is only 8 KB, while A is 16 MB,
            // does not meaningfully change the gemv timing.
            ybuf.copy_from_slice(black_box(&y_init));

            let a = MatRef::new(black_box(&abuf), (n, n));
            let x = VecRef::new(black_box(&xbuf));
            let y = VecMut::new(black_box(&mut ybuf));

            gemv_n(
                black_box(alpha),
                black_box(beta),
                a,
                x,
                y,
            );

            black_box(&ybuf);
        });
}

#[divan::bench(args = MATRIX_SIZES)]
fn blas_sgemv_n(bencher: Bencher, n: usize) {
    let alpha = black_box(0.1);
    let beta  = black_box(0.1);

    let rng = &mut bench_rng(0);

    let xbuf: Vec<f32> = make_vec_random(n, rng);
    let abuf: Vec<f32> = make_vec_random(n * n, rng);

    let y_init: Vec<f32> = make_vec_random(n, rng);
    let mut ybuf = y_init.clone();

    bencher
        .counter(BytesCount::new(bytes_count_f32(1, 2, 3, n) as u64))
        .counter(ItemsCount::new(flops_count(2, 2, 3, n) as u64))
        .bench_local(|| {
            ybuf.copy_from_slice(black_box(&y_init));

            unsafe {
                cblas_sgemv(
                    cblas_sys::CBLAS_LAYOUT::CblasColMajor,
                    cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
                    n as i32,
                    n as i32,
                    black_box(alpha),
                    black_box(abuf.as_ptr()),
                    n as i32,
                    black_box(xbuf.as_ptr()),
                    1,
                    black_box(beta),
                    black_box(ybuf.as_mut_ptr()),
                    1,
                );
            }

            black_box(&ybuf);
        });
}
