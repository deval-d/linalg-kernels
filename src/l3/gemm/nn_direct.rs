// nn_direct.rs 

use crate::l1::scal;
use crate::types::{MatRef, MatMut}; 
use crate::l3::gemm::nn_direct_microkernel::{
    sgemm_nn_micro, 
    dgemm_nn_micro, 
}; 

pub(crate) const NC_F32: usize = 128; 
pub(crate) const NC_F64: usize = 64; 
pub(crate) const KC_F32: usize = 128; 
pub(crate) const KC_F64: usize = 64; 

#[inline(always)]
pub(crate) fn sgemm_nn( 
    alpha: f32, 
    beta: f32, 
    a: MatRef<'_, f32>, 
    b: MatRef<'_, f32>, 
    mut c: MatMut<'_, f32>, 
) { 
    let (_, k_a) = a.dimension(); 
    let (k_b, n) = b.dimension(); 
    assert_eq!(k_a, k_b, "A and B inner dimension must match"); 

    let k = k_a; 

    let n_nc = n.div_ceil(NC_F32); 
    let n_kc = k.div_ceil(KC_F32); 

    for nc_idx in 0..n_nc { 
        let j0 = nc_idx * NC_F32; 
        let j1 = (j0 + NC_F32).min(n); 

        let mut c_panel = c.col_panel_mut(j0..j1);
        let b_panel = b.col_panel(j0..j1); 

        if beta != 0.0 { 
            for j in 0..c_panel.n_cols() { 
                scal(beta, c_panel.col_mut(j));
            }
        }

        for kc_idx in 0..n_kc { 
            let kc_beg = kc_idx * KC_F32; 
            let kc_end = (kc_beg + KC_F32).min(k); 

            let a_panel = a.col_panel(kc_beg..kc_end); 

            sgemm_nn_micro(
                alpha, 
                a_panel,
                b_panel,
                c_panel.reborrow(),
                kc_beg
            );
        }
    }
}

#[inline(always)]
pub(crate) fn dgemm_nn( 
    alpha: f64, 
    beta: f64, 
    a: MatRef<'_, f64>, 
    b: MatRef<'_, f64>, 
    mut c: MatMut<'_, f64>, 
) { 
    let (_, k_a) = a.dimension(); 
    let (k_b, n) = b.dimension(); 
    assert_eq!(k_a, k_b, "A and B inner dimension must match"); 

    let k = k_a; 

    let n_nc = n.div_ceil(NC_F64); 
    let n_kc = k.div_ceil(KC_F64); 

    for nc_idx in 0..n_nc { 
        let j0 = nc_idx * NC_F64; 
        let j1 = (j0 + NC_F64).min(n); 

        let mut c_panel = c.col_panel_mut(j0..j1);
        let b_panel = b.col_panel(j0..j1); 

        for j in 0..c_panel.n_cols() { 
            scal(beta, c_panel.col_mut(j));
        }

        for kc_idx in 0..n_kc { 
            let kc_beg = kc_idx * KC_F64; 
            let kc_end = (kc_beg + KC_F64).min(k); 

            let a_panel = a.col_panel(kc_beg..kc_end); 

            dgemm_nn_micro(
                alpha, 
                a_panel,
                b_panel,
                c_panel.reborrow(),
                kc_beg
            );
        }
    }
}









