use crate::tensor::Tensor;

pub fn add<T: std::ops::Add<Output = T> + Copy>(
    t1: &Tensor<T>,
    t2: &Tensor<T>,
) -> Tensor<T> {
    assert_eq!(t1.shape, t2.shape, "Shape mismatch");
    let data = t1.data.iter().zip(&t2.data).map(|(&a, &b)| a + b).collect();
    Tensor { shape: t1.shape.clone(), data }
}

pub fn sub<T: std::ops::Sub<Output = T> + Copy>(
    t1: &Tensor<T>,
    t2: &Tensor<T>,
) -> Tensor<T> {
    assert_eq!(t1.shape, t2.shape, "Shape mismatch");
    let data = t1.data.iter().zip(&t2.data).map(|(&a, &b)| a - b).collect();
    Tensor { shape: t1.shape.clone(), data }
}