use lak::l1::dot::dot;
use lak::types::VecRef;

#[inline(never)]
fn sdot(x: &[f32], y: &[f32]) -> f32 {
    let x = VecRef::new(x);
    let y = VecRef::new(y);

    dot(x, y)
}

fn main() {
    let x = vec![1.0f32; 1024];
    let y = vec![2.0f32; 1024];

    let out = sdot(&x, &y);

    std::hint::black_box(out);
}
