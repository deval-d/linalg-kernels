use super::common::assert_close;  

use blas_src as _; 
use cblas_sys::cblas_daxpy; 
use lak::l1::axpy::axpy; 
use lak::types::{VecRef, VecMut}; 
use lak::helpers::make_vec_random;


#[test] 
fn daxpy() { 
    let length = 1024; 
    let alpha = 3.1415926; 

    let xbuf: Vec<f64> = make_vec_random(length); 
    let mut ybuf: Vec<f64> = make_vec_random(length); 

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
