use std::mem::ManuallyDrop;

use super::Matrix;

impl<K, const M: usize, const N: usize> Matrix<K, M, N> {
    pub fn transpose(self) -> Matrix<K, N, M> {
        // Prevents automatic dropping to avoid double free since we use unsafe ptr::read below
        let scalars = ManuallyDrop::new(self);

        Matrix::from_fn(|x, y| unsafe { std::ptr::read(&scalars[y][x]) })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transpose() {
        let matrix = Matrix::from([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(matrix.transpose(), Matrix::from([[1, 4], [2, 5], [3, 6],]));

        let matrix = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(
            matrix.transpose(),
            Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1],])
        );

        let matrix = Matrix::from([[1, 2, 3]]);
        assert_eq!(matrix.transpose(), Matrix::from([[1], [2], [3],]));

        let matrix = Matrix::from([[1], [2], [3]]);
        assert_eq!(matrix.transpose(), Matrix::from([[1, 2, 3],]));

        let matrix: Matrix<i32, 0, 0> = Matrix::from([]);
        assert_eq!(matrix.transpose(), Matrix::from([]));

        let matrix = Matrix::from([[1, 2], [3, 4]]);
        assert_eq!(matrix.transpose(), Matrix::from([[1, 3], [2, 4],]));
    }
}
