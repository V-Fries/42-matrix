use crate::One;

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
    K: Default + One,
{
    pub fn translation(x: K, y: K, z: K) -> Self {
        Self::from_row_major_order([
            [K::ONE, K::default(), K::default(), x],
            [K::default(), K::ONE, K::default(), y],
            [K::default(), K::default(), K::ONE, z],
            [K::default(), K::default(), K::default(), K::ONE],
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Vector;

    #[test]
    fn translation() {
        let x = 24.;
        let y = 42.;
        let z = 89.;

        assert_eq!(
            Matrix::translation(x, y, z),
            Matrix::from_row_major_order([
                [1., 0., 0., x],
                [0., 1., 0., y],
                [0., 0., 1., z],
                [0., 0., 0., 1.],
            ])
        );

        assert_eq!(
            Vector::from([x, y, z, 1.]),
            &Matrix::translation(x, y, z) * &Vector::from([0., 0., 0., 1.])
        );
    }
}
