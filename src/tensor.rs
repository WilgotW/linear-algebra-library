use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor<T> {
    pub shape: Vec<usize>,
    pub data: Vec<T>,
}

impl<T: Copy> Tensor<T> {
    pub fn from_vec(shape: Vec<usize>, data: Vec<T>) -> Self {
        let expected_len: usize = shape.iter().product();
        assert_eq!(data.len(), expected_len, "Data size does not match shape");
        Tensor {shape, data}
    }

    pub fn get_dimensions(&self) -> usize {
        return self.shape.len();
    }

    pub fn total_size(&self) -> usize{
        return self.shape.iter().product();
    }
}

impl<T> Tensor<T> {
    fn flatten_index(&self, indicies: &[usize]) -> usize {
        assert_eq!(indicies.len(), self.shape.len(), "Incorecct number of indicies");

        let mut idx = 0;
        let mut stride = 1;

        for (i, &dim) in self.shape.iter().rev().enumerate() {
            let rev_i = self.shape.len() - 1 - i;
            idx += indicies[rev_i] * stride;
            stride *= dim;
        }

        return idx;
    }
}

impl<T> Index<&[usize]> for Tensor<T> {
    type Output = T;
    
    fn index(&self, index: &[usize]) -> &Self::Output {
        let flat = self.flatten_index(index);
        return &self.data[flat]
    }
}

impl<T> IndexMut<&[usize]> for Tensor<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let flat = self.flatten_index(index);
        &mut self.data[flat]
    }
}