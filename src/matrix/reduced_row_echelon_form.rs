use std::ops::{DivAssign, Mul, SubAssign};

use crate::approximately_equal::ApproximatelyEqual;

use super::Matrix;


impl<K, const M: usize, const N: usize> Matrix<K, M, N>
    where
        K: Clone + ApproximatelyEqual + Default + for<'a> DivAssign<&'a K>
            + for<'a> Mul<&'a K, Output = K> + SubAssign
{
    pub fn reduced_row_echelon_form(self) -> Self {
        let mut m = self.row_echelon_form();
        
        let mut last_y = N;
        while let Some((nest_one_x, next_one_y)) = Self::find_next_one(&m, last_y) {
            for y in 0..next_one_y {
                let scale = m[nest_one_x][y].clone(); 
                m[nest_one_x][y] = K::default();
                for x in (nest_one_x + 1)..M {
                    let tmp = m[x][next_one_y].clone() * &scale;
                    m[x][y] -= tmp;
                }
            }
            last_y = next_one_y;
        }
        m
    }

    fn find_next_one(&self, last_y: usize) -> Option<(usize, usize)> {
        if last_y == 1 {
            return None;
        }

        for y in (0..last_y).rev() {
            for x in 0..self.get_x_size() {
                if !self[x][y].clone().approximately_equal(&K::default()) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn reduced_row_echelon_form() {
        let u = Matrix::from_row_major_order([
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 0., 1.],
        ]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::from_row_major_order([
                [1., 0., 0.],
                [0., 1., 0.],
                [0., 0., 1.],
        ]));

        let u = Matrix::from_row_major_order([
            [1., 2.],
            [3., 4.],
        ]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::from_row_major_order([
                [1., 0.],
                [0., 1.],
        ]));

        let u = Matrix::from_row_major_order([
            [1., 2.],
            [2., 4.],
        ]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::from_row_major_order([
                [1., 2.],
                [0., 0.],
        ]));
        
        let u = Matrix::from_row_major_order([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::from_row_major_order([
                [1.0, 0.625, 0.0, 0.0, -12.1666667],
                [0.0, 0.0, 1.0, 0.0, -3.6666667],
                [0.0, 0.0, 0.0, 1.0, 29.5],
        ]));

        let u = Matrix::<i32, 1, 0>::new([[]]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::<i32, 1, 0>::new([[]]));

        
        let u = Matrix::<i32, 0, 0>::new([]);
        assert_eq!(u.reduced_row_echelon_form(), Matrix::<i32, 0, 0>::new([]));
    }
}
