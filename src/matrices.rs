use crate::tuples::*;
use std::fmt::Debug;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Matrix<N> {
  pub const fn new(input: [[f64; N]; N]) -> Self {
    Self(input)
  }

  fn transpose(&self) -> Self {
    let mut output = [[0.0f64; N]; N];

    for (i, row) in self.0.iter().enumerate() {
      for (j, value) in row.iter().enumerate() {
        output[j][i] = *value;
      }
    }

    Matrix::new(output)
  }
}

impl<const N: usize> ops::Index<usize> for Matrix<N> {
  type Output = [f64; N];

  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl<const N: usize> ops::Mul<Self> for Matrix<N> {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    let mut output = [[0.0; N]; N];

    for row in 0..N {
      for col in 0..N {
        output[row][col] = (0..N).fold(0.0, |acc, n| acc + self[row][n] * other[n][col]);
      }
    }

    Matrix::new(output)
  }
}

impl ops::Mul<Tuple> for Matrix<4> {
  type Output = Tuple;

  fn mul(self, Tuple(x, y, z, w): Tuple) -> Self::Output {
    let mut results = [0.0; 4];

    for (i, [a, b, c, d]) in self.0.into_iter().enumerate() {
      results[i] = a * x + b * y + c * z + d * w;
    }

    Tuple::from(results)
  }
}

impl<const N: usize> PartialEq for Matrix<N> {
  fn eq(&self, other: &Self) -> bool {
    const EPSILON: f64 = 0.00001;

    for (row_a, row_b) in self.0.iter().zip(other.0.iter()) {
      for (a, b) in row_a.iter().zip(row_b.iter()) {
        if (a - b).abs() > EPSILON {
          return false;
        }
      }
    }

    true
  }
}

impl Matrix<2> {
  fn determinant(&self) -> f64 {
    let Matrix([[a, b], [c, d]]) = self;

    a * d - b * c
  }
}

macro_rules! define_methods {
  ($size:literal) => {
    impl Matrix<$size> {
      fn submatrix(&self, exclude_row: usize, exclude_column: usize) -> Matrix<{ $size - 1 }> {
        let mut output = [[0.0f64; ($size - 1)]; ($size - 1)];

        let mut count = 0;

        for (i, row) in self.0.iter().enumerate() {
          if i == exclude_row {
            continue;
          }

          for (j, val) in row.iter().enumerate() {
            if j == exclude_column {
              continue;
            }

            let row_num = count / ($size - 1);
            let col_num = count % ($size - 1);

            output[row_num][col_num] = *val;
            count += 1;
          }
        }

        Matrix(output)
      }

      fn minor(&self, exclude_row: usize, exclude_column: usize) -> f64 {
        self.submatrix(exclude_row, exclude_column).determinant()
      }

      fn cofactor(&self, exclude_row: usize, exclude_column: usize) -> f64 {
        let minor = self.minor(exclude_row, exclude_column);

        match (exclude_row + exclude_column) % 2 {
          1 => -minor,
          _ => minor,
        }
      }

      fn determinant(&self) -> f64 {
        (0..$size).fold(0.0, |acc, col| acc + self.cofactor(0, col) * self.0[0][col])
      }

      pub fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();

        if determinant == 0.0 {
          return None;
        }

        let mut output = [[0.0; $size]; $size];

        for row in 0..$size {
          for col in 0..$size {
            let cofactor = self.cofactor(row, col);

            output[col][row] = cofactor / determinant;
          }
        }

        Some(Matrix(output))
      }
    }
  };
}

