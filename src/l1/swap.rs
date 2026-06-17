// swap.rs 

use std::mem::swap as memswap;
use crate::types::VecMut; 
use crate::assert_length_eq; 

/// swaps contents of x and y 
///
/// x <-> y 
///
/// args: 
/// * x: [VecMut] - vector x 
/// * y: [VecMut] - vector y 
pub fn swap<T>( 
    mut x: VecMut<'_, T>, 
    mut y: VecMut<'_, T>, 
) { 
    assert_length_eq!(x, y); 

    let x_slice = x.as_slice_mut(); 
    let y_slice = y.as_slice_mut(); 

    for (xv, yv) in x_slice.iter_mut().zip(y_slice.iter_mut()) { 
        memswap(xv, yv); 
    }
}
