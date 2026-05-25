use super::common::{assert_close, ITERATIONS}; 

use blas_src as _; 
use lak::types::{VecMut, MatRef, Transpose, Triangular}; 
use lak::l2::trsv::trsv; 
use cblas_sys::cblas_dtrsv; 
use lak::helpers::{make_behaved_mat_dd_f64, make_vec_random, test_rng}; 

fn dtrsv_ln(case: u64) { 
    let n = 512; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(n, &mut rng); 
    let abuf: Vec<f64> = make_behaved_mat_dd_f64(n, &mut rng);
    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let a = MatRef::new(&abuf, (n, n));

    trsv(
        Triangular::Lower, 
        Transpose::NoTranspose,
        a, 
        x,
    );

    unsafe { 
        cblas_dtrsv( 
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


#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dtrsv_ln(case); 
    }
}


