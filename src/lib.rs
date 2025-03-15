//matrix module avaible.
pub mod matrix;
pub mod utils;

pub mod ops {
    pub mod matrix_ops;
    pub mod vector_ops;
    pub mod tensor_ops;
}

pub use matrix::Matrix;
pub use ops::matrix_ops;   