use rand::rngs::StdRng;
use rand::SeedableRng;
 
const BENCH_RNG_CASE: u64 = 0; 

#[allow(dead_code)]
pub const SIZES: &[usize] = &[
128, 256, 512, 768,
1024, 1536,
2048, 3072,
4096, 6144,
8192, 
];


#[allow(dead_code)] 
pub const MATRIX_SIZES_L2: &[usize] = &[ 
    128, 256, 512, 1024, 2048, 
]; 

#[allow(dead_code)] 
pub const MATRIX_SIZES_L3: &[usize] = &[ 
    16, 32, 64, 128, 256, 384, 512, 768, 1024, 1280, 1536, 1792
]; 

/// calculates 4 * (alpha * n^beta + gamma * n)
/// for single precision
#[allow(dead_code)]
pub fn bytes_count_f32(
    alpha: f32, 
    beta: i32,
    gamma: f32,
    n: f32,
) -> f32 {
    (std::mem::size_of::<f32>() as f32) * (alpha * n.powi(beta) + gamma * n) 
}

/// calculates 8 * (alpha * n^beta + gamma * n)
/// for double precision
#[allow(dead_code)]
pub fn bytes_count_f64(
    alpha: f64, 
    beta: i32,
    gamma: f64,
    n: f64,
) -> f64 {
    (std::mem::size_of::<f64>() as f64) * (alpha * n.powi(beta) + gamma * n) 
}


/// calculates alpha * n^beta + gamma * n
/// args: 
/// * alpha: [usize] - scalar multiplier in front of n^beta
/// * beta: [u32] - exponent of n 
/// * gamma: [usize] - scalar multiplier in front of n 
/// * n: [usize] - length of vector(s)
#[allow(dead_code)]
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

