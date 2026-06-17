use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, SubAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Fma;

use crate::blas::helpers::{
    char_2_diag, char_2_transpose, char_2_triangular, ptr_2_matref, ptr_2_vecmut,
};
use crate::fused::ftrmsv::N_ROWS_PER_CHUNK;
use crate::l1::dot::LANES;
use crate::l2::trsv;

/// BLAS LP64 interface for LAK [trsv]
pub unsafe fn trsv_lp64<T>(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i32,
    a: *const T,
    lda: i32,
    x: *mut T,
    incx: i32,
) where
    T: Copy
        + AddAssign
        + SubAssign
        + DivAssign
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + SimdElement
        + Fma,

    Simd<T, N_ROWS_PER_CHUNK>:
        SimdFloat<Scalar = T> + AddAssign + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Fma,
{
    unsafe {
        let _diag = char_2_diag(diag);
        let uplo = char_2_triangular(uplo);
        let trans = char_2_transpose(trans);
        let a = ptr_2_matref(n, n, a, lda);
        let x = ptr_2_vecmut(n, x, incx);

        trsv(uplo, trans, a, x);
    }
}

/// BLAS ILP64 interface for LAK [trsv]
pub unsafe fn trsv_ilp64<T>(
    uplo: u8,
    trans: u8,
    diag: u8,
    n: i64,
    a: *const T,
    lda: i64,
    x: *mut T,
    incx: i64,
) where
    T: Copy
        + AddAssign
        + SubAssign
        + DivAssign
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + SimdElement
        + Fma,

    Simd<T, N_ROWS_PER_CHUNK>:
        SimdFloat<Scalar = T> + AddAssign + Mul<Output = Simd<T, N_ROWS_PER_CHUNK>> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Fma,
{
    unsafe {
        let _diag = char_2_diag(diag);
        let uplo = char_2_triangular(uplo);
        let trans = char_2_transpose(trans);
        let a = ptr_2_matref(n, n, a, lda);
        let x = ptr_2_vecmut(n, x, incx);

        trsv(uplo, trans, a, x);
    }
}
