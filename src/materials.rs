use crate::{
    lights::PointLight,
    tuples::{color, Tuple},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub const fn new() -> Self {
        Self {
            color: color!(1, 1, 1),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
    ) -> Tuple {
        let effective_color = self.color.hadamard_product(light.intensity);

        let lightv = (light.position - *position).normalize();

        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot_product(*normalv);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (color!(0, 0, 0), color!(0, 0, 0))
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflect_dot_eye = -lightv.reflect(*normalv).dot_product(*eyev);

            if reflect_dot_eye <= 0.0 {
                (diffuse, color!(0, 0, 0))
            } else {
                (
                    diffuse,
                    light.intensity * self.specular * reflect_dot_eye.powf(self.shininess),
                )
            }
        };

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tuples::*;

    #[test]
    fn test_constructor() {
        let material = Material::new();

        assert_eq!(color!(1, 1, 1), material.color);
        assert_eq!(0.1, material.ambient);
        assert_eq!(0.9, material.diffuse);
        assert_eq!(0.9, material.specular);
        assert_eq!(200.0, material.shininess);
    }

    #[test]
    fn test_lighting() {
        let material = Material::new();
        let position = point!(0, 0, 0);

        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        assert_eq!(
            color!(1.9, 1.9, 1.9),
            material.lighting(&light, &position, &eyev, &normalv)
        );

        let eyev = vector!(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = vector!(0, 0, -1);
        let light = PointLight::new(point!(0, 0, -10), color!(1, 1, 1));
        assert_eq!(
            color!(1.0, 1.0, 1.0),
            material.lighting(&light, &position, &eyev, &normalv)
        );

        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = PointLight::new(point!(0, 10, -10), color!(1, 1, 1));
        assert_eq!(
            color!(0.7364, 0.7364, 0.7364),
            material.lighting(&light, &position, &eyev, &normalv)
        );

        let eyev = vector!(0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = vector!(0, 0, -1);
        let light = PointLight::new(point!(0, 10, -10), color!(1, 1, 1));
        assert_eq!(
            color!(1.6364, 1.6364, 1.6364),
            material.lighting(&light, &position, &eyev, &normalv)
        );

        let eyev = vector!(0, 0, -1);
        let normalv = vector!(0, 0, -1);
        let light = PointLight::new(point!(0, 0, 10), color!(1, 1, 1));
        assert_eq!(
            color!(0.1, 0.1, 0.1),
            material.lighting(&light, &position, &eyev, &normalv)
        );
    }
}
