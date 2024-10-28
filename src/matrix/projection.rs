use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Abs, One, Radian, Tan};

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
    K: Default
        + Abs
        + One
        + Tan
        + Add<Output = K>
        + Div<Output = K>
        + for<'a> Mul<&'a K, Output = K>
        + Clone
        + Neg<Output = K>
        + for<'a> Sub<&'a K, Output = K>
        + for<'a> Add<&'a K, Output = K>
        + Mul<Output = K>
        + Sub<Output = K>
        + PartialOrd,
{
    pub fn perspective_opengl(vertical_fov: Radian<K>, aspect_ratio: K, near: K, far: K) -> Self {
        debug_assert!(near < far);
        debug_assert!(aspect_ratio.clone().abs() > K::default());
        debug_assert!(Radian::into_inner(Radian::clone(&vertical_fov)).abs() > K::default());

        let half_fov_tan = (Radian::into_inner(vertical_fov) / (K::ONE + K::ONE)).tan();

        Matrix::from_row_major_order([
            [
                K::ONE / (aspect_ratio * &half_fov_tan),
                K::default(),
                K::default(),
                K::default(),
            ],
            [
                K::default(),
                K::ONE / half_fov_tan,
                K::default(),
                K::default(),
            ],
            [
                K::default(),
                K::default(),
                -(far.clone() + &near) / (far.clone() - &near),
                -((K::ONE + K::ONE) * far.clone() * &near) / (far - near),
            ],
            [K::default(), K::default(), -K::ONE, K::default()],
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Degree;

    #[test]
    fn perspective_opengl() {
        let m = Matrix::perspective_opengl(
            Degree::from(45.0f32).into(),
            1920.0f32 / 1080.0f32,
            0.1f32,
            10.0f32,
        );

        assert_eq!(
            m,
            Matrix::from_row_major_order([
                [1.358, 0., 0., 0.],
                [0., 2.41421, 0., 0.],
                [0., 0., -1.0202, -0.20202],
                [0., 0., -1., 0.],
            ])
        );
    }
}
