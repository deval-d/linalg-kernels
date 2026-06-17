// types.rs 

use std::ops::Range;

/// enum for transpose ops 
/// * [Transpose::NoTranspose] for no-transpose ops
/// * [Transpose::Transpose] for transpose ops
#[derive(Clone, Copy, Debug)] 
pub enum Transpose { 
    NoTranspose, 
    Transpose, 
}

/// enum for triangular ops 
/// * [Triangular::Upper] for upper-triangular ops 
/// * [Triangular::Lower] for lower-triangular ops
#[derive(Clone, Copy, Debug)] 
pub enum Triangular { 
    Upper, 
    Lower, 
}

/// immutable vector type 
#[derive(Clone, Copy, Debug)]
pub struct VecRef<'a, T> { 
    buffer: &'a [T], 
}

/// mutable vector type
#[derive(Debug)]
pub struct VecMut<'a, T> { 
    buffer: &'a mut [T], 
}

/// immutable matrix type 
/// column major 
#[derive(Clone, Copy, Debug)]
pub struct MatRef<'a, T> { 
    buffer: &'a [T], 
    dimension: (usize, usize), 
}

/// mutable matrix type 
/// column major 
#[derive(Debug)]
pub struct MatMut<'a, T> { 
    buffer: &'a mut [T],
    dimension: (usize, usize), 
}

impl<'a, T> VecRef<'a, T> { 
    /// constructs [VecRef] with given slice 
    pub fn new(buffer: &'a [T]) -> Self { 
        Self { buffer }
    }

    /// returns length of internal slice 
    pub fn length(&self) -> usize { 
        self.buffer.len()
    }

    /// accesses full internal immutable slice 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer
    }

    /// accesses internal immutable slice over a given index range 
    pub fn slice(&self, range: Range<usize>) -> &[T] { 
        &self.buffer[range.start..range.end]
    }

    /// checks whether internal length is equal to given length parameter
    pub fn has_equal_length(&self, length: usize) -> bool { 
        self.buffer.len() == length
    }
}

impl<'a, T> VecMut<'a, T> { 
    /// constructs [VecMut] with given slice 
    pub fn new(buffer: &'a mut [T]) -> Self { 
        Self { buffer }
    }

    /// returns length of internal slice 
    pub fn length(&self) -> usize { 
        self.buffer.len()
    }

    /// accesses full internal slice as immutable 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer
    }

    /// accesses internal immutable slice over a given index range 
    pub fn slice(&self, range: Range<usize>) -> &[T] { 
        &self.buffer[range.start..range.end]
    }

    /// accesses internal mutable slice over a given index range 
    pub fn slice_mut(&mut self, range: Range<usize>) -> &mut [T] { 
        &mut self.buffer[range.start..range.end]
    }

    /// accesses full internal slice as mutable 
    pub fn as_slice_mut(&mut self) -> &mut [T] { 
        self.buffer
    }

    /// checks whether internal length is equal to given length parameter
    pub fn has_equal_length(&self, length: usize) -> bool { 
        self.buffer.len() == length
    }

    /// used for calling routines over and over again 
    /// on the stored internal mutable slice 
    ///
    /// borrows self mutably
    ///
    /// example:
    /// ```
    /// use lak::l1::scal;
    /// use lak::types::VecMut;
    ///
    /// let mut x = [1.0, 2.0, 3.0];
    /// let mut x = VecMut::new(&mut x);
    ///
    /// scal(2.0, x.reborrow());
    /// scal(3.0, x.reborrow());
    /// ```
    pub fn reborrow(&mut self) -> VecMut<'_, T> { 
        VecMut::new(self.as_slice_mut())
    }
}


