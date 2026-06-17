#![feature(portable_simd)]

//! Linear algebra kernels.
//!
//! LAK is a personal project. Its guiding question is whether fully-safe Rust and
//! contiguous-memory-only BLAS routines can stay minimal and elegant without 
//! sacrificing much performance. Benchmarks can be found at
//! <https://devald.dev/notes/linalg-kernels/lak_8.pdf> 
//!
//! LAK provides small lightweight BLAS-like routines over contiguous Rust slices. 
//! Matrices are stored column-major.
//!
//! The main API is organized by BLAS level: 
//!
//! * [l1] - vector-vector routines like [l1::dot] and [l1::axpy] 
//! * [l2] - matrix-vector routines like [l2::gemv] and [l2::trsv] 
//! * [l3] - matrix-matrix routines like [l3::gemm] 
//!
//! Currently, [l3::gemm] is the only level-3 routine implemented. Additionally, LAK 
//! only accepts real [f32]/[f64] scalars. 
//!
//! GEMM also provides direct [f32] [l3::sgemm] and [f64] [l3::dgemm] for maximum 
//! performance. Level-1 and level-2 routines are all generic. 
//!
//! Example:
//! ```
//! use lak::l3::gemm;
//! use lak::types::{MatMut, MatRef, Transpose};
//!
//! // column-major 2 x 3 matrix:
//! // [1 3 5]
//! // [2 4 6]
//! let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
//!
//! // column-major 3 x 2 matrix:
//! // [ 7 10]
//! // [ 8 11]
//! // [ 9 12]
//! let b = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
//!
//! let mut c = [0.0, 0.0, 0.0, 0.0];
//!
//! let alpha = 1.0;
//! let beta = 0.0;
//!
//! let a = MatRef::new(&a, (2, 3));
//! let b = MatRef::new(&b, (3, 2));
//! let mut c = MatMut::new(&mut c, (2, 2));
//! 
//! // c.reborrow() used to allow c.as_slice() afterwards. 
//! gemm(
//!     Transpose::NoTranspose,
//!     Transpose::NoTranspose,
//!     alpha,
//!     beta,
//!     a,
//!     b,
//!     c.reborrow(),
//! );
//!
//! // C = A * B 
//! // [ 76 103] 
//! // [100 136] 
//! assert_eq!(c.as_slice(), &[76.0, 100.0, 103.0, 136.0]);
//! ```
//!
//! Data is passed through view types: 
//!
//! * [types::VecRef] and [types::VecMut] for vectors 
//! * [types::MatRef] and [types::MatMut] for column-major matrices
//!
//! The [blas] module provides unsafe BLAS-shaped LP64 and ILP64 wrapper functions.
//! These wrappers keep the historical BLAS argument order, but LAK only accepts 
//! contiguous vectors and matrices: 
//!
//! * vector increments must be `1` 
//! * matrix leading dimensions must be equal to the number of rows. 
//!
//! This crate currently uses `rustc 1.94.0-nightly` for `portable_simd`.  
//!
//! Informal notes from writing LAK can be found at <https://devald.dev>. 

pub mod types; 
pub mod traits; 
pub mod helpers;

pub mod l1; 
pub mod l2; 
pub mod l3; 
pub mod fused; 

pub mod blas; 
