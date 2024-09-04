use std::ops::{AddAssign, Div, DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::{Matrix, MatrixSlice};

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
    where
        K: Clone + ApproximatelyEqual + Default + for<'a> DivAssign<&'a K>
            + for<'a> Mul<&'a K, Output = K> + SubAssign + AddAssign 
            + for<'a> Div<&'a K, Output = K>
{
    pub fn row_echelon_form<const SHOULD_LEAD_WITH_ONES: bool>(mut self) -> Self {
        Self::_row_echelon_form::<SHOULD_LEAD_WITH_ONES>(MatrixSlice::new(&mut self, 0, 0));
        self
    }

    fn _row_echelon_form<const SHOULD_LEAD_WITH_ONES: bool>(mut matrix: MatrixSlice<'_, K, M, N>) {
        if matrix.get_x_size() == 0 {
            return;
        }

        let Some(first_non_zero) = Self::get_first_non_zero::<SHOULD_LEAD_WITH_ONES>(&mut matrix) else {
            Self::_row_echelon_form::<SHOULD_LEAD_WITH_ONES>(matrix.sub_slice(1, 0));
            return
        };

        for y in 1..matrix.get_y_size() {
            let scale = if SHOULD_LEAD_WITH_ONES {
                matrix[(0, y)].clone()
            } else {
                matrix[(0, y)].clone() / &first_non_zero
            };

            if scale.clone().approximately_equal(&K::default()) {
                continue;
            }
            for x in 0..matrix.get_x_size() {
                let tmp = matrix[(x, 0)].clone() * &scale;
                matrix[(x, y)] -= tmp;
            }
        }

        Self::_row_echelon_form::<SHOULD_LEAD_WITH_ONES>(matrix.sub_slice(1, 1));
    }

    fn get_first_non_zero<const SHOULD_LEAD_WITH_ONES: bool>(matrix: &mut MatrixSlice<K, M, N>) 
                                                             -> Option<K> {
        let first_non_zero_y = Self::first_non_zero_y_index(matrix)?;
        let first_non_zero = matrix[(0, first_non_zero_y)].clone();

        if first_non_zero_y != 0 { 
            for x in 0..matrix.get_x_size() {
                let tmp = if SHOULD_LEAD_WITH_ONES {
                    matrix[(x, first_non_zero_y)].clone() / &first_non_zero
                } else {
                    matrix[(x, first_non_zero_y)].clone()
                };
                matrix[(x, 0)] += tmp;
            }
        } else if SHOULD_LEAD_WITH_ONES {
            matrix.apply_op_to_row(0, |elem| *elem /= &first_non_zero);
        }
        Some(first_non_zero)
    }

    fn first_non_zero_y_index(matrix: &MatrixSlice<K, M, N>) -> Option<usize> {
        (0..matrix.get_y_size())
            .find(|y| !matrix[(0, *y)].clone().approximately_equal(&K::default()))
    }
}

