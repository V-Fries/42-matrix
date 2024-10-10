use crate::sqrt::Sqrt;
use crate::vector::Vector;
use std::ops::{Add, DivAssign, Mul};

impl<K, const N: usize> Vector<K, N>
where
    K: Default
        + Clone
        + for<'a> Mul<&'a K, Output = K>
        + Add<Output = K>
        + Sqrt
        + for<'a> DivAssign<&'a K>,
{
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
        assert_eq!(Vector::from([3., 0.]).normalize(), Vector::from([1., 0.]));
        assert_eq!(Vector::from([0., 5.]).normalize(), Vector::from([0., 1.]));
    }
}
