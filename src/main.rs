use std::{
    fs,
    io::{BufWriter, Write},
};

pub mod canvas;
pub mod constants;
pub mod convert;
pub mod intersections;
pub mod lights;
pub mod materials;
pub mod matrices;
pub mod rays;
pub mod spheres;
pub mod transformations;
#[macro_use]
pub mod tuples;

use canvas::Canvas;
use lights::PointLight;
use rays::Ray;
use spheres::Sphere;
use tuples::Tuple;

fn main() {
    let file = fs::File::create("out.ppm").expect("failed to open file");
    let mut writer = BufWriter::new(file);

    let canvas_pixels = 500;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let ray_origin = point!(0, 0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut sphere = Sphere::new();
    sphere.material.color = color!(0.1, 1, 0.1);

    let light = PointLight::new(point!(-10, 10, -10), color!(1, 1, 1));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = point!(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let intersections = sphere.intersect(&ray);

            if let Some(hit) = intersections.hit() {
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = -ray.direction;

                let color = hit.object.material.lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    canvas
        .write_out(&mut writer)
        .expect("failed to write ppm to file");
    writer.flush().expect("failed to flush buffer");
}
