// asum.rs

use std::ops::{Add, AddAssign};
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
        + Add<Output=T>, 

    Simd<T, LANES>: SimdFloat<Scalar = T> 
        + AddAssign
        + Add<Output=Simd<T, LANES>>
{
    let x_slice = x.as_slice(); 

    let (x_chunks, x_tail) = x_slice.as_chunks::<{LANES * 4}>(); 

    let mut accumulator0 = Simd::<T, LANES>::splat(T::default());
    let mut accumulator1 = Simd::<T, LANES>::splat(T::default()); 
    let mut accumulator2 = Simd::<T, LANES>::splat(T::default()); 
    let mut accumulator3 = Simd::<T, LANES>::splat(T::default()); 


    for &x_chunk in x_chunks.iter() { 
        let x_vec0 = Simd::from_slice(&x_chunk[0..LANES]); 
        let x_vec1 = Simd::from_slice(&x_chunk[LANES..2 * LANES]); 
        let x_vec2 = Simd::from_slice(&x_chunk[2 * LANES..3 * LANES]); 
        let x_vec3 = Simd::from_slice(&x_chunk[3 * LANES..4 * LANES]); 

        accumulator0 += x_vec0.abs(); 
        accumulator1 += x_vec1.abs(); 
        accumulator2 += x_vec2.abs(); 
        accumulator3 += x_vec3.abs(); 
    }

    let mut accumulator = accumulator0 + accumulator1 + accumulator2 + accumulator3; 

    let (x_tail_chunks, x_tail_tail) = x_tail.as_chunks::<LANES>(); 

    for &x_tail_chunk in x_tail_chunks.iter() { 
        let x_vec = Simd::from_array(x_tail_chunk); 
        accumulator += x_vec.abs(); 
    }

    let mut sum = T::default(); 
    for &xt in x_tail_tail.iter() { 
        sum += xt.abs(); 
    }

    accumulator.reduce_sum() + sum
}
