use crate::eq_epsilon::EqEpsilon;

impl EqEpsilon for f64 {
    const EQ_EPSILON: Self = 0.0001;
}
