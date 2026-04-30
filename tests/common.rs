// common.rs

use std::ops::Sub;
use lak::traits::{Abs, Max, TestFloat}; 

pub fn assert_close<T>( 
    a: &[T], 
    b: &[T], 
) 
where 
    T: TestFloat 
        + Sub<Output = T> 
        + Abs
        + Max, 
{ 
    assert_eq!(a.len(), b.len());

    for (i, (&x, &y)) in a.iter().zip(b.iter()).enumerate() { 
        let delta = (x - y).abs(); 
        let tolerance = T::ATOL + T::RTOL * x.abs().max(y.abs()); 

        assert!( 
            delta <= tolerance, 
            "mismatch at idx {i}: {x} vs. {y} / δ={delta}, ϵ={tolerance}"
        ); 
    }
}
