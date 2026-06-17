// ger.rs 

use std::ops::{AddAssign, Mul};

use crate::traits::Fma;
use crate::types::{MatMut, VecRef, VecMut}; 
use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows}; 

use crate::l1::axpy::axpy; 

/// general rank 1 update 
///
/// A <- alpha * xy^T + A 
///
/// args: 
/// * alpha: T - scales xy^T 
/// * a: [MatMut] - matrix A 
/// * x: [VecRef] - vector x 
/// * y: [VecRef] - vector y 
pub fn ger<T>( 
    alpha: T, 
    mut a: MatMut<'_, T>, 
    x: VecRef<'_, T>, 
    y: VecRef<'_, T>, 
) 
where    
    T: Copy 
        + AddAssign 
        + Mul<Output=T>
        + Fma, 
{ 
    assert_length_eq_n_cols!(a, y); 
    assert_length_eq_n_rows!(a, x); 

    let n_rows = a.n_rows(); 

    let y_slice = y.as_slice(); 
    let a_slice = a.as_slice_mut();

    for (j, &yj) in y_slice.iter().enumerate() { 
        let col_beg = j * n_rows; 
        let col_end = (j + 1) * n_rows; 
        let a_vec = VecMut::new(&mut a_slice[col_beg..col_end]);
        
        axpy(alpha * yj, x, a_vec); 
    }
}

