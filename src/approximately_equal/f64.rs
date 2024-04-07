use crate::approximately_equal::ApproximatelyEqual;
use crate::eq_epsilon::EqEpsilon;

impl ApproximatelyEqual for f64 {
    fn approximately_equal(&self, other: &Self) -> bool {
        (self.clone() - *other).abs() < Self::EQ_EPSILON
    }
}
