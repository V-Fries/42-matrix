use std::ops::{AddAssign, Div, DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::Matrix;

impl<K, const N: usize> Matrix<K, N, N>
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
    pub fn determinant(self) -> K {
        if N == 0 {
            return K::default();
        }

        let m = self.row_echelon_form::<false>();
        (1..N).fold(m[0][0].clone(), |acc, i| acc * &m[i][i])
    }
}

#[cfg(test)]
mod test {
    use crate::approximately_equal::assert_approximately_equal;

    use super::*;

    #[test]
    fn determinant() {
        let u = Matrix::from_row_major_order([]);
        assert_approximately_equal(u.determinant(), 0.);

        let u = Matrix::from_row_major_order([[1., -1.], [-1., 1.]]);
        assert_approximately_equal(u.determinant(), 0.);

        let u = Matrix::from_row_major_order([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
        assert_approximately_equal(u.determinant(), 8.);

        let u = Matrix::from_row_major_order([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
        assert_approximately_equal(u.determinant(), -174.);

        let u = Matrix::from_row_major_order([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ]);
        assert_approximately_equal(u.determinant(), 1032.);

        // Test for a 1x1 matrix (edge case)
        let u = Matrix::from_row_major_order([[5.]]);
        assert_approximately_equal(u.determinant(), 5.);

        // Test for a 2x2 matrix with a non-zero determinant
        let u = Matrix::from_row_major_order([[3., 8.], [4., 6.]]);
        assert_approximately_equal(u.determinant(), -14.);

        // Test for a 2x2 singular matrix (determinant should be zero)
        let u = Matrix::from_row_major_order([[2., 4.], [1., 2.]]);
        assert_approximately_equal(u.determinant(), 0.);

        // Test for a 3x3 matrix with a known integer determinant
        let u = Matrix::from_row_major_order([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        assert_approximately_equal(u.determinant(), 0.);

        // Test for a 4x4 matrix with a known determinant
        let u = Matrix::from_row_major_order([
            [3., 2., 0., 1.],
            [4., 0., 1., 2.],
            [3., 0., 2., 1.],
            [9., 2., 3., 1.],
        ]);
        assert_approximately_equal(u.determinant(), 24.);

        // Test for a 5x5 matrix with all integers
        let u = Matrix::from_row_major_order([
            [1., 0., 2., -1., 3.],
            [3., 0., 0., 5., 6.],
            [2., 1., 4., -3., 2.],
            [4., 2., 0., 1., 7.],
            [1., 5., 2., 3., 4.],
        ]);
        assert_approximately_equal(u.determinant(), -840.);

        // Test for a 4x4 singular matrix (linearly dependent rows/columns)
        let u = Matrix::from_row_major_order([
            [2., -3., 1., 2.],
            [2., -3., 1., 2.],
            [4., 6., 8., 10.],
            [0., 0., 0., 0.],
        ]);
        assert_approximately_equal(u.determinant(), 0.);

        // Test for a 3x3 identity matrix (determinant should be 1)
        let u = Matrix::from_row_major_order([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_approximately_equal(u.determinant(), 1.);
    }
}
