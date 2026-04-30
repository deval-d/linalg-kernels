use super::common::assert_close;  

use blas_src as _; 
use cblas_sys::cblas_dnrm2; 
use lak::l1::nrm2::nrm2; 
use lak::types::VecRef; 
use lak::helpers::make_vec_random; 

#[test] 
fn dnrm2() { 
    let length = 1024; 

    let xbuf: Vec<f64> = make_vec_random(length); 

    let xbuf_blas = xbuf.clone(); 

    let x = VecRef::new(&xbuf); 

    let lak_result = nrm2(x); 

    let blas_result = unsafe { 
        cblas_dnrm2( 
            length as i32, 
            xbuf_blas.as_ptr(), 
            1, 
        )
    }; 

    assert_close(&[lak_result], &[blas_result]); 
}


