use std::ops::{Mul, Add};
use crate::abs::Abs;
use crate::sqrt::Sqrt;
use crate::vector::Vector;

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + Add<Output=K> + Abs {
    #[allow(dead_code)]
    pub fn norm_1(&self) -> K {
        self.iter()
            .fold(Default::default(), |acc, elem| acc + elem.clone().abs())
    }
}

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + for<'a> Mul<&'a K, Output=K> + Add<Output=K> + Sqrt {
    #[allow(dead_code)]
    pub fn norm(&self) -> K {
        self.iter()
            .fold(Default::default(), |acc: K, elem| acc + elem.clone() * elem)
            .sqrt()
    }
}

impl<K, const N: usize> Vector<K, N>
    where
        K: Default + Clone + Abs + PartialOrd {
    #[allow(dead_code)]
    pub fn norm_inf(&self) -> K {
        self.iter()
            .fold(Default::default(), |result, elem| {
                // Could use K::max() if I ever implement a Max trait
                let elem_abs = elem.clone().abs();
                if elem_abs > result {
                    return elem_abs;
                }
                result
            })
    }
}

#[cfg(test)]
mod test {
    use crate::approximately_equal::assert_approximately_equal;
    use super::*;

    #[test]
    fn norms() {
        let v = Vector::from([0., 0., 0.]);
        assert_approximately_equal(v.norm_1(), 0.);
        assert_approximately_equal(v.norm(), 0.);
        assert_approximately_equal(v.norm_inf(), 0.);

        let v = Vector::from([1., 2., 3.]);
        assert_approximately_equal(v.norm_1(), 6.);
        assert_approximately_equal(v.norm(), 3.74165738);
        assert_approximately_equal(v.norm_inf(), 3.);

        let v = Vector::from([-1., -2.]);
        assert_approximately_equal(v.norm_1(), 3.);
        assert_approximately_equal(v.norm(), 2.236067977);
        assert_approximately_equal(v.norm_inf(), 2.);
    }
}
