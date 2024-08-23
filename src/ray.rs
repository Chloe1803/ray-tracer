//Module pour la gestion des rayons

use crate::vec3::Vec3;
use crate::color::Color;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[derive(Debug)]
pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f64,
    pub color: Color,
}
