use super::common::{assert_close, ITERATIONS};  

use blas_src as _; 
use cblas_sys::cblas_ddot; 
use lak::l1::dot; 
use lak::types::VecRef; 
use lak::helpers::{make_vec_random, test_rng};

fn ddot(case: u64) { 
    let length = 1024; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(length, &mut rng); 
    let ybuf: Vec<f64> = make_vec_random(length, &mut rng); 

    let xbuf_blas = xbuf.clone(); 
    let ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecRef::new(&ybuf); 

    let lak_result = dot(x, y); 

    let blas_result = unsafe { 
        cblas_ddot( 
            length as i32, 
            xbuf_blas.as_ptr(), 
            1, 
            ybuf_blas.as_ptr(), 
            1, 
        )
    }; 

    assert_close(&[lak_result], &[blas_result]); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        ddot(case); 
    }
}

