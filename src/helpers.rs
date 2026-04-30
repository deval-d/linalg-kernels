// helpers.rs 

use rand::distributions::{Distribution, Standard}; 
use rand::rngs::StdRng; 
use rand::{Rng, SeedableRng}; 

pub const TEST_SEED: u64 = 676767; 

#[allow(dead_code)] 
pub fn test_rng(case: u64) -> StdRng { 
    StdRng::seed_from_u64(TEST_SEED + case)
}

#[allow(dead_code)] 
pub fn make_vec_random<T, R>( 
    length: usize,
    rng: &mut R, 
) -> Vec<T> 
where 
    T: Default + Copy, 
    R: Rng, 
    Standard: Distribution<T>,
{ 
    assert!(length != 0, "length must be nonzero"); 

    let mut buffer = vec![T::default(); length]; 
    let distribution = Standard; 

    for value in buffer.iter_mut() { 
        *value = distribution.sample(rng); 
    }

    buffer
}
