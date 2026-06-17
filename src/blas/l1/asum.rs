use std::ops::{Add, AddAssign};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::Abs;

use crate::blas::helpers::ptr_2_vecref;
use crate::l1::asum;

const LANES: usize = 32;

/// BLAS LP64 interface for LAK [asum]
pub unsafe fn asum_lp64<T>(n: i32, x: *const T, incx: i32) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Abs + Add<Output = T>,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Add<Output = Simd<T, LANES>>,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        asum(x)
    }
}

/// BLAS ILP64 interface for LAK [asum]
pub unsafe fn asum_ilp64<T>(n: i64, x: *const T, incx: i64) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Abs + Add<Output = T>,

    Simd<T, LANES>: SimdFloat<Scalar = T> + AddAssign + Add<Output = Simd<T, LANES>>,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        asum(x)
    }
}
