// errors.rs 

#[derive(Debug, PartialEq, Eq)]
pub enum MatErr { 
    InvalidDimension { buffer_length: usize, dimension: (usize, usize) } 
}

impl std::fmt::Display for MatErr { 
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self { 
            MatErr::InvalidDimension { buffer_length, dimension } => { 
                let i = dimension.0; 
                let j = dimension.1; 
                let required_length = i * j; 
                write!(f, "the buffer length {buffer_length} must match dimensions {i} x {j} = {required_length}")
            }, 
        }
    }
}
impl std::error::Error for MatErr {} 

