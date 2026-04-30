// copy.rs 

use crate::types::{VecRef, VecMut}; 
use crate::assert_length_eq; 

/// copies x into y 
///
/// args: 
/// * x: [VecRef<'_, T>] 
/// * y: [VecMut<'_, T>] 
pub fn copy<T>( 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where T: Copy 
{
    assert_length_eq!(x, y); 

    let x_slice = x.as_slice(); 
    let y_slice = y.as_slice_mut(); 

    // no simd needed, already fast 
    for (&xv, yv) in x_slice.iter().zip(y_slice.iter_mut()) { 
        *yv = xv; 
    }
}
