use crate::matrix::Matrix;

//Matrix add function, data must be able to be added and dublicated, returns new matrix
pub fn add <T: std::ops::Add<Output = T> + Copy>(
    a: &Matrix<T>, 
    b: &Matrix<T>,
) -> Matrix<T> {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);

    //avoid resizing the Vec by pre-allocating space for the result matrix
    let mut result_data: Vec<T> = Vec::with_capacity(a.data.len());

    for i in 0..a.data.len() {
        let sum = a.data[i] + b.data[i];
        result_data.push(sum);
    }

    return Matrix::from_vec(a.rows, a.cols, result_data)
}

pub fn sub <T: std::ops::Sub<Output = T> + Copy>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> Matrix<T> {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);

    let mut result_data: Vec<T> = Vec::with_capacity(a.data.len());

    for i in 0..a.data.len() {
        let sum = a.data[i] - b.data[i];
        result_data.push(sum);
    }

    return Matrix::from_vec(a.rows, a.cols, result_data)
}

pub fn mul_scalar <T: std::ops::Mul<Output = T> + Copy>(
    a: &Matrix<T>,
    scalar: T,
) -> Matrix<T> {

    let mut result_data: Vec<T> = Vec::with_capacity(a.data.len());

    for i in 0..a.data.len() {
        let new_val: T = a.data[i] * scalar;
        result_data.push(new_val);
    }

    return Matrix::from_vec(a.rows, a.cols, result_data)
}

pub fn dot_product<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Default>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> T {
    assert_eq!(a.rows, b.rows);
    assert_eq!(a.cols, b.cols);

    let mut result_data: T = T::default();

    for i in 0..a.data.len() {
        result_data = result_data + (a.data[i] * b.data[i]);
    }

    return result_data;
}

pub fn mat_mul<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy>(
    a: &Matrix<T>,
    b: &Matrix<T>,
) -> Matrix<T> {
    assert_eq!(a.cols, b.rows, "dimension missmatch");

    let mut result_data: Vec<T> = Vec::with_capacity(a.cols * b.rows);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = a[(i, 0)] * b[(0, j)];

            for k in 1..a.cols {
                sum = sum + (a[(i, k)] * b[(k, j)]);
            }

            result_data.push(sum);
        }
    }

    return Matrix::from_vec(a.rows, b.cols, result_data);
}