use std::ops::{Mul, MulAssign};

use crate::blas::helpers::ptr_2_vecmut;
use crate::l1::scal;

/// BLAS LP64 interface for LAK [scal]
pub unsafe fn scal_lp64<T>(n: i32, alpha: T, x: *mut T, incx: i32)
where
    T: Mul<Output = T> + Copy + MulAssign,
{
    unsafe {
        let x = ptr_2_vecmut(n, x, incx);
        scal(alpha, x);
    }
}

/// BLAS ILP64 interface for LAK [scal]
pub unsafe fn scal_ilp64<T>(n: i64, alpha: T, x: *mut T, incx: i64)
where
    T: Mul<Output = T> + Copy + MulAssign,
{
    unsafe {
        let x = ptr_2_vecmut(n, x, incx);
        scal(alpha, x);
    }
}
