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

#[allow(dead_code)]
pub fn make_behaved_mat_dd_f64(
    n: usize,
    rng: &mut StdRng,
) -> Vec<f64> {
    assert!(n != 0, "n must be nonzero");

    let mut a = vec![0.0f64; n * n];

    for j in 0..n {
        for i in j + 1..n {
            a[i + j * n] = rng.gen_range(-1.0..1.0);
        }
    }

    for i in 0..n {
        let mut row_sum = 0.0;

        for j in 0..i {
            row_sum += a[i + j * n].abs();
        }

        a[i + i * n] = row_sum + 1.0;
    }

    a
}
