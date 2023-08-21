use crate::Tuple;
use std::ops::Mul;

// Module to handle matrix operations.
//
// The notation is as follows
//
//  Matrix A (R rows, C columns)
//      a11  a12  ...   a1n
//      a21  a22  ...   a1n
//      a31  a32  ...   a1n
//       .    .          .
//      am1  am2  ...   amn
//
//  Matrix B (P rows, Q columns)
//      b11  b12  ...   b1q
//      b21  b22  ...   b1q
//      b31  b32  ...   b1q
//       .    .          .
//      bp1  bp2  ...   bpq
//

#[derive(Debug)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn new() -> Self {
        let data = [[0.0; C]; R];
        Matrix { data }
    }

    fn new_init(data: [[f64; C]; R]) -> Self {
        Matrix { data }
    }
}

// impl<const R: usize, const C: usize, const P: usize, const Q: usize> Matrix<R, C> {
//     fn submatrix(&self, row: usize, col: usize) -> Matrix<P, Q> {
//         return Matrix::new<P, Q>();
//     }
// }

// The methods below are only applicable on square matrices
impl<const R: usize> Matrix<R, R> {
    // Create an identity Matrix
    fn new_identity() -> Self {
        let mut m = Matrix::new();
        for i in 0..R {
            m.data[i][i] = 1.0;
        }
        return m;
    }

