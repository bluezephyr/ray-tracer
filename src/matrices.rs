use crate::Tuple;
use std::f64;
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
//  For square matrices, R is used, e.g., Matrix<R, R>
//  If more than one matrix is used, the notation for the second matrix is
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

pub trait Submatrix<Rhs = Self> {
    type Output;

    // Specifiy the row and column to delete
    // Note that the size of the output matrix must be R-1, C-1 for a RxC matrix.
    fn submatrix(self, row: usize, col: usize) -> Option<Self::Output>;
}

const EPSILON: f64 = 0.00001;

impl<const R: usize, const C: usize> Matrix<R, C> {
    fn new() -> Self {
        let data = [[0.0; C]; R];
        Matrix { data }
    }

    fn new_init(data: [[f64; C]; R]) -> Self {
        Matrix { data }
    }
}

// Matrix transformations for 4x4 matrices
impl Matrix<4, 4> {
    fn translation(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    fn scaling(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    // Rotation around the x axis in radians
    fn rotation_x(r: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    // Rotation around the y axis in radians
    fn rotation_y(r: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    // Rotation around the z axis in radians
    fn rotation_z(r: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4, 4> {
        return Matrix::<4, 4>::new_init([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }
}

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

    // The functions minor, cofactor, determinant (det), and inversion are only
    // implemented for small, square matrices) - up to 4x4 matrices are supported.
    fn minor(&self, row: usize, col: usize) -> Option<f64> {
        match R {
            2 => return self.det(),
            3 => {
                let m: Matrix<2, 2> = self.submatrix(row, col).unwrap();
                return m.det();
            }
            4 => {
                let m: Matrix<3, 3> = self.submatrix(row, col).unwrap();
                return m.det();
            }
            _ => return None,
        }
    }

    fn cofactor(&self, row: usize, col: usize) -> Option<f64> {
        if R > 4 {
            return None;
        }

        let mut min = self.minor(row, col).unwrap();
        if (row + col) % 2 != 0 {
            min = -min;
        }

        return Some(min);
    }

    fn det(&self) -> Option<f64> {
        let data = self.data;
        match R {
            // The determinant for a 2x2 matrix is simple to calculate
            2 => return Some(data[0][0] * data[1][1] - data[0][1] * data[1][0]),

            // The determinant for a larger matrix is calculated by adding each item in
            // one row (first row selected) multiplied by its cofactor. The operation
            // uses the cofactor, minor, and det functions recursively.
            3 | 4 => {
                let mut det = 0.0;
                for col in 0..R {
                    det = det + data[0][col] * self.cofactor(0, col).unwrap();
                }
                return Some(det);
            }
            _ => return None,
        }
    }

    fn invert(&self) -> Option<Matrix<R, R>> {
        if R > 4 {
            return None;
        }

        let det = self.det().unwrap();
        if det == 0.0 {
            return None;
        }

        let mut m = Matrix::<R, R>::new();
        for row in 0..R {
            for col in 0..R {
                let c = self.cofactor(row, col).unwrap();
                m.data[col][row] = c / det;
            }
        }

        return Some(m);
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..R {
            for col in 0..C {
                if (self.data[row][col] - other.data[row][col]).abs() > EPSILON {
                    return false;
                }
            }
        }
        return true;
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

fn to_matrix(tuple: &Tuple) -> Matrix<4, 1> {
    return Matrix::<4, 1>::new_init([[tuple.x], [tuple.y], [tuple.z], [tuple.w]]);
}

impl Mul<&Tuple> for &Matrix<4, 4> {
    type Output = Matrix<4, 1>;

    fn mul(self, t: &Tuple) -> Matrix<4, 1> {
        return self * &to_matrix(t);
    }
}

// Since feature(generic_const_exprs) is experimental (see https://github.com/rust-lang/rust/issues/76560)
// there is currently no generic way to check the size of the output at compile time.
impl<const R: usize, const C: usize, const P: usize, const Q: usize> Submatrix<&Matrix<P, Q>>
    for &Matrix<R, C>
{
    type Output = Matrix<P, Q>;

    fn submatrix(self, row: usize, col: usize) -> Option<Matrix<P, Q>> {
        // Make sure that the input parameters and size of the output matrix are correct
        if row >= R || col >= C || R - 1 != P || C - 1 != Q {
            return None;
        }

        let mut s = Matrix::<P, Q>::new();
        let mut out_row = 0;

        for r in 0..R {
            let mut out_col = 0;
            if r != row {
                for c in 0..C {
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
    fn calculate_submatrix_for_3x3_matrix() {
        let m = Matrix::new_init([
            [1.0, 5.0, 0.0],  //
            [-3.0, 2.0, 7.0], //
            [0.0, 6.0, -3.0], //
        ]);
        let m_sub: Option<Matrix<2, 2>> = m.submatrix(3, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<2, 2>> = m.submatrix(0, 3);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<1, 2>> = m.submatrix(0, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<2, 3>> = m.submatrix(0, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<2, 2>> = m.submatrix(0, 2);
        assert_eq!(
            m_sub,
            Some(Matrix::new_init([
                [-3.0, 2.0], //
                [0.0, 6.0],  //
            ]))
        );
        let m_sub: Option<Matrix<2, 2>> = m.submatrix(2, 1);
        assert_eq!(
            m_sub,
            Some(Matrix::new_init([
                [1.0, 0.0],  //
                [-3.0, 7.0], //
            ]))
        );
    }

    #[test]
    fn calculate_submatrix_for_4x4_matrix() {
        let m = Matrix::new_init([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let m_sub: Option<Matrix<3, 3>> = m.submatrix(4, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<3, 3>> = m.submatrix(0, 4);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<2, 3>> = m.submatrix(0, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<3, 2>> = m.submatrix(0, 0);
        assert_eq!(m_sub, None);
        let m_sub: Option<Matrix<3, 3>> = m.submatrix(2, 1);
        assert_eq!(
            m_sub,
            Some(Matrix::new_init([
                [-6.0, 1.0, 6.0],
                [-8.0, 8.0, 6.0],
                [-7.0, -1.0, 1.0],
            ]))
        );
        let m_sub: Option<Matrix<3, 3>> = m.submatrix(0, 3);
        assert_eq!(
            m_sub,
            Some(Matrix::new_init([
                [-8.0, 5.0, 8.0],
                [-1.0, 0.0, 8.0],
                [-7.0, 1.0, -1.0],
            ]))
        );
    }

    #[test]
    fn calculate_minor_of_a_3x3_matrix() {
        let m = Matrix::new_init([
            [3.0, 5.0, 0.0],   //
            [2.0, -1.0, -7.0], //
            [6.0, -1.0, 5.0],  //
        ]);
        let sub: Matrix<2, 2> = m.submatrix(1, 0).unwrap();
        assert_eq!(m.minor(1, 0), sub.det());
    }

    #[test]
    fn calculate_cofactor_for_3x3_matrix() {
        let m = Matrix::new_init([
            [3.0, 5.0, 0.0],   //
            [2.0, -1.0, -7.0], //
            [6.0, -1.0, 5.0],  //
        ]);
        assert_eq!(m.minor(0, 0), Some(-12.0));
        assert_eq!(m.cofactor(0, 0), Some(-12.0));
        assert_eq!(m.minor(1, 0), Some(25.0));
        assert_eq!(m.cofactor(1, 0), Some(-25.0));
    }

    #[test]
    fn calculate_determinant_for_2x2_matrix() {
        let m = Matrix::new_init([
            [1.0, 5.0],  //
            [-3.0, 2.0], //
        ]);
        assert!(m.det() == Some(17.0));
    }

    #[test]
    fn calculate_determinant_for_3x3_matrix() {
        let m = Matrix::new_init([
            [1.0, 2.0, 6.0],   //
            [-5.0, 8.0, -4.0], //
            [2.0, 6.0, 4.0],   //
        ]);
        assert_eq!(m.cofactor(0, 0), Some(56.0));
        assert_eq!(m.cofactor(0, 1), Some(12.0));
        assert_eq!(m.cofactor(0, 2), Some(-46.0));
        assert_eq!(m.det(), Some(-196.0));
    }

    #[test]
    fn calculate_determinant_for_4x4_matrix() {
        let m = Matrix::new_init([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), Some(690.0));
        assert_eq!(m.cofactor(0, 1), Some(447.0));
        assert_eq!(m.cofactor(0, 2), Some(210.0));
        assert_eq!(m.cofactor(0, 3), Some(51.0));
        assert_eq!(m.det(), Some(-4071.0));
    }

    #[test]
    fn det_cofactor_minor_invert_not_supported_for_larger_matrices() {
        let m = Matrix::<5, 5>::new();
        assert_eq!(m.cofactor(0, 0), None);
        assert_eq!(m.minor(0, 0), None);
        assert_eq!(m.det(), None);
        assert_eq!(m.invert(), None);
    }

    #[test]
    fn matrix_with_det_0_is_not_invertible() {
        let m = Matrix::new_init([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m.invert(), None);
    }

    #[test]
    fn invert_4x4_matrix() {
        let m = Matrix::new_init([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let m_inv = m.invert().unwrap();
        let m_inv_calc = Matrix::new_init([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(m.det(), Some(532.0));
        assert_eq!(m.cofactor(0, 0), Some(116.0));
        assert_eq!(m.cofactor(0, 1), Some(-430.0));
        assert_eq!(m.cofactor(0, 2), Some(-42.0));
        assert_eq!(m.cofactor(0, 3), Some(-278.0));
        assert_eq!(m.cofactor(1, 0), Some(240.0));
        assert_eq!(m.cofactor(1, 1), Some(-775.0));
        assert_eq!(m.cofactor(1, 2), Some(-119.0));
        assert_eq!(m.cofactor(1, 3), Some(-433.0));
        assert_eq!(m.cofactor(2, 0), Some(128.0));
        assert_eq!(m.cofactor(2, 1), Some(-236.0));
        assert_eq!(m.cofactor(2, 2), Some(-28.0));
        assert_eq!(m.cofactor(2, 3), Some(-160.0));
        assert_eq!(m.cofactor(3, 0), Some(-24.0));
        assert_eq!(m.cofactor(3, 1), Some(277.0));
        assert_eq!(m.cofactor(3, 2), Some(105.0));
        assert_eq!(m.cofactor(3, 3), Some(163.0));
        assert_eq!(m_inv.data[2][3], 105.0 / 532.0);
        assert_eq!(m_inv.data[3][2], -160.0 / 532.0);
        assert_eq!(m_inv, m_inv_calc);
    }

    #[test]
    fn invert_4x4_matrix_2() {
        let m = Matrix::new_init([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let m_inv = Matrix::new_init([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(m.invert().unwrap(), m_inv);
    }

    #[test]
    fn invert_4x4_matrix_3() {
        let m = Matrix::new_init([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m_inv = Matrix::new_init([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(m.invert().unwrap(), m_inv);
    }

    #[test]
    fn multiply_matrix_product_with_its_inverse() {
        let m_a = Matrix::new_init([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m_b = Matrix::new_init([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let m_c = &m_a * &m_b;
        assert_eq!(&m_c * &m_b.invert().unwrap(), m_a);
    }

    #[test]
    fn invert_identity_matrix_returns_identity_matrix() {
        let identity = Matrix::<4, 4>::new_identity();
        assert_eq!(identity.invert().unwrap(), identity);
    }

    #[test]
    fn multiply_matrix_with_its_inverse_returns_identity_matrix() {
        let m = Matrix::new_init([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        assert_eq!(&m * &m.invert().unwrap(), Matrix::<4, 4>::new_identity());
    }

    #[test]
    fn inverted_transposed_matrix_equals_transposed_inverted_matrix() {
        let m = Matrix::new_init([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        assert_eq!(
            m.transpose().invert().unwrap(),
            m.invert().unwrap().transpose()
        );
    }

    #[test]
    fn multiply_with_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = &transform * &Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(p, to_matrix(&Tuple::point(2.0, 1.0, 7.0)));
    }

    #[test]
    fn multiply_with_inverted_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0).invert().unwrap();
        assert_eq!(
            &transform * &Tuple::point(-3.0, 4.0, 5.0),
            to_matrix(&Tuple::point(-8.0, 7.0, 3.0))
        );
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = &transform * &Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(v, to_matrix(&Tuple::vector(-3.0, 4.0, 5.0)));
    }

    #[test]
    fn scaling_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = &transform * &Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(p, to_matrix(&Tuple::point(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn scaling_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = &transform * &Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(v, to_matrix(&Tuple::vector(-8.0, 18.0, 32.0)));
    }

    #[test]
    fn multiply_with_inverted_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0).invert().unwrap();
        let v = &transform * &Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(v, to_matrix(&Tuple::vector(-2.0, 2.0, 2.0)));
    }

    #[test]
    fn reflection_applied_to_a_point() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(-2.0, 3.0, 4.0)));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let half_quarter = Matrix::rotation_x(f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(f64::consts::PI / 2.0);
        let p = &half_quarter * &Tuple::point(0.0, 1.0, 0.0);
        let p2 = &full_quarter * &Tuple::point(0.0, 1.0, 0.0);
        assert_eq!(
            p,
            to_matrix(&Tuple::point(
                0.0,
                2.0_f64.sqrt() / 2.0,
                2.0_f64.sqrt() / 2.0
            ))
        );
        assert_eq!(p2, to_matrix(&Tuple::point(0.0, 0.0, 1.0)));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let half_quarter = Matrix::rotation_y(f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(f64::consts::PI / 2.0);
        let p = &half_quarter * &Tuple::point(0.0, 0.0, 1.0);
        let p2 = &full_quarter * &Tuple::point(0.0, 0.0, 1.0);
        assert_eq!(
            p,
            to_matrix(&Tuple::point(
                2.0_f64.sqrt() / 2.0,
                0.0,
                2.0_f64.sqrt() / 2.0
            ))
        );
        assert_eq!(p2, to_matrix(&Tuple::point(1.0, 0.0, 0.0)));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let half_quarter = Matrix::rotation_z(f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(f64::consts::PI / 2.0);
        let p = &half_quarter * &Tuple::point(0.0, 1.0, 0.0);
        let p2 = &full_quarter * &Tuple::point(0.0, 1.0, 0.0);
        assert_eq!(
            p,
            to_matrix(&Tuple::point(
                -2.0_f64.sqrt() / 2.0,
                2.0_f64.sqrt() / 2.0,
                0.0
            ))
        );
        assert_eq!(p2, to_matrix(&Tuple::point(-1.0, 0.0, 0.0)));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(5.0, 3.0, 4.0)));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(6.0, 3.0, 4.0)));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(2.0, 5.0, 4.0)));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(2.0, 7.0, 4.0)));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(2.0, 3.0, 6.0)));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = &transform * &Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(p, to_matrix(&Tuple::point(2.0, 3.0, 7.0)));
    }

    #[test]
    fn transformations_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let p2 = &a * &p;
        assert_eq!(p2, to_matrix(&Tuple::point(1.0, -1.0, 0.0)));
        let p3 = &b * &p2;
        assert_eq!(p3, to_matrix(&Tuple::point(5.0, -5.0, 0.0)));
        let p4 = &c * &p3;
        assert_eq!(p4, to_matrix(&Tuple::point(15.0, 0.0, 7.0)));
    }

    #[test]
    fn chained_transformation_are_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let t = &(&c * &b) * &a;
        assert_eq!(&t * &p, to_matrix(&Tuple::point(15.0, 0.0, 7.0)));
    }
}
