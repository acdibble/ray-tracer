use crate::tuples::Tuple;
use std::fmt::Debug;
use std::ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> Matrix<N> {
  pub const fn new(input: [[f64; N]; N]) -> Self {
    Self(input)
  }

  pub fn transpose(self) -> Self {
    let mut output = [[0.0f64; N]; N];

    for (i, row) in self.0.into_iter().enumerate() {
      for (j, value) in row.into_iter().enumerate() {
        output[j][i] = value;
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
    let tuple = Tuple::new_point(1, 2, 3);

    assert_eq!(Tuple::new_point(18, 24, 33), matrix * tuple)
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

    let tuple = Tuple::new(1, 2, 3, 4);

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
}
