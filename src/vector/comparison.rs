use crate::vector::Vector;

impl<K, const N: usize> PartialEq for Vector<K, N>
    where
        K: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        return self.scalars == other.scalars;
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;

    // &Vector<K> == &Vector<K>
    #[test]
    fn equality() {
        let v1 = Vector::new([4, 324, 5]);
        assert_eq!(v1 == v1.clone(), true);
        assert_eq!(v1 == Vector::new([v1[0] + 1, v1[1], v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1] + 1, v1[2]]), false);
        assert_eq!(v1 == Vector::new([v1[0], v1[1], v1[2] - 1]), false);
    }
}
