#![feature(portable_simd)]

use std::hint::black_box;
use std::simd::{Simd, StdFloat};

use lak::l1::scal::scal;
use lak::types::{MatRef, VecMut, VecRef};

const NC    : usize = 128;
const MR    : usize = 256;
const NR    : usize = 4;
const LANES : usize = 32;

// ------------------------------------------------------------
// Local helper: only used for leftover rows in faxpy_struct_f32_local.
// For n = 2048 with MR = 256 and LANES = 32, this should not matter.
// ------------------------------------------------------------
#[inline(never)]
pub fn axpy_slice_local(alpha: f32, x: &[f32], y: &mut [f32]) {
    debug_assert_eq!(x.len(), y.len());

    for i in 0..x.len() {
        y[i] += alpha * x[i];
    }
}

// ------------------------------------------------------------
// NEW KERNEL SHAPE:
// Local f32-only copy of your new full-column faxpy.
// Takes MatRef / VecRef / VecMut.
// ------------------------------------------------------------
#[inline(never)]
pub fn faxpy_struct_f32_local(
    alpha : f32,
    a     : MatRef<'_, f32>,
    x     : VecRef<'_, f32>,
    mut y : VecMut<'_, f32>,
) {
    let (n_rows, n_cols) = a.dimension();

    let a_slice = a.as_slice();
    let x_slice = x.as_slice();
    let y_slice = y.as_slice_mut();

    let mut i_base = 0;

    while i_base < n_rows {
        let mr = (n_rows - i_base).min(MR);

        let y_panel = &mut y_slice[i_base..i_base + mr];

        let (y_chunks, y_tail) = y_panel.as_chunks_mut::<LANES>();
        let n_chunks = y_chunks.len();

        if n_chunks > 0 {
            let mut j = 0;

            while j + NR <= n_cols {
                let x0 = x_slice[j];
                let x1 = x_slice[j + 1];
                let x2 = x_slice[j + 2];
                let x3 = x_slice[j + 3];

                let x0v = Simd::<f32, LANES>::splat(x0 * alpha);
                let x1v = Simd::<f32, LANES>::splat(x1 * alpha);
                let x2v = Simd::<f32, LANES>::splat(x2 * alpha);
                let x3v = Simd::<f32, LANES>::splat(x3 * alpha);

                let col0_beg = j * n_rows + i_base;
                let col0_end = col0_beg + mr;

                let col0 = &a_slice[col0_beg..col0_end];
                let col1 = &a_slice[col0_beg + n_rows..col0_end + n_rows];
                let col2 = &a_slice[col0_beg + 2 * n_rows..col0_end + 2 * n_rows];
                let col3 = &a_slice[col0_beg + 3 * n_rows..col0_end + 3 * n_rows];

                let (col0_chunks, _) = col0.as_chunks::<LANES>();
                let (col1_chunks, _) = col1.as_chunks::<LANES>();
                let (col2_chunks, _) = col2.as_chunks::<LANES>();
                let (col3_chunks, _) = col3.as_chunks::<LANES>();

                for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() {
                    let mut yv = Simd::<f32, LANES>::from_array(*y_chunk);

                    let a0 = Simd::<f32, LANES>::from_array(col0_chunks[chunk_idx]);
                    let a1 = Simd::<f32, LANES>::from_array(col1_chunks[chunk_idx]);
                    let a2 = Simd::<f32, LANES>::from_array(col2_chunks[chunk_idx]);
                    let a3 = Simd::<f32, LANES>::from_array(col3_chunks[chunk_idx]);

                    yv = a0.mul_add(x0v, yv);
                    yv = a1.mul_add(x1v, yv);
                    yv = a2.mul_add(x2v, yv);
                    yv = a3.mul_add(x3v, yv);

                    *y_chunk = yv.to_array();
                }

                j += NR;
            }

            while j < n_cols {
                let xj = alpha * x_slice[j];
                let xv = Simd::<f32, LANES>::splat(xj);

                let col = &a_slice[j * n_rows + i_base..j * n_rows + i_base + mr];
                let (col_chunks, _) = col.as_chunks::<LANES>();

                for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() {
                    let mut yv = Simd::<f32, LANES>::from_array(*y_chunk);
                    let av = Simd::<f32, LANES>::from_array(col_chunks[chunk_idx]);

                    yv = av.mul_add(xv, yv);

                    *y_chunk = yv.to_array();
                }

                j += 1;
            }
        }

        if !y_tail.is_empty() {
            for (j, &xj) in x_slice.iter().enumerate() {
                let col_beg = j * n_rows + n_chunks * LANES + i_base;
                let col_end = col_beg + y_tail.len();

                let av = &a_slice[col_beg..col_end];

                axpy_slice_local(xj * alpha, av, &mut *y_tail);
            }
        }

        i_base += mr;
    }
}

