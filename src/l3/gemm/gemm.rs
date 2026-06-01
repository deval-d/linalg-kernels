// gemm.rs 

use crate::traits::GemmDispatch; 
use crate::types::{MatMut, MatRef, Transpose};
use crate::l3::gemm::gemm_nn::{sgemm_nn, dgemm_nn};

/// single precision general matrix-matrix multiplication.
///
/// C <- alpha * AB + beta * C 
///
/// args: 
/// * atrans: [Transpose] - whether to use A or A^T 
/// * btrans: [Transpose] - whether to use B or B^T 
/// * alpha: [f32] - scales AB 
/// * beta: [f32] - scales C 
/// * a: [MatRef] - matrix A 
/// * b: [MatRef] - matrix B 
/// * c: [MatMut] - matrix C
///
/// This is the direct [f32] entry point. Prefer this over generic [gemm] in
/// performance-critical code when the scalar type is already known.
#[inline(always)]
pub fn sgemm( 
    atrans: Transpose, 
    btrans: Transpose, 
    alpha: f32, 
    beta: f32, 
    a: MatRef<'_, f32>, 
    b: MatRef<'_, f32>, 
    c: MatMut<'_, f32>, 
) { 
    match (atrans, btrans) { 
        (Transpose::NoTranspose, Transpose::NoTranspose) => sgemm_nn(alpha, beta, a, b, c),
        (_, _) => unimplemented!(), 
    }
}

/// double precision general matrix-matrix multiplication.
///
/// C <- alpha * AB + beta * C 
///
/// args: 
/// * atrans: [Transpose] - whether to use A or A^T 
/// * btrans: [Transpose] - whether to use B or B^T 
/// * alpha: [f64] - scales AB 
/// * beta: [f64] - scales C 
/// * a: [MatRef] - matrix A 
/// * b: [MatRef] - matrix B 
/// * c: [MatMut] - matrix C
///
/// This is the direct [f64] entry point. Prefer this over generic [gemm] in
/// performance-critical code when the scalar type is already known.
#[inline(always)]
pub fn dgemm( 
    atrans: Transpose, 
    btrans: Transpose, 
    alpha: f64, 
    beta: f64, 
    a: MatRef<'_, f64>, 
    b: MatRef<'_, f64>, 
    c: MatMut<'_, f64>, 
) { 
    match (atrans, btrans) { 
        (Transpose::NoTranspose, Transpose::NoTranspose) => dgemm_nn(alpha, beta, a, b, c),
        (_, _) => unimplemented!(), 
    }
}

/// general matrix-matrix multiplication 
///
/// C <- alpha * AB + beta * C 
///
/// args: 
/// * atrans: [Transpose] - whether to use A or A^T 
/// * btrans: [Transpose] - whether to use B or B^T 
/// * alpha: T - scales AB 
/// * beta: T - scales C 
/// * a: [MatRef] - matrix A 
/// * b: [MatRef] - matrix B 
/// * c: [MatMut] - matrix C 
///
/// This generic wrapper dispatches to the appropriate scalar implementation.
/// For peak benchmarking or the most predictable codegen, call [sgemm] or
/// [dgemm] directly.
pub fn gemm<T>( 
    atrans: Transpose, 
    btrans: Transpose, 
    alpha: T, 
    beta: T, 
    a: MatRef<'_, T>, 
    b: MatRef<'_, T>, 
    c: MatMut<'_, T>, 
) 
where T: GemmDispatch
{ 
    T::gemm(atrans, btrans, alpha, beta, a, b, c);
}

