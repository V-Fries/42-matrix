use std::ops::{AddAssign, Div, DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::Matrix;

impl<K, const M: usize, const N: usize> Matrix<K, M, N>
where
    K: Clone
        + ApproximatelyEqual
        + Default
        + for<'a> DivAssign<&'a K>
        + for<'a> Mul<&'a K, Output = K>
        + SubAssign
        + AddAssign
        + for<'a> Div<&'a K, Output = K>,
{
    pub fn reduced_row_echelon_form(self) -> Self {
        self._reduced_row_echelon_form(None)
    }

    pub(in crate::matrix) fn _reduced_row_echelon_form(
        self,
        mut inverse: Option<&mut Self>,
    ) -> Self {
        let mut m = self._row_echelon_form::<true>(&mut inverse);

        let mut last_y = N;
        while let Some((next_one_x, next_one_y)) = Self::find_next_one(&m, last_y) {
            for y in 0..next_one_y {
                let scale = m[next_one_x][y].clone();

                m[next_one_x][y] = K::default();
                for x in (next_one_x + 1)..M {
                    let tmp = m[x][next_one_y].clone() * &scale;
                    m[x][y] -= tmp;
                }

                if let Some(inverse) = inverse.as_mut() {
                    for x in 0..M {
                        let tmp = inverse[x][next_one_y].clone() * &scale;
                        inverse[x][y] -= tmp;
                    }
                }
            }
            last_y = next_one_y;
        }
        m
    }

    fn find_next_one(&self, last_y: usize) -> Option<(usize, usize)> {
        if last_y == 1 {
            // TODO remove this and put 1 instead of 0 in the loop
            return None;
        }

        for y in (0..last_y).rev() {
            for x in 0..self.get_x_size() {
                if !self[x][y].clone().approximately_equal(&K::default()) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reduced_row_echelon_form() {
        let u = Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.],])
        );

        let u = Matrix::from_row_major_order([[1., 2.], [3., 4.]]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 0.], [0., 1.],])
        );

        let u = Matrix::from_row_major_order([[1., 2.], [2., 4.]]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 2.], [0., 0.],])
        );

        let u = Matrix::from_row_major_order([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::from_row_major_order([
                [1.0, 0.625, 0.0, 0.0, -12.1666667],
                [0.0, 0.0, 1.0, 0.0, -3.6666667],
                [0.0, 0.0, 0.0, 1.0, 29.5],
            ])
        );

        let u = Matrix::<i32, 1, 0>::from([[]]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::<i32, 1, 0>::from([[]])
        );

        let u = Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.],])
        );

        let zero_matrix = Matrix::from_row_major_order([[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]]);
        assert_eq!(
            zero_matrix.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[0., 0., 0.], [0., 0., 0.], [0., 0., 0.],])
        );

        let already_rref = Matrix::from_row_major_order([[1., 2., 0.], [0., 0., 1.], [0., 0., 0.]]);
        assert_eq!(
            already_rref.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 2., 0.], [0., 0., 1.], [0., 0., 0.],])
        );

        let rectangular_matrix = Matrix::from_row_major_order([[1., 2., 3.], [4., 5., 6.]]);
        assert_eq!(
            rectangular_matrix.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 0., -1.], [0., 1., 2.],])
        );

        let row_op_matrix =
            Matrix::from_row_major_order([[1., 2., -1.], [2., 3., -1.], [-1., -1., 1.]]);
        assert_eq!(
            row_op_matrix.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.],])
        );

        let larger_matrix = Matrix::from_row_major_order([
            [2., 1., -1., -2.],
            [-3., -1., 2., 3.],
            [-2., 1., 2., 1.],
            [1., 2., -1., -1.],
        ]);
        assert_eq!(
            larger_matrix.reduced_row_echelon_form(),
            Matrix::from_row_major_order([
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ])
        );

        let single_row = Matrix::from_row_major_order([[3., 6., 9.]]);
        assert_eq!(
            single_row.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1., 2., 3.],])
        );

        let single_column = Matrix::from_row_major_order([[4.], [8.], [12.]]);
        assert_eq!(
            single_column.reduced_row_echelon_form(),
            Matrix::from_row_major_order([[1.], [0.], [0.],])
        );
    }
}
