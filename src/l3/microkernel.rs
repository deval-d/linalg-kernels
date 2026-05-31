// microkernel.rs 

use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement}; 
use std::ops::{AddAssign, Mul, MulAssign}; 

use crate::l1::axpy; 
use crate::traits::Fma; 
use crate::fused::faxpy::faxpy;
use crate::types::{MatRef, MatMut, VecRef, VecMut}; 

use crate::fused::faxpy::N_ROWS_PER_CHUNK; 

pub(crate) const MR: usize = 16; 
pub(crate) const NR: usize = 4; 

pub(crate) fn microkernel<T>( 
    alpha: T,
    a_panel: MatRef<'_, T>, 
    b_panel: MatRef<'_, T>, 
    mut c_panel: MatMut<'_, T>, 
    kc_start: usize, 
) 
where 
    T: SimdElement 
        + Copy 
        + AddAssign
        + MulAssign 
        + Mul<Output=T>
        + Fma,

    Simd<T, MR>: Fma, 

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T> 
        + AddAssign
        + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> 
        + Fma,
{ 
    let (a_rows, kc) = a_panel.dimension(); 
    let (k, b_cols) = b_panel.dimension(); 
    let (m, nc) = c_panel.dimension(); 

    debug_assert!(b_cols == nc, "B and C must be NC-wide"); 
    debug_assert!(a_rows == m, "A and C must be m-long");
    debug_assert!(kc_start + kc <= k, "A panel must fit inside B's k dimension");

    for j in (0..nc).step_by(NR) { 
        let nr = (nc - j).min(NR); 

        if nr == NR { 

            // full NR panel kernel 
            kernel_mrnr(
                alpha, 
                a_panel, 
                b_panel, 
                c_panel.reborrow(), 
                kc_start, 
                kc, 
                j,
                m,
                k, 
            );  

        } else {

            // leftover columns one by one 
            let j0 = j; 
            let j1 = nc; 
            let b_slice = b_panel.as_slice(); 

            for jj in j0..j1 { 
                let ccol = c_panel.col_mut(jj);
                let b_start = jj * k + kc_start; 
                let bcol = VecRef::new(&b_slice[b_start..b_start + kc]); 
        
                faxpy(
                    alpha, 
                    a_panel, 
                    bcol,
                    ccol
                );

            }
        }
    }
}


fn kernel_mrnr<T>( 
    alpha: T,
    a_panel: MatRef<'_, T>, 
    b_panel: MatRef<'_, T>, 
    mut c_panel: MatMut<'_, T>, 
    kc_start: usize, 
    kc: usize, 
    j: usize, 
    m: usize,
    k: usize,
) 
where 
    T: SimdElement 
        + Copy 
        + AddAssign 
        + MulAssign
        + Mul<Output=T>
        + Fma,

    Simd<T, MR>: Fma, 
{ 
    debug_assert!(NR == 4, "NR must equal 4"); 

    let c_slice  = c_panel.as_slice_mut();
    let c_base   = j * m; 
    let c_block  = &mut c_slice[c_base..c_base + NR * m]; 
    let (cl, cr) = c_block.split_at_mut(m * 2); 
    let (c0, c1) = cl.split_at_mut(m); 
    let (c2, c3) = cr.split_at_mut(m);  

    let (c0_chunks, c0_tail) = c0.as_chunks_mut::<MR>();
    let (c1_chunks, c1_tail) = c1.as_chunks_mut::<MR>(); 
    let (c2_chunks, c2_tail) = c2.as_chunks_mut::<MR>();
    let (c3_chunks, c3_tail) = c3.as_chunks_mut::<MR>(); 
    let n_chunks = c0_chunks.len(); 

    let a_slice = a_panel.as_slice(); 
    let b_slice = b_panel.as_slice(); 

    for chunk_idx in 0..n_chunks { 
        let mut c0vec = Simd::from_array(c0_chunks[chunk_idx]);
        let mut c1vec = Simd::from_array(c1_chunks[chunk_idx]); 
        let mut c2vec = Simd::from_array(c2_chunks[chunk_idx]); 
        let mut c3vec = Simd::from_array(c3_chunks[chunk_idx]); 

        for kk in 0..kc { 
            let row_beg = chunk_idx * MR; 
            let col_beg = kk * m; 

            let acol = &a_slice[col_beg + row_beg..col_beg + row_beg + MR]; 
            let avec = Simd::<T, MR>::from_slice(acol);

            let keff = kc_start + kk; 
            let b0 = b_slice[j * k + keff]; 
            let b1 = b_slice[(j + 1) * k + keff]; 
            let b2 = b_slice[(j + 2) * k + keff]; 
            let b3 = b_slice[(j + 3) * k + keff]; 
            let b0vec = Simd::<T, MR>::splat(alpha * b0); 
            let b1vec = Simd::<T, MR>::splat(alpha * b1); 
            let b2vec = Simd::<T, MR>::splat(alpha * b2); 
            let b3vec = Simd::<T, MR>::splat(alpha * b3); 

            c0vec = b0vec.fma(avec, c0vec); 
            c1vec = b1vec.fma(avec, c1vec); 
            c2vec = b2vec.fma(avec, c2vec); 
            c3vec = b3vec.fma(avec, c3vec); 
        }

        c0vec.copy_to_slice(&mut c0_chunks[chunk_idx]);
        c1vec.copy_to_slice(&mut c1_chunks[chunk_idx]); 
        c2vec.copy_to_slice(&mut c2_chunks[chunk_idx]); 
        c3vec.copy_to_slice(&mut c3_chunks[chunk_idx]); 
    }

    // leftover tail
    let tail_len = c0_tail.len(); 
    if tail_len != 0 { 
        let mr_tail_idx = n_chunks * MR; 

        let mut c0 = VecMut::new(c0_tail); 
        let mut c1 = VecMut::new(c1_tail); 
        let mut c2 = VecMut::new(c2_tail); 
        let mut c3 = VecMut::new(c3_tail); 

        for kk in 0..kc { 
            let row_beg = mr_tail_idx; 
            let col_beg = kk * m; 

            let acol = VecRef::new(
                &a_slice[col_beg + row_beg..col_beg + row_beg + tail_len]
            ); 

            let keff = kc_start + kk;
            let b0 = b_slice[j * k + keff]; 
            let b1 = b_slice[(j + 1) * k + keff]; 
            let b2 = b_slice[(j + 2) * k + keff]; 
            let b3 = b_slice[(j + 3) * k + keff]; 

            axpy(alpha * b0, acol, c0.reborrow());
            axpy(alpha * b1, acol, c1.reborrow()); 
            axpy(alpha * b2, acol, c2.reborrow()); 
            axpy(alpha * b3, acol, c3.reborrow()); 
        }   
    }
}
