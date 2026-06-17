// scal.rs

use std::ops::{Mul, MulAssign};

use crate::types::VecMut; 

/// scales x by alpha 
///
/// x <- alpha * x
///
/// args: 
/// * alpha: T - scales x 
/// * x: [VecMut] - vector x 
pub fn scal<T>( 
    alpha: T, 
    mut x: VecMut<'_, T>
) 
where 
    T: Mul<Output=T>
        + Copy 
        + MulAssign, 
{ 
    let x_slice = x.as_slice_mut(); 

    // no simd needed, already fast 
    for x_value in x_slice.iter_mut() { 
        *x_value *= alpha 
    }
}
