use std::ops::{BitXor, BitXorAssign, Mul, Sub};

use crate::vector::Vector;

// &Vector<K, 3> ^ &Vector<K, 3>
impl<K> BitXor<&Vector<K, 3>> for &Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    type Output = Vector<K, 3>;

    fn bitxor(self, other: &Vector<K, 3>) -> Self::Output {
        Vector::<K, 3>::from([
            self[1].clone() * &other[2] - other[1].clone() * &self[2],
            other[0].clone() * &self[2] - self[0].clone() * &other[2],
            self[0].clone() * &other[1] - other[0].clone() * &self[1],
        ])
    }
}

// &Vector<K, 3> ^ Vector<K, 3>
impl<K> BitXor<Vector<K, 3>> for &Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    type Output = Vector<K, 3>;

    fn bitxor(self, other: Vector<K, 3>) -> Self::Output { self ^ &other }
}

// Vector<K, 3> ^ &Vector<K, 3>
impl<K> BitXor<&Vector<K, 3>> for Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    type Output = Self;

    fn bitxor(self, other: &Self) -> Self::Output { &self ^ other }
}

// Vector<K, 3> ^ Vector<K, 3>
impl<K> BitXor<Vector<K, 3>> for Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output { &self ^ &other }
}

// Vector<K, 3> ^= &Vector<K, 3>
impl<K> BitXorAssign<&Vector<K, 3>> for Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    fn bitxor_assign(&mut self, other: &Self) { *self = &*self ^ other }
}

// Vector<K, 3> ^= Vector<K, 3>
impl<K> BitXorAssign<Vector<K, 3>> for Vector<K, 3>
    where
        K: Clone + for<'a> Mul<&'a K, Output=K> + Sub<K, Output=K> {
    fn bitxor_assign(&mut self, other: Self) { *self = &*self ^ &other }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cross_product() {
        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        assert_eq!(&(&v1 ^ &v2), &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        assert_eq!(&(&v1 ^ &v2), &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        assert_eq!(&(&v1 ^ &v2), &Vector::from([17., -58., -16.]));

        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        assert_eq!(&(&v1 ^ v2), &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        assert_eq!(&(&v1 ^ v2), &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        assert_eq!(&(&v1 ^ v2), &Vector::from([17., -58., -16.]));

        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        assert_eq!(&(v1 ^ &v2), &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        assert_eq!(&(v1 ^ &v2), &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        assert_eq!(&(v1 ^ &v2), &Vector::from([17., -58., -16.]));

        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        assert_eq!(&(v1 ^ v2), &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        assert_eq!(&(v1 ^ v2), &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        assert_eq!(&(v1 ^ v2), &Vector::from([17., -58., -16.]));

        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        let mut v3 = v1.clone();
        v3 ^= &v2;
        assert_eq!(&v3, &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        let mut v3 = v1.clone();
        v3 ^= &v2;
        assert_eq!(&v3, &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        let mut v3 = v1.clone();
        v3 ^= &v2;
        assert_eq!(&v3, &Vector::from([17., -58., -16.]));

        let v1 = Vector::from([0., 0., 1.]);
        let v2 = Vector::from([1., 0., 0.]);
        let mut v3 = v1.clone();
        v3 ^= v2;
        assert_eq!(&v3, &Vector::from([0., 1., 0.]));
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([4., 5., 6.]);
        let mut v3 = v1.clone();
        v3 ^= v2;
        assert_eq!(&v3, &Vector::from([-3., 6., -3.]));
        let v1 = Vector::from([4., 2., -3.]);
        let v2 = Vector::from([-2., -5., 16.]);
        let mut v3 = v1.clone();
        v3 ^= v2;
        assert_eq!(&v3, &Vector::from([17., -58., -16.]));
    }
}
