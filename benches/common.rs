#[allow(dead_code)]
pub const SIZES: &[usize] = &[
    512, 1024, 1536, 2048,
];

/// args: 
/// * alpha: [usize] - scalar multiplier (# of vectors)
/// * n: [usize] - length of vector(s)
pub fn bytes_read_f32(alpha: usize, n: usize) -> usize {
    alpha * n * std::mem::size_of::<f32>()
}