define_methods!(4);
define_methods!(3);

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_constructor() {
    let matrix = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [5.5, 6.5, 7.5, 8.5],
      [9.0, 10.0, 11.0, 12.0],
      [13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(1.0, matrix[0][0]);
    assert_eq!(4.0, matrix[0][3]);
    assert_eq!(5.5, matrix[1][0]);
    assert_eq!(7.5, matrix[1][2]);
    assert_eq!(11.0, matrix[2][2]);
    assert_eq!(13.5, matrix[3][0]);
    assert_eq!(15.5, matrix[3][2]);

    let matrix = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);
    assert_eq!(-3.0, matrix[0][0]);
    assert_eq!(5.0, matrix[0][1]);
    assert_eq!(1.0, matrix[1][0]);
    assert_eq!(-2.0, matrix[1][1]);
  }

  #[test]
  fn test_equal() {
    let a = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);
    let b = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);

    assert_eq!(a, b);

    let a = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);
    let b = Matrix::new([
      [5.0, 4.0, 3.0, 2.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 6.0, 7.0, 8.0],
      [1.0, 2.0, 3.0, 4.0],
    ]);

    assert_ne!(a, b);
  }

  #[test]
  fn test_mul() {
    let a = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [5.0, 6.0, 7.0, 8.0],
      [9.0, 8.0, 7.0, 6.0],
      [5.0, 4.0, 3.0, 2.0],
    ]);

    let b = Matrix::new([
      [-2.0, 1.0, 2.0, 3.0],
      [3.0, 2.0, 1.0, -1.0],
      [4.0, 3.0, 6.0, 5.0],
      [1.0, 2.0, 7.0, 8.0],
    ]);

    let c = Matrix::new([
      [20.0, 22.0, 50.0, 48.0],
      [44.0, 54.0, 114.0, 108.0],
      [40.0, 58.0, 110.0, 102.0],
      [16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(c, a * b);
  }

  #[test]
  fn test_mul_tuple() {
    let matrix = Matrix::new([
      [1.0, 2.0, 3.0, 4.0],
      [2.0, 4.0, 4.0, 2.0],
      [8.0, 6.0, 4.0, 1.0],
      [0.0, 0.0, 0.0, 1.0],
    ]);
    let tuple = point!(1, 2, 3);

    assert_eq!(point!(18, 24, 33), matrix * tuple)
  }

  #[test]
  fn test_identity() {
    let identity = [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ];

    let matrix = Matrix::new([
      [0.0, 1.0, 2.0, 4.0],
      [1.0, 2.0, 4.0, 8.0],
      [2.0, 4.0, 8.0, 16.0],
      [4.0, 8.0, 16.0, 32.0],
    ]);

    assert_eq!(matrix.clone(), matrix * Matrix::new(identity));

    let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

    assert_eq!(tuple, Matrix::new(identity) * tuple);
  }

  #[test]
  fn test_transpose() {
    let a = Matrix::new([
      [0.0, 9.0, 3.0, 0.0],
      [9.0, 8.0, 0.0, 8.0],
      [1.0, 8.0, 5.0, 3.0],
      [0.0, 0.0, 5.0, 8.0],
    ]);

    assert_eq!(
      Matrix::new([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0]
      ]),
      a.transpose()
    );
  }

  #[test]
  fn test_determinant_2() {
    let matrix = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);

    assert_eq!(17.0, matrix.determinant());
  }

  #[test]
  fn test_determinant_3() {
    let matrix = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

    assert_eq!(-196.0, matrix.determinant());
  }

  #[test]
  fn test_determinant_4() {
    let matrix = Matrix::new([
      [-2.0, -8.0, 3.0, 5.0],
      [-3.0, 1.0, 7.0, 3.0],
      [1.0, 2.0, -9.0, 6.0],
      [-6.0, 7.0, 7.0, -9.0],
    ]);

    assert_eq!(-4071.0, matrix.determinant());
  }

  #[test]
  fn test_submatrix_3() {
    let matrix = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

    assert_eq!(
      Matrix::new([[-3.0, 2.0], [0.0, 6.0]]),
      matrix.submatrix(0, 2)
    );
  }

  #[test]
  fn test_submatrix_4() {
    let matrix = Matrix::new([
      [-6.0, 1.0, 1.0, 6.0],
      [-8.0, 5.0, 8.0, 6.0],
      [-1.0, 0.0, 8.0, 2.0],
      [-7.0, 1.0, -1.0, 1.0],
    ]);

    assert_eq!(
      Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]),
      matrix.submatrix(2, 1)
    );
  }

  #[test]
  fn test_minor_3() {
    let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    assert_eq!(25.0, matrix.minor(1, 0));
  }

  #[test]
  fn test_cofactor_3() {
    let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

    assert_eq!(-12.0, matrix.minor(0, 0));
    assert_eq!(-12.0, matrix.cofactor(0, 0));
    assert_eq!(25.0, matrix.minor(1, 0));
    assert_eq!(-25.0, matrix.cofactor(1, 0));
  }

  #[test]
  fn test_inverse() {
    let matrix = Matrix::new([
      [8.0, -5.0, 9.0, 2.0],
      [7.0, 5.0, 6.0, 1.0],
      [-6.0, 0.0, 9.0, 6.0],
      [-3.0, 0.0, -9.0, -4.0],
    ]);

    assert_eq!(
      Some(Matrix::new([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
      ])),
      matrix.inverse()
    );

    let matrix = Matrix::new([
      [9.0, 3.0, 0.0, 9.0],
      [-5.0, -2.0, -6.0, -3.0],
      [-4.0, 9.0, 6.0, 4.0],
      [-7.0, 6.0, 6.0, 2.0],
    ]);

    assert_eq!(
      Some(Matrix::new([
        [-0.04074, -0.07778, 0.14444, -0.22222],
        [-0.07778, 0.03333, 0.36667, -0.33333],
        [-0.02901, -0.14630, -0.10926, 0.12963],
        [0.17778, 0.06667, -0.26667, 0.33333],
      ])),
      matrix.inverse()
    );

    let a = Matrix::new([
      [3.0, -9.0, 7.0, 3.0],
      [3.0, -8.0, 2.0, -9.0],
      [-4.0, 4.0, 4.0, 1.0],
      [-6.0, 5.0, -1.0, 1.0],
    ]);
    let b = Matrix::new([
      [8.0, 2.0, 2.0, 2.0],
      [3.0, -1.0, 7.0, 0.0],
      [7.0, 0.0, 5.0, 4.0],
      [6.0, -2.0, 0.0, 5.0],
    ]);
    let c = a * b;
    assert_eq!(a, c * b.inverse().unwrap());
  }
}
