use super::Cos;

impl Cos for f32 {
    fn cos(&self) -> Self {
        f32::cos(*self)
    }
}
