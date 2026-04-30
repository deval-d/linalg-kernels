// common.rs

use std::ops::Sub;
use rand::thread_rng; 
use rand::distributions::{Distribution, Standard}; 

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

/// makes a [Vec<T>] of given length with randomized entries 
#[allow(dead_code)]
pub fn make_vec<T>(
    length: usize
) -> Vec<T>
where
    T: Default
        + Copy, 

    Standard: Distribution<T>, 
{ 
    assert!(length != 0, "len must be nonzero");

    let mut buffer = vec![T::default(); length]; 

    let mut rng = thread_rng(); 
    let distribution = Standard; 
    for value in buffer.iter_mut() { 
        *value = distribution.sample(&mut rng);  
    }

    buffer
}
