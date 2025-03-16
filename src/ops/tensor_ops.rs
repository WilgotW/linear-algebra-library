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

pub fn mul_scalar<T: std::ops::Mul<Output = T> + Copy>(
    t: &Tensor<T>,
    scalar: T,
) -> Tensor<T> {
    let data = t.data.iter().map(|&x| x * scalar).collect();

    Tensor { shape: t.shape.clone(), data }
}

pub fn mul<T: std::ops::Mul<Output = T> + Copy>(
    t1: &Tensor<T>,
    t2: &Tensor<T>,
) -> Tensor<T> {
    assert_eq!(t1.shape, t2.shape, "Shape mismatch");
    let data = t1.data.iter().zip(&t2.data).map(|(&a, &b)| a * b).collect();

    Tensor { shape: t1.shape.clone(), data }
}

pub fn reshape<T: Copy>(
    t: &Tensor<T>,
    new_shape: Vec<usize>,
) -> Tensor<T> {
    let new_size: usize = new_shape.iter().product();
    assert_eq!(new_size, t.data.len(), "Invalid reshape size");

    Tensor { shape: new_shape, data: t.data.clone() }
}

pub fn broadcast<T: std::ops::Mul<Output = T> + Copy>(
    t1: &Tensor<T>,
    t2: &Tensor<T>,
) -> Tensor<T> {
    if t2.shape.is_empty() {
        let data = t1.data.iter().map(|&x| x * t2.data[0]).collect();
        return Tensor { shape: t1.shape.clone(), data };
    }

    panic!("Advanced broadcasting not yet implemented");
}

pub fn slice<T: Copy>(
    t: &Tensor<T>,
    index: usize,
) -> Tensor<T> {
    assert!(t.shape.len() >= 1, "Can not slice 0D tensor");

    let sub_shape = t.shape[1..].to_vec();
    let sub_len: usize = sub_shape.iter().product();
    let offset = index * sub_len;
    let data = t.data[offset..offset + sub_len].to_vec();

    Tensor { shape: sub_shape, data }
}