use std::ops::{AddAssign, Mul};
use crate::vector::Vector;

// &Vector<K> * &Vector<K> (Dot product)
impl<K: Clone + AddAssign + for<'a> Mul<&'a K, Output=K> + Default,
    const N: usize> Mul<&Vector<K, N>> for &Vector<K, N> {
    type Output = K;

    fn mul(self, other: &Vector<K, N>) -> Self::Output {
        let mut result: K = Default::default();
        for i in 0..N {
            result += self[i].clone() * &other[i];
        }
        result
    }
}

// &Vector<K> * Vector<K> (Dot product)
impl<K: Clone + AddAssign + for<'a> Mul<&'a K, Output=K> + Default,
    const N: usize> Mul<Vector<K, N>> for &Vector<K, N> {
    type Output = K;

    fn mul(self, other: Vector<K, N>) -> Self::Output { self * &other }
}

// Vector<K> * &Vector<K> (Dot product)
impl<K: Clone + AddAssign + for<'a> Mul<&'a K, Output=K> + Default,
    const N: usize> Mul<&Self> for Vector<K, N> {
    type Output = K;

    fn mul(self, other: &Self) -> Self::Output { &self * other }
}

// Vector<K> * Vector<K> (Dot product)
impl<K: Clone + AddAssign + for<'a> Mul<&'a K, Output=K> + Default,
    const N: usize> Mul<Self> for Vector<K, N> {
    type Output = K;

    fn mul(self, other: Self) -> Self::Output { &self * &other }
}

#[cfg(test)]
mod test {
    use super::*;

    // Vector<K> * Vector<K>
    #[test]
    fn dot_product() {
        let v1 = Vector::new([34., 2., 4.]);
        let v2 = Vector::new([3., 23., 24.]);
        let expected_result = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        assert_eq!(expected_result, &v1 * &v2);
        assert_eq!(expected_result, &v1 * v2.clone());
        assert_eq!(expected_result, v1.clone() * &v2);
        assert_eq!(expected_result, v1 * v2);
    }
}
