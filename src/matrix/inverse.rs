use std::ops::{AddAssign, Div, DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use crate::one::One;

use super::Matrix;

impl<K, const N: usize> Matrix<K, N, N>
    where
        K: Clone + ApproximatelyEqual + Default + for<'a> DivAssign<&'a K>
            + for<'a> Mul<&'a K, Output = K> + SubAssign + AddAssign 
            + for<'a> Div<&'a K, Output = K> + One + PartialEq {
    #[allow(dead_code)]
    fn inverse(self) -> Result<Self, ()> {
        let mut inverse = Self::identity();
        let row_echelon = self._reduced_row_echelon_form(Some(&mut inverse));

        if row_echelon != Self::identity() {
            return Err(());
        }
        Ok(inverse)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invert_matrix() {
        // Test 1: Identity Matrix (3x3)
        let u = Matrix::from_row_major_order([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0],
        ])));

        // Test 2: Scalar Matrix (3x3)
        let u = Matrix::from_row_major_order([
            [2., 0., 0.],
            [0., 2., 0.],
            [0., 0., 2.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [0.5, 0.0, 0.0],
                    [0.0, 0.5, 0.0],
                    [0.0, 0.0, 0.5],
        ])));

        // Test 3: Random Invertible Matrix (3x3)
        let u = Matrix::from_row_major_order([
            [8., 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [0.649425287, 0.097701149, -0.655172414],
                    [-0.781609195, -0.126436782, 0.965517241],
                    [0.143678161, 0.074712644, -0.206896552],
        ])));

        // Test 4: Zero Matrix (3x3) (Non-invertible)
        let u = Matrix::from_row_major_order([
            [0., 0., 0.],
            [0., 0., 0.],
            [0., 0., 0.],
        ]);
        assert_eq!(u.inverse(), Err(()));

        // Test 5: Singular Matrix (3x3) (Determinant is zero, non-invertible)
        let u = Matrix::from_row_major_order([
            [1., 2., 3.],
            [4., 5., 6.],
            [7., 8., 9.],
        ]);
        assert_eq!(u.inverse(), Err(()));

        // Test 6: Negative Values in Matrix (Invertible)
        let u = Matrix::from_row_major_order([
            [-2., 2., 3.],
            [1., -1., 4.],
            [3., 2., -1.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [-7./55., 8./55., 1./5.],
                    [13./55., -7./55., 1./5.],
                    [1./11., 2./11., 0.],
        ])));

        // Test 7: 2x2 Matrix (Simple case)
        let u = Matrix::from_row_major_order([
            [4., 7.],
            [2., 6.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [0.6, -0.7],
                    [-0.2, 0.4],
        ])));

        // Test 8: Inverse of an Already Inverted Matrix (Should return the original matrix)
        let original = Matrix::from_row_major_order([
            [8., 5., -2.],
            [4., 7., 20.],
            [7., 6., 1.],
        ]);
        let inv = original.clone().inverse().unwrap();
        assert_eq!(inv.inverse(), Ok(original));

        // Test 10: Matrix with Fractions (Invertible)
        let u = Matrix::from_row_major_order([
            [0.5, 0.25, 0.],
            [0.25, 0.5, 0.],
            [0., 0., 1.],
        ]);
        assert_eq!(u.inverse(), Ok(Matrix::from_row_major_order([
                    [8./3., -4./3., 0.],
                    [-4./3., 8./3., 0.0],
                    [0.0, 0.0, 1.0],
        ])));
    }
}
