use std::ops::{Add, Mul};

use super::Matrix;

impl<K, const N: usize, const M: usize, const P: usize> Mul<&Matrix<K, P, N>> for &Matrix<K, N, M>
where
    K: Default + for<'a> Mul<&'a K, Output = K> + Add<Output = K> + Clone,
{
    type Output = Matrix<K, P, M>;

    fn mul(self, rhs: &Matrix<K, P, N>) -> Self::Output {
        Self::Output::from_fn(|x, y| {
            (0..N).fold(Default::default(), |acc, i| {
                acc + self[i][y].clone() * &rhs[x][i]
            })
        })
    }
}

impl<K, const N: usize, const M: usize, const P: usize> Mul<Matrix<K, P, N>> for &Matrix<K, N, M>
where
    K: Default + for<'a> Mul<&'a K, Output = K> + Add<Output = K> + Clone,
{
    type Output = Matrix<K, P, M>;

    fn mul(self, rhs: Matrix<K, P, N>) -> Self::Output {
        self * &rhs
    }
}

impl<K, const N: usize, const M: usize, const P: usize> Mul<&Matrix<K, P, N>> for Matrix<K, N, M>
where
    K: Default + for<'a> Mul<&'a K, Output = K> + Add<Output = K> + Clone,
{
    type Output = Matrix<K, P, M>;

    fn mul(self, rhs: &Matrix<K, P, N>) -> Self::Output {
        &self * rhs
    }
}

impl<K, const N: usize, const M: usize, const P: usize> Mul<Matrix<K, P, N>> for Matrix<K, N, M>
where
    K: Default + for<'a> Mul<&'a K, Output = K> + Add<Output = K> + Clone,
{
    type Output = Matrix<K, P, M>;

    fn mul(self, rhs: Matrix<K, P, N>) -> Self::Output {
        &self * &rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_matrix_times_matrix() {
        let u = Matrix::from_row_major_order([[1., 0.], [0., 1.]]);
        let v = Matrix::from_row_major_order([[1., 0.], [0., 1.]]);
        assert_eq!(&u * &v, Matrix::from_row_major_order([[1., 0.], [0., 1.],]));

        let u = Matrix::from_row_major_order([[1., 0.], [0., 1.]]);
        let v = Matrix::from_row_major_order([[2., 1.], [4., 2.]]);
        assert_eq!(u * &v, Matrix::from_row_major_order([[2., 1.], [4., 2.],]));

        let u = Matrix::from_row_major_order([[3., -5.], [6., 8.]]);
        let v = Matrix::from_row_major_order([[2., 1.], [4., 2.]]);
        assert_eq!(
            &u * v,
            Matrix::from_row_major_order([[-14., -7.], [44., 22.],])
        );

        let u = Matrix::from_row_major_order([[1., -2., 1.], [2., 1., 3.]]);
        let v = Matrix::from_row_major_order([[2., 1.], [3., 2.], [1., 1.]]);
        assert_eq!(
            u * v,
            Matrix::from_row_major_order([[-3., -2.], [10., 7.],])
        );
    }
}
