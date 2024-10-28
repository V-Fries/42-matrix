use std::ops::{Add, AddAssign, DivAssign, Mul, Neg, Sub, SubAssign};

use crate::{One, Sqrt, Vector};

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
    K: Clone
        + Default
        + One
        + for<'a> SubAssign<&'a K>
        + for<'a> Mul<&'a K, Output = K>
        + Sub<K, Output = K>
        + Add<Output = K>
        + Sqrt
        + for<'a> DivAssign<&'a K>
        + AddAssign
        + Neg<Output = K>,
{
    pub fn look_at(
        camera_pos: impl Into<Vector<K, 3>>,
        target_point: impl Into<Vector<K, 3>>,
        up: impl Into<Vector<K, 3>>,
    ) -> Self {
        let camera_pos = camera_pos.into();
        let forward = (target_point.into() - &camera_pos).normalize();
        let right = (&forward ^ up.into()).normalize();
        let up = &right ^ &forward;

        Matrix::from_row_major_order([
            [
                right[0].clone(),
                right[1].clone(),
                right[2].clone(),
                -(&camera_pos * right),
            ],
            [
                up[0].clone(),
                up[1].clone(),
                up[2].clone(),
                -(&camera_pos * up),
            ],
            [
                -forward[0].clone(),
                -forward[1].clone(),
                -forward[2].clone(),
                &camera_pos * forward,
            ],
            [K::default(), K::default(), K::default(), K::ONE],
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn look_at() {
        let m = Matrix::look_at([2., 2., 2.], [0., 0., 0.], [0., 0., 1.]);
        assert_eq!(
            m,
            [
                [-0.707107, -0.408248, 0.57735, 0.],
                [0.707107, -0.408248, 0.57735, 0.],
                [0., 0.816497, 0.57735, 0.],
                [-0., -0., -3.4641, 1.],
            ]
            .into()
        );
    }
}
