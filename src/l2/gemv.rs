// gemv.rs 

use std::ops::{AddAssign, Mul, MulAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows}; 
use crate::types::{MatRef, VecMut, VecRef};
use crate::l1::scal::scal;
use crate::fused::faxpy::{faxpy, N_ROWS_PER_CHUNK}; 
use crate::traits::Fma; 

pub const NC: usize = 128; 

pub fn gemv_n<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where  
    T: Copy 
        + Clone 
        + AddAssign 
        + Mul<Output=T> 
        + MulAssign 
        + SimdElement
        + Fma, 

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T>
        + AddAssign 
        + Mul<Output=Simd<T, N_ROWS_PER_CHUNK>>
        + Fma,
{ 
    assert_length_eq_n_cols!(a, x); 
    assert_length_eq_n_rows!(a, y);

    scal(beta, y.reborrow()); 

    for (cols, a_panel) in a.col_panels(NC) {
        let x_panel = VecRef::new(x.slice(cols)); 
        faxpy(alpha, a_panel, x_panel, y.reborrow());
    }
}
