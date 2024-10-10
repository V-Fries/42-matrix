use crate::matrix::Matrix;
use std::ops::{Mul, MulAssign};

// Matrix<K> *= &K
impl<K, const X: usize, const Y: usize> MulAssign<&K> for Matrix<K, X, Y>
where
    K: for<'a> MulAssign<&'a K>,
{
    fn mul_assign(&mut self, scalar: &K) {
        for x in 0..X {
            for y in 0..Y {
                self[x][y] *= scalar;
            }
        }
    }
}

// Matrix<K> * &K
impl<K, const X: usize, const Y: usize> Mul<&K> for Matrix<K, X, Y>
where
    K: for<'a> MulAssign<&'a K>,
{
    type Output = Matrix<K, X, Y>;

    fn mul(mut self, scalar: &K) -> Self::Output {
        self *= scalar;
        self
    }
}

// Matrix<K> *= K
impl<K, const X: usize, const Y: usize> MulAssign<K> for Matrix<K, X, Y>
where
    K: for<'a> MulAssign<&'a K>,
{
    fn mul_assign(&mut self, scalar: K) {
        *self *= &scalar;
    }
}

// Matrix<K> * K
impl<K, const X: usize, const Y: usize> Mul<K> for Matrix<K, X, Y>
where
    K: for<'a> MulAssign<&'a K>,
{
    type Output = Matrix<K, X, Y>;

    fn mul(mut self, scalar: K) -> Self::Output {
        self *= &scalar;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Matrix<K> *= K
    #[test]
    fn mul_assign_k() {
        let v = Matrix::from([[4., 2.], [23., 12.]]);
        let k = 5.;
        let mut result = v.clone();
        result *= &k;
        assert_eq!(v[0][0] * k, result[0][0]);
        assert_eq!(v[0][1] * k, result[0][1]);
        assert_eq!(v[1][0] * k, result[1][0]);
        assert_eq!(v[1][1] * k, result[1][1]);

        let mut result = v.clone();
        result *= k;
        assert_eq!(v[0][0] * k, result[0][0]);
        assert_eq!(v[0][1] * k, result[0][1]);
        assert_eq!(v[1][0] * k, result[1][0]);
        assert_eq!(v[1][1] * k, result[1][1]);
    }

    // Matrix<K> * K
    #[test]
    fn mul_k() {
        let v = Matrix::from([[4., 2.], [23., 12.]]);
        let k = 5.;
        let result = v.clone() * &k;
        assert_eq!(v[0][0] * k, result[0][0]);
        assert_eq!(v[0][1] * k, result[0][1]);
        assert_eq!(v[1][0] * k, result[1][0]);
        assert_eq!(v[1][1] * k, result[1][1]);

        let result = v.clone() * k;
        assert_eq!(v[0][0] * k, result[0][0]);
        assert_eq!(v[0][1] * k, result[0][1]);
        assert_eq!(v[1][0] * k, result[1][0]);
        assert_eq!(v[1][1] * k, result[1][1]);
    }
}
