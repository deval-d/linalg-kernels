// iamax.rs 

use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::types::VecRef; 
use crate::traits::Abs;

const LANES: usize = 16; 

/// returns [usize] index for largest absolute value element in x 
///
/// args: 
/// * x: [VecRef] 
///
/// returns: 
/// * [usize] - index for largest absolute value element in x. 
/// * 0       - if input vector empty. 
pub fn iamax<T>( 
    x: VecRef<'_, T>
) -> usize
where 
    T: SimdElement
        + Copy 
        + Default
        + PartialOrd
        + Abs,

    Simd<T, LANES>: SimdFloat<Scalar=T>
{ 
    let x_slice = x.as_slice(); 
    let (x_chunks, x_tail) = x_slice.as_chunks::<LANES>(); 

    let mut max = T::default(); 
    let mut max_idx = 0; 

    for (idx, &x_chunk) in x_chunks.iter().enumerate() { 
        let x_vec = Simd::from_array(x_chunk).abs();
        let x_vec_max = x_vec.reduce_max(); 

        if x_vec_max > max { 
            for lane in 0..LANES { 
                let x_val = x_vec[lane]; 

                if x_val > max { 
                    max = x_val; 
                    max_idx = idx * LANES + lane 
                }
            }
        }
    } 

    let simd_len = x_chunks.len() * LANES; 
    for (idx, &xv) in x_tail.iter().enumerate() { 
        let x_val = xv.abs(); 
        if x_val > max { 
            max = x_val; 
            max_idx = simd_len + idx;
        }
    }

    max_idx
}
