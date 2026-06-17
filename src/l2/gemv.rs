// gemv.rs 

use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};
use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::traits::Fma; 
use crate::types::{MatRef, VecMut, VecRef, Transpose};
use crate::{assert_length_eq_n_cols, assert_length_eq_n_rows}; 

use crate::l1::dot::dot; 
use crate::l1::scal::scal; 
use crate::fused::faxpy::faxpy; 
use crate::l1::dot::LANES; 
use crate::fused::faxpy::N_ROWS_PER_CHUNK;

pub(crate) const N_COLS_PER_PANEL: usize = 128; 

/// no transpose gemv 
pub(crate) fn gemv_n<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
) 
where  
    T: AddAssign 
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

    for (cols, a_panel) in a.col_panels(N_COLS_PER_PANEL) {
        let x_panel = VecRef::new(x.slice(cols)); 
        faxpy(alpha, a_panel, x_panel, y.reborrow());
    }
}

/// transpose gemv 
pub(crate) fn gemv_t<T>( 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    mut y: VecMut<'_, T>, 
)
where 
    T: Copy 
        + Mul<Output=T>
        + MulAssign 
        + SimdElement 
        + Default
        + AddAssign
        + Fma 
        + Add<Output=T>, 

    Simd<T, LANES>: SimdFloat<Scalar=T>
        + Fma
        + AddAssign,      
{ 
    assert_length_eq_n_cols!(a, y); 
    assert_length_eq_n_rows!(a, x); 

    scal(beta, y.reborrow()); 

    let y_slice = y.as_slice_mut(); 

    for j in 0..a.n_cols() { 
        let aj = a.col(j); 
        y_slice[j] += alpha * dot(aj, x); 
    }
} 


/// general matrix-vector multiplication 
///
/// y <- alpha * Ax + beta * y 
///
/// args: 
/// * trans: [Transpose] - whether to use A or A^T 
/// * alpha: T - scales A * x 
/// * beta: T - scales y 
/// * a: [MatRef] - matrix A 
/// * x: [VecRef] - vector x 
/// * y: [VecMut] - vector y
///
/// example:
/// ```
/// use lak::l2::gemv;
/// use lak::types::{MatRef, VecMut, VecRef, Transpose};
///
/// // col-major 2 x 3 matrix:
/// // [1 3 5]
/// // [2 4 6]
/// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
/// let x = [1.0, 1.0, 1.0];
/// let mut y = [0.0, 0.0];
///
/// let alpha = 1.0;
/// let beta = 0.0;
///
/// let a = MatRef::new(&a, (2, 3));
/// let x = VecRef::new(&x);
/// let mut y = VecMut::new(&mut y);
/// 
/// // reborrow() used to allow y.as_slice() afterwards in the assert.
/// gemv(Transpose::NoTranspose, alpha, beta, a, x, y.reborrow());
///
/// assert_eq!(y.as_slice(), &[9.0, 12.0]);
/// ```
pub fn gemv<T>( 
    trans: Transpose, 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    x: VecRef<'_, T>, 
    y: VecMut<'_, T>, 
)  
where  
    T: AddAssign 
        + Mul<Output=T> 
        + MulAssign 
        + SimdElement
        + Fma
        + Default
        + Add<Output=T>, 

    Simd<T, N_ROWS_PER_CHUNK>: SimdFloat<Scalar=T>
        + AddAssign 
        + Mul<Output=Simd<T, N_ROWS_PER_CHUNK>>
        + Fma,

    Simd<T, LANES>: SimdFloat<Scalar=T>
        + Fma
        + AddAssign,      
{
    match trans { 
        Transpose::NoTranspose => gemv_n(alpha, beta, a, x, y), 
        Transpose::Transpose   => gemv_t(alpha, beta, a, x, y), 
    }
}


