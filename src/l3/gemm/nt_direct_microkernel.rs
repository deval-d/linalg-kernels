// nt_direct_microkernel.rs 

use std::simd::Simd; 
use crate::l1::axpy;
use crate::traits::Fma; 
use crate::types::{MatMut, MatRef, VecMut, VecRef}; 

// tunable 
pub(crate) const MR_F32: usize = 16; 
pub(crate) const MR_F64: usize = 8; 

// not tunable 
pub(crate) const NR_F32: usize = 4; 
pub(crate) const NR_F64: usize = 4; 

#[inline(always)]
pub(crate) fn sgemm_nt_micro( 
    alpha: f32, 
    a_panel: MatRef<'_, f32>, 
    b: MatRef<'_, f32>, 
    mut c_panel: MatMut<'_, f32>, 
    kc_beg: usize, 
    j_beg: usize, 
) { 
    let (m, kc) = a_panel.dimension(); 
    let (n, k) = b.dimension(); 
    let nc = c_panel.n_cols(); 
    debug_assert_eq!(m,  c_panel.n_rows()); 
    debug_assert!(j_beg + nc <= n); 
    debug_assert!(kc_beg + kc <= k);

    for j in (0..nc).step_by(NR_F32) {
        let nr = (nc - j).min(NR_F32); 

        // full NR wide panel 
        if nr == NR_F32 { 

            skernel_mrnr(
                alpha, 
                a_panel, 
                b, 
                c_panel.reborrow(), 
                kc_beg, 
                j_beg,
                kc, 
                j,
                m, 
                n,
            );

        } else {
            // leftover cols 
            let j0 = j; 
            let j1 = nc; 
            let b_slice = b.as_slice(); 

            for jj in j0..j1 { 
                let mut ccol = c_panel.col_mut(jj); 
                let jeff = j_beg + jj; 

                for kk in 0..kc { 
                    let keff = kc_beg + kk; 
                    let bval = b_slice[keff * n + jeff]; 
                    axpy(
                        alpha * bval, 
                        a_panel.col(kk), 
                        ccol.reborrow(),
                    );
                }
            }
        }
    }
} 



#[inline(always)]
pub(crate) fn dgemm_nt_micro( 
    alpha: f64, 
    a_panel: MatRef<'_, f64>, 
    b: MatRef<'_, f64>, 
    mut c_panel: MatMut<'_, f64>, 
    kc_beg: usize, 
    j_beg: usize, 
) { 
    let (m, kc) = a_panel.dimension(); 
    let (n, k) = b.dimension(); 
    let nc = c_panel.n_cols(); 
    debug_assert_eq!(m,  c_panel.n_rows()); 
    debug_assert!(j_beg + nc <= n); 
    debug_assert!(kc_beg + kc <= k);

    for j in (0..nc).step_by(NR_F64) {
        let nr = (nc - j).min(NR_F64); 

        // full NR wide panel 
        if nr == NR_F64 { 

            dkernel_mrnr(alpha, 
                a_panel, 
                b, 
                c_panel.reborrow(), 
                kc_beg,
                j_beg,
                kc, 
                j,
                m,
                n,
            );

        } else {
            // leftover cols 
            let j0 = j; 
            let j1 = nc; 
            let b_slice = b.as_slice(); 

            for jj in j0..j1 { 
                let mut ccol = c_panel.col_mut(jj); 
                let jeff = j_beg + jj; 

                for kk in 0..kc { 
                    let keff = kc_beg + kk; 
                    let bval = b_slice[keff * n + jeff]; 
                    axpy(
                        alpha * bval, 
                        a_panel.col(kk), 
                        ccol.reborrow(),
                    );
                }
            }
        }
    }
}


