use crate::approximately_equal::ApproximatelyEqual;

impl ApproximatelyEqual for i32 {
    fn approximately_equal(self, other: &Self) -> bool {
        self == *other
    }
}
