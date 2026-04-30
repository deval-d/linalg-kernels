// traits.rs 

use std::{fmt::Display, ops::{Add, Mul, Sub}}; 

/// computes absolute value 
pub trait Abs { 
    fn abs(self) -> Self; 
}   

impl Abs for f32 { 
    /// computes absolute value if [f32]
    fn abs(self) -> Self { 
        f32::abs(self)
    }
}

impl Abs for f64 { 
    /// computes absolute value if [f64]
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
    const RTOL: Self = 1e-12;
    const ATOL: Self = 1e-12;
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


