use super::common::{make_vec, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_ddot; 
use lak::l1::dot::dot; 
use lak::types::VecRef; 

#[test] 
fn ddot() { 
    let length = 1024; 

    let xbuf: Vec<f64> = make_vec(length); 
    let ybuf: Vec<f64> = make_vec(length); 

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

