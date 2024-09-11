use std::ops::{AddAssign, Div, DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::Matrix;

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
    where
        K: Clone + ApproximatelyEqual + Default + for<'a> DivAssign<&'a K>
            + for<'a> Mul<&'a K, Output = K> + SubAssign + AddAssign 
            + for<'a> Div<&'a K, Output = K>
{
    pub fn row_echelon_form<const SHOULD_LEAD_WITH_ONES: bool>(self) -> Self {
        self._row_echelon_form::<SHOULD_LEAD_WITH_ONES>(&mut None)
    }

    pub(in crate::matrix) fn _row_echelon_form<const SHOULD_LEAD_WITH_ONES: bool>(
        mut self,
        inverse: &mut Option<&mut Self>,
    ) -> Self {
        let mut start_x = 0;
        let mut start_y = 0;

        while start_x < X && start_y < Y {
            let Some(first_non_zero) = self.get_first_non_zero::<SHOULD_LEAD_WITH_ONES>(
                start_x, start_y, inverse
            ) else {
                start_x += 1;
                continue;
            };

            for y in (start_y + 1)..Y {
                let scale = if SHOULD_LEAD_WITH_ONES {
                    self[start_x][y].clone()
                } else {
                    self[start_x][y].clone() / &first_non_zero
                };

                if scale.clone().approximately_equal(&K::default()) {
                    continue;
                }

                self.substract_row(start_x, start_y, y, &scale);
                if let Some(inverse) = inverse {
                    inverse.substract_row(0, start_y, y, &scale);
                }
            }
            start_y += 1;
            start_x += 1;
        }
        self
    }

    fn substract_row(&mut self, start_x: usize, start_y: usize, y: usize, scale: &K) {
        for x in start_x..X {
            let tmp = self[x][start_y].clone() * scale;
            self[x][y] -= tmp;
        }

    }

    fn get_first_non_zero<const SHOULD_LEAD_WITH_ONES: bool>(&mut self,
                                                             x: usize,
                                                             y: usize,
                                                             inverse: &mut Option<&mut Self>)
                                                             -> Option<K> {
        let first_non_zero_y = self.first_non_zero_y_index(x, y)?;
        let first_non_zero = self[x][first_non_zero_y].clone();

        if first_non_zero_y != y { 
            for x in x..X {
                let tmp = if SHOULD_LEAD_WITH_ONES {
                    self[x][first_non_zero_y].clone() / &first_non_zero
                } else {
                    self[x][first_non_zero_y].clone()
                };
                self[x][y] += tmp;
            }
            if let Some(inverse) = inverse {
                for x in 0..X {
                    let tmp = if SHOULD_LEAD_WITH_ONES {
                        inverse[x][first_non_zero_y].clone() / &first_non_zero
                    } else {
                        inverse[x][first_non_zero_y].clone()
                    };
                    inverse[x][y] += tmp;
                }
            }
        } else if SHOULD_LEAD_WITH_ONES {
            let op = |elem: &mut K| *elem /= &first_non_zero;
            self.apply_op_to_row(x, y, op);
            if let Some(inverse) = inverse { inverse.apply_op_to_row(0, y, op) };
        }
        Some(first_non_zero)
    }

    fn first_non_zero_y_index(&self, x: usize, y_start: usize) -> Option<usize> {
        (y_start..Y).find(|y| !self[x][*y].clone().approximately_equal(&K::default()))
    }
}

