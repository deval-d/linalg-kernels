// nt_blocked.rs

use crate::l1::scal;
use crate::l3::gemm_impl::nn_blocked::{
    CBlockMut,
    KC_F32,
    KC_F64,
    MC_F32,
    MC_F64,
    NC_F32,
    NC_F64,
};
use crate::l3::gemm_impl::nt_blocked_microkernel::{
    dgemm_nt_blocked_micro,
    dgemm_nt_blocked_tail,
    sgemm_nt_blocked_micro,
    sgemm_nt_blocked_tail,
    NR_F32,
    NR_F64,
};
use crate::types::{MatMut, MatRef};


#[inline(always)]
pub(crate) fn sgemm_nt_blocked(
    alpha: f32,
    beta: f32,
    a: MatRef<'_, f32>,
    b: MatRef<'_, f32>,
    mut c: MatMut<'_, f32>,
) {
    let (m, k_a) = a.dimension();
    let (n, k_b) = b.dimension();

    assert_eq!(k_a, k_b, "A and B inner dimension must match");
    assert_eq!(m, c.n_rows(), "A and C row dimensions must match");
    assert_eq!(n, c.n_cols(), "B and C col dimensions must match");

    let k = k_a;

    for j0 in (0..n).step_by(NC_F32) {
        let j1 = (j0 + NC_F32).min(n);

        let mut c_panel = c.col_panel_mut(j0..j1);

        if alpha == 0.0 {
            scale_c_panel_f32(beta, c_panel.reborrow());
            continue;
        }

        let nc = j1 - j0;
        let nr_full = (nc / NR_F32) * NR_F32;
        let packed_b = spack_b_panel(alpha, b, j0, nc, nr_full);

        for i0 in (0..m).step_by(MC_F32) {
            let i1 = (i0 + MC_F32).min(m);

            let mut c_block = CBlockMut::new(
                c_panel.as_slice_mut(),
                (m, nc),
                i0..i1,
            );

            scale_c_block_f32(beta, c_block.reborrow());

            for kc_idx in 0..k.div_ceil(KC_F32) {
                let kc_beg = kc_idx * KC_F32;
                let kc_end = (kc_beg + KC_F32).min(k);
                let kc = kc_end - kc_beg;

                let a_panel = a.col_panel(kc_beg..kc_end);

                if nr_full != 0 {
                    let b_beg = kc_idx * (nr_full / NR_F32) * KC_F32 * NR_F32;
                    let b_end = b_beg + (nr_full / NR_F32) * KC_F32 * NR_F32;

                    sgemm_nt_blocked_micro(
                        a_panel,
                        &packed_b[b_beg..b_end],
                        c_block.reborrow(),
                        kc,
                        KC_F32,
                        nr_full,
                    );
                }

                if nr_full != nc {
                    sgemm_nt_blocked_tail(
                        alpha,
                        a_panel,
                        b,
                        c_block.reborrow(),
                        kc_beg,
                        kc,
                        nr_full,
                        j0,
                    );
                }
            }
        }
    }
}


#[inline(always)]
pub(crate) fn dgemm_nt_blocked(
    alpha: f64,
    beta: f64,
    a: MatRef<'_, f64>,
    b: MatRef<'_, f64>,
    mut c: MatMut<'_, f64>,
) {
    let (m, k_a) = a.dimension();
    let (n, k_b) = b.dimension();

    assert_eq!(k_a, k_b, "A and B inner dimension must match");
    assert_eq!(m, c.n_rows(), "A and C row dimensions must match");
    assert_eq!(n, c.n_cols(), "B and C col dimensions must match");

    let k = k_a;

    for j0 in (0..n).step_by(NC_F64) {
        let j1 = (j0 + NC_F64).min(n);

        let mut c_panel = c.col_panel_mut(j0..j1);

        if alpha == 0.0 {
            scale_c_panel_f64(beta, c_panel.reborrow());
            continue;
        }

        let nc = j1 - j0;
        let nr_full = (nc / NR_F64) * NR_F64;
        let packed_b = dpack_b_panel(alpha, b, j0, nc, nr_full);

        for i0 in (0..m).step_by(MC_F64) {
            let i1 = (i0 + MC_F64).min(m);

            let mut c_block = CBlockMut::new(
                c_panel.as_slice_mut(),
                (m, nc),
                i0..i1,
            );

            scale_c_block_f64(beta, c_block.reborrow());

            for kc_idx in 0..k.div_ceil(KC_F64) {
                let kc_beg = kc_idx * KC_F64;
                let kc_end = (kc_beg + KC_F64).min(k);
                let kc = kc_end - kc_beg;

                let a_panel = a.col_panel(kc_beg..kc_end);

                if nr_full != 0 {
                    let b_beg = kc_idx * (nr_full / NR_F64) * KC_F64 * NR_F64;
                    let b_end = b_beg + (nr_full / NR_F64) * KC_F64 * NR_F64;

                    dgemm_nt_blocked_micro(
                        a_panel,
                        &packed_b[b_beg..b_end],
                        c_block.reborrow(),
                        kc,
                        KC_F64,
                        nr_full,
                    );
                }

                if nr_full != nc {
                    dgemm_nt_blocked_tail(
                        alpha,
                        a_panel,
                        b,
                        c_block.reborrow(),
                        kc_beg,
                        kc,
                        nr_full,
                        j0,
                    );
                }
            }
        }
    }
}


