use lak::l1::dot;
use lak::traits::Fma;
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

    let out = dot_naive(&x, &y);
    let out2 = sdot(&x, &y); 

    std::hint::black_box(out);
    std::hint::black_box(out2);
}

#[inline(never)]
fn dot_naive( 
    x: &[f32], 
    y: &[f32], 
) -> f32 {
    let x = VecRef::new(x); 
    let y = VecRef::new(y); 

    let xdata = x.as_slice(); 
    let ydata = y.as_slice(); 

    let mut sum = 0.0; 
    for (&xi, &yi) in xdata.iter().zip(ydata.iter()) { 
        sum = xi.fma(yi, sum); 
    }

    sum
}

