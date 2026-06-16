use crate::traits::GemmDispatch;

use crate::blas::helpers::{char_2_transpose, ptr_2_matmut, ptr_2_matref};
use crate::l3::{dgemm, gemm, sgemm};
use crate::types::Transpose;

/// BLAS LP64 interface for LAK [gemm]
pub unsafe fn gemm_lp64<T>(
    transa: u8,
    transb: u8,
    m: i32,
    n: i32,
    k: i32,
    alpha: T,
    a: *const T,
    lda: i32,
    b: *const T,
    ldb: i32,
    beta: T,
    c: *mut T,
    ldc: i32,
) where
    T: GemmDispatch,
{
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        gemm(transa, transb, alpha, beta, a, b, c);
    }
}

/// BLAS ILP64 interface for LAK [gemm]
pub unsafe fn gemm_ilp64<T>(
    transa: u8,
    transb: u8,
    m: i64,
    n: i64,
    k: i64,
    alpha: T,
    a: *const T,
    lda: i64,
    b: *const T,
    ldb: i64,
    beta: T,
    c: *mut T,
    ldc: i64,
) where
    T: GemmDispatch,
{
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        gemm(transa, transb, alpha, beta, a, b, c);
    }
}

/// BLAS LP64 interface for LAK [sgemm]
/// Performance critical direct-[f32] variant. 
pub unsafe fn sgemm_lp64(
    transa: u8,
    transb: u8,
    m: i32,
    n: i32,
    k: i32,
    alpha: f32,
    a: *const f32,
    lda: i32,
    b: *const f32,
    ldb: i32,
    beta: f32,
    c: *mut f32,
    ldc: i32,
) {
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        sgemm(transa, transb, alpha, beta, a, b, c);
    }
}

/// BLAS ILP64 interface for LAK [sgemm]
/// Performance critical direct-[f32] variant. 
pub unsafe fn sgemm_ilp64(
    transa: u8,
    transb: u8,
    m: i64,
    n: i64,
    k: i64,
    alpha: f32,
    a: *const f32,
    lda: i64,
    b: *const f32,
    ldb: i64,
    beta: f32,
    c: *mut f32,
    ldc: i64,
) {
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        sgemm(transa, transb, alpha, beta, a, b, c);
    }
}

/// BLAS LP64 interface for LAK [dgemm]
/// Performance critical direct-[f64] variant. 
pub unsafe fn dgemm_lp64(
    transa: u8,
    transb: u8,
    m: i32,
    n: i32,
    k: i32,
    alpha: f64,
    a: *const f64,
    lda: i32,
    b: *const f64,
    ldb: i32,
    beta: f64,
    c: *mut f64,
    ldc: i32,
) {
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        dgemm(transa, transb, alpha, beta, a, b, c);
    }
}

/// BLAS ILP64 interface for LAK [dgemm]
/// Performance critical direct-[f64] variant. 
pub unsafe fn dgemm_ilp64(
    transa: u8,
    transb: u8,
    m: i64,
    n: i64,
    k: i64,
    alpha: f64,
    a: *const f64,
    lda: i64,
    b: *const f64,
    ldb: i64,
    beta: f64,
    c: *mut f64,
    ldc: i64,
) {
    unsafe {
        let transa = char_2_transpose(transa);
        let transb = char_2_transpose(transb);

        let (a_rows, a_cols) = match transa {
            Transpose::NoTranspose => (m, k),
            Transpose::Transpose => (k, m),
        };
        let (b_rows, b_cols) = match transb {
            Transpose::NoTranspose => (k, n),
            Transpose::Transpose => (n, k),
        };

        let a = ptr_2_matref(a_rows, a_cols, a, lda);
        let b = ptr_2_matref(b_rows, b_cols, b, ldb);
        let c = ptr_2_matmut(m, n, c, ldc);

        dgemm(transa, transb, alpha, beta, a, b, c);
    }
}
