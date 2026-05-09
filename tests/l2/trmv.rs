use super::common::{assert_close, ITERATIONS}; 

use blas_src as _; 
use lak::types::{VecMut, MatRef, Transpose, Triangular}; 
use lak::l2::trmv::trmv; 
use cblas_sys::cblas_dtrmv; 
use lak::helpers::{test_rng, make_vec_random}; 

fn dtrmv_ln(case: u64) { 
    let n = 512; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(n, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n * n, &mut rng);
    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let a = MatRef::new(&abuf, (n, n));

    trmv(
        Triangular::Lower, 
        Transpose::NoTranspose,
        a, 
        x,
    );

    unsafe { 
        cblas_dtrmv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor,
            cblas_sys::CBLAS_UPLO::CblasLower, 
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
            cblas_sys::CBLAS_DIAG::CblasNonUnit, 
            n as i32, 
            abuf.as_ptr(), 
            n as i32, 
            xbuf_blas.as_mut_ptr(), 
            1 as i32,
        )
    } 

    assert_close(&xbuf, &xbuf_blas); 
}

fn dtrmv_lt(case: u64) { 
    let n = 512; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(n, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n * n, &mut rng);
    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let a = MatRef::new(&abuf, (n, n));

    trmv(
        Triangular::Lower, 
        Transpose::Transpose,
        a, 
        x,
    );

    unsafe { 
        cblas_dtrmv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor,
            cblas_sys::CBLAS_UPLO::CblasLower, 
            cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
            cblas_sys::CBLAS_DIAG::CblasNonUnit, 
            n as i32, 
            abuf.as_ptr(), 
            n as i32, 
            xbuf_blas.as_mut_ptr(), 
            1 as i32,
        )
    } 

    assert_close(&xbuf, &xbuf_blas); 
}

fn dtrmv_un(case: u64) { 
    let n = 512; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(n, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n * n, &mut rng);
    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let a = MatRef::new(&abuf, (n, n));

    trmv(
        Triangular::Upper, 
        Transpose::NoTranspose,
        a, 
        x,
    );

    unsafe { 
        cblas_dtrmv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor,
            cblas_sys::CBLAS_UPLO::CblasUpper, 
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
            cblas_sys::CBLAS_DIAG::CblasNonUnit, 
            n as i32, 
            abuf.as_ptr(), 
            n as i32, 
            xbuf_blas.as_mut_ptr(), 
            1 as i32,
        )
    } 

    assert_close(&xbuf, &xbuf_blas); 
}

fn dtrmv_ut(case: u64) { 
    let n = 512; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(n, &mut rng); 
    let abuf: Vec<f64> = make_vec_random(n * n, &mut rng);
    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let a = MatRef::new(&abuf, (n, n));

    trmv(
        Triangular::Upper, 
        Transpose::Transpose,
        a, 
        x,
    );

    unsafe { 
        cblas_dtrmv( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor,
            cblas_sys::CBLAS_UPLO::CblasUpper, 
            cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
            cblas_sys::CBLAS_DIAG::CblasNonUnit, 
            n as i32, 
            abuf.as_ptr(), 
            n as i32, 
            xbuf_blas.as_mut_ptr(), 
            1 as i32,
        )
    } 

    assert_close(&xbuf, &xbuf_blas); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dtrmv_ln(case); 
        dtrmv_un(case);
        dtrmv_lt(case);
        dtrmv_ut(case);
    }
}

