use crate::sqrt::Sqrt;
use crate::vector::Vector;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

impl<K, const N: usize> Vector<K, N>
where
    K: Default
        + Clone
        + for<'a> Mul<&'a K, Output = K>
        + AddAssign
        + Add<Output = K>
        + Sqrt
        + Div<Output = K>
        + Sub<Output = K>,
{
    pub fn angle_cos(&self, v2: &Self) -> K {
        self * v2 / (self.norm() * &v2.norm())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::approximately_equal::assert_approximately_equal;

    #[test]
    fn angle_cos() {
        let v1 = Vector::from([1., 0.]);
        let v2 = Vector::from([1., 0.]);
        assert_approximately_equal(v1.angle_cos(&v2), 1.0);

        let v1 = Vector::from([1., 0.]);
        let v2 = Vector::from([0., 1.]);
        assert_approximately_equal(v1.angle_cos(&v2), 0.0);

        let v1 = Vector::from([-1., 1.]);
        let v2 = Vector::from([1., -1.]);
        assert_approximately_equal(v1.angle_cos(&v2), -1.0);

        let v1 = Vector::from([2., 1.]);
        let v2 = Vector::from([4., 2.]);
        assert_approximately_equal(v1.angle_cos(&v2), 1.0);

        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        assert_approximately_equal(v1.angle_cos(&v2), 0.974631846);
    }
}
