use crate::{constants::EPSILON, transformations::*};
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

#[macro_export]
macro_rules! point {
  ($x:expr , $y:expr , $z:expr) => {
    Tuple::new($x as f64, $y as f64, $z as f64, 1.0)
  };
}

pub(crate) use point;

#[macro_export]
macro_rules! vector {
  ($x:expr , $y:expr , $z:expr) => {
    Tuple::new($x as f64, $y as f64, $z as f64, 0.0)
  };
}

pub(crate) use vector;

#[macro_export]
macro_rules! color {
  ($x:expr , $y:expr , $z:expr) => {
    Tuple::new($x as f64, $y as f64, $z as f64, 0.0)
  };
}

pub(crate) use color;

impl Tuple {
  pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
    Self(x, y, z, w)
  }

  pub const fn from([x, y, z, w]: [f64; 4]) -> Self {
    Self::new(x, y, z, w)
  }

  pub fn as_color(self) -> (f64, f64, f64) {
    let Tuple(r, g, b, kind) = self;

    assert_eq!(kind, 0.0);

    (r, g, b)
  }

  fn is_vector(&self) -> bool {
    self.3 == 0.0
  }

  fn is_point(&self) -> bool {
    self.3 == 1.0
  }

  pub fn magnitude(self) -> f64 {
    let Tuple(x, y, z, w) = self;

    (x.powf(2.0) + y.powf(2.0) + z.powf(2.0) + w.powf(2.0)).sqrt()
  }

  pub fn normalize(self) -> Self {
    let Tuple(x, y, z, kind) = self;

    assert_eq!(kind, 0.0);

    let magnitude = self.magnitude();

    vector!(x / magnitude, y / magnitude, z / magnitude)
  }

  pub fn dot_product(self, Tuple(x2, y2, z2, kind2): Self) -> f64 {
    let Tuple(x1, y1, z1, kind1) = self;
    assert_eq!(kind1, 0.0);
    assert_eq!(kind2, 0.0);

    x1 * x2 + y1 * y2 + z1 * z2
  }

  pub fn cross_product(self, Tuple(x2, y2, z2, kind1): Self) -> Self {
    let Tuple(x1, y1, z1, kind2) = self;
    assert_eq!(kind1, 0.0);
    assert_eq!(kind2, 0.0);

    vector!(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
  }

  pub fn hadamard_product(self, Tuple(r2, g2, b2, kind1): Self) -> Self {
    let Tuple(r1, g1, b1, kind2) = self;
    assert_eq!(kind1, 0.0);
    assert_eq!(kind2, 0.0);

    color!(r1 * r2, g1 * g2, b1 * b2)
  }

  pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
    translation(x, y, z) * self
  }

  pub fn rotate_x(self, radians: f64) -> Self {
    rotation(Axis::X, radians) * self
  }

  pub fn rotate_y(self, radians: f64) -> Self {
    rotation(Axis::Y, radians) * self
  }

  pub fn rotate_z(self, radians: f64) -> Self {
    rotation(Axis::Z, radians) * self
  }

  pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
    scaling(x, y, z) * self
  }

  pub fn shear(
    self,
    x_to_y: f64,
    x_to_z: f64,
    y_to_x: f64,
    y_to_z: f64,
    z_to_y: f64,
    z_to_x: f64,
  ) -> Self {
    shearing(x_to_y, x_to_z, y_to_x, y_to_z, z_to_y, z_to_x) * self
  }

  pub fn reflect(&self, normal: Self) -> Self {
    *self - normal * 2.0 * self.dot_product(normal)
  }
}

impl ops::Add<Self> for Tuple {
  type Output = Self;

  fn add(self, Tuple(x2, y2, z2, k2): Self) -> Self {
    let Tuple(x1, y1, z1, k1) = self;

    Tuple(x1 + x2, y1 + y2, z1 + z2, k1 + k2)
  }
}

impl ops::Sub<Self> for Tuple {
  type Output = Self;

  fn sub(self, Tuple(x2, y2, z2, w2): Self) -> Self {
    let Tuple(x1, y1, z1, w1) = self;

    Tuple(x1 - x2, y1 - y2, z1 - z2, w1 - w2)
  }
}

impl PartialEq for Tuple {
  fn eq(&self, Tuple(x2, y2, z2, kind2): &Self) -> bool {
    let Tuple(x1, y1, z1, kind1) = self;

    ((x1 - x2).abs() < EPSILON)
      && ((y1 - y2).abs() < EPSILON)
      && ((z1 - z2).abs() < EPSILON)
      && (kind1 == kind2)
  }
}

impl Eq for Tuple {}

impl ops::Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self::Output {
    let Tuple(x, y, z, kind) = self;

    Self(-x, -y, -z, kind)
  }
}

