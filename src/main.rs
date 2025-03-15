use linalg_lib::Matrix;
use linalg_lib:: matrix_ops;

fn main() {
    let a = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
    let b = Matrix::from_vec(2, 2, vec![8, 2, 5, 74]);
    let result = matrix_ops::add(&a, &b);

    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    println!("Result (A + B): {:?}", result);
}
