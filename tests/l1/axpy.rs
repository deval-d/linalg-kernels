use super::common::{assert_close, ITERATIONS};  

use blas_src as _; 
use cblas_sys::cblas_daxpy; 
use lak::l1::axpy::axpy; 
use lak::types::{VecRef, VecMut}; 
use lak::helpers::{make_vec_random, test_rng};

fn daxpy(case: u64) { 
    let length = 1024; 
    let alpha = 3.1415926; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(length, &mut rng); 
    let mut ybuf: Vec<f64> = make_vec_random(length, &mut rng); 

    let xbuf_blas = xbuf.clone(); 
    let mut ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecMut::new(&mut ybuf); 

    axpy(alpha, x, y); 

    unsafe { 
        cblas_daxpy( 
            length as i32, 
            alpha, 
            xbuf_blas.as_ptr(), 
            1, 
            ybuf_blas.as_mut_ptr(), 
            1, 
        )
    }; 

    assert_close(&ybuf, &ybuf_blas); 
}

#[test]
fn main() { 
    for case in 0..ITERATIONS { 
        daxpy(case); 
    }
}
