// nt_blocked_microkernel.rs

use std::simd::Simd;

use crate::l1::axpy;
use crate::l3::gemm::nn_blocked::CBlockMut;
use crate::traits::Fma;
use crate::types::{MatRef, VecMut, VecRef};


// tunable
pub(crate) const MR_F32: usize = 16;
pub(crate) const MR_F64: usize = 8;

// not tunable
pub(crate) const NR_F32: usize = 4;
pub(crate) const NR_F64: usize = 4;


#[inline(always)]
pub(crate) fn sgemm_nt_blocked_micro(
    a_panel: MatRef<'_, f32>,
    packed_b: &[f32],
    mut c_block: CBlockMut<'_, f32>,
    kc: usize,
    kc_storage: usize,
    nr_full: usize,
) {
    let (m, kc_a) = a_panel.dimension();

    debug_assert_eq!(kc, kc_a);
    debug_assert_eq!(nr_full % NR_F32, 0);
    debug_assert!(kc <= kc_storage);

    for j in (0..nr_full).step_by(NR_F32) {
        skernel_mrnr(
            a_panel,
            packed_b,
            c_block.reborrow(),
            kc,
            kc_storage,
            j,
            m,
        );
    }
}


#[inline(always)]
pub(crate) fn dgemm_nt_blocked_micro(
    a_panel: MatRef<'_, f64>,
    packed_b: &[f64],
    mut c_block: CBlockMut<'_, f64>,
    kc: usize,
    kc_storage: usize,
    nr_full: usize,
) {
    let (m, kc_a) = a_panel.dimension();

    debug_assert_eq!(kc, kc_a);
    debug_assert_eq!(nr_full % NR_F64, 0);
    debug_assert!(kc <= kc_storage);

    for j in (0..nr_full).step_by(NR_F64) {
        dkernel_mrnr(
            a_panel,
            packed_b,
            c_block.reborrow(),
            kc,
            kc_storage,
            j,
            m,
        );
    }
}


#[inline(always)]
pub(crate) fn sgemm_nt_blocked_tail(
    alpha: f32,
    a_panel: MatRef<'_, f32>,
    b: MatRef<'_, f32>,
    mut c_block: CBlockMut<'_, f32>,
    kc_beg: usize,
    kc: usize,
    j_beg: usize,
    j_offset: usize,
) {
    let (m, kc_a) = a_panel.dimension();
    let (n, k) = b.dimension();
    let nc = c_block.n_cols();

    debug_assert_eq!(kc, kc_a);
    debug_assert!(j_beg <= nc);
    debug_assert!(j_offset + nc <= n);
    debug_assert!(kc_beg + kc <= k);

    let row_beg = c_block.row_beg();
    let row_end = c_block.row_end();
    let a_slice = a_panel.as_slice();
    let b_slice = b.as_slice();

    for j in j_beg..nc {
        let mut ccol = c_block.col_mut(j);
        let jeff = j_offset + j;

        for kk in 0..kc {
            let acol = VecRef::new(
                &a_slice[kk * m + row_beg..kk * m + row_end]
            );

            let keff = kc_beg + kk;
            let bval = b_slice[keff * n + jeff];

            axpy(
                alpha * bval,
                acol,
                ccol.reborrow(),
            );
        }
    }
}


#[inline(always)]
pub(crate) fn dgemm_nt_blocked_tail(
    alpha: f64,
    a_panel: MatRef<'_, f64>,
    b: MatRef<'_, f64>,
    mut c_block: CBlockMut<'_, f64>,
    kc_beg: usize,
    kc: usize,
    j_beg: usize,
    j_offset: usize,
) {
    let (m, kc_a) = a_panel.dimension();
    let (n, k) = b.dimension();
    let nc = c_block.n_cols();

    debug_assert_eq!(kc, kc_a);
    debug_assert!(j_beg <= nc);
    debug_assert!(j_offset + nc <= n);
    debug_assert!(kc_beg + kc <= k);

    let row_beg = c_block.row_beg();
    let row_end = c_block.row_end();
    let a_slice = a_panel.as_slice();
    let b_slice = b.as_slice();

    for j in j_beg..nc {
        let mut ccol = c_block.col_mut(j);
        let jeff = j_offset + j;

        for kk in 0..kc {
            let acol = VecRef::new(
                &a_slice[kk * m + row_beg..kk * m + row_end]
            );

            let keff = kc_beg + kk;
            let bval = b_slice[keff * n + jeff];

            axpy(
                alpha * bval,
                acol,
                ccol.reborrow(),
            );
        }
    }
}


