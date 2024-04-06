use crate::eq_epsilon::EqEpsilon;

impl EqEpsilon for f32 {
    const EQ_EPSILON: Self = 0.0001;
}
