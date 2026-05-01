// gemv.rs 

use std::ops::{AddAssign, Mul};

use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows}; 
use crate::types::{MatRef, VecMut, VecRef};
use crate::l1::axpy::axpy; 
use crate::l1::scal::scal; 

pub fn gemv_n<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where 
    T: Clone 
        + Copy 
        + AddAssign
        + Mul<Output = T>,
{
    assert_length_eq_n_cols!(a, x); 
    assert_length_eq_n_rows!(a, y); 

    let n_rows  = a.n_rows(); 
    let x_slice = x.as_slice(); 

    scal(beta, y.reborrow()); 

    for (j, &x_j) in x_slice.iter().enumerate() { 
        let a_j = a.specific_slice((0, j), (n_rows, j)); 

        let a_j_vec = VecRef::new(a_j);
        axpy(alpha * x_j, a_j_vec, y.reborrow()); 
    }
}
