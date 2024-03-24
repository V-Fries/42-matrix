use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Matrix<K, const X: usize, const Y: usize> {
    pub(in crate::matrix) scalars: [[K; Y]; X],
}

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y> {
    #[allow(dead_code)]
    pub fn new(scalars: [[K; Y]; X]) -> Matrix<K, X, Y> { Matrix { scalars } }

    #[allow(dead_code)]
    pub const fn is_square(&self) -> bool { X == Y }

    #[allow(dead_code)]
    pub const fn get_x_size(&self) -> usize { X }

    #[allow(dead_code)]
    pub const fn get_y_size(&self) -> usize { Y }
}

impl<K, const X: usize, const Y: usize> Index<usize> for Matrix<K, X, Y> {
    type Output = [K; Y];

    fn index(&self, index: usize) -> &Self::Output {
        &self.scalars[index]
    }
}

impl<K, const X: usize, const Y: usize> IndexMut<usize> for Matrix<K, X, Y> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.scalars[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let scalars = [[1, 2], [3, 4]];
        let matrix = Matrix::new(scalars.clone());
        assert_eq!(matrix.scalars, scalars);
    }

    #[test]
    fn is_square() {
        let matrix = Matrix::new([[1, 2], [3, 4]]);
        assert_eq!(matrix.is_square(), true);

        let matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix.is_square(), false);
    }

    #[test]
    fn get_y_and_get_x() {
        const X: usize = 1;
        const Y: usize = 2;

        let matrix = Matrix::<f32, X, Y>::new([[4.0f32, 4.0f32]]);
        assert_eq!(X, matrix.get_x_size());
        assert_eq!(Y, matrix.get_y_size());
    }

    #[test]
    fn index() {
        let scalars = [[1, 2], [3, 4]];
        let mut matrix = Matrix::new(scalars.clone());
        const INDEX_1: usize = 0;
        const INDEX_2: usize = 1;
        assert_eq!(scalars[INDEX_1][INDEX_2], matrix[INDEX_1][INDEX_2]);
        const NEW_VALUE: i32 = 42;
        matrix[INDEX_1][INDEX_2] = NEW_VALUE;
        assert_eq!(matrix[INDEX_1][INDEX_2], NEW_VALUE);
    }
}
