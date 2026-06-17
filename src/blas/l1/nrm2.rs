use std::ops::{Add, AddAssign, Mul};
use std::simd::num::SimdFloat;
use std::simd::{Simd, SimdElement};

use crate::traits::{Fma, Sqrt};

use crate::blas::helpers::ptr_2_vecref;
use crate::l1::nrm2;

const LANES: usize = 32;

/// BLAS LP64 interface for LAK [nrm2]
pub unsafe fn nrm2_lp64<T>(n: i32, x: *const T, incx: i32) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Sqrt + Add<Output = T> + Mul<Output = T> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T>
        + AddAssign
        + Mul<Output = Simd<T, LANES>>
        + Add<Output = Simd<T, LANES>>
        + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        nrm2(x)
    }
}

/// BLAS ILP64 interface for LAK [nrm2]
pub unsafe fn nrm2_ilp64<T>(n: i64, x: *const T, incx: i64) -> T
where
    T: SimdElement + Copy + Default + AddAssign + Sqrt + Add<Output = T> + Mul<Output = T> + Fma,

    Simd<T, LANES>: SimdFloat<Scalar = T>
        + AddAssign
        + Mul<Output = Simd<T, LANES>>
        + Add<Output = Simd<T, LANES>>
        + Fma,
{
    unsafe {
        let x = ptr_2_vecref(n, x, incx);
        nrm2(x)
    }
}
