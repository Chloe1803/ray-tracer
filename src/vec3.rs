use std::ops::{Add, Sub, Mul};

//Module pour la gestion des vecteurs 3D

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            Vec3::new(0.0, 0.0, 0.0) 
        } else {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        }
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Implémenter d'autres opérations comme l'addition, soustraction, etc.
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}


impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

pub fn parse_vec3(value: &str) -> Vec3 {
    let cleaned_value = value.trim_matches(|p| p == '(' || p == ')');
        let parts: Vec<&str> = cleaned_value.split(',')
                                            .map(|v| v.trim())
                                            .collect();


        if parts.len() != 3 {
            panic!("Invalid Vec3 format: expected 3 coordinates, found {}, for {}", parts.len(), value);
        }

        let x: f64 = parts[0].parse().expect("Failed to parse x coordinate as f64");
        let y: f64 = parts[1].parse().expect("Failed to parse y coordinate as f64");
        let z: f64 = parts[2].parse().expect("Failed to parse z coordinate as f64");

        Vec3 { x, y, z }
}
