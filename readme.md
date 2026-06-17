# LAK

LAK is a small personal linear algebra crate with BLAS-like kernels over
contiguous Rust slices. Its goal was to see whether a safe, contiguous-only Rust
library can stay minimal and elegant without sacrificing much performance.
Benchmarks are available [here](https://devald.dev/notes/linalg-kernels/lak_8.pdf).

The API is organized around BLAS levels:

- `lak::l1`: vector-vector routines like `dot`, `axpy`, `scal`, and `nrm2`
- `lak::l2`: matrix-vector routines like `gemv`, `ger`, `trmv`, and `trsv`
- `lak::l3`: matrix-matrix routines like `gemm`

Matrices are column-major. The safe API uses `MatRef`, `MatMut`, `VecRef`, and
`VecMut` view types from `lak::types`. Only Level-3 `gemm` is implemented right
now, and it is currently optimized for smaller matrices, roughly under
`256 x 256`.

```rust
use lak::l3::gemm;
use lak::types::{MatMut, MatRef, Transpose};

// column-major 2 x 3 matrix:
// [1 3 5]
// [2 4 6]
let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

// column-major 3 x 2 matrix:
// [ 7 10]
// [ 8 11]
// [ 9 12]
let b = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];

let mut c = [0.0, 0.0, 0.0, 0.0];

let alpha = 1.0;
let beta = 0.0;

let a = MatRef::new(&a, (2, 3));
let b = MatRef::new(&b, (3, 2));
let mut c = MatMut::new(&mut c, (2, 2));

// c.reborrow() used to allow c.as_slice() afterwards.
gemm(
    Transpose::NoTranspose,
    Transpose::NoTranspose,
    alpha,
    beta,
    a,
    b,
    c.reborrow(),
);

// C = A * B
// [ 76 103]
// [100 136]
assert_eq!(c.as_slice(), &[76.0, 100.0, 103.0, 136.0]);
```

This crate currently targets nightly Rust because it uses `portable_simd`.
