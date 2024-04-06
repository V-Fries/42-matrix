use std::ops::{AddAssign, DivAssign, Mul};
use crate::sqrt::Sqrt;
use crate::vector::Vector;

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + for<'a> Mul<&'a K, Output=K> + AddAssign + Sqrt
        + for<'a> DivAssign<&'a K> {
    pub fn normalize(self) -> Self {
        let norm = self.norm();
        self / norm
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn norm() {
        assert_eq!(Vector::new([3., 0.]).normalize(), Vector::new([1., 0.]));
        assert_eq!(Vector::new([0., 5.]).normalize(), Vector::new([0., 1.]));
    }
}
