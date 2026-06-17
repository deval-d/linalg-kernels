use crate::blas::helpers::ptr_2_vecmut;
use crate::l1::swap;

/// BLAS LP64 interface for LAK [swap]
pub unsafe fn swap_lp64<T>(n: i32, x: *mut T, incx: i32, y: *mut T, incy: i32) {
    unsafe {
        let x = ptr_2_vecmut(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        swap(x, y);
    }
}

/// BLAS ILP64 interface for LAK [swap]
pub unsafe fn swap_ilp64<T>(n: i64, x: *mut T, incx: i64, y: *mut T, incy: i64) {
    unsafe {
        let x = ptr_2_vecmut(n, x, incx);
        let y = ptr_2_vecmut(n, y, incy);
        swap(x, y);
    }
}
