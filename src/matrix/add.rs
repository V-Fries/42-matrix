use std::ops::{Add, AddAssign};

use crate::matrix::Matrix;

// Matrix<K> += &Matrix<K>
impl<K, const X: usize, const Y: usize> AddAssign<&Self> for Matrix<K, X, Y>
    where
        K: for<'a> AddAssign<&'a K> {
    fn add_assign(&mut self, other: &Self) {
        for x in 0..X {
            for y in 0..Y {
                self[x][y] += &other[x][y];
            }
        }
    }
}

// Matrix<K> + &Matrix<K>
impl<K, const X: usize, const Y: usize> Add<&Self> for Matrix<K, X, Y>
    where
        K: for<'a> AddAssign<&'a K> {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

// Matrix<K> += Matrix<K>
impl<K, const X: usize, const Y: usize> AddAssign<Self> for Matrix<K, X, Y>
    where
        K: for<'a> AddAssign<&'a K> {
    fn add_assign(&mut self, other: Self) { *self += &other }
}

// Matrix<K> + Matrix<K>
impl<K, const X: usize, const Y: usize> Add<Self> for Matrix<K, X, Y>
    where
        K: for<'a> AddAssign<&'a K> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += &other;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Matrix<K> += Matrix<K>
    #[test]
    fn add_assign_matrix() {
        let mut scalars_m1 = [[23., 21.], [43., 53.]];
        let scalars_m2 = [[5., 23.], [4., 2.]];
        let m1 = Matrix::from(scalars_m1.clone());
        let m2 = Matrix::from(scalars_m2.clone());
        add_lists(&mut scalars_m1, &scalars_m2);

        let mut result = m1.clone();
        result += &m2;
        assert_eq!(result[0][0], scalars_m1[0][0]);
        assert_eq!(result[0][1], scalars_m1[0][1]);
        assert_eq!(result[1][0], scalars_m1[1][0]);
        assert_eq!(result[1][1], scalars_m1[1][1]);

        result = m1.clone();
        result += m2;
        assert_eq!(result[0][0], scalars_m1[0][0]);
        assert_eq!(result[0][1], scalars_m1[0][1]);
        assert_eq!(result[1][0], scalars_m1[1][0]);
        assert_eq!(result[1][1], scalars_m1[1][1]);
    }

    // Matrix<K> + Matrix<K>
    #[test]
    fn add_matrix() {
        let mut scalars_m1 = [[23., 21.], [43., 53.]];
        let scalars_m2 = [[5., 23.], [4., 2.]];
        let m1 = Matrix::from(scalars_m1.clone());
        let m2 = Matrix::from(scalars_m2.clone());
        add_lists(&mut scalars_m1, &scalars_m2);

        let result = m1.clone() + &m2;
        assert_eq!(result[0][0], scalars_m1[0][0]);
        assert_eq!(result[0][1], scalars_m1[0][1]);
        assert_eq!(result[1][0], scalars_m1[1][0]);
        assert_eq!(result[1][1], scalars_m1[1][1]);

        let result = m1 + m2;
        assert_eq!(result[0][0], scalars_m1[0][0]);
        assert_eq!(result[0][1], scalars_m1[0][1]);
        assert_eq!(result[1][0], scalars_m1[1][0]);
        assert_eq!(result[1][1], scalars_m1[1][1]);
    }

    fn add_lists<K: for<'a> AddAssign<&'a K>, const X: usize, const Y: usize>(
        sub_list1: &mut [[K; Y]; X],
        sub_list2: &[[K; Y]; X],
    ) {
        for x in 0..X {
            for y in 0..Y {
                sub_list1[x][y] += &sub_list2[x][y];
            }
        }
    }
}
