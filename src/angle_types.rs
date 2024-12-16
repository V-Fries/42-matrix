use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone)]
pub struct Radian<K>(K);
#[derive(Clone)]
pub struct Degree<K>(K);

macro_rules! define_op {
    ($t:ident, $op:ident, $op_fn:ident, $op_assign:ident, $op_assign_fn:ident) => {
        impl<K> $op<$t<K>> for $t<K>
        where
            K: $op<Output = K>,
        {
            type Output = Self;

            fn $op_fn(self, other: Self) -> Self {
                Self($t::into_inner(self).$op_fn($t::into_inner(other)))
            }
        }

        impl<K> $op<&$t<K>> for $t<K>
        where
            K: for<'a> $op<&'a K, Output = K>,
        {
            type Output = Self;

            fn $op_fn(self, other: &Self) -> Self {
                Self($t::into_inner(self).$op_fn(&*other))
            }
        }

        impl<K> $op_assign<$t<K>> for $t<K>
        where
            K: $op_assign<K>,
        {
            fn $op_assign_fn(&mut self, other: Self) {
                self.deref_mut().$op_assign_fn($t::into_inner(other));
            }
        }

        impl<K> $op_assign<&$t<K>> for $t<K>
        where
            K: for<'a> $op_assign<&'a K>,
        {
            fn $op_assign_fn(&mut self, other: &Self) {
                self.deref_mut().$op_assign_fn(&*other);
            }
        }
    };
}

macro_rules! define_base_methods {
    ($t:ident) => {
        impl<K> $t<K> {
            pub fn new(inner: K) -> Self {
                Self(inner)
            }
        }

        define_op!($t, Add, add, AddAssign, add_assign);
        define_op!($t, Sub, sub, SubAssign, sub_assign);
        define_op!($t, Mul, mul, MulAssign, mul_assign);
        define_op!($t, Div, div, DivAssign, div_assign);

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
            pub fn into_inner(self) -> K {
                self.0
            }
        }
    };
}

define_base_methods!(Radian);
define_base_methods!(Degree);

macro_rules! define_angle_type {
    ($t:ty, $self_type:ident) => {
        impl From<$t> for $self_type<$t> {
            fn from(value: $t) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! define_angle_types_for {
    ($t:ty, $pi:expr, $degree_180:expr) => {
        define_angle_type!($t, Radian);
        impl From<Degree<$t>> for Radian<$t> {
            fn from(degree: Degree<$t>) -> Self {
                Self(Degree::<$t>::into_inner(degree) * $pi / $degree_180)
            }
        }

        define_angle_type!($t, Degree);
        impl From<Radian<$t>> for Degree<$t> {
            fn from(radian: Radian<$t>) -> Self {
                Self(Radian::<$t>::into_inner(radian) * $degree_180 / $pi)
            }
        }
    };
}

define_angle_types_for!(f32, std::f32::consts::PI, 180.0f32);
define_angle_types_for!(f64, std::f64::consts::PI, 180.0f64);
