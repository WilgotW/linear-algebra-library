#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>
}

//Functions for Matrix if it can be Copied
impl<T: Copy + Default> Matrix<T> { 
    //creates new Matrix with zeros.
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols, 
            data: vec![T::default(); rows * cols],
        }
    }
}

impl<T: Copy> Matrix<T>{
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        //check if matrix dimensions are equal to data amount. if not, panic
        assert_eq!(data.len(), rows * cols);
        Matrix { rows, cols, data }
    }
}