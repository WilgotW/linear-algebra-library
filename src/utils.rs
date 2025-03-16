use crate::matrix::Matrix;

pub fn pretty_print_matrix<T:std::fmt::Display>(title: &str, m: &Matrix<T>){
    println!("{}", title);
    for i in 0..m.rows {
        for j in 0..m.cols {
            print!("{} ", m[(i, j)]);
        }
        println!();
    }
}