use std::ops::{Add, AddAssign};

use crate::vector::Vector;

// Vector<K> += &Vector<K>
impl<K: for<'a> AddAssign<&'a K>, const N: usize> AddAssign<&Self> for Vector<K, N> {
    fn add_assign(&mut self, other: &Self) {
        for i in 0..N {
            self[i] += &other[i];
        }
    }
}

// Vector<K> + &Vector<K>
impl<K: for<'a> AddAssign<&'a K>, const N: usize> Add<&Self> for Vector<K, N> {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self += other;
        self
    }
}

// Vector<K> += Vector<K>
impl<K: for<'a> AddAssign<&'a K>, const N: usize> AddAssign<Self> for Vector<K, N> {
    fn add_assign(&mut self, other: Self) { *self += &other; }
}

// Vector<K> + Vector<K>
impl<K: for<'a> AddAssign<&'a K>, const N: usize> Add<Self> for Vector<K, N> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += &other;
        self
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    // Vector<K> += Vector<K>
    #[test]
    fn add_assign_vec() {
        let scalars_v1 = [23., 21., 33.];
        let scalars_v2 = [4., 2., 3.];
        let v1 = Vector::new(scalars_v1.clone());
        let v2 = Vector::new(scalars_v2.clone());

        let mut result = v1.clone();
        result += &v2;
        assert_eq!(v1[0] + v2[0], result[0]);
        assert_eq!(v1[1] + v2[1], result[1]);
        assert_eq!(v1[2] + v2[2], result[2]);

        result = v1.clone();
        result += v2.clone();
        assert_eq!(v1[0] + v2[0], result[0]);
        assert_eq!(v1[1] + v2[1], result[1]);
        assert_eq!(v1[2] + v2[2], result[2]);
    }

    // Vector<K> + Vector<K>
    #[test]
    fn add_vec() {
        let scalars_v1 = [23., 21., 33.];
        let scalars_v2 = [4., 2., 3.];
        let v1 = Vector::new(scalars_v1.clone());
        let v2 = Vector::new(scalars_v2.clone());

        let result = v1.clone() + &v2;
        assert_eq!(v1[0] + v2[0], result[0]);
        assert_eq!(v1[1] + v2[1], result[1]);
        assert_eq!(v1[2] + v2[2], result[2]);

        let result = v1.clone() + v2.clone();
        assert_eq!(v1[0] + v2[0], result[0]);
        assert_eq!(v1[1] + v2[1], result[1]);
        assert_eq!(v1[2] + v2[2], result[2]);
    }
}
