use std::fmt::Debug;

pub trait ApproximatelyEqual {
    fn approximately_equal(&self, other: &Self) -> bool;
}

#[allow(dead_code)]
pub fn assert_approximately_equal<K>(a: K, b: K)
    where
        K: Debug + Clone + ApproximatelyEqual {
    if !a.approximately_equal(&b) {
        panic!("approximately_equal({a:?}, {b:?}) failed");
    }
}