#[inline(always)]
fn skernel_mrnr(
    a_panel: MatRef<'_, f32>,
    packed_b: &[f32],
    mut c_block: CBlockMut<'_, f32>,
    kc: usize,
    kc_storage: usize,
    j: usize,
    m: usize,
) {
    debug_assert!(NR_F32 == 4);

    let row_beg = c_block.row_beg();
    let row_end = c_block.row_end();
    let mc = c_block.n_rows();

    let c_slice = c_block.as_slice_mut();
    let c_base = j * m;
    let c_panel = &mut c_slice[c_base..c_base + NR_F32 * m];

    let (cl, cr) = c_panel.split_at_mut(m * 2);
    let (c0_full, c1_full) = cl.split_at_mut(m);
    let (c2_full, c3_full) = cr.split_at_mut(m);

    let c0 = &mut c0_full[row_beg..row_end];
    let c1 = &mut c1_full[row_beg..row_end];
    let c2 = &mut c2_full[row_beg..row_end];
    let c3 = &mut c3_full[row_beg..row_end];

    let (c0_chunks, c0_tail) = c0.as_chunks_mut::<MR_F32>();
    let (c1_chunks, c1_tail) = c1.as_chunks_mut::<MR_F32>();
    let (c2_chunks, c2_tail) = c2.as_chunks_mut::<MR_F32>();
    let (c3_chunks, c3_tail) = c3.as_chunks_mut::<MR_F32>();
    let n_chunks = c0_chunks.len();

    let a_slice = a_panel.as_slice();

    let nr_idx = j / NR_F32;
    let b_base = nr_idx * kc_storage * NR_F32;

    for chunk_idx in 0..n_chunks {
        let row_idx = row_beg + chunk_idx * MR_F32;

        let mut c0vec = Simd::from_array(c0_chunks[chunk_idx]);
        let mut c1vec = Simd::from_array(c1_chunks[chunk_idx]);
        let mut c2vec = Simd::from_array(c2_chunks[chunk_idx]);
        let mut c3vec = Simd::from_array(c3_chunks[chunk_idx]);

        for kk in 0..kc {
            let acol = &a_slice[kk * m + row_idx..kk * m + row_idx + MR_F32];
            let avec = Simd::<f32, MR_F32>::from_slice(acol);

            let b_beg = b_base + kk * NR_F32;
            let b0vec = Simd::<f32, MR_F32>::splat(packed_b[b_beg]);
            let b1vec = Simd::<f32, MR_F32>::splat(packed_b[b_beg + 1]);
            let b2vec = Simd::<f32, MR_F32>::splat(packed_b[b_beg + 2]);
            let b3vec = Simd::<f32, MR_F32>::splat(packed_b[b_beg + 3]);

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

    let tail_len = c0_tail.len();
    if tail_len != 0 {
        let row_idx = row_beg + n_chunks * MR_F32;

        debug_assert_eq!(row_idx + tail_len, row_beg + mc);

        let mut c0 = VecMut::new(c0_tail);
        let mut c1 = VecMut::new(c1_tail);
        let mut c2 = VecMut::new(c2_tail);
        let mut c3 = VecMut::new(c3_tail);

        for kk in 0..kc {
            let acol = VecRef::new(
                &a_slice[kk * m + row_idx..kk * m + row_idx + tail_len]
            );

            let b_beg = b_base + kk * NR_F32;

            axpy(packed_b[b_beg],     acol, c0.reborrow());
            axpy(packed_b[b_beg + 1], acol, c1.reborrow());
            axpy(packed_b[b_beg + 2], acol, c2.reborrow());
            axpy(packed_b[b_beg + 3], acol, c3.reborrow());
        }
    }
}


#[inline(always)]
fn dkernel_mrnr(
    a_panel: MatRef<'_, f64>,
    packed_b: &[f64],
    mut c_block: CBlockMut<'_, f64>,
    kc: usize,
    kc_storage: usize,
    j: usize,
    m: usize,
) {
    debug_assert!(NR_F64 == 4);

    let row_beg = c_block.row_beg();
    let row_end = c_block.row_end();
    let mc = c_block.n_rows();

    let c_slice = c_block.as_slice_mut();
    let c_base = j * m;
    let c_panel = &mut c_slice[c_base..c_base + NR_F64 * m];

    let (cl, cr) = c_panel.split_at_mut(m * 2);
    let (c0_full, c1_full) = cl.split_at_mut(m);
    let (c2_full, c3_full) = cr.split_at_mut(m);

    let c0 = &mut c0_full[row_beg..row_end];
    let c1 = &mut c1_full[row_beg..row_end];
    let c2 = &mut c2_full[row_beg..row_end];
    let c3 = &mut c3_full[row_beg..row_end];

    let (c0_chunks, c0_tail) = c0.as_chunks_mut::<MR_F64>();
    let (c1_chunks, c1_tail) = c1.as_chunks_mut::<MR_F64>();
    let (c2_chunks, c2_tail) = c2.as_chunks_mut::<MR_F64>();
    let (c3_chunks, c3_tail) = c3.as_chunks_mut::<MR_F64>();
    let n_chunks = c0_chunks.len();

    let a_slice = a_panel.as_slice();

    let nr_idx = j / NR_F64;
    let b_base = nr_idx * kc_storage * NR_F64;

    for chunk_idx in 0..n_chunks {
        let row_idx = row_beg + chunk_idx * MR_F64;

        let mut c0vec = Simd::from_array(c0_chunks[chunk_idx]);
        let mut c1vec = Simd::from_array(c1_chunks[chunk_idx]);
        let mut c2vec = Simd::from_array(c2_chunks[chunk_idx]);
        let mut c3vec = Simd::from_array(c3_chunks[chunk_idx]);

        for kk in 0..kc {
            let acol = &a_slice[kk * m + row_idx..kk * m + row_idx + MR_F64];
            let avec = Simd::<f64, MR_F64>::from_slice(acol);

            let b_beg = b_base + kk * NR_F64;
            let b0vec = Simd::<f64, MR_F64>::splat(packed_b[b_beg]);
            let b1vec = Simd::<f64, MR_F64>::splat(packed_b[b_beg + 1]);
            let b2vec = Simd::<f64, MR_F64>::splat(packed_b[b_beg + 2]);
            let b3vec = Simd::<f64, MR_F64>::splat(packed_b[b_beg + 3]);

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

    let tail_len = c0_tail.len();
    if tail_len != 0 {
        let row_idx = row_beg + n_chunks * MR_F64;

        debug_assert_eq!(row_idx + tail_len, row_beg + mc);

        let mut c0 = VecMut::new(c0_tail);
        let mut c1 = VecMut::new(c1_tail);
        let mut c2 = VecMut::new(c2_tail);
        let mut c3 = VecMut::new(c3_tail);

        for kk in 0..kc {
            let acol = VecRef::new(
                &a_slice[kk * m + row_idx..kk * m + row_idx + tail_len]
            );

            let b_beg = b_base + kk * NR_F64;

            axpy(packed_b[b_beg],     acol, c0.reborrow());
            axpy(packed_b[b_beg + 1], acol, c1.reborrow());
            axpy(packed_b[b_beg + 2], acol, c2.reborrow());
            axpy(packed_b[b_beg + 3], acol, c3.reborrow());
        }
    }
}
