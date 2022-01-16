use crate::{constants::EPSILON, intersections::*, matrices::*, rays::*, tuples::*};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    origin: Tuple,
    radius: f64,
    transform: Matrix<4>,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && (self.radius - other.radius).abs() < EPSILON
    }
}

impl Sphere {
    pub const fn new() -> Self {
        Self {
            origin: Tuple::new(0.0, 0.0, 0.0, 1.0),
            radius: 1.0,
            transform: Matrix::<4>::identity(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let ray = ray.transform(&self.transform.inverse().unwrap());

        let sphere_to_ray = ray.origin - self.origin;

        let a = ray.direction.dot_product(ray.direction);
        let b = 2.0 * ray.direction.dot_product(sphere_to_ray);
        let c = sphere_to_ray.dot_product(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::empty();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        let diff = t1 - t2;

        if diff.abs() < EPSILON {
            Intersections::from(&[t1], *self)
        } else if diff < 0.0 {
            Intersections::from(&[t1, t2], *self)
        } else {
            Intersections::from(&[t2, t1], *self)
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4>) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transformations::*;

    #[test]
    fn test_intersect() {
        let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let sphere = Sphere::new();
        assert_eq!(
            Intersections(vec![
                Intersection::new(4.0, sphere),
                Intersection::new(6.0, sphere),
            ]),
            sphere.intersect(&ray)
        );

        let ray = Ray::new(point!(0, 1, -5), vector!(0, 0, 1));
        let sphere = Sphere::new();
        assert_eq!(
            Intersections(vec![Intersection::new(5.0, sphere)]),
            sphere.intersect(&ray)
        );

        let ray = Ray::new(point!(0, 2, -5), vector!(0, 0, 1));
        let sphere = Sphere::new();
        assert_eq!(Intersections(vec![]), sphere.intersect(&ray));

        let ray = Ray::new(point!(0, 0, 0), vector!(0, 0, 1));
        let sphere = Sphere::new();
        assert_eq!(
            Intersections(vec![
                Intersection::new(-1.0, sphere),
                Intersection::new(1.0, sphere),
            ]),
            sphere.intersect(&ray)
        );

        let ray = Ray::new(point!(0, 0, 5), vector!(0, 0, 1));
        let sphere = Sphere::new();
        assert_eq!(
            Intersections(vec![
                Intersection::new(-6.0, sphere),
                Intersection::new(-4.0, sphere),
            ]),
            sphere.intersect(&ray)
        );

        let ray = Ray::new(point!(0, 0, -5), vector!(0, 0, 1));
        let mut sphere = Sphere::new();
        sphere.set_transform(scaling(2.0, 2.0, 2.0));
        assert_eq!(
            Intersections(vec![
                Intersection::new(3.0, sphere),
                Intersection::new(7.0, sphere),
            ]),
            sphere.intersect(&ray)
        );
    }

    #[test]
    fn test_transform() {
        let mut sphere = Sphere::new();

        assert_eq!(Matrix::<4>::identity(), sphere.transform);

        sphere.set_transform(translation(2.0, 3.0, 4.0));
        assert_eq!(translation(2.0, 3.0, 4.0), sphere.transform);
    }
}
