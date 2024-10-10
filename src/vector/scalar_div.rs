use std::ops::{Div, DivAssign};

use crate::vector::Vector;

// Vector<K> /= &K
impl<K, const N: usize> DivAssign<&K> for Vector<K, N>
where
    K: for<'a> DivAssign<&'a K>,
{
    fn div_assign(&mut self, other: &K) {
        for i in 0..N {
            self[i] /= other;
        }
    }
}

// Vector<K> / &K
impl<K, const N: usize> Div<&K> for Vector<K, N>
where
    K: for<'a> DivAssign<&'a K>,
{
    type Output = Self;

    fn div(mut self, other: &K) -> Self::Output {
        self /= other;
        self
    }
}

// Vector<K> /= K
impl<K, const N: usize> DivAssign<K> for Vector<K, N>
where
    K: for<'a> DivAssign<&'a K>,
{
    fn div_assign(&mut self, other: K) {
        *self /= &other;
    }
}

// Vector<K> / K
impl<K, const N: usize> Div<K> for Vector<K, N>
where
    K: for<'a> DivAssign<&'a K>,
{
    type Output = Self;

    fn div(mut self, other: K) -> Self::Output {
        self /= &other;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    // Vector<K> /= K
    #[test]
    fn mul_assign_k() {
        let v = Vector::from([4., 2., 3.]);
        let k = 5.;

        let mut result = v.clone();
        result /= &k;
        assert_eq!(v[0] / k, result[0]);
        assert_eq!(v[1] / k, result[1]);
        assert_eq!(v[2] / k, result[2]);

        result = v.clone();
        result /= k;
        assert_eq!(v[0] / k, result[0]);
        assert_eq!(v[1] / k, result[1]);
        assert_eq!(v[2] / k, result[2]);
    }

    // Vector<K> / K
    #[test]
    fn mul_k() {
        let v = Vector::from([4., 2., 3.]);
        let k = 5.;

        let result = v.clone() / &k;
        assert_eq!(v[0] / k, result[0]);
        assert_eq!(v[1] / k, result[1]);
        assert_eq!(v[2] / k, result[2]);

        let result = v.clone() / k;
        assert_eq!(v[0] / k, result[0]);
        assert_eq!(v[1] / k, result[1]);
        assert_eq!(v[2] / k, result[2]);
    }
}
