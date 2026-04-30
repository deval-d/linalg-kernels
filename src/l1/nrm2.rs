// nrm2.rs 

use std::ops::{Add, AddAssign, Mul};
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::types::VecRef;
use crate::traits::Sqrt;

const LANES: usize = 32; 

/// computes norm of x
///
/// args: 
/// * x: [VecRef<'_, T>] 
///
/// returns: 
/// * norm of x 
pub fn nrm2<T>( 
    x: VecRef<'_, T>, 
) -> T
where T: SimdElement 
    + Copy
    + Default
    + AddAssign 
    + Sqrt
    + Add<Output = T> 
    + Mul<Output = T>,

    Simd<T, LANES>: SimdFloat<Scalar = T>
    + AddAssign 
    + Mul<Output = Simd<T, LANES>> 
    + Add<Output = Simd<T, LANES>>, 
{ 
    let x_slice = x.as_slice(); 

    let (x_chunks, x_tail) = x_slice.as_chunks::<LANES>();

    let mut accumulator = Simd::<T, LANES>::splat(T::default()); 
    
    for &x_chunk in x_chunks.iter() { 
        let x_vec = Simd::from_array(x_chunk); 
        accumulator += x_vec * x_vec;
    }   

    let mut sum = T::default(); 
    for &xt in x_tail.iter() { 
        sum += xt * xt; 
    }

    (accumulator.reduce_sum() + sum).sqrt()
}
