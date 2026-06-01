// traits.rs 

use std::fmt::Display; 
use std::ops::{Add, Mul, Sub}; 
use std::simd::{LaneCount, Simd, StdFloat, SupportedLaneCount};

use crate::l3::gemm::gemm::{dgemm, sgemm};
use crate::types::{MatMut, MatRef, Transpose}; 

/// uses fma 
pub trait Fma {
    /// computes (self * a) + b 
    fn fma(self, b: Self, c: Self) -> Self; 
}

impl Fma for f32 { 
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self { 
        self.mul_add(b, c)
    }
}

impl Fma for f64 { 
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self { 
        self.mul_add(b, c)
    }
}

impl<const LANES: usize> Fma for Simd<f32, LANES> 
where 
    LaneCount<LANES>: SupportedLaneCount, 
{
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self { 
        self.mul_add(b, c) 
    }
}

impl<const LANES: usize> Fma for Simd<f64, LANES> 
where 
    LaneCount<LANES>: SupportedLaneCount, 
{
    /// computes (self * a) + b 
    #[inline(always)]
    fn fma(self, b: Self, c: Self) -> Self { 
        self.mul_add(b, c)
    }   
}

/// computes absolute value 
pub trait Abs { 
    fn abs(self) -> Self; 
}   

impl Abs for f32 { 
    /// computes absolute value of [f32]
    fn abs(self) -> Self { 
        f32::abs(self)
    }
}

impl Abs for f64 { 
    /// computes absolute value of [f64]
    fn abs(self) -> Self { 
        f64::abs(self)
    }
}


/// computes square root 
pub trait Sqrt { 
    fn sqrt(self) -> Self; 
}

impl Sqrt for f32 { 
    /// computes square root if [f32]
    fn sqrt(self) -> Self { 
        f32::sqrt(self)
    }
}

impl Sqrt for f64 { 
    /// computes square root if [f64]
    fn sqrt(self) -> Self { 
        f64::sqrt(self)
    }
}

/// returns max between 
pub trait Max { 
    fn max(self, other: Self) -> Self; 
}

impl Max for f64 { 
    /// computes max between two [f64]s 
    fn max(self, other: Self) -> Self { 
        if self >= other { 
            self 
        } else { 
            other
        }
    }
}


/// used in tests
pub trait TestFloat:
    Copy
    + PartialOrd
    + Sub<Output = Self>
    + Add<Output = Self>
    + Mul<Output = Self>
    + Abs
    + Max 
    + Display
{
    const RTOL: Self;
    const ATOL: Self;
}

impl TestFloat for f64 {
    const RTOL: Self = 1e-14;
    const ATOL: Self = 1e-14;
}



/// defines SIMD vector width in L1 routines 
pub trait SimdScalarL1: Copy { 
    const LANES: usize; 
}

impl SimdScalarL1 for f32 { 
    const LANES: usize = 32; 
}

impl SimdScalarL1 for f64 { 
    const LANES: usize = 16; 
}


pub trait GemmDispatch: Sized {
    fn gemm( 
        atrans: Transpose, 
        btrans: Transpose, 
        alpha: Self, 
        beta: Self, 
        a: MatRef<'_, Self>, 
        b: MatRef<'_, Self>, 
        c: MatMut<'_, Self>,
    ); 
}

impl GemmDispatch for f32 {
    fn gemm( 
        atrans: Transpose, 
        btrans: Transpose, 
        alpha: f32, 
        beta: f32, 
        a: MatRef<'_, f32>, 
        b: MatRef<'_, f32>, 
        c: MatMut<'_, f32>,      
    ) { 
        sgemm(atrans, btrans, alpha, beta, a, b, c);
    }
}

impl GemmDispatch for f64 { 
    fn gemm( 
        atrans: Transpose, 
        btrans: Transpose, 
        alpha: f64, 
        beta: f64, 
        a: MatRef<'_, f64>, 
        b: MatRef<'_, f64>, 
        c: MatMut<'_, f64>,      
    ) {
        dgemm(atrans, btrans, alpha, beta, a, b, c);
    }
}







