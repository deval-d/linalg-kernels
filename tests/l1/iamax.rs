use super::common::make_vec;  

use blas_src as _; 
use cblas_sys::cblas_idamax; 
use lak::l1::iamax::iamax; 
use lak::types::VecRef; 

#[test] 
fn idamax() { 
    let length = 1024; 

    let xbuf: Vec<f64> = make_vec(length); 

    let xbuf_blas = xbuf.clone(); 

    let x = VecRef::new(&xbuf); 

    let lak_result = iamax(x); 

    let blas_result = unsafe { 
        cblas_idamax( 
            length as i32, 
            xbuf_blas.as_ptr(), 
            1, 
        )
    }; 

    assert_eq!(lak_result, blas_result as usize); 
}


