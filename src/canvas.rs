use crate::convert::u8_to_str;
use crate::tuples::*;

type Pixel = (f64, f64, f64);

struct PPM {
  data: String,
  current_line_len: usize,
}

impl PPM {
  fn new(width: usize, height: usize) -> Self {
    let mut data = String::with_capacity((width + 1) * height * 3);
    data.push_str("P3\n");
    data.push_str(&width.to_string());
    data.push(' ');
    data.push_str(&height.to_string());
    data.push('\n');
    data.push_str("255");

    Self {
      data,
      current_line_len: 0,
    }
  }

  fn write(&mut self, value: &str) {
    if self.current_line_len + value.len() + 2 > 70 {
      self.new_line();
    } else if self.current_line_len != 0 {
      self.data.push(' ');
      self.current_line_len += 1;
    }

    self.data.push_str(value);
    self.current_line_len += value.len();
  }

  fn to_string(mut self) -> String {
    if !self.data.ends_with('\n') {
      self.data.push('\n');
    }
    self.data
  }

  fn new_line(&mut self) {
    self.data.push('\n');
    self.current_line_len = 0;
  }
}

fn clamp_value(value: &f64) -> &'static str {
  if *value < 0.0 {
    return "0";
  }

  if *value > 1.0 {
    return "255";
  }

  u8_to_str((value * 255.).round() as u8)
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

  pub fn write_pixel(&mut self, x: usize, y: usize, tuple: Tuple) {
    if let Some(row) = self.pixels.get_mut(y) {
      if let Some(pixel) = row.get_mut(x) {
        *pixel = tuple.as_color();
      }
    }
  }

  pub fn to_string(&self) -> String {
    let mut output = PPM::new(self.width, self.height);

    for row in &self.pixels {
      output.new_line();
      for (r, g, b) in row {
        output.write(clamp_value(r));
        output.write(clamp_value(g));
        output.write(clamp_value(b));
      }
    }

    output.to_string()
  }

  #[cfg(test)]
  fn iter_mut(&mut self) -> impl Iterator<Item = &mut Vec<Pixel>> {
    self.pixels.iter_mut()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::tuples::Tuple;

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
    let c1 = Tuple::new_color(1.5, 0, 0);
    let c2 = Tuple::new_color(0, 0.5, 0);
    let c3 = Tuple::new_color(-0.5, 0, 1);

    canvas.write_pixel(0, 0, c1);
    canvas.write_pixel(2, 1, c2);
    canvas.write_pixel(4, 2, c3);

    let string = canvas.to_string();
    let mut lines = string.lines().skip(3);

    assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", lines.next().unwrap());
    assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", lines.next().unwrap());
    assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", lines.next().unwrap());
  }

  #[test]
  fn test_ppm_line_length() {
    let mut canvas = Canvas::new(10, 2);

    let color = Tuple::new_color(1, 0.8, 0.6).as_color();
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
