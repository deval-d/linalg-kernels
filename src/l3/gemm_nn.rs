// gemm_nn.rs

use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};
use std::ops::{AddAssign, Mul, MulAssign};

use crate::l1::scal;
use crate::traits::Fma;
use crate::types::{MatRef, MatMut};
use crate::fused::faxpy::N_ROWS_PER_CHUNK; 
use crate::l3::microkernel::{ 
    MR, 
    microkernel, 
}; 

pub(crate) const NC: usize = 64; 
pub(crate) const KC: usize = 64; 

pub(crate) fn gemm_nn<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    b: MatRef<'_, T>, 
    mut c: MatMut<'_, T>, 
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
    let k_a = a.n_cols(); 
    let (k_b, n) = b.dimension(); 
    let k = { 
        assert_eq!(k_a, k_b, "A and B inner dimension k must match"); 
        k_a 
    }; 

    let num_nc = n.div_ceil(NC); 
    let num_kc = k.div_ceil(KC); 
    
    for nc_idx in 0..num_nc { 
        let j0 = nc_idx * NC; 
        let j1 = (j0 + NC).min(n); 

        let mut c_panel = c.col_panel_mut(j0..j1); 
        let b_panel = b.col_panel(j0..j1); 
        
        for j in 0..c_panel.n_cols() { 
            scal(beta, c_panel.col_mut(j)); 
        }

        for kc_idx in 0..num_kc { 
            let kc_beg = kc_idx * KC; 
            let kc_end = (kc_beg + KC).min(k); 

            let a_panel = a.col_panel(kc_beg..kc_end); 

            microkernel(alpha, a_panel, b_panel, c_panel.reborrow(), kc_beg);
        }
    }


}
