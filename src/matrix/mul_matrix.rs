use std::ops::{Add, Mul};

use super::Matrix;

impl<K, const M: usize, const N: usize, const P: usize> Mul<&Matrix<K, N, P>> for Matrix<K, M, N> 
where
    K: Default + for<'a> Mul<&'a K, Output=K> + Add<Output = K> + Clone
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: &Matrix<K, N, P>) -> Self::Output {
        Self::Output::from_fn(|x, y| {
            (0..N).fold(Default::default(), |acc, i| {
                acc + self[x][i].clone() * &rhs[i][y]
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_matrix_times_matrix() {
        let u = Matrix::new([
            [1., 0.],
            [0., 1.],
        ]);
        let v = Matrix::new([
            [1., 0.],
            [0., 1.],
        ]);
        assert_eq!(&(u * &v), &Matrix::new([
            [1., 0.],
            [0., 1.],
        ]));

        let u = Matrix::new([
            [1., 0.],
            [0., 1.],
        ]);
        let v = Matrix::new([
            [2., 1.],
            [4., 2.],
        ]);
        assert_eq!(&(u * &v), &Matrix::new([
            [2., 1.],
            [4., 2.],
        ]));

        let u = Matrix::new([
            [3., -5.],
            [6., 8.],
        ]);
        let v = Matrix::new([
            [2., 1.],
            [4., 2.],
        ]);
        assert_eq!(&(u * &v), &Matrix::new([
            [-14., -7.],
            [44., 22.],
        ]));

        let u = Matrix::new([
            [1., -2., 1.],
            [2., 1., 3.],
        ]);
        let v = Matrix::new([
            [2., 1.],
            [3., 2.],
            [1., 1.],
        ]);
        assert_eq!(&(u * &v), &Matrix::new([
            [-3., -2.],
            [10., 7.],
        ]));
    }
}
