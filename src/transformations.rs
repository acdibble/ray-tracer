use crate::matrices::Matrix;

pub const fn translation(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

macro_rules! translate {
    ($x:expr, $y:expr, $z:expr) => {
        translation($x as f64, $y as f64, $z as f64)
    };
}

pub const fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
    Matrix::new([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

macro_rules! scale {
    ($x:expr, $y:expr, $z:expr) => {
        scaling($x as f64, $y as f64, $z as f64)
    };
}

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn rotation(axis: Axis, radians: f64) -> Matrix<4> {
    match axis {
        Axis::X => Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [1.0, radians.cos(), -radians.sin(), 0.0],
            [1.0, radians.sin(), radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        Axis::Y => Matrix::new([
            [radians.cos(), 0.0, radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-radians.sin(), 0.0, radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
        Axis::Z => Matrix::new([
            [radians.cos(), -radians.sin(), 0.0, 0.0],
            [radians.sin(), radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]),
    }
}

macro_rules! rotate_x {
    ($radians:expr) => {
        rotation(Axis::X, $radians)
    };
}

macro_rules! rotate_y {
    ($radians:expr) => {
        rotation(Axis::Y, $radians)
    };
}

macro_rules! rotate_z {
    ($radians:expr) => {
        rotation(Axis::Z, $radians)
    };
}

pub fn shearing(
    x_to_y: f64,
    x_to_z: f64,
    y_to_x: f64,
    y_to_z: f64,
    z_to_y: f64,
    z_to_x: f64,
) -> Matrix<4> {
    Matrix::new([
        [1.0, x_to_y, x_to_z, 0.0],
        [y_to_x, 1.0, y_to_z, 0.0],
        [z_to_x, z_to_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

macro_rules! shear {
    (
    $x_to_y:expr,
    $x_to_z:expr,
    $y_to_x:expr,
    $y_to_z:expr,
    $z_to_y:expr,
    $z_to_x:expr
    ) => {
        shearing(
            $x_to_y as f64,
            $x_to_z as f64,
            $y_to_x as f64,
            $y_to_z as f64,
            $z_to_x as f64,
            $z_to_y as f64,
        )
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn test_translation() {
        let transform = translate!(5, -3, 2);
        let point = point!(-3, 4, 5);

        assert_eq!(point!(2, 1, 7), transform * point);

        assert_eq!(point!(-8, 7, 3), transform.inverse().unwrap() * point);

        let vector = vector!(-3, 4, 5);
        assert_eq!(vector, transform * vector);
    }

    #[test]
    fn test_scaling() {
        let transform = scale!(2, 3, 4);
        let point = point!(-4, 6, 8);

        assert_eq!(point!(-8, 18, 32), transform * point);

        let vector = vector!(-4, 6, 8);

        assert_eq!(vector!(-8, 18, 32), transform * vector);
        assert_eq!(vector!(-2, 2, 2), transform.inverse().unwrap() * vector);

        let transform = scale!(-1, 1, 1);
        let point = point!(2, 3, 4);
        assert_eq!(point!(-2, 3, 4), transform * point);
    }

    #[test]
    fn test_rotation_x() {
        use std::f64::consts::PI;

        let point = point!(0, 1, 0);
        let half_quarter = rotate_x!(PI / 4.0);
        let full_quarter = rotate_x!(PI / 2.0);

        assert_eq!(
            point!(0, 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0),
            half_quarter * point
        );
        assert_eq!(point!(0, 0, 1), full_quarter * point);
        assert_eq!(
            point!(0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0),
            half_quarter.inverse().unwrap() * point
        );
    }

    #[test]
    fn test_rotation_y() {
        use std::f64::consts::PI;

        let point = point!(0, 0, 1);
        let half_quarter = rotate_y!(PI / 4.0);
        let full_quarter = rotate_y!(PI / 2.0);

        assert_eq!(
            point!(2f64.sqrt() / 2.0, 0, 2f64.sqrt() / 2.0),
            half_quarter * point
        );
        assert_eq!(point!(1, 0, 0), full_quarter * point);
    }

    #[test]
    fn test_rotation_z() {
        use std::f64::consts::PI;

        let point = point!(0, 1, 0);
        let half_quarter = rotate_z!(PI / 4.0);
        let full_quarter = rotate_z!(PI / 2.0);

        assert_eq!(
            point!(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0),
            half_quarter * point
        );
        assert_eq!(point!(-1, 0, 0), full_quarter * point);
    }

    #[test]
    fn test_shearing() {
        let point = point!(2, 3, 4);

        let transform = shear!(1, 0, 0, 0, 0, 0);
        assert_eq!(point!(5, 3, 4), transform * point);

        let transform = shear!(0, 1, 0, 0, 0, 0);
        assert_eq!(point!(6, 3, 4), transform * point);

        let transform = shear!(0, 0, 1, 0, 0, 0);
        assert_eq!(point!(2, 5, 4), transform * point);

        let transform = shear!(0, 0, 0, 1, 0, 0);
        assert_eq!(point!(2, 7, 4), transform * point);

        let transform = shear!(0, 0, 0, 0, 1, 0);
        assert_eq!(point!(2, 3, 6), transform * point);

        let transform = shear!(0, 0, 0, 0, 0, 1);
        assert_eq!(point!(2, 3, 7), transform * point);
    }
}
