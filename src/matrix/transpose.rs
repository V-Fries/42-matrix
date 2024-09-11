use super::Matrix;

impl<K, const M: usize, const N: usize> Matrix<K, M, N> 
    where
        K: Clone
{
    pub fn transpose(&self) -> Matrix<K, N, M> {
        Matrix::from_fn(|x, y| self[y][x].clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;    

    #[test]
    fn transpose() {
        let matrix = Matrix::new([
            [1, 2, 3],
            [4, 5, 6],
        ]);
        assert_eq!(matrix.transpose(), Matrix::new([
            [1, 4],
            [2, 5],
            [3, 6],
        ]));

        let matrix = Matrix::new([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
        assert_eq!(matrix.transpose(), Matrix::new([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]));

        let matrix = Matrix::new([
            [1, 2, 3],
        ]);
        assert_eq!(matrix.transpose(), Matrix::new([
            [1],
            [2],
            [3],
        ]));

        let matrix = Matrix::new([
            [1],
            [2],
            [3],
        ]);
        assert_eq!(matrix.transpose(), Matrix::new([
            [1, 2, 3],
        ]));

        let matrix: Matrix<i32, 0, 0> = Matrix::new([]);
        assert_eq!(matrix.transpose(), Matrix::new([]));

        let matrix = Matrix::new([
            [1, 2],
            [3, 4],
        ]);
        assert_eq!(matrix.transpose(), Matrix::new([
            [1, 3],
            [2, 4],
        ]));
    }
}
