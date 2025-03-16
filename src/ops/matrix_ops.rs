use crate::matrix::Matrix;

pub fn identity<T: Copy + From<i32>>(n: usize) -> Matrix<T> {
    assert!(n > 0, "n must be greater than 0");

    let mut data = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            if i == j {
                data.push(T::from(1));
            } else {
                data.push(T::from(0));
            }
        }
    }

    return Matrix::from_vec(n, n, data)
}
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

pub fn transpose<T: Copy>(
    a: &Matrix<T>,
) -> Matrix<T> {
    let mut result_data: Vec<T> = Vec::with_capacity(a.data.len());

    for i in 0..a.cols {
        for j in 0..a.rows{
            result_data.push(a[(j, i)]);
        }
    }

    return Matrix::from_vec(a.cols, a.rows, result_data);
}

pub fn determinant<T>(
    a: &Matrix<T>
) -> T
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + From<i32>,
{
    assert_eq!(a.rows, a.cols, "matrix must be square");

    let n = a.rows;
    // 1x1 matrix
    if n == 1 {
        return a[(0, 0)];
    }
    //2x2 matrix
    if n == 2 {
        return a[(0, 0)] * a[(1, 1)] - a[(0, 1)] * a[(1, 0)];
    }

    // Recursive case
    let mut result: T = T::from(0);

    for i in 0..n {
        let mut sub_data = Vec::new();

        for row in 1..n {
            for col in 0..n {
                if col != i {
                    sub_data.push(a[(row, col)]);
                }
            }
        }

        let submatrix = Matrix::from_vec(n - 1, n - 1, sub_data);
        let sign = if i % 2 == 0 { T::from(1) } else { T::from(-1) };
        result = result + sign * a[(0, i)] * determinant(&submatrix);
    }

    return result;
}

pub fn inverse<T>(
    a: &Matrix<T>,
) -> Matrix<T> 
where
    T: Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::cmp::PartialEq
        + From<i32>, 
{
    assert_eq!(a.rows, a.cols, "Matrix must be square");

    let det: T = determinant(a);
    assert!(det != T::from(0), "Matrix is singular, det = 0");

    let n = a.rows;

    let mut cofactor_data = Vec::with_capacity(n * n);

    for i in 0..a.rows {
        for j in 0..a.cols{
            
            let mut minor_data: Vec<T> = Vec::new();

            for row in 0..n {
                if row == i {continue;}
                for col in 0..n {
                    if col == j {continue;}

                    minor_data.push(a[(row, col)]);
                }   
            }

            let minor_matrix = Matrix::from_vec(n-1, n-1, minor_data);
            let sign = if (j + i) % 2 == 0 {T::from(1)} else {T::from(-1)};
            let cofactor = sign * determinant(&minor_matrix);
            cofactor_data.push(cofactor);
        }
    }
    let cofactor_matrix = Matrix::from_vec(n, n, cofactor_data);
    let adjugate = transpose(&cofactor_matrix);

    //A^-1 = 1/det(A) * adjugate(A)
    let inverse_data = adjugate.data.iter().map(|&x| x / det).collect();

    return Matrix::from_vec(n, n, inverse_data);
}