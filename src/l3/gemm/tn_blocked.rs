// tn_blocked.rs

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
use crate::l3::gemm_impl::tn_blocked_microkernel::{
    dgemm_tn_blocked_micro,
    dgemm_tn_blocked_tail,
    sgemm_tn_blocked_micro,
    sgemm_tn_blocked_tail,
    NR_F32,
    NR_F64,
};
use crate::types::{MatMut, MatRef};


#[inline(always)]
pub(crate) fn sgemm_tn_blocked(
    alpha: f32,
    beta: f32,
    a: MatRef<'_, f32>,
    b: MatRef<'_, f32>,
    mut c: MatMut<'_, f32>,
) {
    let (k_a, m) = a.dimension();
    let (k_b, n) = b.dimension();

    assert_eq!(k_a, k_b, "A and B inner dimension must match");
    assert_eq!(m, c.n_rows(), "A and C row dimensions must match");
    assert_eq!(n, c.n_cols(), "B and C col dimensions must match");

    let k = k_a;

    for (j_range, b_panel) in b.col_panels(NC_F32) {
        let j0 = j_range.start;
        let j1 = j_range.end;

        let mut c_panel = c.col_panel_mut(j0..j1);

        if alpha == 0.0 {
            scale_c_panel_f32(beta, c_panel.reborrow());
            continue;
        }

        let nc = j1 - j0;
        let nr_full = (nc / NR_F32) * NR_F32;
        let packed_b = spack_b_panel(alpha, b_panel, nr_full);

        for i0 in (0..m).step_by(MC_F32) {
            let i1 = (i0 + MC_F32).min(m);
            let mc = i1 - i0;

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

                let packed_a = spack_a_panel(a, i0, mc, kc_beg, kc);

                if nr_full != 0 {
                    let b_beg = kc_idx * (nr_full / NR_F32) * KC_F32 * NR_F32;
                    let b_end = b_beg + (nr_full / NR_F32) * KC_F32 * NR_F32;

                    sgemm_tn_blocked_micro(
                        &packed_a,
                        &packed_b[b_beg..b_end],
                        c_block.reborrow(),
                        kc,
                        KC_F32,
                        mc,
                        nr_full,
                    );
                }

                if nr_full != nc {
                    sgemm_tn_blocked_tail(
                        &packed_a,
                        alpha,
                        b_panel,
                        c_block.reborrow(),
                        kc_beg,
                        kc,
                        mc,
                        nr_full,
                    );
                }
            }
        }
    }
}


#[inline(always)]
pub(crate) fn dgemm_tn_blocked(
    alpha: f64,
    beta: f64,
    a: MatRef<'_, f64>,
    b: MatRef<'_, f64>,
    mut c: MatMut<'_, f64>,
) {
    let (k_a, m) = a.dimension();
    let (k_b, n) = b.dimension();

    assert_eq!(k_a, k_b, "A and B inner dimension must match");
    assert_eq!(m, c.n_rows(), "A and C row dimensions must match");
    assert_eq!(n, c.n_cols(), "B and C col dimensions must match");

    let k = k_a;

    for (j_range, b_panel) in b.col_panels(NC_F64) {
        let j0 = j_range.start;
        let j1 = j_range.end;

        let mut c_panel = c.col_panel_mut(j0..j1);

        if alpha == 0.0 {
            scale_c_panel_f64(beta, c_panel.reborrow());
            continue;
        }

        let nc = j1 - j0;
        let nr_full = (nc / NR_F64) * NR_F64;
        let packed_b = dpack_b_panel(alpha, b_panel, nr_full);

        for i0 in (0..m).step_by(MC_F64) {
            let i1 = (i0 + MC_F64).min(m);
            let mc = i1 - i0;

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

                let packed_a = dpack_a_panel(a, i0, mc, kc_beg, kc);

                if nr_full != 0 {
                    let b_beg = kc_idx * (nr_full / NR_F64) * KC_F64 * NR_F64;
                    let b_end = b_beg + (nr_full / NR_F64) * KC_F64 * NR_F64;

                    dgemm_tn_blocked_micro(
                        &packed_a,
                        &packed_b[b_beg..b_end],
                        c_block.reborrow(),
                        kc,
                        KC_F64,
                        mc,
                        nr_full,
                    );
                }

                if nr_full != nc {
                    dgemm_tn_blocked_tail(
                        &packed_a,
                        alpha,
                        b_panel,
                        c_block.reborrow(),
                        kc_beg,
                        kc,
                        mc,
                        nr_full,
                    );
                }
            }
        }
    }
}


