mod add;
mod comparison;
mod determinant;
mod inverse;
mod look_at;
#[allow(clippy::module_inception)]
mod matrix;
mod matrix_slice;
mod mul;
mod mul_matrix;
mod mul_vector;
mod reduced_row_echelon_form;
mod rotation;
mod row_echelon_form;
mod sub;
mod trace;
mod translation;
mod transpose;

pub use matrix::Matrix;
pub use matrix_slice::MatrixSlice;
