use crate::blas::helpers::{ptr_2_vecmut, ptr_2_vecref};
use crate::l1::copy;

/// BLAS LP64 interface for LAK [copy()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn copy_lp64<T>(n: i32, x: *const T, incx: i32, y: *mut T, incy: i32)
where
    T: Copy,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        copy(x, y);
    }
}

/// BLAS ILP64 interface for LAK [copy()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn copy_ilp64<T>(n: i64, x: *const T, incx: i64, y: *mut T, incy: i64)
where
    T: Copy,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        copy(x, y);
    }
}
