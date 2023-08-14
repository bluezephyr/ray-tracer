use std::ops::Mul;

pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    fn new() -> Self {
        let data = [[0.0; COLS]; ROWS];
        Matrix { data }
    }

    fn new_init(data: [[f64; COLS]; ROWS]) -> Self {
        Matrix { data }
    }
}

impl<const ROWS: usize, const COLS: usize> PartialEq for Matrix<ROWS, COLS> {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

// A * B = M
impl<const ROWS: usize, const COLS: usize> Mul for Matrix<ROWS, COLS> {
    type Output = Matrix<ROWS, COLS>;

    fn mul(self, other: Matrix<ROWS, COLS>) -> Matrix<ROWS, COLS> {
        let mut result = Matrix::<ROWS, COLS>::new();

        // Each col in B will 'generate' a col in M
        for col in 0..COLS {
            for row in 0..ROWS {
                for item in 0..ROWS {
                    result.data[row][col] =
                        result.data[row][col] + self.data[row][item] * other.data[item][col]
                }
            }
        }
        return result;
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
        let b = Matrix::<4, 4>::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert!(a == b);
    }

    #[test]
    fn matrices_diffrent() {
        let a = Matrix::<4, 4>::new_init([
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
            [1.0, 2.0, 3.0, 4.0],
        ]);
        let b = Matrix::<4, 4>::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert!(a != b);
    }

    #[test]
    fn multiply_4x4_matrices() {
        let a = Matrix::<4, 4>::new_init([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::<4, 4>::new_init([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        assert!(
            a * b
                == Matrix::<4, 4>::new_init([
                    [20.0, 22.0, 50.0, 48.0],
                    [44.0, 54.0, 114.0, 108.0],
                    [40.0, 58.0, 110.0, 102.0],
                    [16.0, 26.0, 46.0, 42.0],
                ])
        );
    }
}
