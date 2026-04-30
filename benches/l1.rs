mod common;
use common::{SIZES, bytes_read_f32};

use blas_src as _;
use cblas_sys::{
    cblas_sasum,
    cblas_saxpy,
    cblas_scopy,
    cblas_sdot,
    cblas_isamax,
    cblas_snrm2,
    cblas_sscal,
    cblas_sswap,
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
pub fn lak_sasum(bencher: Bencher, n: usize) { 
    let xbuf: Vec<f32> = make_vec_random(n); 
    let x = VecRef::new(&xbuf); 

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| { asum(x) });
}

#[divan::bench(args = SIZES)]
fn blas_sasum(bencher: Bencher, n: usize) {
    let xbuf = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| { 
            unsafe {
                cblas_sasum(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
}


// axpy \\

#[divan::bench(args = SIZES)]
pub fn lak_saxpy(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_read_f32(3, n) as u64))
        .bench_local(|| {
            let y = VecMut::new(&mut ybuf);
            axpy(3.1415926_f32, x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_saxpy(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(3, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_saxpy(
                    n as i32,
                    3.1415926_f32,
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
pub fn lak_scopy(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench_local(|| {
            let y = VecMut::new(&mut ybuf);
            copy(x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_scopy(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_scopy(
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
pub fn lak_sdot(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let ybuf: Vec<f32> = make_vec_random(n);

    let x = VecRef::new(&xbuf);
    let y = VecRef::new(&ybuf);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench(|| {
            dot(x, y)
        });
}

#[divan::bench(args = SIZES)]
fn blas_sdot(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let ybuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench(|| {
            unsafe {
                cblas_sdot(
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
pub fn lak_isamax(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| {
            iamax(x)
        });
}

#[divan::bench(args = SIZES)]
fn blas_isamax(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| {
            unsafe {
                cblas_isamax(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
}


// nrm2 \\

#[divan::bench(args = SIZES)]
pub fn lak_snrm2(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);
    let x = VecRef::new(&xbuf);

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| {
            nrm2(x)
        });
}

#[divan::bench(args = SIZES)]
fn blas_snrm2(bencher: Bencher, n: usize) {
    let xbuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(1, n) as u64))
        .bench(|| {
            unsafe {
                cblas_snrm2(
                    n as i32,
                    xbuf.as_ptr(),
                    1,
                )
            }
        });
} 


// scal \\

#[divan::bench(args = SIZES)]
pub fn lak_sscal(bencher: Bencher, n: usize) {
    let mut xbuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench_local(|| {
            let x = VecMut::new(&mut xbuf);
            scal(3.1415926_f32, x);
        });
}

#[divan::bench(args = SIZES)]
fn blas_sscal(bencher: Bencher, n: usize) {
    let mut xbuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(2, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_sscal(
                    n as i32,
                    3.1415926_f32,
                    xbuf.as_mut_ptr(),
                    1,
                );
            }
        });
}


// swap \\

#[divan::bench(args = SIZES)]
pub fn lak_sswap(bencher: Bencher, n: usize) {
    let mut xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(4, n) as u64))
        .bench_local(|| {
            let x = VecMut::new(&mut xbuf);
            let y = VecMut::new(&mut ybuf);

            swap(x, y);
        });
}

#[divan::bench(args = SIZES)]
fn blas_sswap(bencher: Bencher, n: usize) {
    let mut xbuf: Vec<f32> = make_vec_random(n);
    let mut ybuf: Vec<f32> = make_vec_random(n);

    bencher
        .counter(BytesCount::new(bytes_read_f32(4, n) as u64))
        .bench_local(|| {
            unsafe {
                cblas_sswap(
                    n as i32,
                    xbuf.as_mut_ptr(),
                    1,
                    ybuf.as_mut_ptr(),
                    1,
                );
            }
        });
}
