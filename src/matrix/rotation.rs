use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::{Cos, One, Radian, Sin, Sqrt, Vector};

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
    // TODO this is stupid, I should create a trait for each methods and make it include the
    // necessary subtraits
    K: Default
        + Clone
        + One
        + Cos
        + Sin
        + for<'a> Sub<&'a K, Output = K>
        + for<'a> Add<&'a K, Output = K>
        + Add<Output = K>
        + Sqrt
        + for<'a> DivAssign<&'a K>
        + for<'a> MulAssign<&'a K>
        + Sub<K, Output = K>
        + for<'a> Mul<&'a K, Output = K>
        + for<'a> AddAssign<&'a K>,
{
    pub fn rotate(
        m: &Matrix<K, 4, 4>,
        axis: impl Into<Vector<K, 3>>,
        angle: impl Into<Radian<K>>,
    ) -> Self {
        let axis = axis.into().normalize();
        let angle_rad = angle.into();
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        let axis_correction = axis.clone() * (K::ONE - &cos);

        Self::from([
            m[0].clone() * (axis_correction[0].clone() * &axis[0] + &cos)
                + m[1].clone() * (axis_correction[0].clone() * &axis[1] + sin.clone() * &axis[2])
                + m[2].clone() * (axis_correction[0].clone() * &axis[2] - sin.clone() * &axis[1]),
            m[0].clone() * (axis_correction[1].clone() * &axis[0] - sin.clone() * &axis[2])
                + m[1].clone() * (axis_correction[1].clone() * &axis[1] + &cos)
                + m[2].clone() * (axis_correction[1].clone() * &axis[2] + sin.clone() * &axis[0]),
            m[0].clone() * (axis_correction[2].clone() * &axis[0] + sin.clone() * &axis[1])
                + m[1].clone() * (axis_correction[2].clone() * &axis[1] - sin * &axis[0])
                + m[2].clone() * (axis_correction[2].clone() * &axis[2] + cos),
            m[3].clone(),
        ])
    }
}

impl<K> Matrix<K, 4, 4>
where
    K: Default + Clone + One + Cos + Sin + Neg<Output = K>,
{
    ///     1       0       0       0
    ///     0       cos     -sin    0
    ///     0       sin     cos     0
    ///     0       0       0       1
    pub fn rotate_x(angle: Radian<K>) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();

        Self::from_row_major_order([
            [K::ONE, K::default(), K::default(), K::default()],
            [K::default(), cos.clone(), -sin.clone(), K::default()],
            [K::default(), sin, cos, K::default()],
            [K::default(), K::default(), K::default(), K::ONE],
        ])
    }

    ///     cos     0       sin     0
    ///     0       1       0       0
    ///     -sin    0       cos     0
    ///     0       0       0       1
    pub fn rotate_y(angle: Radian<K>) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();

        Self::from_row_major_order([
            [cos.clone(), K::default(), sin.clone(), K::default()],
            [K::default(), K::ONE, K::default(), K::default()],
            [-sin, K::default(), cos, K::default()],
            [K::default(), K::default(), K::default(), K::ONE],
        ])
    }

    ///     cos     -sin    0       0
    ///     sin     cos     0       0
    ///     0       0       1       0
    ///     0       0       0       1
    pub fn rotate_z(angle: Radian<K>) -> Self {
        let sin = angle.sin();
        let cos = angle.cos();

        Self::from_row_major_order([
            [cos.clone(), -sin.clone(), K::default(), K::default()],
            [sin, cos, K::default(), K::default()],
            [K::default(), K::default(), K::ONE, K::default()],
            [K::default(), K::default(), K::default(), K::ONE],
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::Degree;

    use super::*;

    #[test]
    fn rotate() {
        let mut r = Matrix::rotate(
            &Matrix::identity(),
            Vector::from([0.33, 0., 0.67]),
            Degree::from(67.0f32),
        );
        assert_eq!(
            r,
            Matrix::from_row_major_order([
                [0.509679, -0.825775, 0.241501, 0.],
                [0.825775, 0.390731, -0.406725, 0.],
                [0.241501, 0.406725, 0.881052, 0.],
                [0., 0., 0., 1.],
            ])
        );

        r[3][3] = 45.0f32;
        r[3][2] = 489.0f32;
        r[1][3] = 5.0f32;
        let r = Matrix::rotate(&r, [0.45, 23., 55.], Degree::from(54.0f32));
        assert_eq!(
            r,
            Matrix::from_row_major_order([
                [-0.392406, -0.878929, 0.27111, 0.],
                [0.903271, -0.423853, -0.0667148, 0.],
                [0.173548, 0.218706, 0.960233, 489.],
                [3.73781, 3.24569, 0.703037, 45.],
            ])
        );
    }

    // TODO add unit tests for rotate x/y/z
}
