use rand::rngs::StdRng;
use rand::SeedableRng;
 
const BENCH_RNG_CASE: u64 = 0; 

#[allow(dead_code)]
pub const SIZES: &[usize] = &[
    512, 1024, 1536, 2048, 4096, 8192, 16384, 32768, 65536, 
];

/// args: 
/// * alpha: [usize] - scalar multiplier (# of vectors)
/// * n: [usize] - length of vector(s)
pub fn bytes_read_f32(alpha: usize, n: usize) -> usize {
    alpha * n * std::mem::size_of::<f32>()
}

pub fn bench_rng(case: u64) -> StdRng { 
    StdRng::seed_from_u64(BENCH_RNG_CASE + case) 
}

