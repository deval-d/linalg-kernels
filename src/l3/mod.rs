#[path = "gemm/mod.rs"]
mod gemm_impl;

pub use gemm_impl::gemm::{dgemm, gemm, sgemm};
