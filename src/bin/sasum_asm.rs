use lak::l1::asum::asum;
use lak::types::VecRef;

#[inline(never)]
fn sasum(x: &[f32]) -> f32 {
    let x = VecRef::new(x);
    asum(x)
}

fn main() {
    let x = vec![1.0; 1024];

    let y = sasum(&x);

    std::hint::black_box(y);
}
