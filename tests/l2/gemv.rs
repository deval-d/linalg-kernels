use super::common::{assert_close, ITERATIONS}; 

use blas_src as _; 
use lak::types::{VecRef, VecMut, MatRef, Transpose}; 
use lak::l2::gemv; 
use cblas_sys::cblas_dgemv; 
use lak::helpers::{test_rng, make_vec_random}; 

fn dgemv_n(case: u64) { 
    let alpha  = 3.1415926;
    let beta   = 6.7676767; 
    let n_rows = 1024; 
    let n_cols = 512; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(n_cols, &mut rng); 
    let mut ybuf: Vec<f64> = make_vec_random(n_rows, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n_cols * n_rows, &mut rng);

    let mut ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecMut::new(&mut ybuf); 
    let a = MatRef::new(&abuf, (n_rows, n_cols));

    gemv(Transpose::NoTranspose, alpha, beta, a, x, y);

    unsafe { 
        cblas_dgemv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor, 
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
            n_rows as i32, 
            n_cols as i32, 
            alpha, 
            abuf.as_ptr(), 
            n_rows as i32, 
            xbuf.as_ptr(), 
            1 as i32,
            beta, 
            ybuf_blas.as_mut_ptr(),
            1 as i32, 
        )
    }

    assert_close(&ybuf, &ybuf_blas); 
}

fn dgemv_t(case: u64) { 
    let alpha  = 3.1415926;
    let beta   = 6.7676767; 
    let n_rows = 1024; 
    let n_cols = 512; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(n_rows, &mut rng); 
    let mut ybuf: Vec<f64> = make_vec_random(n_cols, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n_cols * n_rows, &mut rng);

    let mut ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecMut::new(&mut ybuf); 
    let a = MatRef::new(&abuf, (n_rows, n_cols));

    gemv(Transpose::Transpose, alpha, beta, a, x, y);

    unsafe { 
        cblas_dgemv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor, 
            cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
            n_rows as i32, 
            n_cols as i32, 
            alpha, 
            abuf.as_ptr(), 
            n_rows as i32, 
            xbuf.as_ptr(), 
            1 as i32,
            beta, 
            ybuf_blas.as_mut_ptr(),
            1 as i32, 
        )
    }

    assert_close(&ybuf, &ybuf_blas); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
       dgemv_n(case); 
       dgemv_t(case); 
    }
}