// ------------------------------------------------------------
// OLD KERNEL SHAPE:
// Local f32-only copy of old saxpyf_contiguous.
// Takes raw slices directly.
// x is already alpha-scaled.
// y is already beta-scaled.
// ------------------------------------------------------------
#[inline(never)]
pub fn saxpyf_contiguous_local(
    n_rows : usize,
    n_cols : usize,
    x      : &[f32],
    a      : &[f32],
    lda    : usize,
    y      : &mut [f32],
) {
    let mut row_base = 0;

    while row_base < n_rows {
        let mr = (n_rows - row_base).min(MR);

        let y_panel = &mut y[row_base..row_base + mr];

        let (y_chunks, y_tail) = y_panel.as_chunks_mut::<LANES>();
        let n_chunks = y_chunks.len();

        if n_chunks > 0 {
            let mut col = 0;

            while col + NR <= n_cols {
                let x0 = x[col];
                let x1 = x[col + 1];
                let x2 = x[col + 2];
                let x3 = x[col + 3];

                let x0v = Simd::<f32, LANES>::splat(x0);
                let x1v = Simd::<f32, LANES>::splat(x1);
                let x2v = Simd::<f32, LANES>::splat(x2);
                let x3v = Simd::<f32, LANES>::splat(x3);

                if x0 != 0.0 || x1 != 0.0 || x2 != 0.0 || x3 != 0.0 {
                    let col0 = &a[col * lda + row_base..col * lda + row_base + mr];
                    let col1 = &a[(col + 1) * lda + row_base..(col + 1) * lda + row_base + mr];
                    let col2 = &a[(col + 2) * lda + row_base..(col + 2) * lda + row_base + mr];
                    let col3 = &a[(col + 3) * lda + row_base..(col + 3) * lda + row_base + mr];

                    let (col0_chunks, _) = col0.as_chunks::<LANES>();
                    let (col1_chunks, _) = col1.as_chunks::<LANES>();
                    let (col2_chunks, _) = col2.as_chunks::<LANES>();
                    let (col3_chunks, _) = col3.as_chunks::<LANES>();

                    for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() {
                        let mut yv = Simd::<f32, LANES>::from_array(*y_chunk);

                        if x0 != 0.0 {
                            let a0 = Simd::<f32, LANES>::from_array(col0_chunks[chunk_idx]);
                            yv = a0.mul_add(x0v, yv);
                        }

                        if x1 != 0.0 {
                            let a1 = Simd::<f32, LANES>::from_array(col1_chunks[chunk_idx]);
                            yv = a1.mul_add(x1v, yv);
                        }

                        if x2 != 0.0 {
                            let a2 = Simd::<f32, LANES>::from_array(col2_chunks[chunk_idx]);
                            yv = a2.mul_add(x2v, yv);
                        }

                        if x3 != 0.0 {
                            let a3 = Simd::<f32, LANES>::from_array(col3_chunks[chunk_idx]);
                            yv = a3.mul_add(x3v, yv);
                        }

                        *y_chunk = yv.to_array();
                    }
                }

                col += NR;
            }

            while col < n_cols {
                let xj = x[col];

                if xj != 0.0 {
                    let col_beg = row_base + col * lda;
                    let col_end = col_beg + mr;

                    let col_slice = &a[col_beg..col_end];
                    let (col_chunks, _) = col_slice.as_chunks::<LANES>();

                    let xv = Simd::<f32, LANES>::splat(xj);

                    for (chunk_idx, y_chunk) in y_chunks.iter_mut().enumerate() {
                        let mut yv = Simd::<f32, LANES>::from_array(*y_chunk);
                        let av = Simd::<f32, LANES>::from_array(col_chunks[chunk_idx]);

                        yv = av.mul_add(xv, yv);

                        *y_chunk = yv.to_array();
                    }
                }

                col += 1;
            }
        }

        if !y_tail.is_empty() {
            let tail_beg = row_base + n_chunks * LANES;
            let tail_len = y_tail.len();

            for r in 0..tail_len {
                let row_idx = tail_beg + r;
                let mut acc = y[row_idx];

                for (col_idx, &xj) in x.iter().enumerate() {
                    if xj != 0.0 {
                        acc += xj * a[row_idx + col_idx * lda];
                    }
                }

                y[row_idx] = acc;
            }
        }

        row_base += mr;
    }
}

