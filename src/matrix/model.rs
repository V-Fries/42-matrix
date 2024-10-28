use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Sub};

use crate::{Cos, One, Radian, Sin, Sqrt, Vector};

use super::Matrix;

impl<K> Matrix<K, 4, 4>
where
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
    pub fn model(
        rotation_axis: impl Into<Vector<K, 3>>,
        angle: impl Into<Radian<K>>,
        position: impl Into<Vector<K, 3>>,
        scale: impl Into<Vector<K, 3>>,
    ) -> Self {
        let axis = rotation_axis.into().normalize();
        let angle_rad = angle.into();
        let sin = angle_rad.sin();
        let cos = angle_rad.cos();
        let axis_correction = axis.clone() * (K::ONE - &cos);

        let rot_a = axis_correction[0].clone() * &axis[0] + &cos;
        let rot_b = axis_correction[0].clone() * &axis[1] + sin.clone() * &axis[2];
        let rot_c = axis_correction[0].clone() * &axis[2] - sin.clone() * &axis[1];
        let rot_d = axis_correction[1].clone() * &axis[0] - sin.clone() * &axis[2];
        let rot_e = axis_correction[1].clone() * &axis[1] + &cos;
        let rot_f = axis_correction[1].clone() * &axis[2] + sin.clone() * &axis[0];
        let rot_g = axis_correction[2].clone() * &axis[0] + sin.clone() * &axis[1];
        let rot_h = axis_correction[2].clone() * &axis[1] - sin * &axis[0];
        let rot_i = axis_correction[2].clone() * &axis[2] + cos;

        let scale = scale.into();
        let [pos_x, pos_y, pos_z] = position.into().into();

        Self::from([
            [
                rot_a * &scale[0],
                rot_b * &scale[0],
                rot_c * &scale[0],
                K::default(),
            ],
            [
                rot_d * &scale[1],
                rot_e * &scale[1],
                rot_f * &scale[1],
                K::default(),
            ],
            [
                rot_g * &scale[2],
                rot_h * &scale[2],
                rot_i * &scale[2],
                K::default(),
            ],
            [pos_x, pos_y, pos_z, K::ONE],
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::Degree;

    use super::*;

    #[test]
    fn model() {
        let rot_axis = [0.33f32, 0., 0.67];
        let rot_rad = Degree::from(67.0f32);
        let pos = [24.0f32, 42., 89.];
        let scale = [45.0f32, 56., 4.];

        assert_eq!(
            Matrix::model(rot_axis, rot_rad.clone(), pos, scale),
            Matrix::translation(pos[0], pos[1], pos[2])
                * Matrix::rotate(&Matrix::identity(), rot_axis, rot_rad)
                * Matrix::scale(scale[0], scale[1], scale[2])
        );
    }
}
