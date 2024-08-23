use crate::vec3::*;
use crate::color::*;
use crate::ray::*;

//Module pour les objets géométriques (sphère, plan, cube, etc.)
#[derive(Debug)]
pub struct Shape {
    pub shape_type: String,
    pub color: String,
    pub location: Vec3,
}

#[derive(Debug)]
pub enum Object {
    Sphere(Sphere)
}

impl Object {
    pub fn new(shape: Shape) -> Result<Self, String> {
        match shape.shape_type.as_str() {
            "sphere" => Ok(Object::Sphere(Sphere::new(shape))),
            _ => Err("Invalid shape type".to_string()),
        }
    }  
}

pub trait Renderable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn color(&self) -> Color;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
}

impl Sphere {
    //radius par défault 10
    pub fn new(shape :Shape)->Self{
        let center = shape.location;
        let radius = 10.00;
        let color = get_color(&shape.color);

        Sphere {center, radius, color}
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}

impl Renderable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            // Ajoutez d'autres variantes d'objets ici si nécessaire
        }
    }

    fn color(&self) -> Color {
        match self {
            Object::Sphere(sphere) => sphere.color(),
            // Ajoutez d'autres variantes d'objets ici si nécessaire
        }
    }
}


impl Renderable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        //le vecteur entre l'origine du rayon et le centre de l'objet 
        let oc = ray.origin - self.center;

        //coefficients a, b et c de l'équation quadratique qui décrit l'intersection entre le rayon et l'objet
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        //discrimant de l'equation quadratique, s'il est supérieur à 0, il y a deux point d'intersection
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            //les valeurs t1 et t2 qui représentent les distances le long du rayon où les intersections se produisent
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);


            //il faut choisir la plus petite valeur positive entre t1 et t2 comme valeur de t (la distance à laquelle l'intersection se produit).
            let t = if t1 > 0.0 && t2 > 0.0 {
                t1.min(t2)  
            } else if t1 > 0.0 {
                t1  // t1 est positif, t2 est négatif ou non valide
            } else if t2 > 0.0 {
                t2  // t2 est positif, t1 est négatif ou non valide
            } else {
                return None; // Aucune intersection valide
            };

            //point d'intersection du rayon
            let point = ray.origin + ray.direction * t;

            //calcule la normale à la surface de l'objet au point d'intersection (normal) en utilisant le centre de l'objet et le point d'intersection
            let normal = (point - self.center).normalize();

            //si tous est ok renvoie une intersection
            return Some(Intersection { point, normal, distance: t, color: self.color });
        }

        None
    }

    fn color(&self) -> Color {
        self.color
    }
}