    fn transpose(&self) -> Self {
        let mut transposed = Matrix::new_init(self.data);
        for row in 0..R {
            for col in 0..R {
                transposed.data[row][col] = self.data[col][row];
            }
        }
        return transposed;
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

// Multiplication of two matrices A * B
// The number of columns in A must match the number of rows in B
// i.e. A is <R, C> and B is <P, Q>, where C == P
// The output is a matrix of size <R ,Q>
impl<const R: usize, const C: usize, const Q: usize> Mul<&Matrix<C, Q>> for &Matrix<R, C> {
    type Output = Matrix<R, Q>;

    fn mul(self, other: &Matrix<C, Q>) -> Matrix<R, Q> {
        let mut result = Matrix::<R, Q>::new();

        // Each col in B will 'generate' a col in the result Matrix
        for col in 0..Q {
            for row in 0..R {
                for item in 0..C {
                    result.data[row][col] =
                        result.data[row][col] + self.data[row][item] * other.data[item][col]
                }
            }
        }
        return result;
    }
}

impl Mul<&Tuple> for &Matrix<4, 4> {
    type Output = Matrix<4, 1>;

    fn mul(self, t: &Tuple) -> Matrix<4, 1> {
        return self * &Matrix::<4, 1>::new_init([[t.x], [t.y], [t.z], [t.w]]);
    }
}

// The determinant is simple to calculate for 2x2 matrices
impl Matrix<2, 2> {
    fn det(self) -> f64 {
        let d = self.data;
        return d[0][0] * d[1][1] - d[0][1] * d[1][0];
    }
}

// Since feature(generic_const_exprs) is experimental (see https://github.com/rust-lang/rust/issues/76560)
// there is currently no generic way to implement certain operations. Thus, these needs to be
// specific.
impl Matrix<3, 3> {
    fn submatrix_3x3(&self, row: usize, col: usize) -> Option<Matrix<2, 2>> {
        if row > 2 || col > 2 {
            return None;
        }

        let mut s = Matrix::<2, 2>::new();
        let mut out_row = 0;

        for r in 0..3 {
            let mut out_col = 0;
            if r != row {
                for c in 0..3 {
                    if c != col {
                        s.data[out_row][out_col] = self.data[r][c];
                        out_col = out_col + 1;
                    }
                }
                out_row = out_row + 1;
            }
        }

        return Some(s);
    }
}

impl Matrix<4, 4> {
    fn submatrix_4x4(&self, row: usize, col: usize) -> Option<Matrix<3, 3>> {
        if row > 3 || col > 3 {
            return None;
        }

        let mut s = Matrix::<3, 3>::new();
        let mut out_row = 0;

        for r in 0..4 {
            let mut out_col = 0;
            if r != row {
                for c in 0..4 {
                    if c != col {
                        s.data[out_row][out_col] = self.data[r][c];
                        out_col = out_col + 1;
                    }
                }
                out_row = out_row + 1;
            }
        }

        return Some(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix() {
        let m = Matrix::<3, 2>::new();
        assert!(m.data[0][0] == 0.0);
        assert!(m.data[0][1] == 0.0);
        assert!(m.data[1][0] == 0.0);
        assert!(m.data[1][1] == 0.0);
        assert!(m.data[2][0] == 0.0);
        assert!(m.data[2][1] == 0.0);
    }

    #[test]
    fn new_matrix_2x2_init() {
        let m = Matrix::<2, 2>::new_init([[1.0, 2.0], [3.0, 4.0]]);
        assert!(m.data[0][0] == 1.0);
        assert!(m.data[0][1] == 2.0);
        assert!(m.data[1][0] == 3.0);
        assert!(m.data[1][1] == 4.0);
    }
    #[test]
    fn new_matrix_3x3_init() {
        let m = Matrix::<3, 3>::new_init([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert!(m.data[0][0] == -3.0);
        assert!(m.data[1][1] == -2.0);
        assert!(m.data[2][2] == 1.0);
    }

    #[test]
    fn new_matrix_4x4_init() {
        let m = Matrix::<4, 4>::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert!(m.data[0][0] == 1.0);
        assert!(m.data[0][3] == 4.0);
        assert!(m.data[1][0] == 5.5);
        assert!(m.data[1][2] == 7.5);
        assert!(m.data[2][2] == 11.0);
        assert!(m.data[3][0] == 13.5);
        assert!(m.data[3][2] == 15.5);
    }

    #[test]
    fn matrices_identical() {
        let a = Matrix::<4, 4>::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(a, b);
    }

    #[test]
    fn matrices_diffrent() {
        let a = Matrix::new_init([
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
            [1.0, 2.0, 3.0, 4.0],
        ]);
        let b = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_ne!(a, b);
    }

    #[test]
    fn multiply_4x4_matrices() {
        let a = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new_init([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert_eq!(
            &a * &b,
            Matrix::new_init([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn multiply_different_size_matrices() {
        let a = Matrix::new_init([
            [1.0, 2.0, 3.0], //
            [4.0, 5.0, 6.0],
        ]);
        let b = Matrix::new_init([
            [7.0, 8.0],  //
            [9.0, 10.0], //
            [11.0, 12.0],
        ]);

        assert_eq!(
            &a * &b,
            Matrix::new_init([
                [58.0, 64.0], //
                [139.0, 154.0]
            ])
        );
    }

    #[test]
    fn multiply_4x4_matrix_with_4x1_matrix() {
        let a = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Matrix::new_init([
            [1.0], //
            [2.0], //
            [3.0], //
            [1.0], //
        ]);

        assert_eq!(
            &a * &t,
            Matrix::new_init([
                [18.0], //
                [24.0], //
                [33.0], //
                [1.0]
            ])
        );
    }

    #[test]
    fn multiply_4x4_matrix_with_tuple() {
        let a = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert_eq!(
            &a * &t,
            Matrix::new_init([
                [18.0], //
                [24.0], //
                [33.0], //
                [1.0]
            ])
        );
    }

    #[test]
    fn multiply_4x4_matrix_with_identity_matrix() {
        let a = Matrix::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let i = Matrix::<4, 4>::new_identity();
        assert!(&a * &i == a);
    }

    #[test]
    fn transpose_4x4_matrix() {
        let a = Matrix::new_init([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let a_transposed = Matrix::new_init([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert!(a.transpose() == a_transposed);
    }

    #[test]
    fn transpose_identity_matrix_returns_identity_matrix() {
        let identity = Matrix::<3, 3>::new_identity();
        assert!(identity.transpose() == identity);
    }

    #[test]
    fn calculate_determinant_for_2x2_matrix() {
        let m = Matrix::new_init([
            [1.0, 5.0],  //
            [-3.0, 2.0], //
        ]);
        assert!(m.det() == 17.0);
    }

    #[test]
    fn calculate_submatrix_for_3x3_matrix() {
        let m = Matrix::new_init([
            [1.0, 5.0, 0.0],  //
            [-3.0, 2.0, 7.0], //
            [0.0, 6.0, -3.0], //
        ]);
        assert_eq!(m.submatrix_3x3(3, 0), None);
        assert_eq!(m.submatrix_3x3(0, 3), None);
        assert_eq!(
            m.submatrix_3x3(0, 2),
            Some(Matrix::new_init([
                [-3.0, 2.0], //
                [0.0, 6.0],  //
            ]))
        );
        assert_eq!(
            m.submatrix_3x3(2, 1),
            Some(Matrix::new_init([
                [1.0, 0.0],  //
                [-3.0, 7.0], //
            ]))
        );
    }

    #[test]
    fn calculate_subnatrix_for_4x4_matrix() {
        let m = Matrix::new_init([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        assert_eq!(m.submatrix_4x4(4, 0), None);
        assert_eq!(m.submatrix_4x4(0, 4), None);
        assert_eq!(
            m.submatrix_4x4(2, 1),
            Some(Matrix::new_init([
                [-6.0, 1.0, 6.0],
                [-8.0, 8.0, 6.0],
                [-7.0, -1.0, 1.0],
            ]))
        );
        assert_eq!(
            m.submatrix_4x4(0, 3),
            Some(Matrix::new_init([
                [-8.0, 5.0, 8.0],
                [-1.0, 0.0, 8.0],
                [-7.0, 1.0, -1.0],
            ]))
        );
    }
}
