pub trait Tan: Sized {
    fn tan(&self) -> Self;
}

macro_rules! impl_tan {
    ($t:ident) => {
        impl Tan for $t {
            fn tan(&self) -> Self {
                $t::tan(*self)
            }
        }
    };
}

impl_tan!(f32);
impl_tan!(f64);
