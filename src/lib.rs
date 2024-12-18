mod abs;
mod angle_types;
mod approximately_equal;
mod cos;
mod eq_epsilon;
mod lerp;
mod matrix;
mod one;
mod sin;
mod sqrt;
mod tan;
mod vector;

pub use matrix::{Matrix, MatrixSlice};
pub use vector::Vector;
pub type Vec2<K> = Vector<K, 2>;
pub type Vec3<K> = Vector<K, 3>;
pub type Vec4<K> = Vector<K, 4>;

pub use lerp::lerp;

pub use abs::Abs;
pub use angle_types::{Degree, Radian};
pub use approximately_equal::{assert_approximately_equal, ApproximatelyEqual};
pub use cos::Cos;
pub use eq_epsilon::EqEpsilon;
pub use one::One;
pub use sin::Sin;
pub use sqrt::Sqrt;
pub use tan::Tan;