#[inline(always)]
fn spack_b_panel(
    alpha: f32,
    b: MatRef<'_, f32>,
    j_beg: usize,
    nc: usize,
    nr_full: usize,
) -> Vec<f32> {
    let (n, k) = b.dimension();

    debug_assert!(j_beg + nc <= n);
    debug_assert!(nr_full <= nc);

    let n_kc = k.div_ceil(KC_F32);
    let n_nr = nr_full / NR_F32;
    let mut packed = vec![0.0; n_kc * n_nr * KC_F32 * NR_F32];

    let b_slice = b.as_slice();

    for kc_idx in 0..n_kc {
        let kc_beg = kc_idx * KC_F32;
        let kc_end = (kc_beg + KC_F32).min(k);
        let kc = kc_end - kc_beg;

        let kc_pack_beg = kc_idx * n_nr * KC_F32 * NR_F32;

        for nr_idx in 0..n_nr {
            let j = nr_idx * NR_F32;
            let nr_pack_beg = kc_pack_beg + nr_idx * KC_F32 * NR_F32;

            for kk in 0..kc {
                let keff = kc_beg + kk;
                let jeff = j_beg + j;
                let b_pack_beg = nr_pack_beg + kk * NR_F32;

                packed[b_pack_beg]     = alpha * b_slice[keff * n + jeff];
                packed[b_pack_beg + 1] = alpha * b_slice[keff * n + jeff + 1];
                packed[b_pack_beg + 2] = alpha * b_slice[keff * n + jeff + 2];
                packed[b_pack_beg + 3] = alpha * b_slice[keff * n + jeff + 3];
            }
        }
    }

    packed
}


#[inline(always)]
fn dpack_b_panel(
    alpha: f64,
    b: MatRef<'_, f64>,
    j_beg: usize,
    nc: usize,
    nr_full: usize,
) -> Vec<f64> {
    let (n, k) = b.dimension();

    debug_assert!(j_beg + nc <= n);
    debug_assert!(nr_full <= nc);

    let n_kc = k.div_ceil(KC_F64);
    let n_nr = nr_full / NR_F64;
    let mut packed = vec![0.0; n_kc * n_nr * KC_F64 * NR_F64];

    let b_slice = b.as_slice();

    for kc_idx in 0..n_kc {
        let kc_beg = kc_idx * KC_F64;
        let kc_end = (kc_beg + KC_F64).min(k);
        let kc = kc_end - kc_beg;

        let kc_pack_beg = kc_idx * n_nr * KC_F64 * NR_F64;

        for nr_idx in 0..n_nr {
            let j = nr_idx * NR_F64;
            let nr_pack_beg = kc_pack_beg + nr_idx * KC_F64 * NR_F64;

            for kk in 0..kc {
                let keff = kc_beg + kk;
                let jeff = j_beg + j;
                let b_pack_beg = nr_pack_beg + kk * NR_F64;

                packed[b_pack_beg]     = alpha * b_slice[keff * n + jeff];
                packed[b_pack_beg + 1] = alpha * b_slice[keff * n + jeff + 1];
                packed[b_pack_beg + 2] = alpha * b_slice[keff * n + jeff + 2];
                packed[b_pack_beg + 3] = alpha * b_slice[keff * n + jeff + 3];
            }
        }
    }

    packed
}


#[inline(always)]
fn scale_c_panel_f32(
    beta: f32,
    mut c: MatMut<'_, f32>,
) {
    for j in 0..c.n_cols() {
        let mut ccol = c.col_mut(j);

        if beta == 0.0 {
            ccol.as_slice_mut().fill(0.0);
        } else if beta != 1.0 {
            scal(beta, ccol);
        }
    }
}


#[inline(always)]
fn scale_c_panel_f64(
    beta: f64,
    mut c: MatMut<'_, f64>,
) {
    for j in 0..c.n_cols() {
        let mut ccol = c.col_mut(j);

        if beta == 0.0 {
            ccol.as_slice_mut().fill(0.0);
        } else if beta != 1.0 {
            scal(beta, ccol);
        }
    }
}


#[inline(always)]
fn scale_c_block_f32(
    beta: f32,
    mut c: CBlockMut<'_, f32>,
) {
    for j in 0..c.n_cols() {
        let mut ccol = c.col_mut(j);

        if beta == 0.0 {
            ccol.as_slice_mut().fill(0.0);
        } else if beta != 1.0 {
            scal(beta, ccol);
        }
    }
}


#[inline(always)]
fn scale_c_block_f64(
    beta: f64,
    mut c: CBlockMut<'_, f64>,
) {
    for j in 0..c.n_cols() {
        let mut ccol = c.col_mut(j);

        if beta == 0.0 {
            ccol.as_slice_mut().fill(0.0);
        } else if beta != 1.0 {
            scal(beta, ccol);
        }
    }
}
