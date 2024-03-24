use std::default::Default;
use std::ops::{AddAssign, Mul, MulAssign};

use crate::vector::Vector;

// Vector<K> *= &K
impl<K: for<'a> MulAssign<&'a K>, const N: usize> MulAssign<&K> for Vector<K, N> {
    fn mul_assign(&mut self, other: &K) {
        for i in 0..N {
            self[i] *= other;
        }
    }
}

// Vector<K> * &K
impl<K: for<'a> MulAssign<&'a K>, const N: usize> Mul<&K> for Vector<K, N> {
    type Output = Self;

    fn mul(mut self, other: &K) -> Self::Output {
        self *= other;
        self
    }
}

// Vector<K> * &Vector<K> (Dot product)
impl<K: Clone + AddAssign + for<'a> Mul<&'a K, Output=K> + Default + AddAssign, const N: usize>
Mul<&Self> for Vector<K, N> {
    type Output = K;

    fn mul(self, other: &Self) -> Self::Output {
        assert_eq!(self.size(), other.size());

        let mut result: K = Default::default();
        for i in 0..N {
            result += self[i].clone() * &other[i];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    // Vector<K> *= &K
    #[test]
    fn mul_assign_k() {
        let v = Vector::new([4., 2., 3.]);
        let k = 5.;
        let mut result = v.clone();
        result *= &k;
        assert_eq!(v[0] * k, result[0]);
        assert_eq!(v[1] * k, result[1]);
        assert_eq!(v[2] * k, result[2]);
    }

    // Vector<K> * &K
    #[test]
    fn mul_k() {
        let v = Vector::new([4., 2., 3.]);
        let k = 5.;
        let result = v.clone() * &k;
        assert_eq!(v[0] * k, result[0]);
        assert_eq!(v[1] * k, result[1]);
        assert_eq!(v[2] * k, result[2]);
    }

    // Vector<K> * &Vector<K>
    #[test]
    fn dot_product() {
        let v1 = Vector::new([34., 2., 4.]);
        let v2 = Vector::new([3., 23., 24.]);
        let expected_result = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        assert_eq!(expected_result, v1 * &v2);
    }
}
