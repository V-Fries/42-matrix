use crate::matrix::Matrix;

impl<K: PartialEq, const X: usize, const Y: usize> PartialEq for Matrix<K, X, Y> {
    fn eq(&self, other: &Self) -> bool { return self.scalars == other.scalars; }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    // &Matrix<K> == &Matrix<K>
    #[test]
    fn equality() {
        let m1 = Matrix::new([[4, 324, 5], [43, 542, 54]]);
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
