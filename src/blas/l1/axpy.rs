use crate::traits::Fma;
use std::ops::{AddAssign, Mul};

use crate::blas::helpers::{ptr_2_vecmut, ptr_2_vecref};
use crate::l1::axpy;

/// BLAS LP64 interface for LAK [axpy()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn axpy_lp64<T>(n: i32, alpha: T, x: *const T, incx: i32, y: *mut T, incy: i32)
where
    T: Copy + AddAssign + Mul<Output = T> + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        axpy(alpha, x, y);
    }
}

/// BLAS ILP64 interface for LAK [axpy()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn axpy_ilp64<T>(n: i64, alpha: T, x: *const T, incx: i64, y: *mut T, incy: i64)
where
    T: Copy + AddAssign + Mul<Output = T> + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        axpy(alpha, x, y);
    }
}
