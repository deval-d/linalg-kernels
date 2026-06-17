use super::common::ITERATIONS; 

use blas_src as _; 
use cblas_sys::cblas_idamax; 
use lak::l1::iamax; 
use lak::types::VecRef; 
use lak::helpers::{make_vec_random, test_rng};

fn idamax(case: u64) { 
    let length = 1024; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(length, &mut rng); 
    
    let x = VecRef::new(&xbuf); 

    let lak_result = iamax(x); 
    let blas_result = unsafe { 
        cblas_idamax( 
            length as i32, 
            xbuf.as_ptr(), 
            1, 
        )
    }; 

    assert_eq!(lak_result, blas_result as usize); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        idamax(case); 
    }
}
