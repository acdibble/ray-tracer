use std::fs;
use std::io::{BufWriter, Write};

pub mod canvas;
pub mod convert;
pub mod matrices;
pub mod tuples;

use canvas::Canvas;
use tuples::Tuple;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let file = fs::File::create("out.ppm").expect("failed to open file");
    let mut writer = BufWriter::new(file);
    let mut canvas = Canvas::new(900, 500);

    let mut projectile = Projectile {
        position: Tuple::new_point(0, 1, 0),
        velocity: Tuple::new_vector(1, 1.8, 0).normalize() * 11.25,
    };
    let env = Environment {
        gravity: Tuple::new_vector(0, -0.1, 0),
        wind: Tuple::new_vector(-0.01, 0, 0),
    };

    while projectile.position.1 > 0.0 {
        projectile = tick(&env, projectile);
        let x = projectile.position.0.round() as usize;
        let y = canvas.height - (projectile.position.1.round() as usize);
        canvas.write_pixel(x, y, Tuple::new_color(1, 0, 0));
    }

    canvas
        .write_out(&mut writer)
        .expect("failed to write ppm to file");
    writer.flush().expect("failed to flush buffer");
}
