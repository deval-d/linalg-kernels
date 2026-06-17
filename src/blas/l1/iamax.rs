use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Abs;

use crate::blas::helpers::{ptr_2_vecref, LAKInt};
use crate::l1::iamax;

const LANES: usize = 16;

/// BLAS LP64 interface for LAK [iamax]
pub unsafe fn iamax_lp64<T>(n: i32, x: *const T, incx: i32) -> i32
where
    T: SimdElement + Copy + Default + PartialOrd + Abs,

    Simd<T, LANES>: SimdFloat<Scalar = T>,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        i32::from_usize(iamax(x))
    }
}

/// BLAS ILP64 interface for LAK [iamax]
pub unsafe fn iamax_ilp64<T>(n: i64, x: *const T, incx: i64) -> i64
where
    T: SimdElement + Copy + Default + PartialOrd + Abs,

    Simd<T, LANES>: SimdFloat<Scalar = T>,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        i64::from_usize(iamax(x))
    }
}
