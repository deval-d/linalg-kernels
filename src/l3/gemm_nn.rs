// gemm_nn.rs 

use std::ops::{AddAssign, Mul, MulAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::l1::scal;
use crate::types::{MatMut, MatRef, VecRef}; 
use crate::traits::Fma; 

use crate::fused::faxpy::{N_COLS_PER_CHUNK, N_ROWS_PER_CHUNK}; 
use crate::fused::faxpy::faxpy; 

pub(crate) fn gemm_nn<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    b: MatRef<'_, T>, 
    mut c: MatMut<'_, T>, 
) 
where 
    T: SimdElement 
        + AddAssign<T>
        + MulAssign
        + Mul<Output=T>
        + Copy
        + Fma, 

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output=Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma, 
{ 
    let (m, n) = a.dimension(); 
    let (o, p) = b.dimension(); 
    let (q, r) = c.dimension(); 

    assert_eq!(n, o, "A and B dimensions don't match for AB"); 
    assert_eq!(m, q, "AB and C dimensions don't match for AB + C"); 
    assert_eq!(p, r, "AB and C dimensions don't match for AB + C"); 

    // (m x k) * (k x n) 
    let k = n; 
    let n = p; 

    for j in 0..n { 
        let c_col = c.col_mut(j); 
        scal(beta, c_col);
    }

    let b_slice = b.as_slice(); 
    for (a_cols, a_panel) in a.col_panels(N_COLS_PER_CHUNK) {
        for j in 0..n { 
            let b_col = VecRef::new(&b_slice[j * k + a_cols.start..j * k + a_cols.end]); 
            let c_col = c.col_mut(j); 

            faxpy(alpha, a_panel, b_col, c_col); 
        }
    }
}
