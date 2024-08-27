use crate::color::*;
use crate::objects::*;
use crate::vec3::*;
use crate::camera::*;
use crate::utils::*;

//Module pour la gestion de la scène (gestion des objets et des lumières)

#[derive(Debug)]
pub struct SceneParams {
    pub image_size: (u32, u32),
    pub background_color: Color,
    pub camera: Camera,
    pub lights: Vec<Light>, 
    pub objects: Vec<Object>,
}


#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    // Calcule l'éclairement de la lumière à un point donné
    pub fn light_at(&self, point: Vec3) -> (f64, Color) {
        // Pour l'exemple, la lumière diminue avec la distance
        let distance = (self.position - point).length();
        let attenuation = 1.0 / (distance * distance);
        let intensity = self.intensity * attenuation;
        (intensity, self.color)
    }
}

impl SceneParams {

    pub fn render(&self, destination: &str){
        let (width, height) = self.image_size;
        let mut image = vec![vec![get_color("black"); width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                let u = x as f64 / width as f64;
                let v = y as f64 / height as f64;

                let ray = self.camera.get_ray(u, v);
                image[y as usize][x as usize] = color(&ray, &self);
            }
        }

    // Sauvegarder l'image au format PPM
    save_image(destination, &image);
    }
    
}