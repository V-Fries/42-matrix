use std::ops::{Add, Mul};
use crate::matrix::Matrix;
use crate::vector::Vector;

// Maxtrix<K> * &Vector<K>
impl<K, const M: usize, const N: usize> Mul<&Vector<K, N>> for &Matrix<K, M, N>
    where
        K: Clone + Default + Add<Output=K> + for<'a> Mul<&'a K, Output=K> {
    type Output = Vector<K, M>;

    fn mul(self, vector: &Vector<K, N>) -> Self::Output {
        Vector::from_fn(|index| {
            self[index].iter()
                .zip(vector.iter())
                .fold(Default::default(), |result, (mat_elem, vec_elem)| {
                    result + mat_elem.clone() * vec_elem
                })
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mat_times_vec() {
        let m = Matrix::new([
            [1., 0.],
            [0., 1.],
        ]);
        let v = Vector::new([4., 2.]);
        assert_eq!(&(&m * &v), &Vector::new([4., 2.]));
        let m = Matrix::new([
            [2., 0.],
            [0., 2.],
        ]);
        let v = Vector::new([4., 2.]);
        assert_eq!(&(&m * &v), &Vector::new([8., 4.]));
        let m = Matrix::new([
            [2., -2.],
            [-2., 2.],
        ]);
        let v = Vector::new([4., 2.]);
        assert_eq!(&(&m * &v), &Vector::new([4., -4.]));
        let m = Matrix::new([
            [-4., 2.],
            [1., 4.],
            [5., -3.],
        ]);
        let v = Vector::new([4., 2.]);
        assert_eq!(&(&m * &v), &Vector::new([-12., 12., 14.]));
    }
}

