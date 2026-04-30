// helpers.rs 

use rand::thread_rng; 
use rand::distributions::{Distribution, Standard}; 

/// makes a [Vec<T>] of given length with randomized entries 
#[allow(dead_code)]
pub fn make_vec_random<T>(
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
