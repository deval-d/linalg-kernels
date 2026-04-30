// iamax.rs 

use std::simd::{Mask, Simd, SimdElement}; 
use std::simd::num::SimdFloat; 
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd}; 

use crate::types::VecRef; 
use crate::traits::Abs;

const LANES: usize = 32; 

/// returns [usize] index for largest absolute value element in x 
///
/// args: 
/// * x: [VecRef<'_, f32>] 
///
/// returns: 
/// * [usize] index for largest absolute value element in x
pub fn iamax<T>( 
    x: VecRef<'_, T>
) -> usize
where 
    T: SimdElement
        + Copy 
        + Default
        + PartialOrd
        + Abs,

    Simd<T, LANES>: SimdFloat<Scalar = T>
        + SimdPartialEq<Mask = Mask<<T as SimdElement>::Mask, LANES>>
        + SimdPartialOrd,
{ 
    let x_slice = x.as_slice(); 
    let (x_chunks, x_tail) = x_slice.as_chunks::<LANES>(); 

    let mut max_idx = 0; 
    let mut max_val = T::default(); 

    for (idx, &x_chunk) in x_chunks.iter().enumerate() { 
        let x_vec = Simd::from_array(x_chunk).abs();
        let mask  = x_vec.simd_gt(Simd::splat(max_val)); 

        if mask.any() { 
            for lane in 0..LANES { 
                let x_val = x_vec[lane]; 
                if x_val > max_val { 
                    max_val = x_val; 
                    max_idx = idx * LANES + lane 
                }
            }
        }
    } 

    let simd_len = x_chunks.len() * LANES; 
    for (idx, &xv) in x_tail.iter().enumerate() { 
        let x_val = xv.abs(); 
        if x_val > max_val { 
            max_val = x_val; 
            max_idx = simd_len + idx;
        }
    }

    max_idx
}
