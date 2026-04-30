use super::common::{assert_close, ITERATIONS};  

use blas_src as _; 
use cblas_sys::cblas_dcopy; 
use lak::l1::copy::copy; 
use lak::types::{VecRef, VecMut};
use lak::helpers::{make_vec_random, test_rng};

fn dcopy(case: u64) { 
    let length = 1024; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(length, &mut rng); 
    let mut ybuf: Vec<f64> = make_vec_random(length, &mut rng); 

    let xbuf_blas = xbuf.clone(); 
    let mut ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecMut::new(&mut ybuf); 

    copy(x, y); 

    unsafe { 
        cblas_dcopy( 
            length as i32, 
            xbuf_blas.as_ptr(), 
            1, 
            ybuf_blas.as_mut_ptr(), 
            1, 
        )
    }; 

    assert_close(&ybuf, &xbuf); 
    assert_close(&ybuf, &ybuf_blas); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dcopy(case);
    }
}



