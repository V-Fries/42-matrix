use std::fmt::Debug;
use crate::approximately_equal::ApproximatelyEqual;
use crate::vector::Vector;

impl<K, const N: usize> PartialEq for Vector<K, N>
    where
        K: Clone + ApproximatelyEqual {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if !self[i].approximately_equal(&other[i]) {
                return false;
            }
        }
        true
    }
}

#[allow(dead_code)]
pub fn assert_vector_equal<K, const N: usize>(v1: &Vector<K, N>, v2: &Vector<K, N>)
    where
        K: Debug + Clone + ApproximatelyEqual {
    if v1 != v2 {
        panic!("assert_vector_equal {v1:?}, {v2:?}");
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    // &Vector<K> == &Vector<K>
    #[test]
    fn equality() {
        let v1 = Vector::new([4, 324, 5]);
        assert_eq!(v1 == v1, true);
        assert_eq!(v1 == Vector::new([v1[0] + 1, v1[1], v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1] + 1, v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1], v1[2] - 1]), false);

        let v1 = Vector::new([4.43, 324.45, 5.67]);
        assert_eq!(v1 == v1, true);
        assert_eq!(v1 == Vector::new([v1[0] + 1.5, v1[1], v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1] + 1.5, v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1], v1[2] - 1.5]), false);
    }
}
