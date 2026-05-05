use rand::rngs::StdRng;
use rand::SeedableRng;
 
const BENCH_RNG_CASE: u64 = 0; 

#[allow(dead_code)]
pub const SIZES: &[usize] = &[
    512, 1024, 1536, 2048, 4096, 8192, 16384, 32768, 65536, 
];

#[allow(dead_code)] 
pub const MATRIX_SIZES: &[usize] = &[ 
    128, 256, 512, 1024, 2048, 
]; 

/// calculates 4 * alpha * n^beta + gamma * n
/// args: 
/// * alpha: [usize] - scalar multiplier in front of n^beta
/// * beta: [u32] - exponent of n 
/// * gamma: [usize] - scalar multiplier in front of n 
/// * n: [usize] - length of vector(s)
pub fn bytes_count_f32(
    alpha: usize, 
    beta: u32,
    gamma: usize,
    n: usize
) -> usize {
    std::mem::size_of::<f32>() * (alpha * n.pow(beta) + gamma * n) 
}

/// calculates alpha * n^beta + gamma * n
/// args: 
/// * alpha: [usize] - scalar multiplier in front of n^beta
/// * beta: [u32] - exponent of n 
/// * gamma: [usize] - scalar multiplier in front of n 
/// * n: [usize] - length of vector(s)
pub fn flops_count( 
    alpha: usize, 
    beta: u32, 
    gamma: usize, 
    n: usize, 
) -> usize { 
    alpha * n.pow(beta) + gamma * n
}

pub fn bench_rng(case: u64) -> StdRng { 
    StdRng::seed_from_u64(BENCH_RNG_CASE + case) 
}

