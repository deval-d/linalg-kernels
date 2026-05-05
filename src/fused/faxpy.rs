// faxpy.rs  

use std::ops::{Mul, AddAssign}; 
use std::simd::{Simd, SimdElement}; 
use std::simd::num::SimdFloat; 

use crate::l1::axpy;
use crate::types::{MatRef, VecMut, VecRef};
use crate::traits::Fma; 

pub(crate) const MR: usize = 256; 
pub(crate) const NR: usize = 4; 
pub(crate) const LANES: usize = 32; 

pub fn faxpy<T>( 
    alpha: T, 
    a: MatRef<'_, T>,
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>
) 
where  
    T: SimdElement 
        + AddAssign<T>
        + Mul<Output=T>
        + Copy 
        + Fma, 

    Simd<T, LANES>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, LANES>> 
        + Fma,
{ 
    let (n_rows, n_cols) = a.dimension(); 

    let a_slice = a.as_slice(); 
    let x_slice = x.as_slice(); 
    let y_slice = y.as_slice_mut(); 

    let mut i_base = 0; 
    while i_base < n_rows { 
        let mr = (n_rows - i_base).min(MR); 

        let y_panel = &mut y_slice[i_base..i_base + mr]; 
        
        let (y_chunks, y_tail) = y_panel.as_chunks_mut::<LANES>(); 
        let n_chunks = y_chunks.len(); 

        if n_chunks > 0 { 
            let mut j = 0; 
            while j + NR <= n_cols { 
                let x0 = x_slice[j]; 
                let x1 = x_slice[j + 1]; 
                let x2 = x_slice[j + 2]; 
                let x3 = x_slice[j + 3]; 

                let x0v = Simd::<T, LANES>::splat(x0 * alpha); 
                let x1v = Simd::<T, LANES>::splat(x1 * alpha);
                let x2v = Simd::<T, LANES>::splat(x2 * alpha); 
                let x3v = Simd::<T, LANES>::splat(x3 * alpha);

                let col0_beg = j * n_rows + i_base; 
                let col0_end = col0_beg + mr; 

                let col0 = &a_slice[col0_beg..col0_end]; 
                let col1 = &a_slice[col0_beg + n_rows..col0_end + n_rows]; 
                let col2 = &a_slice[col0_beg + 2 * n_rows..col0_end + 2 * n_rows]; 
                let col3 = &a_slice[col0_beg + 3 * n_rows..col0_end + 3 * n_rows]; 

                let (col0_chunks, _) = col0.as_chunks::<LANES>(); 
                let (col1_chunks, _) = col1.as_chunks::<LANES>(); 
                let (col2_chunks, _) = col2.as_chunks::<LANES>(); 
                let (col3_chunks, _) = col3.as_chunks::<LANES>(); 

                for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() { 
                    let mut yv = Simd::<T, LANES>::from_array(*y_chunk); 

                    let a0 = Simd::from_array(col0_chunks[chunk_idx]); 
                    let a1 = Simd::from_array(col1_chunks[chunk_idx]); 
                    let a2 = Simd::from_array(col2_chunks[chunk_idx]); 
                    let a3 = Simd::from_array(col3_chunks[chunk_idx]); 

                    yv = a0.fma(x0v, yv); 
                    yv = a1.fma(x1v, yv); 
                    yv = a2.fma(x2v, yv); 
                    yv = a3.fma(x3v, yv); 

                    *y_chunk = yv.to_array(); 
                }

                j += NR; 
            }

            // leftover cols 
            while j < n_cols { 
                let xj = alpha * x_slice[j]; 
                let xv = Simd::<T, LANES>::splat(xj); 

                let col = &a_slice[j * n_rows + i_base .. j * n_rows + i_base + mr]; 
                let (col_chunks, _) = col.as_chunks::<LANES>(); 

                for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() { 
                    let mut yv = Simd::from_array(*y_chunk); 
                    let av = Simd::from_array(col_chunks[chunk_idx]); 
                    yv = av.fma(xv, yv); 

                    *y_chunk = yv.to_array(); 
                }

                j += 1; 
            }  
        }

        // leftover rows 
        if !y_tail.is_empty() { 
            for (j, &xj) in x_slice.iter().enumerate() { 
                let col_beg = j * n_rows + n_chunks * LANES + i_base; 
                let col_end = col_beg + y_tail.len(); 

                let av = VecRef::new(&a_slice[col_beg..col_end]); 
                let yv = VecMut::new(y_tail); 
                axpy(xj * alpha, av, yv); 
            }
        }

        i_base += mr; 
    }
}