#[inline(always)]
fn spack_a_panel(
    a: MatRef<'_, f32>,
    i_beg: usize,
    mc: usize,
    kc_beg: usize,
    kc: usize,
) -> Vec<f32> {
    let (k, m) = a.dimension();

    debug_assert!(i_beg + mc <= m);
    debug_assert!(kc_beg + kc <= k);

    let mut packed = vec![0.0; KC_F32 * mc];
    let a_slice = a.as_slice();

    for kk in 0..kc {
        let keff = kc_beg + kk;
        let pack_beg = kk * mc;

        for i in 0..mc {
            packed[pack_beg + i] = a_slice[(i_beg + i) * k + keff];
        }
    }

    packed
}


#[inline(always)]
fn dpack_a_panel(
    a: MatRef<'_, f64>,
    i_beg: usize,
    mc: usize,
    kc_beg: usize,
    kc: usize,
) -> Vec<f64> {
    let (k, m) = a.dimension();

    debug_assert!(i_beg + mc <= m);
    debug_assert!(kc_beg + kc <= k);

    let mut packed = vec![0.0; KC_F64 * mc];
    let a_slice = a.as_slice();

    for kk in 0..kc {
        let keff = kc_beg + kk;
        let pack_beg = kk * mc;

        for i in 0..mc {
            packed[pack_beg + i] = a_slice[(i_beg + i) * k + keff];
        }
    }

    packed
}


#[inline(always)]
fn spack_b_panel(
    alpha: f32,
    b_panel: MatRef<'_, f32>,
    nr_full: usize,
) -> Vec<f32> {
    let (k, _) = b_panel.dimension();

    let n_kc = k.div_ceil(KC_F32);
    let n_nr = nr_full / NR_F32;
    let mut packed = vec![0.0; n_kc * n_nr * KC_F32 * NR_F32];

    let b_slice = b_panel.as_slice();

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
                let b_pack_beg = nr_pack_beg + kk * NR_F32;

                packed[b_pack_beg]     = alpha * b_slice[j * k + keff];
                packed[b_pack_beg + 1] = alpha * b_slice[(j + 1) * k + keff];
                packed[b_pack_beg + 2] = alpha * b_slice[(j + 2) * k + keff];
                packed[b_pack_beg + 3] = alpha * b_slice[(j + 3) * k + keff];
            }
        }
    }

    packed
}


#[inline(always)]
fn dpack_b_panel(
    alpha: f64,
    b_panel: MatRef<'_, f64>,
    nr_full: usize,
) -> Vec<f64> {
    let (k, _) = b_panel.dimension();

    let n_kc = k.div_ceil(KC_F64);
    let n_nr = nr_full / NR_F64;
    let mut packed = vec![0.0; n_kc * n_nr * KC_F64 * NR_F64];

    let b_slice = b_panel.as_slice();

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
                let b_pack_beg = nr_pack_beg + kk * NR_F64;

                packed[b_pack_beg]     = alpha * b_slice[j * k + keff];
                packed[b_pack_beg + 1] = alpha * b_slice[(j + 1) * k + keff];
                packed[b_pack_beg + 2] = alpha * b_slice[(j + 2) * k + keff];
                packed[b_pack_beg + 3] = alpha * b_slice[(j + 3) * k + keff];
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
