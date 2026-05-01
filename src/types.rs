// types.rs 

use crate::errors::MatErr; 

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
    dimension: (usize, usize), 
    buffer: &'a [T], 
}

/// mutable matrix type 
/// column major 
#[derive(Debug)]
pub struct MatMut<'a, T> { 
    dimension: (usize, usize), 
    buffer: &'a mut [T]
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

    /// accesses internal immutable slice 
    /// from idx0 to idx1 exclusive  
    pub fn specific_slice(&self, idx0: usize, idx1: usize) -> &[T] { 
        debug_assert!(idx0 < idx1, "idx0 must be less than idx1");

        &self.buffer[idx0..idx1]
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

    /// accesses full internal slice as mutable 
    pub fn as_slice_mut(&mut self) -> &mut [T] { 
        self.buffer
    }

    /// accesses internal slice as immutable 
    /// from idx0 to idx1 exclusive  
    pub fn specific_slice(&self, idx0: usize, idx1: usize) -> &[T] { 
        debug_assert!(idx0 < idx1, "idx0 must be less than idx1");

        &self.buffer[idx0..idx1]
    }

    /// accesses internal slice as mutable 
    /// from idx0 to indx1 exclusive
    pub fn specific_slice_mut(&mut self, idx0: usize, idx1: usize) -> &mut [T] { 
        debug_assert!(idx0 < idx1, "idx0 must be less than idx1"); 

        &mut self.buffer[idx0..idx1]
    }

    /// checks whether internal length is equal to given length parameter
    pub fn has_equal_length(&self, length: usize) -> bool { 
        self.buffer.len() == length
    }

    /// used for calling routines over and over again 
    /// on a mutable vector 
    pub fn reborrow(&mut self) -> VecMut<'_, T> { 
        VecMut::new(self.as_slice_mut())
    }
}


impl<'a, T> MatRef<'a, T> { 
    /// constructs [MatRef] with given slice and (i, j) dimension
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
    pub fn dimension(&self) -> (usize, usize) { 
        self.dimension
    }

    /// accesses matrix number of rows 
    pub fn n_rows(&self) -> usize { 
        self.dimension.0
    }

    /// accesses matrix number o cols 
    pub fn n_cols(&self) -> usize { 
        self.dimension.1
    }

    /// accesses internal immutable slice 
    /// from matrix element pos1 to pos2 exclusive 
    pub fn specific_slice(&self, pos1: (usize, usize), pos2: (usize, usize)) -> &[T] { 
        debug_assert!(
            pos1.0 * pos1.1 <= pos2.0 * pos2.1, 
            "pos1 must occur before pos2"
        );
        debug_assert!(
            pos1.0 <= self.dimension.0 && pos2.0 <= self.dimension.0,
            "must be a valid row <= # rows"
        ); 
        debug_assert!(
            pos1.1 <= self.dimension.1 && pos2.1 <= self.dimension.1,
            "must be a valid col <= # cols"
        ); 


        let beg_idx = pos1.0 + pos1.1 * self.dimension.0; 
        let end_idx = pos2.0 + pos2.1 * self.dimension.0; 

        &self.buffer[beg_idx..end_idx]
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
    /// constructs [MatRef] with given slice and (i, j) dimension
    pub fn new(buffer: &'a mut [T], dimension: (usize, usize)) -> Result<Self, MatErr> { 
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

        Ok(Self { buffer, dimension } )
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
    pub fn dimension(&self) -> (usize, usize) { 
        self.dimension
    }

    /// accesses matrix number of rows 
    pub fn n_rows(&self) -> usize { 
        self.dimension.0
    }

    /// accesses matrix number o cols 
    pub fn n_cols(&self) -> usize { 
        self.dimension.1
    }

    /// accesses internal slice as immutable
    /// from matrix element pos1 to pos2 exclusive 
    pub fn specific_slice(&self, pos1: (usize, usize), pos2: (usize, usize)) -> &[T] { 
        debug_assert!(
            pos1.0 * pos1.1 <= pos2.0 * pos2.1,
            "pos1 must occur before pos2"
        );
        debug_assert!(
            pos1.0 <= self.dimension.0 && pos2.0 <= self.dimension.0,
            "must be a valid row <= # rows"
        ); 
        debug_assert!(
            pos1.1 <= self.dimension.1 && pos2.1 <= self.dimension.1,
            "must be a valid col <= # cols"
        ); 


        let beg_idx = pos1.0 + pos1.1 * self.dimension.0; 
        let end_idx = pos2.0 + pos2.1 * self.dimension.0; 

        &self.buffer[beg_idx..end_idx]
    }

    /// accesses internal slice as mutable
    /// from matrix element pos1 to pos2 exclusive
    pub fn specific_slice_mut(&mut self, pos1: (usize, usize), pos2: (usize, usize)) -> &mut [T] { 
        debug_assert!(
            pos1.0 * pos1.1 <= pos2.0 * pos2.1,
            "pos1 must occur before pos2"
        );
        debug_assert!(
            pos1.0 < self.dimension.0 && pos2.0 < self.dimension.0,
            "must be a valid row < # rows"
        ); 
        debug_assert!(
            pos1.1 < self.dimension.1 && pos2.1 < self.dimension.1,
            "must be a valid col < # cols"
        ); 


        let beg_idx = pos1.0 + pos1.1 * self.dimension.0; 
        let end_idx = pos2.0 + pos2.1 * self.dimension.0; 

        &mut self.buffer[beg_idx..end_idx]
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

#[macro_export]
macro_rules! assert_length_eq {
    ($x:expr, $y:expr) => {
        assert!(
            $x.has_equal_length($y.length()),
            "number of elements must be equal"
        );
    };
}

#[macro_export]
macro_rules! assert_length_eq_n_cols {
    ($a:expr, $x:expr) => {
        assert!(
            $a.has_len_equal_to_n_cols($x.length()),
            "number of cols in a does not match length of x"
        );
    };
}

#[macro_export]
macro_rules! assert_length_eq_n_rows {
    ($a:expr, $x:expr) => {
        assert!(
            $a.has_len_equal_to_n_rows($x.length()),
            "number of rows in a does not match length of x"
        );
    };
}