impl ops::Mul<f64> for Tuple {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self::Output {
    let Tuple(x, y, z, w) = self;

    Tuple(x * scalar, y * scalar, z * scalar, w)
  }
}

impl ops::Div<f64> for Tuple {
  type Output = Self;

  fn div(self, scalar: f64) -> Self::Output {
    let Tuple(x, y, z, w) = self;

    Tuple(x / scalar, y / scalar, z / scalar, w)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_point() {
    let point = point!(4.3, -4.2, 3.1);
    assert_eq!(4.3, point.0);
    assert_eq!(-4.2, point.1);
    assert_eq!(3.1, point.2);
    assert_eq!(1.0, point.3);
    assert!(point.is_point());
    assert!(!point.is_vector());
  }

  #[test]
  fn test_vector() {
    let point = vector!(4.3, -4.2, 3.1);
    assert_eq!(4.3, point.0);
    assert_eq!(-4.2, point.1);
    assert_eq!(3.1, point.2);
    assert_eq!(0.0, point.3);
    assert!(!point.is_point());
    assert!(point.is_vector())
  }

  #[test]
  fn test_add() {
    let a1 = vector!(3.0, -2.0, 5.0);
    let a2 = vector!(-2.0, 3.0, 1.0);

    assert_eq!(vector!(1.0, 1.0, 6.0), a1 + a2);
  }

  #[test]
  fn test_sub() {
    let p1 = point!(3.0, 2.0, 1.0);
    let p2 = point!(5.0, 6.0, 7.0);

    assert_eq!(vector!(-2.0, -4.0, -6.0), p1 - p2);

    let p = point!(3, 2, 1);
    let v = vector!(5, 6, 7);
    assert_eq!(point!(-2, -4, -6), p - v);

    let v1 = vector!(3, 2, 1);
    let v2 = vector!(5, 6, 7);
    assert_eq!(vector!(-2, -4, -6), v1 - v2);
  }

  #[test]
  fn test_neg() {
    assert_eq!(vector!(-1, 2, -3), -vector!(1, -2, 3))
  }

  #[test]
  fn test_mul() {
    assert_eq!(vector!(3.5, -7, 10.5), vector!(1, -2, 3) * 3.5);
    assert_eq!(vector!(0.5, -1, 1.5), vector!(1, -2, 3) * 0.5);
  }

  #[test]
  fn test_div() {
    assert_eq!(vector!(0.5, -1, 1.5), vector!(1, -2, 3) / 2.);
  }

  #[test]
  fn test_magnitude() {
    assert_eq!(1.0, vector!(1, 0, 0).magnitude());
    assert_eq!(1.0, vector!(0, 1, 0).magnitude());
    assert_eq!(1.0, vector!(0, 0, 1).magnitude());
    assert_eq!(14f64.sqrt(), vector!(1, 2, 3).magnitude());
    assert_eq!(14f64.sqrt(), vector!(-1, -2, -3).magnitude());
  }

  #[test]
  fn test_normalize() {
    assert_eq!(vector!(1, 0, 0), vector!(4, 0, 0).normalize());
    assert_eq!(
      vector!(0.26726, 0.53452, 0.80178),
      vector!(1, 2, 3).normalize()
    );
    assert_eq!(1.0, vector!(1, 2, 3).normalize().magnitude());
  }

  #[test]
  fn test_dot_product() {
    assert_eq!(20.0, vector!(1, 2, 3).dot_product(vector!(2, 3, 4)));
  }

  #[test]
  fn test_cross_product() {
    assert_eq!(
      vector!(-1, 2, -1),
      vector!(1, 2, 3).cross_product(vector!(2, 3, 4))
    );
    assert_eq!(
      vector!(1, -2, 1),
      vector!(2, 3, 4).cross_product(vector!(1, 2, 3))
    );
  }

  #[test]
  fn test_color_constructor() {
    let c = color!(-0.5, 0.4, 1.7);
    assert_eq!(-0.5, c.0);
    assert_eq!(0.4, c.1);
    assert_eq!(1.7, c.2);
    assert_eq!(0.0, c.3);
  }

  #[test]
  fn test_hadamard_product() {
    let c1 = color!(1, 0.2, 0.4);
    let c2 = color!(0.9, 1, 0.1);
    assert_eq!(color!(0.9, 0.2, 0.04), c1.hadamard_product(c2));
  }

  #[test]
  fn test_reflect() {
    let vector = vector!(1, -1, 0);
    let normal = vector!(0, 1, 0);
    assert_eq!(vector!(1, 1, 0), vector.reflect(normal));

    let vector = vector!(0, -1, 0);
    let normal = vector!(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0);
    assert_eq!(vector!(1, 0, 0), vector.reflect(normal));
  }
}