// ------------------------------------------------------------
// gemv1a:
// Your blocked new-API version:
//
// scal(beta, y)
// for each NC column panel:
//     faxpy_struct_f32_local(alpha, a_panel, x_panel, y)
// ------------------------------------------------------------
#[inline(never)]
pub fn gemv1_panel_faxpy_local(
    alpha : f32,
    beta  : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let a = MatRef::new(a_buf, (n, n));
    let x = VecRef::new(x_buf);
    let mut y = VecMut::new(y_buf);

    scal(beta, y.reborrow());

    for (cols, a_panel) in a.col_panels(NC) {
        let x_panel = VecRef::new(x.slice(cols));
        faxpy_struct_f32_local(alpha, a_panel, x_panel, y.reborrow());
    }
}

// ------------------------------------------------------------
// gemv1b:
// Same new faxpy kernel, but full matrix in one call.
// This removes a.col_panels(NC) from the call path.
// ------------------------------------------------------------
#[inline(never)]
pub fn gemv1_single_faxpy_local(
    alpha : f32,
    beta  : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let a = MatRef::new(a_buf, (n, n));
    let x = VecRef::new(x_buf);
    let mut y = VecMut::new(y_buf);

    scal(beta, y.reborrow());

    faxpy_struct_f32_local(alpha, a, x, y);
}

// ------------------------------------------------------------
// gemv2:
// Old-style packed path:
//
// xbuf = alpha * x
// ybuf = beta  * y
// saxpyf_contiguous_local(n, n, xbuf, a, lda=n, ybuf)
// copy back
// ------------------------------------------------------------
#[inline(never)]
pub fn gemv2_packed_saxpyf_local(
    alpha : f32,
    beta  : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let mut x_pack = Vec::with_capacity(n);
    let mut y_pack = Vec::with_capacity(n);

    for i in 0..n {
        x_pack.push(alpha * x_buf[i]);
        y_pack.push(beta * y_buf[i]);
    }

    saxpyf_contiguous_local(
        n,
        n,
        &x_pack,
        a_buf,
        n,
        &mut y_pack,
    );

    y_buf.copy_from_slice(&y_pack);
}

// ------------------------------------------------------------
// Direct hot-kernel callers.
// These are mostly here to force the functions to be used.
// Inspect the actual local kernels directly, not only these wrappers.
// ------------------------------------------------------------
#[inline(never)]
pub fn call_faxpy_struct_kernel_local(
    alpha : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let a = MatRef::new(a_buf, (n, n));
    let x = VecRef::new(x_buf);
    let y = VecMut::new(y_buf);

    faxpy_struct_f32_local(alpha, a, x, y);
}

#[inline(never)]
pub fn call_saxpyf_contiguous_kernel_local(
    n        : usize,
    a_buf    : &[f32],
    x_scaled : &[f32],
    y_scaled : &mut [f32],
) {
    saxpyf_contiguous_local(
        n,
        n,
        x_scaled,
        a_buf,
        n,
        y_scaled,
    );
}

