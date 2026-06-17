use super::common::{ITERATIONS, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_dasum; 
use lak::l1::asum; 
use lak::types::VecRef; 
use lak::helpers::{make_vec_random, test_rng}; 

fn dasum(case: u64) { 
    let length = 1024; 
    
    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(length, &mut rng); 
    let x = VecRef::new(&xbuf); 

    let lak_result = asum(x); 

    let blas_result = unsafe { 
        cblas_dasum( 
            length as i32, 
            xbuf.as_ptr(), 
            1, 
        )
    }; 

    assert_close(&[lak_result], &[blas_result]); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dasum(case);
    }
}