impl<'a, T> MatRef<'a, T> { 
    /// constructs [MatRef] with given slice and (n_rows, n_cols) dimension
    ///
    /// example: 
    ///
    /// ``` 
    /// use lak::types::MatRef; 
    ///
    /// // col-major 2 x 3 matrix: 
    /// // [1 3 5] 
    /// // [2 4 6] 
    /// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 
    /// let a = MatRef::new(&a, (2, 3));
    /// ```
    pub fn new(buffer: &'a [T], dimension: (usize, usize)) -> Self { 
        let i = dimension.0; 
        let j = dimension.1; 
        let buffer_length = buffer.len(); 
        let matrix_length = i * j; 

        assert_eq!(
            matrix_length,
            buffer_length,
            "matrix has invalid dimensions: buffer length is {buffer_length}, \
             but dimensions are {i} x {j} = {matrix_length}",
        );

        Self { buffer, dimension }
    }

    /// accesses internal immutable slice 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer 
    }

    /// accesses matrix dimension 
    /// (n_rows, n_cols)
    pub fn dimension(&self) -> (usize, usize) { 
        self.dimension
    }

    /// accesses matrix number of rows 
    pub fn n_rows(&self) -> usize { 
        self.dimension.0
    }

    /// accesses matrix number of cols 
    pub fn n_cols(&self) -> usize { 
        self.dimension.1
    }

    /// return a [VecRef] for a given column in Self 
    pub fn col(&self, j: usize) -> VecRef<'_, T> {
        let n_rows = self.n_rows(); 
        let beg_idx = n_rows * j; 
        let end_idx = n_rows * (j + 1); 

        let slice = &self.buffer[beg_idx..end_idx]; 
        VecRef::new(slice)
    }

    /// return a [MatRef] for a contiguous column panel of Self
    ///
    /// example: 
    /// ``` 
    /// use lak::types::MatRef; 
    ///
    /// // [1 3 5] 
    /// // [2 4 6] 
    /// let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 
    /// let a = MatRef::new(&a, (2, 3)); 
    ///
    /// // MatRef of columns 1..3 
    /// // [3 5] 
    /// // [4 6] 
    /// let panel = a.col_panel(1..3); 
    /// ```
    pub fn col_panel(&self, cols: Range<usize>) -> MatRef<'_, T> { 
        debug_assert!(
            cols.start < cols.end,
            "start of col range must be < end of col range"
        );
        debug_assert!(
            cols.end <= self.dimension.1, 
            "end of col range must be <= number cols in Self"
        );

        let n_rows = self.n_rows(); 
        let n_cols = cols.end - cols.start; 
        let beg_idx = n_rows * cols.start; 
        let end_idx = n_rows * cols.end; 

        MatRef::new(
            &self.buffer[beg_idx..end_idx], 
            (n_rows, n_cols)
        )     
    }

    /// returns an [Iterator] over [MatRef]s containing column panels that 
    /// span Self. 
    ///
    /// each panel holds nc columns, and the last item is the leftover 
    /// panel with column width < nc 
    ///
    /// args: 
    /// * nc: [usize] - # cols in panel 
    ///
    /// returns: 
    /// * [Iterator] - over ([Range] of column idxs used in panel, [MatRef] of panel itself)
    pub fn col_panels(&self, nc: usize) -> impl DoubleEndedIterator<Item = (Range<usize>, MatRef<'_, T>)> { 
        debug_assert!(nc > 0, "nc must be > 0"); 

        let n_cols = self.n_cols(); 
        (0..n_cols).step_by(nc).map(move |j0| { 
            let j1 = usize::min(j0 + nc, n_cols); 

            (Range {start: j0, end: j1}, self.col_panel(j0..j1))
        })
    }

    /// checks whether matrix n_cols equals given length
    pub fn has_len_equal_to_n_cols(&self, length: usize) -> bool { 
        self.dimension.1 == length 
    }

    /// checks whether matrix n_rows equals given length 
    pub fn has_len_equal_to_n_rows(&self, length: usize) -> bool { 
        self.dimension.0 == length
    }
}

