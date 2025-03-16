use linalg_lib::utils::pretty_print_matrix;
use linalg_lib::Matrix;
use linalg_lib::matrix_ops;

fn main() {
    let a = Matrix::from_vec(2, 3, vec![3, 4, 5, 1, 2, 7]);
    let b = Matrix::from_vec(3, 2, vec![1, 4, 2, 1, 3, 0]);
    let c = Matrix::from_vec(3, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    let d = Matrix::from_vec(3, 3, vec![1, 2, -1, -2, 0, 1, 1, -1, 0]);
    let result = matrix_ops::mat_mul(&a, &b);

    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("Result (A + B): {:?}", result);

    pretty_print_matrix("Transposed of A", &matrix_ops::transpose(&a));
    println!("determinant of C: {:?}", matrix_ops::determinant(&c));
    println!("inverse of D: {:?}", matrix_ops::inverse(&d));

    let identity_matrix: Matrix<i32> = matrix_ops::identity(4);
    pretty_print_matrix("Identity matrix of 4:", &identity_matrix);


}
