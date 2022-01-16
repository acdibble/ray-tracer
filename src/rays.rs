use crate::{matrices::*, tuples::*};

#[derive(Debug)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub const fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        Self {
            origin: self.origin.translate(x, y, z),
            direction: self.direction,
        }
    }

    fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        Self {
            origin: self.origin.scale(x, y, z),
            direction: self.direction.scale(x, y, z),
        }
    }

    pub fn transform(&self, transform: &Matrix<4>) -> Self {
        Self {
            origin: *transform * self.origin,
            direction: *transform * self.direction,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor() {
        let origin = point!(1, 2, 3);
        let direction = vector!(4, 5, 6);

        let ray = Ray::new(origin, direction);
        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn test_position() {
        let ray = Ray::new(point!(2, 3, 4), vector!(1, 0, 0));
        assert_eq!(point!(2, 3, 4), ray.position(0.0));
        assert_eq!(point!(3, 3, 4), ray.position(1.0));
        assert_eq!(point!(1, 3, 4), ray.position(-1.0));
        assert_eq!(point!(4.5, 3, 4), ray.position(2.5));
    }

    #[test]
    fn test_translation() {
        let ray = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
        let moved = ray.translate(3.0, 4.0, 5.0);
        assert_eq!(point!(4, 6, 8), moved.origin);
        assert_eq!(vector!(0, 1, 0), moved.direction);
    }

    #[test]
    fn test_scale() {
        let ray = Ray::new(point!(1, 2, 3), vector!(0, 1, 0));
        let moved = ray.scale(2.0, 3.0, 4.0);
        assert_eq!(point!(2, 6, 12), moved.origin);
        assert_eq!(vector!(0, 3, 0), moved.direction);
    }
}
