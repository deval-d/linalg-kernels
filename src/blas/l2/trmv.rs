use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Fma;

use crate::blas::helpers::{
    char_2_diag, char_2_transpose, char_2_triangular, ptr_2_matref, ptr_2_vecmut,
};
use crate::fused::ftrmsv::N_ROWS_PER_CHUNK;
use crate::l1::dot::LANES;
use crate::l2::trmv;

/// BLAS LP64 interface for LAK [trmv()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn trmv_lp64<T>(
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
        + MulAssign
        + Mul<Output = T>
        + Add<Output = T>
        + Fma
        + SimdElement
        + Default,

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

        trmv(uplo, trans, a, x);
    }
}

/// BLAS ILP64 interface for LAK [trmv()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn trmv_ilp64<T>(
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
        + MulAssign
        + Mul<Output = T>
        + Add<Output = T>
        + Fma
        + SimdElement
        + Default,

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

        trmv(uplo, trans, a, x);
    }
}
