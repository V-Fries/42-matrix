use std::array::IntoIter;
use std::ops::{AddAssign, Index, IndexMut, MulAssign};
use std::slice::{Iter, IterMut};

#[derive(Debug, Clone)]
pub struct Vector<K, const N: usize> {
    pub(in crate::vector) scalars: [K; N],
}

impl<K, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(scalars: [K; N]) -> Self {
        Self { scalars }
    }
}

impl<K, const N: usize> Vector<K, N> {
    pub fn from_fn<F>(callback: F) -> Self
    where
        F: FnMut(usize) -> K,
    {
        Self {
            scalars: std::array::from_fn(callback),
        }
    }
}

impl<K, const N: usize> From<Vector<K, N>> for [K; N] {
    fn from(value: Vector<K, N>) -> Self {
        value.scalars
    }
}

impl<K, const N: usize> Default for Vector<K, N>
where
    K: Default,
{
    #[allow(dead_code)]
    fn default() -> Self {
        Self::from(std::array::from_fn(|_| Default::default()))
    }
}

impl<K, const N: usize> Vector<K, N>
where
    K: Default + Clone + for<'a> MulAssign<&'a K> + for<'a> AddAssign<&'a K>,
{
    #[allow(dead_code)]
    pub fn linear_combination<const NB_OF_VECTORS: usize>(
        vectors: [Vector<K, N>; NB_OF_VECTORS],
        coefs: &[K; NB_OF_VECTORS],
    ) -> Vector<K, N> {
        let mut result = Vector::<K, N>::default();

        for (vector, scalar) in vectors.into_iter().zip(coefs.iter()) {
            result += vector * scalar;
        }
        result
    }
}

// Utils
impl<K, const N: usize> Vector<K, N> {
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        N
    }
}

// iterators
impl<K, const N: usize> Vector<K, N> {
    #[allow(dead_code)]
    pub fn iter(&self) -> Iter<'_, K> {
        self.scalars.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K> {
        self.scalars.iter_mut()
    }
}

impl<K, const N: usize> IntoIterator for Vector<K, N> {
    type Item = K;

    type IntoIter = IntoIter<K, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.scalars.into_iter()
    }
}

// []
impl<K, const N: usize> Index<usize> for Vector<K, N> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.scalars[index]
    }
}

impl<K, const N: usize> IndexMut<usize> for Vector<K, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.scalars[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_vector() {
        let scalars = [3.0f32, 4., 5.];
        let vector = Vector::from(scalars);
        assert_eq!(vector.scalars, scalars);
    }

    #[test]
    fn get_vector_size() {
        let scalars = [3.0f32, 4., 5.];
        let vector = Vector::from(scalars);
        assert_eq!(scalars.len(), vector.size());
    }

    #[test]
    fn linear_combination() {
        let v1 = Vector::from([43., 4325., 5325., 5432.]);
        let v2 = Vector::from([435., 4325., 436., 676.]);
        let k1 = 5.;
        let k2 = 2.;
        let result = Vector::linear_combination([v1.clone(), v2.clone()], &[k1, k2]);
        let expected_result = v1 * k1 + v2 * k2;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn index_operators() {
        let scalars = [3.0f32, 4., 5.];
        let mut vector = Vector::from(scalars);

        const INDEX_GET: usize = 1;
        assert_eq!(scalars[INDEX_GET], vector[INDEX_GET]);

        const INDEX_SET: usize = 2;
        let scalar = scalars[INDEX_SET] + 4.;
        vector[INDEX_SET] = scalar;
        assert_eq!(vector[INDEX_SET], scalar);
    }
}
