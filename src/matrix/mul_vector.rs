use crate::matrix::Matrix;
use crate::vector::Vector;
use std::ops::{Add, Mul};

// &Matrix<K> * &Vector<K>
impl<K, const N: usize, const M: usize> Mul<&Vector<K, N>> for &Matrix<K, N, M>
where
    K: Clone + Default + Add<Output = K> + for<'a> Mul<&'a K, Output = K>,
{
    type Output = Vector<K, M>;

    fn mul(self, vector: &Vector<K, N>) -> Self::Output {
        Vector::from_fn(|y| {
            (0..N).fold(Default::default(), |acc, x| {
                acc + self[x][y].clone() * &vector[x]
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat_times_vec() {
        let m = Matrix::from([[1., 0.], [0., 1.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(&m * &v, Vector::from([4., 2.]));

        let m = Matrix::from([[2., 0.], [0., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(&m * &v, Vector::from([8., 4.]));

        let m = Matrix::from([[2., -2.], [-2., 2.]]);
        let v = Vector::from([4., 2.]);
        assert_eq!(&m * &v, Vector::from([4., -4.]));

        let m = Matrix::from_row_major_order([
            [5.74, 1.46, 3.65],
            [7.64, 1.99, 1.83],
            [2.82, 4.48, 5.23],
            [2.9, 9.22, 8.11],
        ]);
        let v = Vector::from([4.31, 10.66, 2.03]);
        assert_eq!(&m * &v, Vector::from([47.7125, 57.8567, 70.5279, 127.2475]));

        let x = 42.;
        let y = 89.;
        let z = 53.;
        let m = Matrix::from_row_major_order([
            [1., 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ]);
        assert_eq!(
            Vector::from([x, y, z, 1.]),
            &m * &Vector::from([0., 0., 0., 1.])
        );
    }
}
