use std::ops::{Add, AddAssign};

use crate::matrix::Matrix;

// Matrix<K> += &Matrix<K>
impl<K: for<'a> AddAssign<&'a K> + Clone,
    const X: usize,
    const Y: usize> AddAssign<&Self> for Matrix<K, X, Y> {
    fn add_assign(&mut self, other: &Self) {
        for x in 0..X {
            for y in 0..Y {
                self[x][y] += &other[x][y];
            }
        }
    }
}

// Matrix<K> + &Matrix<K>
impl<K: for<'a> AddAssign<&'a K> + Clone,
    const X: usize,
    const Y: usize> Add<&Self> for Matrix<K, X, Y> {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    // Matrix<K> += &K
    #[test]
    fn add_assign_matrix() {
        let mut scalars_v1 = [[23., 21.], [43., 53.]];
        let scalars_v2 = [[5., 23.], [4., 2.]];
        let mut v1 = Matrix::new(scalars_v1.clone());
        let v2 = Matrix::new(scalars_v2.clone());
        v1 += &v2;

        add_lists(&mut scalars_v1, &scalars_v2);
        assert_eq!(v1[0][0], scalars_v1[0][0]);
        assert_eq!(v1[0][1], scalars_v1[0][1]);
        assert_eq!(v1[1][0], scalars_v1[1][0]);
        assert_eq!(v1[1][1], scalars_v1[1][1]);
    }

    // Matrix<K> + &K
    #[test]
    fn add_matrix() {
        let mut scalars_v1 = [[23., 21.], [43., 53.]];
        let scalars_v2 = [[5., 23.], [4., 2.]];
        let v1 = Matrix::new(scalars_v1.clone());
        let v2 = Matrix::new(scalars_v2.clone());
        let result = v1 + &v2;
        add_lists(&mut scalars_v1, &scalars_v2);
        assert_eq!(result[0][0], scalars_v1[0][0]);
        assert_eq!(result[0][1], scalars_v1[0][1]);
        assert_eq!(result[1][0], scalars_v1[1][0]);
        assert_eq!(result[1][1], scalars_v1[1][1]);
    }
}
