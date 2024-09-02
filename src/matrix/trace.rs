use std::ops::Add;

use super::Matrix;

impl<K, const N: usize> Matrix<K, N, N> 
    where
        K: Default + for<'a> Add<&'a K, Output = K>
{
    pub fn trace(&self) -> K {
        (0..N).fold(K::default(), |acc, i| acc + &self[i][i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trace() {
        let u = Matrix::new([
            [1., 0.],
            [0., 1.],
        ]);
        assert_eq!(u.trace(), 2.);
        let u = Matrix::new([
            [2., -5., 0.],
            [4., 3., 7.],
            [-2., 3., 4.],
        ]);
        assert_eq!(u.trace(), 9.);
        let u = Matrix::new([
            [-2., -8., 4.],
            [1., -23., 4.],
            [0., 6., 4.],
        ]);
        assert_eq!(u.trace(), -21.);
    }
}
