use crate::{One, Vector};

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
    K: Default + One,
{
    pub fn scale(x: K, y: K, z: K) -> Self {
        Self::from([
            Vector::from([x, K::default(), K::default(), K::default()]),
            Vector::from([K::default(), y, K::default(), K::default()]),
            Vector::from([K::default(), K::default(), z, K::default()]),
            Vector::from([K::default(), K::default(), K::default(), K::ONE]),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scale() {
        let expected = Vector::from([43., 56., 432., 2.]);
        let v = Vector::from([54., 2., 1., 2.]);
        assert_eq!(
            expected,
            &Matrix::scale(expected[0] / v[0], expected[1] / v[1], expected[2] / v[2]) * &v
        );
    }
}
