use super::common::{make_vec, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_dcopy; 
use lak::l1::copy::copy; 
use lak::types::{VecRef, VecMut}; 

#[test] 
fn dcopy() { 
    let length = 1024; 

    let xbuf: Vec<f64> = make_vec(length); 
    let mut ybuf: Vec<f64> = make_vec(length); 

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