// ------------------------------------------------------------
// Boring nonzero data generation.
// Avoids rand and avoids zero-guard weirdness.
// ------------------------------------------------------------
#[inline(never)]
fn fill_vec(n: usize, seed: f32) -> Vec<f32> {
    let mut v = Vec::with_capacity(n);

    for i in 0..n {
        let t = i as f32;
        v.push(seed + 0.001 * t.sin() + 0.0001 * t.cos());
    }

    v
}

const PANEL64_ROWS: usize = 64;
const PANEL64_COLS: usize = 4;
const PANEL64_NC: usize = 128;

// ------------------------------------------------------------
// Local axpy fallback for tails.
// For n = 2048 and NC = 128, this should not matter because:
// rows divisible by 64, cols divisible by 4.
// ------------------------------------------------------------
#[inline(never)]
pub fn axpy_slice_panel64_local(alpha: f32, x: &[f32], y: &mut [f32]) {
    debug_assert_eq!(x.len(), y.len());

    for i in 0..x.len() {
        y[i] += alpha * x[i];
    }
}

// ------------------------------------------------------------
// This is the local f32-only copy of your faster panel64 faxpy.
//
// It corresponds to:
// N_ROWS_PER_CHUNK = 64
// N_COLS_PER_CHUNK = 4
//
// Takes MatRef / VecRef / VecMut, same high-level shape as your real code.
// ------------------------------------------------------------
#[inline(never)]
pub fn faxpy_panel64_struct_f32_local(
    alpha : f32,
    a     : MatRef<'_, f32>,
    x     : VecRef<'_, f32>,
    mut y : VecMut<'_, f32>,
) {
    let (n_rows, n_cols) = a.dimension();

    let n_row_chunks = n_rows / PANEL64_ROWS;
    let n_col_chunks = n_cols / PANEL64_COLS;

    let a_slice = a.as_slice();
    let x_slice = x.as_slice();
    let y_slice = y.as_slice_mut();

    for col_chunk in 0..n_col_chunks {
        let j = col_chunk * PANEL64_COLS;

        let x1 = Simd::<f32, PANEL64_ROWS>::splat(alpha * x_slice[j]);
        let x2 = Simd::<f32, PANEL64_ROWS>::splat(alpha * x_slice[j + 1]);
        let x3 = Simd::<f32, PANEL64_ROWS>::splat(alpha * x_slice[j + 2]);
        let x4 = Simd::<f32, PANEL64_ROWS>::splat(alpha * x_slice[j + 3]);

        for row_chunk in 0..n_row_chunks {
            let i = row_chunk * PANEL64_ROWS;

            let y_beg = i;
            let y_end = i + PANEL64_ROWS;

            let col1_beg = j * n_rows + i;
            let col2_beg = (j + 1) * n_rows + i;
            let col3_beg = (j + 2) * n_rows + i;
            let col4_beg = (j + 3) * n_rows + i;

            let c1 = Simd::<f32, PANEL64_ROWS>::from_slice(
                &a_slice[col1_beg..col1_beg + PANEL64_ROWS],
            );

            let c2 = Simd::<f32, PANEL64_ROWS>::from_slice(
                &a_slice[col2_beg..col2_beg + PANEL64_ROWS],
            );

            let c3 = Simd::<f32, PANEL64_ROWS>::from_slice(
                &a_slice[col3_beg..col3_beg + PANEL64_ROWS],
            );

            let c4 = Simd::<f32, PANEL64_ROWS>::from_slice(
                &a_slice[col4_beg..col4_beg + PANEL64_ROWS],
            );

            let y_chunk = &mut y_slice[y_beg..y_end];
            let mut yv = Simd::<f32, PANEL64_ROWS>::from_slice(y_chunk);

            yv = c1.mul_add(x1, yv);
            yv = c2.mul_add(x2, yv);
            yv = c3.mul_add(x3, yv);
            yv = c4.mul_add(x4, yv);

            yv.copy_to_slice(y_chunk);
        }
    }

    let row_tail_beg = n_row_chunks * PANEL64_ROWS;
    let col_tail_beg = n_col_chunks * PANEL64_COLS;

    // leftover columns
    for j in col_tail_beg..n_cols {
        let a_vec = &a_slice[j * n_rows..j * n_rows + row_tail_beg];
        let y_vec = &mut y_slice[..row_tail_beg];

        axpy_slice_panel64_local(alpha * x_slice[j], a_vec, y_vec);
    }

    // leftover rows
    if row_tail_beg < n_rows {
        for j in 0..n_cols {
            let x_alpha = alpha * x_slice[j];

            let a_tail = &a_slice[j * n_rows + row_tail_beg..(j + 1) * n_rows];
            let y_tail = &mut y_slice[row_tail_beg..n_rows];

            axpy_slice_panel64_local(x_alpha, a_tail, y_tail);
        }
    }
}

