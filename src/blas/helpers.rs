use crate::types::{MatMut, MatRef, Transpose, Triangular, VecMut, VecRef};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub(crate) trait LAKInt: Copy {
    fn to_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
    fn assert_unit_stride(self);
}

impl LAKInt for i32 {
    fn to_usize(self) -> usize {
        assert!(
            self >= 0,
            "must be non-negative. strides must also be unit."
        );
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        assert!(value <= i32::MAX as usize, "value does not fit in i32.");
        value as i32
    }

    fn assert_unit_stride(self) {
        assert_eq!(self.to_usize(), 1, "LAK works with contiguous memory only.");
    }
}

impl LAKInt for i64 {
    fn to_usize(self) -> usize {
        assert!(
            self >= 0,
            "must be non-negative. strides must also be unit."
        );
        assert!(self <= usize::MAX as i64, "value does not fit in usize.");
        self as usize
    }

    fn from_usize(value: usize) -> Self {
        assert!(value <= i64::MAX as usize, "value does not fit in i64.");
        value as i64
    }

    fn assert_unit_stride(self) {
        assert_eq!(self.to_usize(), 1, "LAK works with contiguous memory only.");
    }
}

pub(crate) fn char_2_transpose(trans: u8) -> Transpose {
    match trans.to_ascii_uppercase() {
        b'N' => Transpose::NoTranspose,
        b'T' | b'C' => Transpose::Transpose,
        _ => panic!("trans must be N, T, or C."),
    }
}

pub(crate) fn char_2_triangular(uplo: u8) -> Triangular {
    match uplo.to_ascii_uppercase() {
        b'U' => Triangular::Upper,
        b'L' => Triangular::Lower,
        _ => panic!("uplo must be U or L."),
    }
}

pub(crate) fn char_2_diag(diag: u8) -> u8 {
    match diag.to_ascii_uppercase() {
        b'N' => b'N',
        b'U' => b'U',
        _ => panic!("diag must be N or U."),
    }
}

/// Converts a BLAS vector descriptor into a LAK
/// [VecRef] type.
///
/// args:
/// * n: [i32]/[i64] - vector length
/// * x: *const f32 - ptr to start of contiguous vector buffer
/// * incx: [i32]/[i64] - stride. must be equal to 1.
pub unsafe fn ptr_2_vecref<'a, I: LAKInt, T>(n: I, x: *const T, incx: I) -> VecRef<'a, T> {
    unsafe {
        incx.assert_unit_stride();

        let nu = n.to_usize();
        let xbuf: &'a [T] = from_raw_parts(x, nu);

        VecRef::new(xbuf)
    }
}

/// Converts a BLAS mutable vector descriptor into a LAK
/// [VecMut] type.
///
/// args:
/// * n: [i32]/[i64] - vector length
/// * x: *mut f32 - mutable ptr to start of contiguous vector buffer
/// * incx: [i32]/[i64] - stride. must be equal to 1.
pub unsafe fn ptr_2_vecmut<'a, I: LAKInt, T>(n: I, x: *mut T, incx: I) -> VecMut<'a, T> {
    unsafe {
        incx.assert_unit_stride();

        let nu = n.to_usize();
        let xbuf: &'a mut [T] = from_raw_parts_mut(x, nu);

        VecMut::new(xbuf)
    }
}

/// Converts a BLAS matrix descriptor into a LAK
/// [MatRef] type.
///
/// args:
/// * m: [i32]/[i64] - number of rows
/// * n: [i32]/[i64] - number of columns  
/// * a: *const f32 - ptr to start of contiguous matrix buffer
/// * lda: [i32]/[i64] - leading dimension. must be equal to m.
pub unsafe fn ptr_2_matref<'a, I: LAKInt, T>(m: I, n: I, a: *const T, lda: I) -> MatRef<'a, T> {
    unsafe {
        let mu = m.to_usize();
        let nu = n.to_usize();
        assert_eq!(lda.to_usize(), mu, "LAK works with contiguous memory only.");

        let abuf: &'a [T] = from_raw_parts(a, mu * nu);

        MatRef::new(abuf, (mu, nu))
    }
}

/// Converts a BLAS matrix descriptor into a LAK
/// [MatMut] type.
///
/// args:
/// * m: [i32]/[i64] - number of rows
/// * n: [i32]/[i64] number of columns  
/// * a: *mut f32 - mutable ptr to start of contiguous matrix buffer
/// * lda: [i32]/[i64] - leading dimension. must be equal to m.
pub unsafe fn ptr_2_matmut<'a, I: LAKInt, T>(m: I, n: I, a: *mut T, lda: I) -> MatMut<'a, T> {
    unsafe {
        let mu = m.to_usize();
        let nu = n.to_usize();
        assert_eq!(lda.to_usize(), mu, "LAK works with contiguous memory only.");

        let abuf: &'a mut [T] = from_raw_parts_mut(a, mu * nu);

        MatMut::new(abuf, (mu, nu))
    }
}
