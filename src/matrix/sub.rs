use std::ops::{Sub, SubAssign};

use crate::matrix::Matrix;

// Matrix<K> -= &Matrix<K>
impl<K, const X: usize, const Y: usize> SubAssign<&Self> for Matrix<K, X, Y>
where
    K: for<'a> SubAssign<&'a K>,
{
    fn sub_assign(&mut self, other: &Self) {
        for x in 0..X {
            for y in 0..Y {
                self[x][y] -= &other[x][y];
            }
        }
    }
}

// Matrix<K> - &Matrix<K>
impl<K, const X: usize, const Y: usize> Sub<&Self> for Matrix<K, X, Y>
where
    K: for<'a> SubAssign<&'a K>,
{
    type Output = Self;

    fn sub(mut self, other: &Self) -> Self::Output {
        self -= other;
        self
    }
}

// Matrix<K> -= Matrix<K>
impl<K, const X: usize, const Y: usize> SubAssign<Self> for Matrix<K, X, Y>
where
    K: for<'a> SubAssign<&'a K>,
{
    fn sub_assign(&mut self, other: Self) {
        *self -= &other
    }
}

// Matrix<K> - Matrix<K>
impl<K, const X: usize, const Y: usize> Sub<Self> for Matrix<K, X, Y>
where
    K: for<'a> SubAssign<&'a K>,
{
    type Output = Self;

    fn sub(mut self, other: Self) -> Self::Output {
        self -= &other;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Matrix<K> -= Matrix<K>
    #[test]
    fn sub_assign_matrix() {
        let mut scalars_v1 = [[23., 21.], [43., 53.]];
        let scalars_v2 = [[5., 23.], [4., 2.]];
        let v1 = Matrix::from(scalars_v1.clone());
        let v2 = Matrix::from(scalars_v2.clone());
        sub_lists(&mut scalars_v1, &scalars_v2);

        let mut result = v1.clone();
        result -= &v2;
        assert_eq!(result[0][0], scalars_v1[0][0]);
        assert_eq!(result[0][1], scalars_v1[0][1]);
        assert_eq!(result[1][0], scalars_v1[1][0]);
        assert_eq!(result[1][1], scalars_v1[1][1]);

        result = v1;
        result -= v2;
        assert_eq!(result[0][0], scalars_v1[0][0]);
        assert_eq!(result[0][1], scalars_v1[0][1]);
        assert_eq!(result[1][0], scalars_v1[1][0]);
        assert_eq!(result[1][1], scalars_v1[1][1]);
    }

    // Matrix<K> - Matrix<K>
    #[test]
    fn sub_matrix() {
        let mut scalars_v1 = [[23., 21.], [43., 53.]];
        let scalars_v2 = [[5., 23.], [4., 2.]];
        let v1 = Matrix::from(scalars_v1.clone());
        let v2 = Matrix::from(scalars_v2.clone());
        sub_lists(&mut scalars_v1, &scalars_v2);

        let mut result = v1.clone() - &v2;
        assert_eq!(result[0][0], scalars_v1[0][0]);
        assert_eq!(result[0][1], scalars_v1[0][1]);
        assert_eq!(result[1][0], scalars_v1[1][0]);
        assert_eq!(result[1][1], scalars_v1[1][1]);

        result = v1 - v2;
        assert_eq!(result[0][0], scalars_v1[0][0]);
        assert_eq!(result[0][1], scalars_v1[0][1]);
        assert_eq!(result[1][0], scalars_v1[1][0]);
        assert_eq!(result[1][1], scalars_v1[1][1]);
    }

    fn sub_lists<K: for<'a> SubAssign<&'a K>, const X: usize, const Y: usize>(
        sub_list1: &mut [[K; Y]; X],
        sub_list2: &[[K; Y]; X],
    ) {
        for x in 0..X {
            for y in 0..Y {
                sub_list1[x][y] -= &sub_list2[x][y];
            }
        }
    }
}