// ------------------------------------------------------------
// Full gemv call path for this panel64 kernel:
//
// scal(beta, y)
// for each NC column panel:
//     faxpy_panel64_struct_f32_local(alpha, a_panel, x_panel, y)
// ------------------------------------------------------------
#[inline(never)]
pub fn gemv_panel64_local(
    alpha : f32,
    beta  : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let a = MatRef::new(a_buf, (n, n));
    let x = VecRef::new(x_buf);
    let mut y = VecMut::new(y_buf);

    scal(beta, y.reborrow());

    for (cols, a_panel) in a.col_panels(PANEL64_NC) {
        let x_panel = VecRef::new(x.slice(cols));

        faxpy_panel64_struct_f32_local(
            alpha,
            a_panel,
            x_panel,
            y.reborrow(),
        );
    }
}

// ------------------------------------------------------------
// Direct caller, only to force the symbol to exist.
// Inspect faxpy_panel64_struct_f32_local itself.
// ------------------------------------------------------------
#[inline(never)]
pub fn call_faxpy_panel64_kernel_local(
    alpha : f32,
    n     : usize,
    a_buf : &[f32],
    x_buf : &[f32],
    y_buf : &mut [f32],
) {
    let a = MatRef::new(a_buf, (n, n));
    let x = VecRef::new(x_buf);
    let y = VecMut::new(y_buf);

    faxpy_panel64_struct_f32_local(alpha, a, x, y);
}

fn main() {
    let n = black_box(2048usize);

    let alpha = black_box(0.1f32);
    let beta  = black_box(0.1f32);

    let a = fill_vec(n * n, 1.0);
    let x = fill_vec(n, 2.0);

    let mut y1 = fill_vec(n, 3.0);
    let mut y2 = y1.clone();
    let mut y3 = y1.clone();
    let mut y4 = y1.clone();
    let mut y5 = y1.clone();

    let mut x_scaled = x.clone();
    for xi in &mut x_scaled {
        *xi *= alpha;
    }

    let mut y_scaled = y1.clone();
    for yi in &mut y_scaled {
        *yi *= beta;
    }

    let mut y_panel64_direct = y1.clone();
    let mut y_panel64_gemv   = y1.clone();

    call_faxpy_panel64_kernel_local(
        black_box(alpha),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y_panel64_direct),
    );

    gemv_panel64_local(
        black_box(alpha),
        black_box(beta),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y_panel64_gemv),
    );

    black_box(&mut y_panel64_direct);
    black_box(&mut y_panel64_gemv);

    gemv1_panel_faxpy_local(
        black_box(alpha),
        black_box(beta),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y1),
    );

    gemv1_single_faxpy_local(
        black_box(alpha),
        black_box(beta),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y2),
    );

    call_faxpy_struct_kernel_local(
        black_box(alpha),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y3),
    );

    gemv2_packed_saxpyf_local(
        black_box(alpha),
        black_box(beta),
        black_box(n),
        black_box(&a),
        black_box(&x),
        black_box(&mut y4),
    );

    call_saxpyf_contiguous_kernel_local(
        black_box(n),
        black_box(&a),
        black_box(&x_scaled),
        black_box(&mut y_scaled),
    );

    black_box(&mut y1);
    black_box(&mut y2);
    black_box(&mut y3);
    black_box(&mut y4);
    black_box(&mut y5);
    black_box(&mut y_scaled);
}
