use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vector<K, const N: usize> {
    pub(in crate::vector) scalars: [K; N],
}

impl<K, const N: usize> Vector<K, N> {
    #[allow(dead_code)]
    pub fn new(scalars: [K; N]) -> Self {
        Self { scalars }
    }

    pub fn size(&self) -> usize { N }
}

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
