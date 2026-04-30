use super::common::{make_vec, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_dasum; 
use lak::l1::asum::asum; 
use lak::types::VecRef; 

#[test] 
fn dasum() { 
    let length = 1024; 

    let xbuf: Vec<f64> = make_vec(length); 

    let xbuf_blas = xbuf.clone(); 

    let x = VecRef::new(&xbuf); 

    let lak_result = asum(x); 

    let blas_result = unsafe { 
        cblas_dasum( 
            length as i32, 
            xbuf_blas.as_ptr(), 
            1, 
        )
    }; 

    assert_close(&[lak_result], &[blas_result]); 
}

