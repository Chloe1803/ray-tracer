//Module utilitaire pour des fonctions diverses

use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Camera {
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
    fov: f64,
    aspect_ratio: f64,
}

#[derive(Debug)]
struct Light {
    position: Vec3,
    intensity: String,
    color: String,
}

#[derive(Debug)]
struct Shape {
    shape_type: String,
    color: String,
    location: Vec3,
}

#[derive(Debug)]
pub struct Scene {
    image_size: (u32, u32),
    background_color: String,
    camera: Camera,
    light: Light,
    shapes: Vec<Shape>,
}


pub fn parse_config_file(file_path: &str) -> Scene {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut image_size = (0, 0);
    let mut background_color = String::new();
    let mut camera_position = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_look_at = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_up = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_fov = 0.0;
    let mut camera_aspect_ratio = 0.0;
    let mut light_position = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut light_intensity = String::new();
    let mut light_color = String::new();
    let mut shapes = Vec::new();
    let mut reading_shapes = false;

    while let Some(Ok(line)) = lines.next()  {
        
        if reading_shapes {
            if line.contains("$$$ end_shape") {
                break;
            }
            let shape_parts: Vec<&str> = line.split('/').collect();
            shapes.push(Shape { shape_type: shape_parts[0].to_string(), color: shape_parts[1].to_string(), location: parse_vec3(shape_parts[2]) })
        }

        if line.contains("$$$ shapes"){
            reading_shapes = true;
            continue;
        }

        if line.contains("$$$ image_size") {
            if let Some(Ok(next_line)) = lines.next(){
                let image_size_parts : Vec<&str> = next_line.split_whitespace().collect();
                image_size = (
                    image_size_parts[0].parse::<u32>().expect("Failed to parse width"),
                    image_size_parts[1].parse::<u32>().expect("Failed to parse height")
                );
            }
        }

        if line.contains("$$$ background_color") {
            if let Some(Ok(next_line)) = lines.next(){
                background_color = next_line;
            }
        }

        if line.contains("$$$ light_position") {
            if let Some(Ok(next_line)) = lines.next(){
                if next_line == "default" {
                    light_position =  Vec3 { x: 10.0, y: 10.0, z: 10.0 };
                }else{
                    light_position = parse_vec3(&next_line)
                }
            }
        }

        if line.contains("$$$ light_intensity") {
            if let Some(Ok(next_line)) = lines.next(){
                light_intensity = next_line;
            }
        }
        
        if line.contains("$$$ light_color") {
            if let Some(Ok(next_line)) = lines.next(){
                light_color = next_line;
            }
        }

        if line.contains("$$$ camera_position") {
            if let Some(Ok(next_line)) = lines.next(){
                camera_position = parse_vec3(&next_line)
            }
        }

        if line.contains("$$$ camera_look_at") {
            if let Some(Ok(next_line)) = lines.next(){
                camera_look_at = parse_vec3(&next_line)
            }
        }

        if line.contains("$$$ camera_up") {
            if let Some(Ok(next_line)) = lines.next(){
                camera_up = parse_vec3(&next_line)
            }
        }

        if line.contains("$$$ camera_fov") {
            if let Some(Ok(next_line)) = lines.next(){
                camera_fov = next_line.parse::<f64>().expect("Failed to parse camera_fov")
            }
        }

        if line.contains("$$$ camera_aspect_ratio") {
            if let Some(Ok(next_line)) = lines.next(){
                camera_aspect_ratio = next_line.parse::<f64>().expect("Failed to parse camera_aspect_ratio")
            }
        }



    }

    Scene {
        image_size,
        background_color,
        camera: Camera {
            position: camera_position,
            look_at: camera_look_at,
            up: camera_up,
            fov: camera_fov,
            aspect_ratio: camera_aspect_ratio,
        },
        light: Light {
            position: light_position,
            intensity: light_intensity,
            color: light_color,
        },
        shapes,
    }
}

fn parse_vec3(value: &str) -> Vec3 {
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

