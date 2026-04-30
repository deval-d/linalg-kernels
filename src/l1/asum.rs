// asum.rs

use std::ops::{Add, AddAssign, Mul};
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::types::VecRef; 
use crate::traits::Abs; 

const LANES: usize = 32; 

/// sum of absolute value of all elements in x 
///
/// args: 
/// * x: [VecRef<'_, T>]
///
/// returns: 
/// * T: sum of absolute values elements in x 
pub fn asum<T>( 
    x: VecRef<'_, T>, 
) -> T 
where 
    T: SimdElement 
        + Copy 
        + Default 
        + AddAssign 
        + Abs 
        + Mul<Output=T>
        + Add<Output=T>, 

    Simd<T, LANES>: SimdFloat<Scalar = T> 
        + AddAssign 
{
    let x_slice = x.as_slice(); 

    let (x_chunks, x_tail) = x_slice.as_chunks::<LANES>(); 

    let mut accumulator = Simd::<T, LANES>::splat(T::default()); 

    for &x_chunk in x_chunks.iter() { 
        let x_vec = Simd::from_array(x_chunk); 
        accumulator += x_vec.abs(); 
    }

    let mut sum = T::default(); 
    for &xt in x_tail.iter() { 
        sum += xt.abs(); 
    }

    accumulator.reduce_sum() + sum
}
