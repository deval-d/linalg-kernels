#[allow(dead_code)]
pub const SIZES: &[usize] = &[
    128, 256, 384, 512, 640, 768, 896, 1024,
    1152, 1280, 1408, 1536, 1664, 1792, 1920, 2048,
];

/// args: 
/// * alpha: [usize] - scalar multiplier (# of vectors)
/// * n: [usize] - length of vector(s)
pub fn bytes_read_f32(alpha: usize, n: usize) -> usize {
    alpha * n * std::mem::size_of::<f32>()
}
