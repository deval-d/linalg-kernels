// dot.rs

use std::ops::{Add, AddAssign, Mul}; 
use std::simd::{Simd, SimdElement};
use std::simd::num::SimdFloat; 

use crate::types::VecRef; 
use crate::assert_length_eq; 

const LANES: usize = 32;

/// the dot product 
///
/// args: 
/// * x: [VecRef<'_, T>] 
/// * y: [VecRef<'_, T>] 
///
/// returns: 
/// * T - dot product 
pub fn dot<T>( 
    x: VecRef<'_, T>, 
    y: VecRef<'_, T>, 
) -> T 
where
    T: SimdElement 
        + Copy
        + Default
        + AddAssign
        + Mul<Output = T> 
        + Add<Output = T>,

    Simd<T, LANES>: SimdFloat<Scalar = T> 
        + AddAssign
        + Mul<Output = Simd<T, LANES>>, 
{ 
    assert_length_eq!(x, y); 

    let x_slice = x.as_slice(); 
    let y_slice = y.as_slice(); 

    let (x_chunks, x_tail) = x_slice.as_chunks::<LANES>();  
    let (y_chunks, y_tail) = y_slice.as_chunks::<LANES>(); 

    let mut accumulator = Simd::<T, LANES>::splat(T::default()); 
    for (&x_chunk, &y_chunk) in x_chunks.iter().zip(y_chunks.iter()) { 
        let x_vec = Simd::from_array(x_chunk); 
        let y_vec = Simd::from_array(y_chunk); 
        accumulator += x_vec * y_vec; 
    }

    let mut sum = T::default(); 
    for (&xt, &yt) in x_tail.iter().zip(y_tail.iter()) { 
        sum += xt * yt; 
    }

    accumulator.reduce_sum() + sum 
}
