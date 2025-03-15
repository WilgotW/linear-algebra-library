use linalg_lib::Matrix;

#[test]
fn test_new(){
    let m = Matrix::<f64>::new(2, 2);
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 2);
    assert_eq!(m.data, vec![0.0; 4]);
}