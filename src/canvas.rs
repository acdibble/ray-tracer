use crate::{convert::u8_to_str, tuples::*};
use std::io::{self, Write};

type Pixel = (f64, f64, f64);

struct PPM<'a, T: Write> {
  buffer: &'a mut T,
  current_line_len: usize,
}

impl<'a, T: Write> PPM<'a, T> {
  fn new(width: usize, height: usize, buffer: &'a mut T) -> Result<Self, io::Error> {
    buffer.write(b"P3\n")?;
    buffer.write(width.to_string().as_bytes())?;
    buffer.write(b" ")?;
    buffer.write(height.to_string().as_bytes())?;
    buffer.write(b"\n")?;
    buffer.write(b"255\n")?;

    Ok(Self {
      buffer,
      current_line_len: 0,
    })
  }

  fn new_line(&mut self) -> Result<(), io::Error> {
    self.buffer.write(b"\n")?;
    self.current_line_len = 0;
    Ok(())
  }
}

impl<T: Write> Write for PPM<'_, T> {
  fn write(&mut self, value: &[u8]) -> Result<usize, io::Error> {
    if self.current_line_len + value.len() + 2 > 70 {
      self.new_line()?;
    } else if self.current_line_len != 0 {
      self.buffer.write(b" ")?;
      self.current_line_len += 1;
    }

    self.buffer.write(value)?;
    self.current_line_len += value.len();
    Ok(value.len())
  }

  fn flush(&mut self) -> Result<(), io::Error> {
    self.buffer.flush()?;
    Ok(())
  }
}

fn clamp_value(value: &f64) -> &'static str {
  if *value < 0.0 {
    return "0";
  }

  if *value > 1.0 {
    return "255";
  }

  u8_to_str((value * 255.0).round() as u8)
}

pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![vec![(0., 0., 0.); width]; height],
    }
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, tuple: &Tuple) {
    if let Some(row) = self.pixels.get_mut(y) {
      if let Some(pixel) = row.get_mut(x) {
        *pixel = tuple.as_color();
      }
    }
  }

  fn write_to_writer<T: Write>(&self, buffer: &mut T) -> Result<(), io::Error> {
    let mut output = PPM::new(self.width, self.height, buffer)?;

    for row in &self.pixels {
      for (r, g, b) in row {
        output.write(clamp_value(r).as_bytes())?;
        output.write(clamp_value(g).as_bytes())?;
        output.write(clamp_value(b).as_bytes())?;
      }

      output.new_line()?;
    }

    Ok(())
  }

  pub fn to_string(&self) -> String {
    let mut buffer = Vec::with_capacity((self.width + 1) * self.height * 3);
    self.write_to_writer(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
  }

  pub fn write_out<T: Write>(&self, writer: &mut T) -> Result<(), io::Error> {
    self.write_to_writer(writer)?;
    Ok(())
  }

  #[cfg(test)]
  fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<Pixel>> {
    self.pixels.iter_mut()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ppm_header() {
    let canvas = Canvas::new(5, 3);
    let string = canvas.to_string();
    let mut lines = string.lines();

    assert_eq!("P3", lines.next().unwrap());
    assert_eq!("5 3", lines.next().unwrap());
    assert_eq!("255", lines.next().unwrap());
  }

  #[test]
  fn test_ppm_pixel_data() {
    let mut canvas = Canvas::new(5, 3);
    let c1 = color!(1.5, 0, 0);
    let c2 = color!(0, 0.5, 0);
    let c3 = color!(-0.5, 0, 1);

    canvas.write_pixel(0, 0, &c1);
    canvas.write_pixel(2, 1, &c2);
    canvas.write_pixel(4, 2, &c3);

    let string = canvas.to_string();
    let mut lines = string.lines().skip(3);

    assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines.next().unwrap());
    assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines.next().unwrap());
    assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines.next().unwrap());
  }

  #[test]
  fn test_ppm_line_length() {
    let mut canvas = Canvas::new(10, 2);

    let color = color!(1, 0.8, 0.6).as_color();
    for row in canvas.iter_mut() {
      for pixel in row.iter_mut() {
        *pixel = color;
      }
    }

    let string = canvas.to_string();
    let mut lines = string.lines().skip(3);

    assert_eq!(
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      lines.next().unwrap()
    );
    assert_eq!(
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
      lines.next().unwrap()
    );
    assert_eq!(
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      lines.next().unwrap()
    );
    assert_eq!(
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
      lines.next().unwrap()
    );
  }

  #[test]
  fn test_ppm_ends_with_newline() {
    let canvas = Canvas::new(5, 3);
    let string = canvas.to_string();

    assert!(string.ends_with('\n'));
  }
}
