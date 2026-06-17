use std::ops::{AddAssign, Mul};

use crate::traits::Fma;

use crate::blas::helpers::{ptr_2_matmut, ptr_2_vecref};
use crate::l2::ger;

/// BLAS LP64 interface for LAK [ger()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn ger_lp64<T>(
    m: i32,
    n: i32,
    alpha: T,
    x: *const T,
    incx: i32,
    y: *const T,
    incy: i32,
    a: *mut T,
    lda: i32,
) where
    T: Copy + AddAssign + Mul<Output = T> + Fma,
{
    unsafe {
        let x = ptr_2_vecref(m, x, incx);
        let y = ptr_2_vecref(n, y, incy);
        let a = ptr_2_matmut(m, n, a, lda);

        ger(alpha, a, x, y);
    }
}

/// BLAS ILP64 interface for LAK [ger()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn ger_ilp64<T>(
    m: i64,
    n: i64,
    alpha: T,
    x: *const T,
    incx: i64,
    y: *const T,
    incy: i64,
    a: *mut T,
    lda: i64,
) where
    T: Copy + AddAssign + Mul<Output = T> + Fma,
{
    unsafe {
        let x = ptr_2_vecref(m, x, incx);
        let y = ptr_2_vecref(n, y, incy);
        let a = ptr_2_matmut(m, n, a, lda);

        ger(alpha, a, x, y);
    }
}
