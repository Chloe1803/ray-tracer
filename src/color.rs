use crate::ray::*;
use crate::scene::*;
use crate::objects::*;
use crate::vec3::*;
use std::ops::{Mul, Add};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
        }
    }

    pub fn scale(&self, factor: f64) -> Color {
        Color {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }

    pub fn to_ppm_values(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.0).clamp(0.0, 255.0) as u8;
        let g = (self.g * 255.0).clamp(0.0, 255.0) as u8;
        let b = (self.b * 255.0).clamp(0.0, 255.0) as u8;
        (r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

pub fn get_color(color_name: &str) -> Color {
    match color_name.to_lowercase().as_str() {
        "red" => Color::new(1.0, 0.0, 0.0),
        "green" => Color::new(0.0, 1.0, 0.0),
        "blue" => Color::new(0.0, 0.0, 1.0),
        "white" => Color::new(1.0, 1.0, 1.0),
        "black" => Color::new(0.0, 0.0, 0.0),
        "yellow" => Color::new(1.0, 1.0, 0.0),
        "cyan" => Color::new(0.0, 1.0, 1.0),
        "magenta" => Color::new(1.0, 0.0, 1.0),
        "gray" | "grey" => Color::new(0.5, 0.5, 0.5),
        "orange" => Color::new(1.0, 0.65, 0.0),
        "purple" => Color::new(0.5, 0.0, 0.5),
        "brown" => Color::new(0.6, 0.3, 0.0),
        _ => {
            eprintln!("Warning: Unknown color '{}', defaulting to black.", color_name);
            Color::new(0.0, 0.0, 0.0) // Couleur par défaut (noir) si la couleur est inconnue
        }
    }
}

pub fn color(ray: &Ray, scene: &Scene_params) -> Color {
    if let Some(intersection) = scene.objects.iter()
        .filter_map(|obj| obj.intersect(ray))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    {
        let mut final_color = compute_lighting(&intersection, &scene, ray); // Couleur avec ombres

        return final_color;
    }

    // Retournez la couleur de fond si aucun objet n'est intersecté
    scene.background_color.clone()
}

fn compute_lighting(intersection: &Intersection, scene: &Scene_params, ray: &Ray) -> Color {
    let mut final_color = Color::new(0.0, 0.0, 0.0);

    for light in &scene.lights {
        // Vecteur de la lumière à l'intersection
        let light_dir = (light.position - intersection.point).normalize();

        // Rayon d'ombre
        let shadow_ray = Ray {
            origin: intersection.point + intersection.normal * 1e-6, // Petit décalage pour éviter l'auto-intersection
            direction: light_dir,
        };

        // Vérifier les intersections avec les objets de la scène
        let in_shadow = scene.objects.iter()
            .filter_map(|obj| obj.intersect(&shadow_ray))
            .any(|shadow_intersection| shadow_intersection.distance < (light.position - intersection.point).length());

        if !in_shadow {
            // Produit scalaire entre la normale et le vecteur lumière
            let diffuse_intensity = light_dir.dot(intersection.normal).max(0.0);

            // Calcul de la couleur finale
            let diffuse_color = intersection.color * diffuse_intensity * light.intensity;
            final_color = final_color + diffuse_color;
        }
    }

    final_color
}




fn is_in_shadow(point: &Vec3, light: &Light, objects: &Vec<Object>) -> bool {
    let shadow_ray = Ray {
        origin: *point,
        direction: (light.position - *point).normalize(),
    };

    objects.iter().any(|obj| obj.intersect(&shadow_ray).is_some())
}



