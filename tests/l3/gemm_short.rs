use super::common::{assert_close, ITERATIONS}; 

use blas_src as _; 
use lak::types::{MatRef, MatMut, Transpose}; 
use lak::l3::gemm::gemm::gemm; 
use cblas_sys::cblas_dgemm; 
use lak::helpers::{test_rng, make_vec_random}; 

fn dgemm_nn(case: u64) { 
    let alpha = 3.1415926;
    let beta  = 6.7676767; 

    let m = 62; 
    let k = 67;
    let n = 53; 

    let mut rng = test_rng(case); 

    let abuf: Vec<f64> = make_vec_random(m * k, &mut rng);
    let bbuf: Vec<f64> = make_vec_random(k * n, &mut rng);
    let mut cbuf: Vec<f64> = make_vec_random(m * n, &mut rng);

    let mut cbuf_blas = cbuf.clone(); 

    let a = MatRef::new(&abuf, (m, k));
    let b = MatRef::new(&bbuf, (k, n)); 
    let c = MatMut::new(&mut cbuf, (m, n)); 

    gemm(Transpose::NoTranspose, Transpose::NoTranspose, alpha, beta, a, b, c);

    unsafe { 
        cblas_dgemm( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor, 
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans, 
            m as i32, 
            n as i32, 
            k as i32, 
            alpha, 
            abuf.as_ptr(), 
            m as i32, 
            bbuf.as_ptr(), 
            k as i32,
            beta, 
            cbuf_blas.as_mut_ptr(),
            m as i32, 
        )
    }

    assert_close(&cbuf, &cbuf_blas); 
}

fn dgemm_nt(case: u64) { 
    let alpha = 3.1415926;
    let beta  = 6.7676767; 

    let m = 62; 
    let k = 67;
    let n = 53; 

    let mut rng = test_rng(case); 

    let abuf: Vec<f64> = make_vec_random(m * k, &mut rng);
    let bbuf: Vec<f64> = make_vec_random(k * n, &mut rng);
    let mut cbuf: Vec<f64> = make_vec_random(m * n, &mut rng);

    let mut cbuf_blas = cbuf.clone(); 

    let a = MatRef::new(&abuf, (m, k));
    let b = MatRef::new(&bbuf, (n, k)); 
    let c = MatMut::new(&mut cbuf, (m, n)); 

    gemm(Transpose::NoTranspose, Transpose::Transpose, alpha, beta, a, b, c);

    unsafe { 
        cblas_dgemm( 
            cblas_sys::CBLAS_LAYOUT::CblasColMajor, 
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
            cblas_sys::CBLAS_TRANSPOSE::CblasTrans, 
            m as i32, 
            n as i32, 
            k as i32, 
            alpha, 
            abuf.as_ptr(), 
            m as i32, 
            bbuf.as_ptr(), 
            n as i32,
            beta, 
            cbuf_blas.as_mut_ptr(),
            m as i32, 
        )
    }

    assert_close(&cbuf, &cbuf_blas); 
}



#[test] 
fn main() { 
    for case in 0..ITERATIONS { 
        dgemm_nn(case); 
        dgemm_nt(case); 
    }
}


