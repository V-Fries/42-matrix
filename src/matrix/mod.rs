mod add;
mod comparison;
mod determinant;
mod inverse;
#[allow(clippy::module_inception)]
mod matrix;
mod matrix_slice;
mod mul;
mod mul_matrix;
mod mul_vector;
mod reduced_row_echelon_form;
mod row_echelon_form;
mod sub;
mod trace;
mod transpose;

pub use matrix::Matrix;
pub use matrix_slice::MatrixSlice;
