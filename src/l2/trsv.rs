// trsv.rs

use std::ops::{DivAssign, Mul, Sub, SubAssign};

use crate::types::{MatRef, VecMut};
use crate::assert_length_eq_n_cols; 


pub(crate) fn trmv_ln<T>( 
    a: MatRef<'_, T>, 
    mut x: VecMut<'_, T>, 
) 
where
    T: Copy 
        + Clone 
        + Mul<Output=T> 
        + Sub<Output=T> 
        + DivAssign
        + SubAssign,
{
    let (n_rows, n_cols) = a.dimension(); 
    assert_eq!(n_rows, n_cols, "matrix must be square nxn to be triangular"); 
    assert_length_eq_n_cols!(a, x); 

    let n = n_rows; 
    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice_mut();

    for j in 0..n { 
        let ajj = a_slice[j * (n + 1)]; 
        x_slice[j * n] /= ajj;

        let xj = x_slice[j * n];

        for i in j + 1..n { 
            x_slice[i] -= a_slice[j * n + i] * xj; 
        }
    }       
}

