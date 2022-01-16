use crate::tuples::*;

pub struct PointLight {
    pub intensity: Tuple,
    pub position: Tuple,
}

impl PointLight {
    pub const fn new(position: Tuple, intensity: Tuple) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
