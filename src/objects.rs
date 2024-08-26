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
    Sphere(Sphere),
    Cube(Cube),
    FlatePlane(FlatePlane),
    Cylinder(Cylinder)
}

impl Object {
    pub fn new(shape: Shape) -> Result<Self, String> {
        match shape.shape_type.as_str() {
            "sphere" => Ok(Object::Sphere(Sphere::new(shape))),
            "cube"=> Ok(Object::Cube(Cube::new(shape))),
            "flateplane" => Ok(Object::FlatePlane(FlatePlane::new(shape))),
            "cylinder" => Ok(Object::Cylinder(Cylinder::new(shape))),
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

#[derive(Debug)]
pub struct Cube {
    pub center: Vec3,
    pub length : f64,
    pub color : Color
}

impl Cube {
    pub fn new(shape: Shape)-> Self {
        let center = shape.location;
        let length = 20.00;
        let color = get_color(&shape.color);

        Cube {center, length, color}
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        let half_length = self.length / 2.0;

        // Vérifier sur quelle face du cube le point se trouve
        if (point.x - self.center.x).abs() > half_length - 1e-4 {
            return Vec3::new((point.x - self.center.x).signum(), 0.0, 0.0);
        }
        if (point.y - self.center.y).abs() > half_length - 1e-4 {
            return Vec3::new(0.0, (point.y - self.center.y).signum(), 0.0);
        }
        if (point.z - self.center.z).abs() > half_length - 1e-4 {
            return Vec3::new(0.0, 0.0, (point.z - self.center.z).signum());
        }

        // Si le point n'est pas exactement sur une face, retourner un vecteur nul (ce qui ne devrait pas arriver pour un point d'intersection bien calculé)
        Vec3::new(0.0, 0.0, 0.0)
    }   
}

#[derive(Debug)]
pub struct FlatePlane {
    pub center: Vec3,
    pub normal: Vec3,
    pub color: Color,
    pub heigth: f64,
    pub width: f64,
}

impl FlatePlane {
    pub fn new(shape: Shape) -> Self {
        let center = shape.location;
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let color = get_color(&shape.color);
        let heigth = 200.00;
        let width = 200.00;

        FlatePlane { center, normal, color,    heigth, width }
    }

    pub fn normal_at(&self, _point: Vec3) -> Vec3 {
        self.normal
    }
    
}

#[derive(Debug)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub color: Color,
}

impl Cylinder {
    pub fn new(shape: Shape) -> Self {
        let center = shape.location;
        let radius = 10.00;
        let height = 20.00;
        let color = get_color(&shape.color);

        Cylinder { center, radius, height, color }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        let mut normal = point - self.center;
        normal.y = 0.0;
        normal.normalize()
    }
    
}

impl Renderable for Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(sphere) => sphere.intersect(ray),
            Object::Cube(cube ) => cube.intersect(ray),
            Object::FlatePlane(flateplane) => flateplane.intersect(ray),
            Object::Cylinder(cylinder) => cylinder.intersect(ray),
        }
    }

    fn color(&self) -> Color {
        match self {
            Object::Sphere(sphere) => sphere.color(),
            Object::Cube(cube)=> cube.color(),
            Object::FlatePlane(flateplane) => flateplane.color(),
            Object::Cylinder(cylinder) => cylinder.color()
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

impl Renderable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let half_length = self.length / 2.0;

        let mut tmin = -f64::INFINITY;
        let mut tmax = f64::INFINITY;

        // Intersection avec les plans en x
        let mut t1 = (self.center.x - half_length - ray.origin.x) / ray.direction.x;
        let mut t2 = (self.center.x + half_length - ray.origin.x) / ray.direction.x;
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        // Intersection avec les plans en y
        t1 = (self.center.y - half_length - ray.origin.y) / ray.direction.y;
        t2 = (self.center.y + half_length - ray.origin.y) / ray.direction.y;
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        // Intersection avec les plans en z
        t1 = (self.center.z - half_length - ray.origin.z) / ray.direction.z;
        t2 = (self.center.z + half_length - ray.origin.z) / ray.direction.z;
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.max(t2));

        // Si tmin est supérieur à tmax, il n'y a pas d'intersection valide
        if tmin > tmax {
            return None;
        }

        // Calculer le point d'intersection et la normale
        let t = if tmin > 0.0 { tmin } else { tmax };
        if t < 0.0 {
            return None;
        }

        let point = ray.origin + ray.direction * t;
        let normal = if (point.x - self.center.x).abs() > half_length - 1e-6 {
            Vec3::new((point.x - self.center.x).signum(), 0.0, 0.0)
        } else if (point.y - self.center.y).abs() > half_length - 1e-6 {
            Vec3::new(0.0, (point.y - self.center.y).signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, (point.z - self.center.z).signum())
        };

        Some(Intersection { point, normal, distance: t, color: self.color })
    }

    fn color(&self) -> Color {
        self.color
    }
}

impl Renderable for FlatePlane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() > 1e-6 {
            let t = (self.center - ray.origin).dot(self.normal) / denom;
            if t > 0.0 {
                let point = ray.origin + ray.direction * t;
                let half_width = self.width / 2.0;
                let half_height = self.heigth / 2.0;
                if (point.x - self.center.x).abs() <= half_width && (point.z - self.center.z).abs() <= half_height {
                    return Some(Intersection { point, normal: self.normal, distance: t, color: self.color });
                }
            }
        }

        None
    }

    fn color(&self) -> Color {
        self.color
    }
    
}

impl Renderable for Cylinder {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let oc = ray.origin - self.center;

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * oc.x * ray.direction.x + 2.0 * oc.z * ray.direction.z;
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b - sqrt_discriminant) / (2.0 * a);
            let t2 = (-b + sqrt_discriminant) / (2.0 * a);

            let t = if t1 > 0.0 && t2 > 0.0 {
                t1.min(t2)
            } else if t1 > 0.0 {
                t1
            } else if t2 > 0.0 {
                t2
            } else {
                return None;
            };

            let point = ray.origin + ray.direction * t;
            if point.y >= self.center.y && point.y <= self.center.y + self.height {
                return Some(Intersection { point, normal: self.normal_at(point), distance: t, color: self.color });
            }
        }

        None
    }

    fn color(&self) -> Color {
        self.color
    }
    
}
