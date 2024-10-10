use crate::{approximately_equal::ApproximatelyEqual, matrix::Matrix};

impl<K, const X: usize, const Y: usize> PartialEq for Matrix<K, X, Y>
where
    K: Clone + ApproximatelyEqual,
{
    fn eq(&self, other: &Self) -> bool {
        for x in 0..X {
            for y in 0..Y {
                if !self[x][y].clone().approximately_equal(&other[x][y]) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    // &Matrix<K> == &Matrix<K>
    #[test]
    fn equality() {
        let m1 = Matrix::from([[4, 324, 5], [43, 542, 54]]);
        assert_eq!(m1 == m1.clone(), true);

        for x in 0..m1.get_x_size() {
            for y in 0..m1.get_y_size() {
                let mut m2 = m1.clone();
                m2[x][y] -= 1;
                assert_eq!(m1 == m2, false);
            }
        }
    }
}
