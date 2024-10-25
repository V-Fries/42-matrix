use super::Sin;

impl Sin for f64 {
    fn sin(&self) -> Self {
        f64::sin(*self)
    }
}
