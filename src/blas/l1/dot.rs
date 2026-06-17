use std::ops::{Add, AddAssign, Mul};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Fma;

use crate::blas::helpers::ptr_2_vecref;
use crate::l1::dot;

const LANES: usize = 32;

/// BLAS LP64 interface for LAK [dot()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn dot_lp64<T>(n: i32, x: *const T, incx: i32, y: *const T, incy: i32) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Mul<Output = T> + Add<Output = T> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecref(n, y, incy);
        dot(x, y)
    }
}

/// BLAS ILP64 interface for LAK [dot()]
///
/// safety: uses `from_raw_parts`/`from_raw_parts_mut` with the given pointers and buffer lengths.
pub unsafe fn dot_ilp64<T>(n: i64, x: *const T, incx: i64, y: *const T, incy: i64) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Mul<Output = T> + Add<Output = T> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        let y = ptr_2_vecref(n, y, incy);
        dot(x, y)
    }
}
