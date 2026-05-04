// faxpy.rs  

use std::ops::{Mul, AddAssign}; 
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::l1::axpy;
use crate::types::{MatRef, VecMut, VecRef};

pub(crate) const N_ROWS_PER_CHUNK: usize = 16; 
pub(crate) const N_COLS_PER_CHUNK: usize = 4; 

/// a "fused" axpy / mini no-transpose gemv panel: 
pub fn faxpy<T>( 
    alpha: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where
    T: SimdElement 
        + AddAssign<T>
        + Mul<Output=T>
        + Copy,

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>>, 
{ 
    let (n_rows, n_cols) = a.dimension(); 
    let n_row_chunks = n_rows / N_ROWS_PER_CHUNK;
    let n_col_chunks = n_cols / N_COLS_PER_CHUNK;

    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice(); 
    let y_slice = y.as_slice_mut(); 

    for col_chunk in 0..n_col_chunks {
        let j = col_chunk * N_COLS_PER_CHUNK;

        let x1 = Simd::<T, N_ROWS_PER_CHUNK>::splat(alpha * x_slice[j]);
        let x2 = Simd::<T, N_ROWS_PER_CHUNK>::splat(alpha * x_slice[j + 1]);
        let x3 = Simd::<T, N_ROWS_PER_CHUNK>::splat(alpha * x_slice[j + 2]);
        let x4 = Simd::<T, N_ROWS_PER_CHUNK>::splat(alpha * x_slice[j + 3]);

        for row_chunk in 0..n_row_chunks {
            let i = row_chunk * N_ROWS_PER_CHUNK;

            let y_beg = i;
            let y_end = i + N_ROWS_PER_CHUNK;

            let col1_beg = j * n_rows + i;
            let col2_beg = (j + 1) * n_rows + i;
            let col3_beg = (j + 2) * n_rows + i;
            let col4_beg = (j + 3) * n_rows + i;

            let c1 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
                &a_slice[col1_beg..col1_beg + N_ROWS_PER_CHUNK],
            );
            let c2 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
                &a_slice[col2_beg..col2_beg + N_ROWS_PER_CHUNK],
            );
            let c3 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
                &a_slice[col3_beg..col3_beg + N_ROWS_PER_CHUNK],
            );
            let c4 = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(
                &a_slice[col4_beg..col4_beg + N_ROWS_PER_CHUNK],
            );

            let ychunk = &mut y_slice[y_beg..y_end];
            let mut yv = Simd::<T, N_ROWS_PER_CHUNK>::from_slice(ychunk);

            yv += x1 * c1;
            yv += x2 * c2;
            yv += x3 * c3;
            yv += x4 * c4;

            yv.copy_to_slice(ychunk);
        }
    }

    let row_tail_beg = n_row_chunks * N_ROWS_PER_CHUNK; 
    let col_tail_beg = n_col_chunks * N_COLS_PER_CHUNK; 

    // doing axpy on leftover columns 
    for j in col_tail_beg..n_cols {
        let a_vec = VecRef::new(&a_slice[j * n_rows..j * n_rows + row_tail_beg]);
        let y_vec = VecMut::new(&mut y_slice[..row_tail_beg]);

        axpy(alpha * x_slice[j], a_vec, y_vec);
    }

    // doing axpy on leftover rows 
    for j in 0..n_cols { 
        let xalpha = alpha * x_slice[j]; 
        let a_tail = &a_slice[j * n_rows + row_tail_beg..(j + 1) * n_rows]; 
        let y_tail = &mut y_slice[row_tail_beg..n_rows];
         
        let a_vec = VecRef::new(a_tail); 
        let y_vec = VecMut::new(y_tail); 
        
        axpy(xalpha, a_vec, y_vec); 
    }
}
