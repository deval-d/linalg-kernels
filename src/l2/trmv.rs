// trmv.rs 

use std::ops::{Add, AddAssign, Mul, MulAssign}; 
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::fused::ftrmv::ftrmv_n;
use crate::traits::Fma; 
use crate::assert_length_eq_n_cols; 

use crate::fused::ftrmv::N_ROWS_PER_CHUNK; 

use crate::l1::axpy::axpy; 
use crate::types::{MatRef, VecRef, VecMut}; 
use crate::types::{Triangular, Transpose}; 

// do not change 
// inner kernels depend on 4
pub(crate) const N_COLS_PER_CHUNK: usize = 4;  

/// trmv 
/// lower-triangular no-transpose variant 
pub(crate) fn trmv_ln<T>( 
    a: MatRef<'_, T>, 
    mut x: VecMut<'_, T>, 
) 
where 
    T: Copy 
    + AddAssign 
    + MulAssign
    + Mul<Output=T>
    + Add<Output=T>
    + Fma 
    + SimdElement,

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{
    let (n_rows, n_cols) = a.dimension(); 
    assert_eq!(n_rows, n_cols, "matrix a must be square nxn to be triangular"); 
    assert_length_eq_n_cols!(a, x);

    let n = n_rows; 
    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice_mut(); 

    for (cols, a_panel) in a.col_panels(N_COLS_PER_CHUNK).rev() { 
        // last panel 
        if cols.end - cols.start != N_COLS_PER_CHUNK { 
            for j in cols.rev() { 
                let xj = x_slice[j];
                let a_col = VecRef::new(&a_slice[j * n + j + 1..(j + 1) * n]); 
                let x_col = VecMut::new(&mut x_slice[j + 1..n]); 

                axpy(xj, a_col, x_col); 

                x_slice[j] *= a_slice[j * (n + 1)]; 
            }   
        } else { 
            let j0 = cols.start; 
            let j1 = cols.end; 

            let col0 = a_panel.col(0);
            let col1 = a_panel.col(1);
            let col2 = a_panel.col(2);
            let col3 = a_panel.col(3);

            // full lower rectangular cols 
            // below the N_COLS_ x N_COLS_ triangular block on diagonal 
            let c0 = &col0.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c1 = &col1.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c2 = &col2.as_slice()[j0 + N_COLS_PER_CHUNK..]; 
            let c3 = &col3.as_slice()[j0 + N_COLS_PER_CHUNK..]; 

            let x0 = x_slice[j0]; 
            let x1 = x_slice[j0 + 1]; 
            let x2 = x_slice[j0 + 2]; 
            let x3 = x_slice[j0 + 3]; 

            ftrmv_n( 
                c0, c1, c2, c3,
                x0, x1, x2, x3, 
                &mut x_slice[j1..n], 
            ); 

            // triangular block on diagonal  
            for j in cols.rev() { 
                let xj = x_slice[j];

                let a_col = VecRef::new(&a_slice[j * n + j + 1..j * n + j1]); 
                let x_col = VecMut::new(&mut x_slice[j + 1..j1]);

                axpy(xj, a_col, x_col);

                x_slice[j] *= a_slice[j * (n + 1)]; 
            }
        }
    }
}

/// trmv 
/// upper-triangular no-transpose variant 
pub(crate) fn trmv_un<T>( 
    a: MatRef<'_, T>, 
    mut x: VecMut<'_, T>,  
)
where 
    T: Copy 
    + AddAssign 
    + MulAssign
    + Mul<Output=T>
    + Add<Output=T>
    + Fma 
    + SimdElement,

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{
    let (n_rows, n_cols) = a.dimension(); 
    assert_eq!(n_rows, n_cols, "matrix a must be square nxn to be triangular"); 
    assert_length_eq_n_cols!(a, x);

    let n = n_cols; 
    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice_mut();

    for j in 0..n_cols { 
        let xj = x_slice[j]; 

        let acol = &a_slice[j * n..j * (n + 1)]; 
        let xcol = &mut x_slice[..j];

        axpy( 
            xj, 
            VecRef::new(acol), 
            VecMut::new(xcol), 
        ); 

        x_slice[j] *= a_slice[j * n + j]; 
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
    + Add<Output=T>
    + Fma
    + SimdElement,

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{ 
   match uplo { 
        Triangular::Upper => { 
            match trans { 
                Transpose::NoTranspose => trmv_un(a, x), 
                Transpose::Transpose   => unimplemented!(), 
            }
        }, 
        Triangular::Lower => { 
            match trans { 
                Transpose::NoTranspose => trmv_ln(a, x), 
                Transpose::Transpose   => unimplemented!(), 
            }
        }
   }
}
