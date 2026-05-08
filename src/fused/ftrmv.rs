// ftrmv.rs 

use std::ops::{Add, AddAssign, Mul}; 
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat;

use crate::traits::Fma;

pub(crate) const N_ROWS_PER_CHUNK: usize = 16; 

/// a kernel for handling full rectangular
/// column panels in trmv no-transpose routines 
///
/// "fuses" four columns ops together to reduce 
/// loads and stores of x
#[allow(clippy::too_many_arguments)]
pub(crate) fn ftrmv_n<T>( 
    col0: &[T], 
    col1: &[T], 
    col2: &[T], 
    col3: &[T], 
    x0: T, 
    x1: T, 
    x2: T, 
    x3: T, 
    x: &mut [T], 
) 
where
    T: SimdElement 
        + AddAssign<T>
        + Mul<Output=T> 
        + Add<Output=T>
        + Copy 
        + Fma, 

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{
    debug_assert!(
        col0.len() == col1.len() && 
        col1.len() == col2.len() && 
        col2.len() == col3.len(), 
        "column vector lengths must be equal"
    ); 

    let n_rows = col1.len(); 
    let n_row_chunks = n_rows / N_ROWS_PER_CHUNK;

    let x0v = Simd::<T, N_ROWS_PER_CHUNK>::splat(x0); 
    let x1v = Simd::<T, N_ROWS_PER_CHUNK>::splat(x1); 
    let x2v = Simd::<T, N_ROWS_PER_CHUNK>::splat(x2); 
    let x3v = Simd::<T, N_ROWS_PER_CHUNK>::splat(x3); 

    for row_chunk in 0..n_row_chunks  { 
        let i = row_chunk * N_ROWS_PER_CHUNK; 

        let c1 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
            &col0[i..i + N_ROWS_PER_CHUNK],
        ); 
        let c2 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
            &col1[i..i + N_ROWS_PER_CHUNK],
        ); 
        let c3 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
            &col2[i..i + N_ROWS_PER_CHUNK],
        ); 
        let c4 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
            &col3[i..i + N_ROWS_PER_CHUNK],
        ); 

        let xchunk = &mut x[i..i + N_ROWS_PER_CHUNK]; 
        let mut xv = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(xchunk); 

        xv = c1.fma(x0v, xv); 
        xv = c2.fma(x1v, xv); 
        xv = c3.fma(x2v, xv); 
        xv = c4.fma(x3v, xv);

        xv.copy_to_slice(xchunk); 
    }

    let tail_beg = n_row_chunks * N_ROWS_PER_CHUNK; 
    for i in tail_beg..n_rows { 
        x[i] += { 
            col0[i] * x0 + 
            col1[i] * x1 + 
            col2[i] * x2 + 
            col3[i] * x3 
        }; 
    }

}


