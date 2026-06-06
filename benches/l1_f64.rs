mod common;
use common::{SIZES, bytes_count_f64, bench_rng};

use blas_src as _;
use cblas_sys::{
    cblas_dasum,
    cblas_daxpy,
    cblas_dcopy,
    cblas_ddot,
    cblas_idamax,
    cblas_dnrm2,
    cblas_dscal,
    cblas_dswap,
};

use lak::helpers::make_vec_random;
use lak::types::{VecRef, VecMut};

use lak::l1::asum::asum;
use lak::l1::axpy::axpy;
use lak::l1::copy::copy;
use lak::l1::dot::dot;
use lak::l1::iamax::iamax;
use lak::l1::nrm2::nrm2;
use lak::l1::scal::scal;
use lak::l1::swap::swap;

use divan::{counter::BytesCount, Bencher};

fn main() {
    divan::main();
}


// asum \\


#[divan::bench(args = SIZES)]
pub fn lak_dasum(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(0); 
    let xbuf: Vec<f64> = make_vec_random(n, rng); 
    let x = VecRef::new(&xbuf); 

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| { asum(x) });
}

#[divan::bench(args = SIZES)]
fn blas_dasum(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(0); 
    let xbuf = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| { 
            unsafe {
                cblas_dasum(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
}


// axpy \\


#[divan::bench(args = SIZES)]
pub fn lak_daxpy(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(1); 
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            let y = VecMut::new(&mut ybuf);
            axpy(3.1415926, x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_daxpy(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(1); 
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(3.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            unsafe {
                cblas_daxpy(
                    n as i32,
                    3.1415926,
                    xbuf.as_ptr(),
                    1,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }
        });
}


// copy \\ 


#[divan::bench(args = SIZES)]
pub fn lak_dcopy(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            let y = VecMut::new(&mut ybuf);
            copy(x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_dcopy(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(3);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dcopy(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }
        });
}


// dot \\ 


#[divan::bench(args = SIZES)]
pub fn lak_ddot(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(5);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let ybuf: Vec<f64> = make_vec_random(n, rng);

    let x = VecRef::new(&xbuf);
    let y = VecRef::new(&ybuf);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            dot(x, y)
        });
}

#[divan::bench(args = SIZES)]
fn blas_ddot(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(5);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let ybuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            unsafe {
                cblas_ddot(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                    ybuf.as_ptr(),
                    1,
                )
            }
        });
}


// iamax \\  


#[divan::bench(args = SIZES)]
pub fn lak_idamax(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(7);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            iamax(x)
        });
}

#[divan::bench(args = SIZES)]
fn blas_idamax(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(7);
    let xbuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            unsafe {
                cblas_idamax(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
}


// nrm2 \\  


#[divan::bench(args = SIZES)]
pub fn lak_dnrm2(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(8);
    let xbuf: Vec<f64> = make_vec_random(n, rng);
    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            nrm2(x)
        });
}

#[divan::bench(args = SIZES)]
fn blas_dnrm2(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(8);
    let xbuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(1.0, 1, 0.0, n as f64) as u64))
        .bench(|| {
            unsafe {
                cblas_dnrm2(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
} 


// scal \\ 


#[divan::bench(args = SIZES)]
pub fn lak_dscal(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(9);
    let mut xbuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            let x = VecMut::new(&mut xbuf);
            scal(3.1415926, x);
        });
}

#[divan::bench(args = SIZES)]
fn blas_dscal(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(9);
    let mut xbuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(2.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dscal(
                    n as i32,
                    3.1415926,
                    xbuf.as_mut_ptr(),
                    1,
                );
            }
        });
}


// swap \\  


#[divan::bench(args = SIZES)]
pub fn lak_dswap(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(10);
    let mut xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(4.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            let x = VecMut::new(&mut xbuf);
            let y = VecMut::new(&mut ybuf);

            swap(x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_dswap(bencher: Bencher, n: usize) {
    let rng = &mut bench_rng(10);
    let mut xbuf: Vec<f64> = make_vec_random(n, rng);
    let mut ybuf: Vec<f64> = make_vec_random(n, rng);

    bencher
        .counter(BytesCount::new(bytes_count_f64(4.0, 1, 0.0, n as f64) as u64))
        .bench_local(|| {
            unsafe {
                cblas_dswap(
                    n as i32,
                    xbuf.as_mut_ptr(),
                    1,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }
        });
}
