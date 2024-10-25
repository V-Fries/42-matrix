use super::Sin;

impl Sin for f32 {
    fn sin(&self) -> Self {
        f32::sin(*self)
    }
}
