use super::Cos;

impl Cos for f64 {
    fn cos(&self) -> Self {
        f64::cos(*self)
    }
}
