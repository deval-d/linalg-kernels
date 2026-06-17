use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Fma;

use crate::blas::helpers::{char_2_transpose, ptr_2_matref, ptr_2_vecmut, ptr_2_vecref};
use crate::fused::faxpy::N_ROWS_PER_CHUNK;
use crate::l1::dot::LANES;
use crate::l2::gemv;
use crate::types::Transpose;

/// BLAS LP64 interface for LAK [gemv]
pub unsafe fn gemv_lp64<T>(
    trans: u8,
    m: i32,
    n: i32,
    alpha: T,
    a: *const T,
    lda: i32,
    x: *const T,
    incx: i32,
    beta: T,
    y: *mut T,
    incy: i32,
) where
    T: AddAssign + Mul<Output = T> + MulAssign + SimdElement + Fma + Default + Add<Output = T>,

    Simd<T, N_ROWS_PER_CHUNK>:
        SimdFloat<Scalar = T> + AddAssign + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + Fma + AddAssign,
{
    unsafe {
        let trans = char_2_transpose(trans);
        let a = ptr_2_matref(m, n, a, lda);
        let x_len = match trans {
            Transpose::NoTranspose => n,
            Transpose::Transpose => m,
        };
        let y_len = match trans {
            Transpose::NoTranspose => m,
            Transpose::Transpose => n,
        };
        let x = ptr_2_vecref(x_len, x, incx);
        let y = ptr_2_vecmut(y_len, y, incy);

        gemv(trans, alpha, beta, a, x, y);
    }
}

/// BLAS ILP64 interface for LAK [gemv]
pub unsafe fn gemv_ilp64<T>(
    trans: u8,
    m: i64,
    n: i64,
    alpha: T,
    a: *const T,
    lda: i64,
    x: *const T,
    incx: i64,
    beta: T,
    y: *mut T,
    incy: i64,
) where
    T: AddAssign + Mul<Output = T> + MulAssign + SimdElement + Fma + Default + Add<Output = T>,

    Simd<T, N_ROWS_PER_CHUNK>:
        SimdFloat<Scalar = T> + AddAssign + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + Fma + AddAssign,
{
    unsafe {
        let trans = char_2_transpose(trans);
        let a = ptr_2_matref(m, n, a, lda);
        let x_len = match trans {
            Transpose::NoTranspose => n,
            Transpose::Transpose => m,
        };
        let y_len = match trans {
            Transpose::NoTranspose => m,
            Transpose::Transpose => n,
        };
        let x = ptr_2_vecref(x_len, x, incx);
        let y = ptr_2_vecmut(y_len, y, incy);

        gemv(trans, alpha, beta, a, x, y);
    }
}
