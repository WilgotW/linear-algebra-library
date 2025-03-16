use linalg_lib::Matrix;
use linalg_lib:: matrix_ops;

fn main() {
    let a = Matrix::from_vec(2, 3, vec![3, 4, 5, 1, 2, 7]);
    let b = Matrix::from_vec(3, 2, vec![1, 4, 2, 1, 3, 0]);
    let result = matrix_ops::mat_mul(&a, &b);

    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("Result (A + B): {:?}", result);
}
