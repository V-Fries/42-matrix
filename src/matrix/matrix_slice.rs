use super::Matrix;
use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct MatrixSlice<'a, K, const M: usize, const N: usize>{
    matrix: &'a mut Matrix<K, M, N>,
    x: usize,
    y: usize,
}

impl<'a, K, const M: usize, const N: usize> MatrixSlice<'a, K, M, N> {
    #[allow(dead_code)]
    pub fn new(matrix: &'a mut Matrix<K, M, N>, x: usize, y: usize) -> Self {
        Self {
            matrix,
            x,
            y
        }
    }

    #[allow(dead_code)]
    pub fn sub_slice(self, x: usize, y: usize) -> Self {
        Self {
            matrix: self.matrix,
            x: self.x + x,
            y: self.y + y,
        }
    }

    #[allow(dead_code)]
    pub fn get_x_size(&self) -> usize { M - self.x }

    #[allow(dead_code)]
    pub fn get_y_size(&self) -> usize { N - self.y }

    #[allow(dead_code)]
    pub fn apply_op_to_row(&mut self, mut row_index: usize, mut op: impl FnMut(&mut K)) {
        row_index += self.y;
        for x in self.x..M {
            op(&mut self.matrix[x][row_index]);
        }
    }
}

impl<K, const M: usize, const N: usize> MatrixSlice<'_, K, M, N> 
    where 
        K: Clone {
    #[allow(dead_code)]
    pub fn swap_rows(&mut self, mut row_1_index: usize, mut row_2_index: usize) {
        row_1_index += self.y;
        row_2_index += self.y;
        for x in self.x..M {
            let tmp = self.matrix[x][row_1_index].clone();
            self.matrix[x][row_1_index] = self.matrix[x][row_2_index].clone();
            self.matrix[x][row_2_index] = tmp;
        }
    }
}

// []
impl<K, const M: usize, const N: usize> Index<(usize, usize)> for MatrixSlice<'_, K, M, N> {
    type Output = K;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.matrix[self.x + x][self.y + y]
    }
}

impl<K, const M: usize, const N: usize> IndexMut<(usize, usize)> for MatrixSlice<'_, K, M, N> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[self.x + x][self.y + y]
    }
}

