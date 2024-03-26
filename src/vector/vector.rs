use std::ops::{AddAssign, Index, IndexMut, MulAssign};

#[derive(Debug, Clone)]
pub struct Vector<K, const N: usize> {
    pub(in crate::vector) scalars: [K; N],
}

// Constructors
impl<K, const N: usize> Vector<K, N> {
    #[allow(dead_code)]
    pub fn new(scalars: [K; N]) -> Self { Self { scalars } }
}

impl<K: Default, const N: usize> Vector<K, N> {
    #[allow(dead_code)]
    pub fn default() -> Self { Vector::new([(); N].map(|_| Default::default())) }
}

impl<K: Default + Clone + for<'a> MulAssign<&'a K> + for<'a> AddAssign<&'a K>, const N: usize> Vector<K, N> {
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
    pub fn size(&self) -> usize { N }
}


// []
impl<K, const N: usize> Index<usize> for Vector<K, N> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output { &self.scalars[index] }
}

impl<K, const N: usize> IndexMut<usize> for Vector<K, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.scalars[index] }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_vector() {
        let scalars = [3.0f32, 4., 5.];
        let vector = Vector::new(scalars.clone());
        assert_eq!(vector.scalars, scalars);
    }

    #[test]
    fn get_vector_size() {
        let scalars = [3.0f32, 4., 5.];
        let vector = Vector::new(scalars.clone());
        assert_eq!(scalars.len(), vector.size());
    }

    #[test]
    fn linear_combination() {
        let v1 = Vector::new([43., 4325., 5325., 5432.]);
        let v2 = Vector::new([435., 4325., 436., 676.]);
        let k1 = 5.;
        let k2 = 2.;
        let result = Vector::linear_combination([v1.clone(), v2.clone()], &[k1, k2]);
        let expected_result = Vector::new([0., 0., 0., 0.]) + v1 * k1 + v2 * k2;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn index_operators() {
        let scalars = [3.0f32, 4., 5.];
        let mut vector = Vector::new(scalars.clone());

        const INDEX_GET: usize = 1;
        assert_eq!(scalars[INDEX_GET], vector[INDEX_GET]);

        const INDEX_SET: usize = 2;
        let scalar = scalars[INDEX_SET] + 4.;
        vector[INDEX_SET] = scalar.clone();
        assert_eq!(vector[INDEX_SET], scalar);
    }
}
