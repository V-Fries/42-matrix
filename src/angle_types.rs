use std::ops::{Deref, DerefMut};

pub struct Radian<K>(K);
pub struct Degree<K>(K);

macro_rules! define_base_methods {
    ($t:ident) => {
        impl<K> Deref for $t<K> {
            type Target = K;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<K> DerefMut for $t<K> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<K> $t<K> {
            fn into_inner(self) -> K {
                self.0
            }
        }
    };
}
define_base_methods!(Radian);
define_base_methods!(Degree);

macro_rules! define_angle_type {
    ($t:ty, $self_type:ident, $other_type:ident) => {
        impl From<$t> for $self_type<$t> {
            fn from(value: $t) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! define_angle_types_for {
    ($t:ty, $pi:expr, $degree_180:expr) => {
        define_angle_type!($t, Radian, Degree);
        impl From<Degree<$t>> for Radian<$t> {
            fn from(degree: Degree<$t>) -> Self {
                Self(Degree::<$t>::into_inner(degree) * $pi / $degree_180)
            }
        }

        define_angle_type!($t, Degree, Radian);
        impl From<Radian<$t>> for Degree<$t> {
            fn from(radian: Radian<$t>) -> Self {
                Self(Radian::<$t>::into_inner(radian) * $degree_180 / $pi)
            }
        }
    };
}

define_angle_types_for!(f32, std::f32::consts::PI, 180.0f32);
define_angle_types_for!(f64, std::f64::consts::PI, 180.0f64);
