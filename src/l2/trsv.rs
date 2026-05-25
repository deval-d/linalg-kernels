// trsv.rs

use std::ops::{Add, AddAssign, DivAssign, Mul, Neg, SubAssign};
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::traits::Fma;
use crate::types::{MatRef, VecMut, VecRef};
use crate::assert_length_eq_n_cols; 

use crate::l2::trmv::N_COLS_PER_CHUNK; 
use crate::fused::ftrmsv::N_ROWS_PER_CHUNK; 

use crate::fused::ftrmsv::ftrmsv_n; 

use crate::l1::axpy::axpy; 

pub(crate) fn trsv_ln<T>( 
    a: MatRef<'_, T>, 
    mut x: VecMut<'_, T>, 
) 
where 
    T: Copy 
        + AddAssign
        + SubAssign 
        + DivAssign
        + Add<Output=T>
        + Mul<Output=T>
        + Neg<Output=T>
        + SimdElement
        + Fma,

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{ 
    let (n_rows, n_cols) = a.dimension(); 
    assert_eq!(n_rows, n_cols, "matrix must be square nxn to be triangular"); 
    assert_length_eq_n_cols!(a, x); 

    let n = n_rows; 
    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice_mut(); 

    for (cols, a_panel) in a.col_panels(N_COLS_PER_CHUNK) { 
        // last panel 
        if cols.end - cols.start != N_COLS_PER_CHUNK { 
            for j in cols { 
                let ajj = a_slice[j * (n + 1)]; 
                x_slice[j] /= ajj; 

                let xj = x_slice[j]; 
                let x_col = VecMut::new(&mut x_slice[j + 1..n]); 
                let a_col = VecRef::new(&a_slice[j * n + j + 1..(j + 1) * n]); 
                
                axpy(-xj, a_col, x_col);   
            }
        } else { 
            let j0 = cols.start; 
            let j1 = cols.end; 

            // triangular block on diagonal 
            for j in cols { 
                let ajj = a_slice[j * (n + 1)]; 
                x_slice[j] /= ajj; 

                let xj = x_slice[j]; 

                for i in j+1..j1 { 
                    x_slice[i] -= a_slice[j * n + i] * xj; 
                }
            }

            let col0 = a_panel.col(0); 
            let col1 = a_panel.col(1); 
            let col2 = a_panel.col(2); 
            let col3 = a_panel.col(3); 

            // full lower rectangular cols 
            let c0 = &col0.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c1 = &col1.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c2 = &col2.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c3 = &col3.as_slice()[j0 + N_COLS_PER_CHUNK..]; 

            // correct x's 
            let x0 = x_slice[j0]; 
            let x1 = x_slice[j0 + 1];
            let x2 = x_slice[j0 + 2]; 
            let x3 = x_slice[j0 + 3];

            ftrmsv_n(
                c0, c1, c2, c3, 
                -x0, -x1, -x2, -x3, 
                &mut x_slice[j1..n], 
            );    
        }
    }
}
