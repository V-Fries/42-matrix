use std::ops::{AddAssign, Mul};

use super::Matrix;

impl<K, const M: usize, const N: usize, const P: usize> Mul<&Matrix<K, N, P>> for Matrix<K, M, N> 
where
    K: Default + for<'a> Mul<&'a K, Output=K> + AddAssign + Clone
{
    type Output = Matrix<K, M, P>;

    fn mul(self, rhs: &Matrix<K, N, P>) -> Self::Output {
        Self::Output::from_fn(|x| std::array::from_fn(|y| {
            let mut result = Default::default();
            for i in 0..N {
                result += self[x][i].clone() * &rhs[i][y];
            }
            result
        }))
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
    }
}
