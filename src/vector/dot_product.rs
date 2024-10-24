use crate::vector::Vector;
use std::ops::{AddAssign, Mul};

// &Vector<K> * &Vector<K>
impl<K, const N: usize> Mul<&Vector<K, N>> for &Vector<K, N>
where
    K: Clone + AddAssign + for<'a> Mul<&'a K, Output = K> + Default,
{
    type Output = K;

    fn mul(self, other: &Vector<K, N>) -> Self::Output {
        let mut result: K = Default::default();
        for i in 0..N {
            result += self[i].clone() * &other[i];
        }
        result
    }
}

// &Vector<K> * Vector<K>
impl<K, const N: usize> Mul<Vector<K, N>> for &Vector<K, N>
where
    K: Clone + AddAssign + for<'a> Mul<&'a K, Output = K> + Default,
{
    type Output = K;

    fn mul(self, other: Vector<K, N>) -> Self::Output {
        self * &other
    }
}

// Vector<K> * &Vector<K>
impl<K, const N: usize> Mul<&Self> for Vector<K, N>
where
    K: Clone + AddAssign + for<'a> Mul<&'a K, Output = K> + Default,
{
    type Output = K;

    fn mul(self, other: &Self) -> Self::Output {
        &self * other
    }
}

// Vector<K> * Vector<K>
impl<K, const N: usize> Mul<Self> for Vector<K, N>
where
    K: Clone + AddAssign + for<'a> Mul<&'a K, Output = K> + Default,
{
    type Output = K;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Vector<K> * Vector<K>
    #[test]
    fn dot_product() {
        let v1 = Vector::from([34., 2., 4.]);
        let v2 = Vector::from([3., 23., 24.]);
        let expected_result = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        assert_eq!(expected_result, &v1 * &v2);
        assert_eq!(expected_result, &v1 * v2.clone());
        assert_eq!(expected_result, v1.clone() * &v2);
        assert_eq!(expected_result, v1 * v2);
    }
}
