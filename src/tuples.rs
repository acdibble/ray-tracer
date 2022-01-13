mod tuples {
  use std::ops;

  #[derive(Debug)]
  struct Tuple(f64, f64, f64, f64);

  impl Tuple {
    fn new<X, Y, Z, W>(x: X, y: Y, z: Z, w: W) -> Self
    where
      X: std::convert::Into<f64>,
      Y: std::convert::Into<f64>,
      Z: std::convert::Into<f64>,
      W: std::convert::Into<f64>,
    {
      Self(x.into(), y.into(), z.into(), w.into())
    }

    fn from_vector<X, Y, Z>(x: X, y: Y, z: Z) -> Self
    where
      X: std::convert::Into<f64>,
      Y: std::convert::Into<f64>,
      Z: std::convert::Into<f64>,
    {
      Self::new(x, y, z, 0)
    }

    fn from_point<X, Y, Z>(x: X, y: Y, z: Z) -> Self
    where
      X: std::convert::Into<f64>,
      Y: std::convert::Into<f64>,
      Z: std::convert::Into<f64>,
    {
      Self::new(x, y, z, 1)
    }

    fn from_color<X, Y, Z>(x: X, y: Y, z: Z) -> Self
    where
      X: std::convert::Into<f64>,
      Y: std::convert::Into<f64>,
      Z: std::convert::Into<f64>,
    {
      Self::new(x, y, z, 0)
    }

    fn is_vector(&self) -> bool {
      self.3 == 0.
    }

    fn is_point(&self) -> bool {
      self.3 == 1.
    }

    pub fn magnitude(self) -> f64 {
      let Tuple(x, y, z, w) = self;

      (x.powf(2.0) + y.powf(2.0) + z.powf(2.0) + w.powf(2.0)).sqrt()
    }

    pub fn normalize(self) -> Self {
      let Tuple(x, y, z, w) = self;
      let magnitude = self.magnitude();

      Tuple(x / magnitude, y / magnitude, z / magnitude, w / magnitude)
    }

    pub fn dot_product(self, Tuple(x2, y2, z2, w2): Self) -> f64 {
      let Tuple(x1, y1, z1, w1) = self;

      x1 * x2 + y1 * y2 + z1 * z2 + w1 * w2
    }

    pub fn cross_product(self, Tuple(x2, y2, z2, _): Self) -> Self {
      let Tuple(x1, y1, z1, _) = self;

      Tuple::from_vector(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }

    pub fn hadamard_product(self, Tuple(r2, g2, b2, _): Self) -> Self {
      let Tuple(r1, g1, b1, _) = self;

      Tuple::from_vector(r1 * r2, g1 * g2, b1 * b2)
    }
  }

  impl ops::Add<Self> for Tuple {
    type Output = Self;

    fn add(self, Tuple(x2, y2, z2, w2): Self) -> Self {
      let Tuple(x1, y1, z1, w1) = self;

      Tuple(x1 + x2, y1 + y2, z1 + z2, w1 + w2)
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
    fn eq(&self, Tuple(x2, y2, z2, w2): &Self) -> bool {
      const EPSILON: f64 = 0.00001;

      let Tuple(x1, y1, z1, w1) = self;

      ((x1 - x2).abs() < EPSILON)
        && ((y1 - y2).abs() < EPSILON)
        && ((z1 - z2).abs() < EPSILON)
        && ((w1 - w2).abs() < EPSILON)
    }
  }

  impl Eq for Tuple {}

  impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
      let Tuple(x, y, z, w) = self;

      Self(-x, -y, -z, -w)
    }
  }

  impl ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
      let Tuple(x, y, z, w) = self;

      Tuple(x * scalar, y * scalar, z * scalar, w * scalar)
    }
  }

  impl ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
      let Tuple(x, y, z, w) = self;

      Tuple(x / scalar, y / scalar, z / scalar, w / scalar)
    }
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_point() {
      let point = Tuple(4.3, -4.2, 3.1, 1.0);
      assert_eq!(4.3, point.0);
      assert_eq!(-4.2, point.1);
      assert_eq!(3.1, point.2);
      assert_eq!(1.0, point.3);
      assert!(point.is_point());
      assert!(!point.is_vector());
    }

    #[test]
    fn test_vector() {
      let point = Tuple(4.3, -4.2, 3.1, 0.0);
      assert_eq!(4.3, point.0);
      assert_eq!(-4.2, point.1);
      assert_eq!(3.1, point.2);
      assert_eq!(0.0, point.3);
      assert!(!point.is_point());
      assert!(point.is_vector())
    }

    #[test]
    fn test_point_constructor() {
      assert_eq!(Tuple::new(4, -4, 3, 1), Tuple::from_point(4, -4, 3))
    }

    #[test]
    fn test_vector_constructor() {
      assert_eq!(Tuple::new(4, -4, 3, 0), Tuple::from_vector(4, -4, 3))
    }

    #[test]
    fn test_add() {
      let a1 = Tuple::new(3, -2, 5, 1);
      let a2 = Tuple::new(-2, 3, 1, 0);

      assert_eq!(Tuple::new(1, 1, 6, 1), a1 + a2);
    }

    #[test]
    fn test_sub() {
      let p1 = Tuple::from_point(3, 2, 1);
      let p2 = Tuple::from_point(5, 6, 7);

      assert_eq!(Tuple::from_vector(-2, -4, -6), p1 - p2);

      let p = Tuple::from_point(3, 2, 1);
      let v = Tuple::from_vector(5, 6, 7);
      assert_eq!(Tuple::from_point(-2, -4, -6), p - v);

      let v1 = Tuple::from_vector(3, 2, 1);
      let v2 = Tuple::from_vector(5, 6, 7);
      assert_eq!(Tuple::from_vector(-2, -4, -6), v1 - v2);
    }

    #[test]
    fn test_neg() {
      assert_eq!(Tuple::new(-1, 2, -3, 4), -Tuple::new(1, -2, 3, -4))
    }

    #[test]
    fn test_mul() {
      assert_eq!(
        Tuple::new(3.5, -7, 10.5, -14),
        Tuple::new(1, -2, 3, -4) * 3.5
      );
      assert_eq!(Tuple::new(0.5, -1, 1.5, -2), Tuple::new(1, -2, 3, -4) * 0.5);
    }

    #[test]
    fn test_div() {
      assert_eq!(Tuple::new(0.5, -1, 1.5, -2), Tuple::new(1, -2, 3, -4) / 2.);
    }

    #[test]
    fn test_magnitude() {
      assert_eq!(1.0, Tuple::from_vector(1, 0, 0).magnitude());
      assert_eq!(1, Tuple::from_vector(0, 1, 0).magnitude());
      assert_eq!(1, Tuple::from_vector(0, 0, 1).magnitude());
      assert_eq!(14f64.sqrt(), Tuple::from_vector(1, 2, 3).magnitude());
      assert_eq!(14f64.sqrt(), Tuple::from_vector(-1, -2, -3).magnitude());
    }

    #[test]
    fn test_normalize() {
      assert_eq!(
        Tuple::from_vector(1, 0, 0),
        Tuple::from_vector(4, 0, 0).normalize()
      );
      assert_eq!(
        Tuple::from_vector(0.26726, 0.53452, 0.80178),
        Tuple::from_vector(1, 2, 3).normalize()
      );
      assert_eq!(1.0, Tuple::from_vector(1, 2, 3).normalize().magnitude());
    }

    #[test]
    fn test_dot_product() {
      assert_eq!(
        20.0,
        Tuple::from_vector(1, 2, 3).dot_product(Tuple::from_vector(2, 3, 4))
      );
    }

    #[test]
    fn test_cross_product() {
      assert_eq!(
        Tuple::from_vector(-1, 2, -1),
        Tuple::from_vector(1, 2, 3).cross_product(Tuple::from_vector(2, 3, 4))
      );
      assert_eq!(
        Tuple::from_vector(1, -2, 1),
        Tuple::from_vector(2, 3, 4).cross_product(Tuple::from_vector(1, 2, 3))
      );
    }

    #[test]
    fn test_color_constructor() {
      let c = Tuple::from_color(-0.5, 0.4, 1.7);
      assert_eq!(-0.5, c.0);
      assert_eq!(0.4, c.1);
      assert_eq!(1.7, c.2);
    }

    #[test]
    fn test_hadamard_product() {
      let c1 = Tuple::from_color(1, 0.2, 0.4);
      let c2 = Tuple::from_color(0.9, 1, 0.1);
      assert_eq!(Tuple::from_color(0.9, 0.2, 0.04), c1.hadamard_product(c2));
    }
  }
}
