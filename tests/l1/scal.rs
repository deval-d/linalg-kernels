use super::common::{assert_close, ITERATIONS};

use blas_src as _; 
use cblas_sys::cblas_dscal; 
use lak::l1::scal; 
use lak::types::VecMut; 
use lak::helpers::{make_vec_random, test_rng}; 

fn dscal(case: u64) { 
    let length = 1024; 
    let alpha = 3.1415926; 

    let mut rng = test_rng(case); 
    let mut xbuf: Vec<f64> = make_vec_random(length, &mut rng); 

    let mut xbuf_blas = xbuf.clone(); 

    let x = VecMut::new(&mut xbuf); 

    scal(alpha, x); 

    unsafe { 
        cblas_dscal( 
            length as i32, 
            alpha, 
            xbuf_blas.as_mut_ptr(), 
            1, 
        )
    }; 

    assert_close(&xbuf, &xbuf_blas); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dscal(case); 
    }
}
