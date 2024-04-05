use std::ops::{Add, Mul, Sub};

#[allow(dead_code)]
pub fn lerp<V: Clone + for<'a> Sub<&'a V, Output=V> + Mul<f32, Output=V> + Add<V, Output=V>>(
    v1: V, v2: V, t: f32,
) -> V {
    let direction = v2 - &v1;
    v1 + direction * t
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0., 1., 0.), 0.);

        assert_eq!(lerp(0., 1., 1.), 1.);

        assert_eq!(lerp(0., 1., 0.5), 0.5);

        assert_eq!(lerp(21., 42., 0.3), 27.3);

        assert_eq!(lerp(Vector::new([2., 1.]),
                        Vector::new([4., 2.]), 0.3),
                   Vector::new([2.6, 1.3]));

        assert_eq!(lerp(Matrix::new([[2., 1.], [3., 4.]]),
                        Matrix::new([[20., 10.], [30., 40.]]), 0.5),
                   Matrix::new([[11., 5.5], [16.5, 22.]]));
    }
}