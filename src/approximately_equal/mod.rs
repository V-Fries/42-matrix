#[allow(clippy::module_inception)]
mod approximately_equal;
mod f32;
mod f64;

mod i32;

#[allow(unused)]
pub use approximately_equal::assert_approximately_equal;
pub use approximately_equal::ApproximatelyEqual;
