use std::ops::{Mul, AddAssign};
use crate::abs::Abs;
use crate::sqrt::Sqrt;
use crate::vector::Vector;

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + AddAssign + Abs {
    #[allow(dead_code)]
    pub fn norm_1(&self) -> K {
        let mut result: K = Default::default();
        for i in 0..N {
            result += self[i].clone().abs();
        }
        result
    }
}

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + for<'a> Mul<&'a K, Output=K> + AddAssign + Sqrt {
    #[allow(dead_code)]
    pub fn norm(&self) -> K {
        let mut result: K = Default::default();
        for i in 0..N {
            result += self[i].clone() * &self[i];
        }
        result.sqrt()
    }
}

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + Abs + PartialOrd {
    #[allow(dead_code)]
    pub fn norm_inf(&self) -> K {
        let mut result: K = Default::default();
        for i in 0..N {
            let tmp = self[i].clone().abs();
            if tmp > result {
                result = tmp;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::assert_approximately_equal;
    use super::*;

    #[test]
    fn norms() {
        let v = Vector::new([0., 0., 0.]);
        assert_approximately_equal!(v.norm_1(), 0.);
        assert_approximately_equal!(v.norm(), 0.);
        assert_approximately_equal!(v.norm_inf(), 0.);

        let v = Vector::new([1., 2., 3.]);
        assert_approximately_equal!(v.norm_1(), 6.);
        assert_approximately_equal!(v.norm(), 3.74165738);
        assert_approximately_equal!(v.norm_inf(), 3.);

        let v = Vector::new([-1., -2.]);
        assert_approximately_equal!(v.norm_1(), 3.);
        assert_approximately_equal!(v.norm(), 2.236067977);
        assert_approximately_equal!(v.norm_inf(), 2.);
    }
}
