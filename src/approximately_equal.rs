use std::ops::Sub;
use crate::abs::Abs;
use crate::eq_epsilon::EqEpsilon;

#[macro_export]
macro_rules! assert_approximately_equal {
    ($arg1:expr, $arg2:expr) => {
        if !crate::approximately_equal::approximately_equal($arg1.clone(), &$arg2) {
            panic!("approximately_equal({}, {}) failed", $arg1, $arg2);
        }
    };
}

#[allow(dead_code)]
pub fn approximately_equal<K>(a: K, b: &K) -> bool
    where
        K: for<'a> Sub<&'a K, Output=K> + Abs + EqEpsilon + PartialOrd {
    (a - b).abs() < K::EQ_EPSILON
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_approximately_equal() {
        assert!(approximately_equal(0.1 + 0.2, &0.3));
    }
}