impl<'a, T> MatMut<'a, T> { 
    /// constructs [MatMut] with given slice and (n_rows, n_cols) dimension
    ///
    /// example: 
    /// ``` 
    /// use lak::types::MatMut; 
    ///
    /// // col-major 2 x 3 matrix: 
    /// // [1 3 5] 
    /// // [2 4 6] 
    /// let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 
    /// let a = MatMut::new(&mut a, (2, 3));
    /// ```
    pub fn new(buffer: &'a mut [T], dimension: (usize, usize)) -> Self { 
        let i = dimension.0; 
        let j = dimension.1; 
        let buffer_length = buffer.len(); 
        let matrix_length = i * j; 

        assert_eq!(
            matrix_length,
            buffer_length,
            "matrix has invalid dimensions: buffer length is {buffer_length}, \
             but dimensions are {i} x {j} = {matrix_length}"
        );

        Self { buffer, dimension }
    }

    /// accesses full internal immutable slice 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer 
    }

    /// accesses full internal slice as mutable 
    pub fn as_slice_mut(&mut self) -> &mut [T] { 
        self.buffer
    }

    /// accesses matrix dimension 
    /// (n_rows, n_cols)
    pub fn dimension(&self) -> (usize, usize) { 
        self.dimension
    }

    /// accesses matrix number of rows 
    pub fn n_rows(&self) -> usize { 
        self.dimension.0
    }

    /// accesses matrix number of cols 
    pub fn n_cols(&self) -> usize { 
        self.dimension.1
    }

    /// return a [VecRef] for a given column in Self 
    pub fn col(&self, j: usize) -> VecRef<'_, T> { 
        let n_rows = self.n_rows(); 
        let beg_idx = n_rows * j; 
        let end_idx = n_rows * (j + 1); 

        let slice = &self.buffer[beg_idx..end_idx]; 
        VecRef::new(slice)
    }

    /// return a [VecMut] for a given column in Self 
    pub fn col_mut(&mut self, j: usize) -> VecMut<'_, T> { 
        let n_rows = self.n_rows(); 
        let beg_idx = n_rows * j; 
        let end_idx = n_rows * (j + 1); 

        let slice = &mut self.buffer[beg_idx..end_idx]; 
        VecMut::new(slice)
    }

    /// return a [MatRef] for a contiguous column panel of Self 
    /// 
    /// contains full columns over a given a range of column indices.
    ///
    /// example: 
    /// ``` 
    /// use lak::types::{MatRef, MatMut}; 
    ///
    /// // [1 3 5] 
    /// // [2 4 6] 
    /// let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 
    /// let a = MatMut::new(&mut a, (2, 3)); 
    ///
    /// // MatRef of columns 1..3 
    /// // [3 5] 
    /// // [4 6] 
    /// let panel = a.col_panel(1..3); 
    /// ```
    pub fn col_panel(&self, cols: Range<usize>) -> MatRef<'_, T> { 
        debug_assert!(
            cols.start < cols.end,
            "start of col range must be < end of col range"
        );
        debug_assert!(
            cols.end <= self.dimension.1, 
            "end of col range must be <= number cols in Self"
        );

        let n_rows = self.n_rows(); 
        let n_cols = cols.end - cols.start; 
        let beg_idx = n_rows * cols.start; 
        let end_idx = n_rows * cols.end; 

        MatRef::new(
            &self.buffer[beg_idx..end_idx], 
            (n_rows, n_cols)
        )     
    }

    /// returns a [MatMut] for a contiguous column panel of Self 
    /// 
    /// contains full columns over a given a range of column indices. 
    ///
    /// example: 
    /// ``` 
    /// use lak::types::MatMut; 
    ///
    /// // [1 3 5] 
    /// // [2 4 6] 
    /// let mut a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; 
    /// let mut a = MatMut::new(&mut a, (2, 3)); 
    ///
    /// // MatMut of columns 1..3 
    /// // [3 5] 
    /// // [4 6] 
    /// let panel = a.col_panel_mut(1..3); 
    /// ```
    pub fn col_panel_mut(&mut self, cols: Range<usize>) -> MatMut<'_, T> { 
        debug_assert!(
            cols.start < cols.end,
            "start of col range must be < end of col range"
        );
        debug_assert!(
            cols.end <= self.dimension.1, 
            "end of col range must be <= number cols in Self"
        );

        let n_rows = self.n_rows(); 
        let n_cols = cols.end - cols.start; 
        let beg_idx = n_rows * cols.start; 
        let end_idx = n_rows * cols.end; 

        MatMut::new(
            &mut self.buffer[beg_idx..end_idx], 
            (n_rows, n_cols)
        )     
    }

    /// return an [Iterator] over [MatRef]s chunks containing column 
    /// panels that span Self. 
    ///
    /// each chunk holds nc columns, and the last item is the leftover 
    /// column panel with n_cols < nc 
    ///
    /// args: 
    /// * nc: [usize] - # cols in panel 
    ///
    /// returns: 
    /// * [Iterator] - over ([Range] of column idxs used in panel, [MatRef] of panel itself)
    pub fn col_panels(&self, nc: usize) -> impl DoubleEndedIterator<Item = (Range<usize>, MatRef<'_, T>)> { 
        debug_assert!(nc > 0, "nc must be > 0");         
        
        let n_cols = self.n_cols();
        (0..n_cols).step_by(nc).map(move |j0| { 
            let j1 = usize::min(j0 + nc, n_cols); 

            (Range {start: j0, end: j1}, self.col_panel(j0..j1))
        })
    }

    /// checks whether matrix n_cols equals given length
    pub fn has_len_equal_to_n_cols(&self, length: usize) -> bool { 
        self.dimension.1 == length 
    }

    /// checks whether matrix n_rows equals given length 
    pub fn has_len_equal_to_n_rows(&self, length: usize) -> bool { 
        self.dimension.0 == length
    }

    /// used for calling routines over and over again 
    /// on the stored internal mutable slice 
    ///
    /// borrows self mutably
    ///
    /// example:
    /// ```
    /// use lak::l2::ger;
    /// use lak::types::{MatMut, VecRef};
    ///
    /// let x = [1.0, 2.0];
    /// let y = [3.0, 4.0];
    /// let mut a = [0.0; 4];
    ///
    /// let x = VecRef::new(&x);
    /// let y = VecRef::new(&y);
    /// let mut a = MatMut::new(&mut a, (2, 2));
    ///
    /// ger(1.0, a.reborrow(), x, y);
    /// ger(1.0, a.reborrow(), x, y);
    /// ```
    pub fn reborrow(&mut self) -> MatMut<'_, T> { 
        let (n_rows, n_cols) = self.dimension();
        MatMut::new(self.as_slice_mut(), (n_rows, n_cols))
    }
}


/// asserts two [VecRef]/[VecMut] have equal length buffers
#[macro_export]
macro_rules! assert_length_eq {
    ($x:expr, $y:expr) => {
        assert!(
            $x.has_equal_length($y.length()),
            "number of elements must be equal"
        );
    };
}


/// asserts the length of a [VecRef]/[VecMut] buffer 
/// equals the number of cols in a [MatRef]/[MatMut] 
///
/// a.assert_length_eq_n_cols(x); 
#[macro_export]
macro_rules! assert_length_eq_n_cols {
    ($a:expr, $x:expr) => {
        assert!(
            $a.has_len_equal_to_n_cols($x.length()),
            "number of cols in a does not match length of x"
        );
    };
}

/// asserts the length of a [VecRef]/[VecMut] buffer 
/// equals the number of rows in a [MatRef]/[MatMut] 
///
/// a.assert_length_eq_n_rows(x); 
#[macro_export]
macro_rules! assert_length_eq_n_rows {
    ($a:expr, $x:expr) => {
        assert!(
            $a.has_len_equal_to_n_rows($x.length()),
            "number of rows in a does not match length of x"
        );
    };
}

