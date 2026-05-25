// gemm.rs 

use std::ops::{AddAssign, Mul, MulAssign}; 
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Fma; 
use crate::types::{MatMut, MatRef, Transpose}; 

use crate::fused::faxpy::N_ROWS_PER_CHUNK; 

use crate::l3::gemm_nn::gemm_nn; 


/// general matrix-matrix multiplication 
///
/// C <- alpha * AB + beta * C 
///
/// args: 
/// * atrans: [Transpose] - whether to use A or A^T 
/// * btrans: [Transpose] - whether to use B or B^T 
/// * alpha: T - scales AB 
/// * beta: T - scales C 
/// * a: [MatRef] - matrix A 
/// * b: [MatRef] - matrix B 
/// * c: [MatMut] - matrix C
pub fn gemm<T>( 
    atrans: Transpose, 
    btrans: Transpose, 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    b: MatRef<'_, T>, 
    c: MatMut<'_, T>, 
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
    match atrans { 
        Transpose::NoTranspose => { 
            match btrans { 
                Transpose::NoTranspose => gemm_nn(alpha, beta, a, b, c), 
                Transpose::Transpose   => unimplemented!(), 
            }
        }, 
        Transpose::Transpose   => { 
            match btrans { 
                Transpose::NoTranspose => unimplemented!(), 
                Transpose::Transpose   => unimplemented!(), 
            }
        }
    }
}
