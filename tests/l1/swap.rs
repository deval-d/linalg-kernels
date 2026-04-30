use super::common::{make_vec, assert_close};  

use blas_src as _; 
use cblas_sys::cblas_dswap; 
use lak::l1::swap::swap; 
use lak::types::VecMut; 

#[test] 
fn dswap() { 
    let length = 1024; 

    let mut xbuf: Vec<f64> = make_vec(length); 
    let mut ybuf: Vec<f64> = make_vec(length); 

    let xbuf_orig = xbuf.clone(); 
    let ybuf_orig = ybuf.clone(); 

    let mut xbuf_blas = xbuf.clone(); 
    let mut ybuf_blas = ybuf.clone(); 

    let x = VecMut::new(&mut xbuf); 
    let y = VecMut::new(&mut ybuf); 

    swap(x, y); 

    unsafe { 
        cblas_dswap( 
            length as i32, 
            xbuf_blas.as_mut_ptr(), 
            1, 
            ybuf_blas.as_mut_ptr(), 
            1, 
        )
    }; 

    assert_close(&ybuf, &xbuf_orig);
    assert_close(&ybuf_blas, &xbuf_orig); 
    assert_close(&xbuf, &ybuf_orig); 
    assert_close(&xbuf_blas, &ybuf_orig); 
}

