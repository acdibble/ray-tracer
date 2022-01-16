use crate::{constants::EPSILON, spheres::*};

#[derive(Debug)]
pub struct Intersection {
    t: f64,
    object: Sphere,
}

impl Intersection {
    pub const fn new(t: f64, object: Sphere) -> Self {
        Self { t, object }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.object == other.object && (self.t - other.t).abs() < EPSILON
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections(pub Vec<Intersection>);

impl Intersections {
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn from(ts: &[f64], object: Sphere) -> Self {
        Self(ts.iter().map(|t| Intersection::new(*t, object)).collect())
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.0.iter().fold(None, |acc, intersection| {
            if intersection.t < 0.0 {
                return acc;
            }

            match acc {
                None => Some(intersection),
                Some(other) if intersection.t < other.t => Some(intersection),
                _ => acc,
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection_constructor() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(3.5, sphere);

        assert_eq!(3.5, intersection.t);
        assert_eq!(sphere, intersection.object);
    }

    #[test]
    fn test_hit() {
        let sphere = Sphere::new();

        let xs = Intersections::from(&[2.0, 1.0], sphere);
        assert_eq!(Intersection::new(1.0, sphere), *xs.hit().unwrap());

        let xs = Intersections::from(&[1.0, -1.0], sphere);
        assert_eq!(Intersection::new(1.0, sphere), *xs.hit().unwrap());

        let xs = Intersections::from(&[-1.0, -2.0], sphere);
        assert_eq!(None, xs.hit());

        let xs = Intersections::from(&[5.0, 7.0, -3.0, 2.0], sphere);
        assert_eq!(Intersection::new(2.0, sphere), *xs.hit().unwrap());
    }
}
