use std::ops::{AddAssign, Div, Mul, Sub};
use crate::sqrt::Sqrt;
use crate::vector::Vector;

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + for<'a> Mul<&'a K, Output=K> + AddAssign + Sqrt + Div<Output=K>
        + Sub<Output=K> {
    pub fn angle_cos(&self, v2: &Self) -> K {
        (self * v2) / (self.norm() * &v2.norm())
    }
}


#[cfg(test)]
mod test {
    use crate::approximately_equal::approximately_equal;
    use crate::assert_approximately_equal;
    use super::*;

    #[test]
    fn angle_cos() {
        let v1 = Vector::new([1., 0.]);
        let v2 = Vector::new([1., 0.]);
        assert_approximately_equal!(v1.angle_cos(&v2), 1.0);

        let v1 = Vector::new([1., 0.]);
        let v2 = Vector::new([0., 1.]);
        assert_approximately_equal!(v1.angle_cos(&v2), 0.0);

        let v1 = Vector::new([-1., 1.]);
        let v2 = Vector::new([1., -1.]);
        assert_approximately_equal!(v1.angle_cos(&v2), -1.0);

        let v1 = Vector::new([2., 1.]);
        let v2 = Vector::new([4., 2.]);
        assert_approximately_equal!(v1.angle_cos(&v2), 1.0);

        let v1 = Vector::new([1., 2., 3.]);
        let v2 = Vector::new([4., 5., 6.]);
        assert_approximately_equal!(v1.angle_cos(&v2), 0.974631846);
    }
}
