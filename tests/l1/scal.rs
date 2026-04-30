use super::common::{make_vec, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_dscal; 
use lak::l1::scal::scal; 
use lak::types::VecMut; 

#[test] 
fn dscal() { 
    let length = 1024; 
    let alpha = 3.1415926; 

    let mut xbuf: Vec<f64> = make_vec(length); 

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

