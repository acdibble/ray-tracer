use std::f64::consts::PI;
use std::fs;
use std::io::{BufWriter, Write};

pub mod canvas;
pub mod convert;
pub mod matrices;
pub mod transformations;
#[macro_use]
pub mod tuples;

use canvas::Canvas;
use tuples::Tuple;

fn main() {
    let file = fs::File::create("out.ppm").expect("failed to open file");
    let mut writer = BufWriter::new(file);
    let mut canvas = Canvas::new(500, 500);

    let twelve = point!(0, 1, 0);
    let white = color!(255, 255, 255);

    for i in 0..12 {
        let current_index = twelve
            .rotate_z(-i as f64 * PI / 6.0)
            .scale(200.0, 200.0, 0.0)
            .translate(250.0, 250.0, 0.0);

        canvas.write_pixel(
            current_index.0.round() as usize,
            current_index.1.round() as usize,
            white,
        );
    }

    canvas
        .write_out(&mut writer)
        .expect("failed to write ppm to file");
    writer.flush().expect("failed to flush buffer");
}
