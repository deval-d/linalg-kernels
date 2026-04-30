// axpy.rs

use std::ops::{AddAssign, Mul};

use crate::types::{VecRef, VecMut}; 
use crate::assert_length_eq; 

/// ax + y 
///
/// args: 
/// * a: T - scalar multiplier 
/// * x: [VecRef<'_, T>] 
/// * y: [VecMut<'_, T>]
///
/// panics: 
/// if x and y do not have same length. 
pub fn axpy<T>( 
    a: T, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where 
    T: Copy + AddAssign + Mul<Output = T>, 
{ 
    assert_length_eq!(x, y); 

    let x_slice = x.as_slice(); 
    let y_slice = y.as_slice_mut(); 

    // no simd needed, already fast
    for (&xv, yv) in x_slice.iter().zip(y_slice.iter_mut()) { 
        *yv += a * xv; 
    }
}
