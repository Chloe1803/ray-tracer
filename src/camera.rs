//Module pour la gestion de la caméra

use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub position: Vec3,              // Position de la caméra
    pub look_at: Vec3,              // Point que la caméra regarde
    pub up: Vec3,                   // Vecteur vers le haut
    pub fov: f64,                   // Champ de vision en degrés
    pub aspect_ratio: f64,          // Rapport largeur/hauteur de l'image
    pub lower_left_corner: Vec3,    // Coin inférieur gauche du plan d'image
    pub horizontal: Vec3,           // Vecteur horizontal du plan d'image
    pub vertical: Vec3,             // Vecteur vertical du plan d'image
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, fov: f64, aspect_ratio: f64) -> Camera {
        let theta = fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - look_at).normalize();
        let u: Vec3 = up.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = position - (viewport_width / 2.0) * u - (viewport_height / 2.0) * v - w;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera {
            position,
            look_at,
            up,
            fov,
            aspect_ratio,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.position;
        Ray {
            origin: self.position,
            direction: direction.normalize(),
        }
    }
}



