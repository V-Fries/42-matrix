use std::ops::{DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::{Matrix, MatrixSlice};

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
    where
        K: Clone + ApproximatelyEqual + Default + for<'a> DivAssign<&'a K>
            + for<'a> Mul<&'a K, Output = K> + SubAssign
{
    pub fn row_echelon_form(mut self) -> Self {
        Self::_row_echelon_form(&mut MatrixSlice::new(&mut self, 0, 0));
        self
    }

    fn _row_echelon_form<'a>(matrix: &'a mut MatrixSlice<'a, K, M, N>) {
        let Some((first_non_zero_x, first_non_zero_y)) = Self::first_non_zero_index(matrix) else {
            return
        };
        let first_non_zero = matrix[(first_non_zero_x, first_non_zero_y)].clone();

        if first_non_zero_y != 0 { matrix.swap_rows(0, first_non_zero_y) }

        matrix.apply_op_to_row(0, |elem| *elem /= &first_non_zero );

        for y in 1..matrix.get_y_size() {
            let scale = matrix[(first_non_zero_x, y)].clone();
            if scale.clone().approximately_equal(&K::default()) {
                continue;
            }
            for x in first_non_zero_x..matrix.get_x_size() {
                let tmp = matrix[(x, 0)].clone() * &scale;
                matrix[(x, y)] -= tmp;
            }
        }
        Self::_row_echelon_form(&mut matrix.sub_slice(0, 1));
    }

    fn first_non_zero_index(matrix: &MatrixSlice<K, M, N>) -> Option<(usize, usize)> {
        for x in 0..matrix.get_x_size() {
            for y in 0..matrix.get_y_size() {
                if !matrix[(x, y)].clone().approximately_equal(&K::default()) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