fn skernel_mrnr( 
    alpha: f32, 
    a_panel: MatRef<'_, f32>, 
    b: MatRef<'_, f32>, 
    mut c_panel: MatMut<'_, f32>, 
    kc_beg: usize, 
    j_beg: usize, 
    kc: usize, 
    j: usize, 
    m: usize, 
    n: usize, 
) { 
    debug_assert!(NR_F32 == 4); 

    let c_slice  = c_panel.as_slice_mut();
    let c_base   = j * m; 
    let c_block  = &mut c_slice[c_base..c_base + NR_F32 * m]; 
    let (cl, cr) = c_block.split_at_mut(m * 2); 
    let (c0, c1) = cl.split_at_mut(m); 
    let (c2, c3) = cr.split_at_mut(m);  

    let (c0_chunks, c0_tail) = c0.as_chunks_mut::<MR_F32>();
    let (c1_chunks, c1_tail) = c1.as_chunks_mut::<MR_F32>(); 
    let (c2_chunks, c2_tail) = c2.as_chunks_mut::<MR_F32>();
    let (c3_chunks, c3_tail) = c3.as_chunks_mut::<MR_F32>(); 
    let n_chunks = c0_chunks.len(); 

    let a_slice = a_panel.as_slice(); 
    let b_slice = b.as_slice(); 

    for chunk_idx in 0..n_chunks { 
        let mut c0vec = Simd::from_array(c0_chunks[chunk_idx]);
        let mut c1vec = Simd::from_array(c1_chunks[chunk_idx]); 
        let mut c2vec = Simd::from_array(c2_chunks[chunk_idx]); 
        let mut c3vec = Simd::from_array(c3_chunks[chunk_idx]); 

        for kk in 0..kc { 
            let row_beg = chunk_idx * MR_F32; 
            let col_beg = kk * m; 

            let acol = &a_slice[col_beg + row_beg..col_beg + row_beg + MR_F32]; 
            let avec = Simd::<f32, MR_F32>::from_slice(acol);

            let keff = kc_beg + kk; 
            let jeff = j_beg + j; 
            let b0 = b_slice[keff * n + jeff]; 
            let b1 = b_slice[keff * n + jeff + 1]; 
            let b2 = b_slice[keff * n + jeff + 2]; 
            let b3 = b_slice[keff * n + jeff + 3]; 
            let b0vec = Simd::<f32, MR_F32>::splat(alpha * b0); 
            let b1vec = Simd::<f32, MR_F32>::splat(alpha * b1); 
            let b2vec = Simd::<f32, MR_F32>::splat(alpha * b2); 
            let b3vec = Simd::<f32, MR_F32>::splat(alpha * b3); 

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
        let mr_tail_idx = n_chunks * MR_F32; 

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

            let keff = kc_beg + kk;
            let jeff = j_beg + j; 
            let b0 = b_slice[keff * n + jeff]; 
            let b1 = b_slice[keff * n + jeff + 1]; 
            let b2 = b_slice[keff * n + jeff + 2]; 
            let b3 = b_slice[keff * n + jeff + 3]; 

            axpy(alpha * b0, acol, c0.reborrow());
            axpy(alpha * b1, acol, c1.reborrow()); 
            axpy(alpha * b2, acol, c2.reborrow()); 
            axpy(alpha * b3, acol, c3.reborrow()); 
        }   
    }
}

fn dkernel_mrnr( 
    alpha: f64, 
    a_panel: MatRef<'_, f64>, 
    b: MatRef<'_, f64>, 
    mut c_panel: MatMut<'_, f64>, 
    kc_beg: usize, 
    j_beg: usize, 
    kc: usize, 
    j: usize, 
    m: usize, 
    n: usize, 
) { 
    debug_assert!(NR_F64 == 4); 

    let c_slice  = c_panel.as_slice_mut();
    let c_base   = j * m; 
    let c_block  = &mut c_slice[c_base..c_base + NR_F64 * m]; 
    let (cl, cr) = c_block.split_at_mut(m * 2); 
    let (c0, c1) = cl.split_at_mut(m); 
    let (c2, c3) = cr.split_at_mut(m);  

    let (c0_chunks, c0_tail) = c0.as_chunks_mut::<MR_F64>();
    let (c1_chunks, c1_tail) = c1.as_chunks_mut::<MR_F64>(); 
    let (c2_chunks, c2_tail) = c2.as_chunks_mut::<MR_F64>();
    let (c3_chunks, c3_tail) = c3.as_chunks_mut::<MR_F64>(); 
    let n_chunks = c0_chunks.len(); 

    let a_slice = a_panel.as_slice(); 
    let b_slice = b.as_slice(); 

    for chunk_idx in 0..n_chunks { 
        let mut c0vec = Simd::from_array(c0_chunks[chunk_idx]);
        let mut c1vec = Simd::from_array(c1_chunks[chunk_idx]); 
        let mut c2vec = Simd::from_array(c2_chunks[chunk_idx]); 
        let mut c3vec = Simd::from_array(c3_chunks[chunk_idx]); 

        for kk in 0..kc { 
            let row_beg = chunk_idx * MR_F64; 
            let col_beg = kk * m; 

            let acol = &a_slice[col_beg + row_beg..col_beg + row_beg + MR_F64]; 
            let avec = Simd::<f64, MR_F64>::from_slice(acol);

            let keff = kc_beg + kk; 
            let jeff = j_beg + j; 
            let b0 = b_slice[keff * n + jeff]; 
            let b1 = b_slice[keff * n + jeff + 1]; 
            let b2 = b_slice[keff * n + jeff + 2]; 
            let b3 = b_slice[keff * n + jeff + 3]; 
            let b0vec = Simd::<f64, MR_F64>::splat(alpha * b0); 
            let b1vec = Simd::<f64, MR_F64>::splat(alpha * b1); 
            let b2vec = Simd::<f64, MR_F64>::splat(alpha * b2); 
            let b3vec = Simd::<f64, MR_F64>::splat(alpha * b3); 

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
        let mr_tail_idx = n_chunks * MR_F64; 

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

            let keff = kc_beg + kk;
            let jeff = j_beg + j; 
            let b0 = b_slice[keff * n + jeff]; 
            let b1 = b_slice[keff * n + jeff + 1]; 
            let b2 = b_slice[keff * n + jeff + 2]; 
            let b3 = b_slice[keff * n + jeff + 3]; 

            axpy(alpha * b0, acol, c0.reborrow());
            axpy(alpha * b1, acol, c1.reborrow()); 
            axpy(alpha * b2, acol, c2.reborrow()); 
            axpy(alpha * b3, acol, c3.reborrow()); 
        }   
    }
}
