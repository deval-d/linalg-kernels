// types.rs 

/// immutable vector type 
#[derive(Clone, Copy)]
pub struct VecRef<'a, T> { 
    buffer: &'a [T], 
}

/// mutable vector type
pub struct VecMut<'a, T> { 
    buffer: &'a mut [T], 
}

impl<'a, T> VecRef<'a, T> { 
    pub fn new(buffer: &'a [T]) -> Self { 
        Self { buffer }
    }

    /// returns length of internal slice 
    pub fn length(&self) -> usize { 
        self.buffer.len()
    }

    /// accesses internal immutable slice 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer
    }

    /// checks whether internal length is equal to given length parameter
    pub fn has_equal_length(&self, length: usize) -> bool { 
        self.buffer.len() == length
    }
}

impl<'a, T> VecMut<'a, T> { 
    pub fn new(buffer: &'a mut [T]) -> Self { 
        Self { buffer }
    }

    /// returns length of internal slice 
    pub fn length(&self) -> usize { 
        self.buffer.len()
    }

    /// accesses internal slice as immutable 
    pub fn as_slice(&self) -> &[T] { 
        self.buffer
    }

    /// accesses internal slice as mutable 
    pub fn as_slice_mut(&mut self) -> &mut [T] { 
        self.buffer
    }

    /// checks whether internal length is equal to given length parameter
    pub fn has_equal_length(&self, length: usize) -> bool { 
        self.buffer.len() == length
    }
}


#[macro_export]
macro_rules! assert_length_eq {
    ($x:expr, $y:expr) => {
        assert!(
            $x.has_equal_length($y.length()),
            "number of logical elements must be equal"
        );
    };
}
