use super::common::{assert_close, ITERATIONS}; 

use blas_src as _; 
use lak::types::{VecRef, MatMut}; 
use lak::l2::ger::ger; 
use cblas_sys::cblas_dger; 
use lak::helpers::{test_rng, make_vec_random}; 

fn dger(case: u64) { 
    let alpha  = 3.1415926;
    let n_rows = 1024; 
    let n_cols = 512; 

    let mut rng = test_rng(case); 
    let xbuf: Vec<f64> = make_vec_random(n_rows, &mut rng); 
    let ybuf: Vec<f64> = make_vec_random(n_cols, &mut rng); 
    let mut abuf: Vec<f64> = make_vec_random(n_cols * n_rows, &mut rng);

    let ybuf_blas = ybuf.clone(); 

    let x = VecRef::new(&xbuf); 
    let y = VecRef::new(&ybuf); 
    let a = MatMut::new(&mut abuf, (n_rows, n_cols));

    ger(alpha, a, x, y);

    unsafe { 
        cblas_dger( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor, 
            n_rows as i32, 
            n_cols as i32, 
            alpha, 
            xbuf.as_ptr(), 
            1 as i32,
            ybuf_blas.as_ptr(),
            1 as i32, 
            abuf.as_mut_ptr(), 
            n_rows as i32, 
        )
    }

    assert_close(&ybuf, &ybuf_blas); 
}

#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
       dger(case); 
    }
}

