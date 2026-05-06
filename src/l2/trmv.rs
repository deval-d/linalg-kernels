// trmv.rs 

use std::ops::{AddAssign, Mul, MulAssign};

use crate::traits::Fma; 
use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows};

use crate::l1::axpy::axpy; 
use crate::types::{MatRef, VecRef, VecMut}; 
use crate::types::{Triangular, Transpose}; 


// lower triangular no-transpose trmv 
pub(crate) fn trmv_ln<T>( 
    a: MatRef<'_, T>, 
    mut x: VecMut<'_, T>, 
) 
where 
    T: Copy 
    + AddAssign 
    + MulAssign
    + Mul<Output=T>
    + Fma,
{ 
    assert_length_eq_n_cols!(a, x); 
    assert_length_eq_n_rows!(a, x); 

    let n = a.n_cols(); 

    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice_mut(); 

    for j in (0..n).rev() { 
        let xj = x_slice[j]; 

        let a_col = VecRef::new(&a_slice[j * n + j + 1..(j + 1) * n]); 
        let x_col = VecMut::new(&mut x_slice[j + 1..n]); 

        axpy(xj, a_col, x_col); 

        x_slice[j] *= a_slice[j * (n + 1)]; 
    }
}


/// triangular matrix-vector multiply 
///
/// x <- A * x, A triangular 
///
/// args: 
/// * uplo: [Triangular] - whether A is lower or upper triangular 
/// * trans: [Transpose] - whether to use A or A^T  
/// * a: [MatRef] - triangular matrix A 
/// * x: [VecMut] - vector x 
pub fn trmv<T>(
    uplo: Triangular,
    trans: Transpose, 
    a: MatRef<'_, T>, 
    x: VecMut<'_, T>, 
) 
where 
    T: Copy 
    + AddAssign 
    + MulAssign
    + Mul<Output=T>
    + Fma,
{ 
   match uplo { 
        Triangular::Upper => unimplemented!(), 
        Triangular::Lower => { 
            match trans { 
                Transpose::NoTranspose => trmv_ln(a, x), 
                Transpose::Transpose   => unimplemented!(), 
            }
        }
   }
}
