//Module utilitaire pour des fonctions diverses
use std::io::Write;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::f64::consts::PI;
use crate::camera::Camera;
use crate::color::*;
use crate::vec3::*;
use crate::objects::*;
use crate::scene::*;


pub fn parse_config_file(file_path: &str) -> SceneParams {
    let path = Path::new(file_path);
    let file = File::open(&path).expect("Could not open file");
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut image_size = (0, 0);
    let mut background_color = get_color("white");
    let mut camera_position = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_look_at = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_up = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut camera_fov = 0.0;
    let mut camera_aspect_ratio = 0.0;
    let mut light_position = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let mut light_intensity = 1.0;
    let mut light_color = get_color("white");
    let mut objects = Vec::new();
    let mut reading_shapes = false;

    while let Some(Ok(line)) = lines.next()  {
        
        if reading_shapes {
            if line.contains("$$$ end_shape") {
                break;
            }
            let shape_parts: Vec<&str> = line.split('/').collect();

            if let Ok(object) =Object::new(Shape { shape_type: shape_parts[0].to_string(), color: shape_parts[1].to_string(), location: parse_vec3(shape_parts[2]) }) {
                objects.push(object)
            }else{
                println!("Invalid Shape");
                break;
            }
            
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
                background_color = get_color(&next_line);
            }
        }

        if line.contains("$$$ light_position") {
            if let Some(Ok(next_line)) = lines.next(){
                if next_line == "default" {
                    light_position =  Vec3 { x: 0.0, y: 50.0, z: 0.0 };
                }else{
                    light_position = parse_vec3(&next_line)
                }
            }
        }

         if line.contains("$$$ light_intensity") {
            if let Some(Ok(next_line)) = lines.next(){
                if next_line == "low" {
                    light_intensity =  0.7;
                }else if next_line == "medium" {
                    light_intensity =  1.0;
                }else if next_line == "high" {
                    light_intensity =  1.3;
                }
            }
         }
        
        if line.contains("$$$ light_color") {
            if let Some(Ok(next_line)) = lines.next(){
                light_color = get_color(&next_line);
            }
        }

        if line.contains("$$$ camera_position") {
            if let Some(Ok(next_line)) = lines.next(){
                if next_line == "north" {
                    camera_position = Vec3 {x:0.0, y:50.0, z:100.0};
                }else if next_line == "west" {
                    camera_position = Vec3 {x:100.0, y:50.0, z:0.0};
                }else if next_line == "south" {
                    camera_position = Vec3 {x:0.0, y:50.0, z:-100.0};
                }else if next_line == "east" {
                    camera_position = Vec3 {x:-100.0, y:50.0, z:0.0};
                }else {
                    camera_position = parse_vec3(&next_line)
                }
            }
        }

        if line.contains("$$$ camera_look_at") {
            if let Some(Ok(next_line)) = lines.next(){
                if next_line == "default" {
                    camera_look_at = Vec3 {x:0.0, y:0.0, z:0.0};
                }else {
                    camera_look_at = parse_vec3(&next_line)
                }
            }
        }

        if line.contains("$$$ camera_orientation") {
            if let Some(Ok(next_line)) = lines.next(){
                let degrees = next_line.parse::<f64>().expect("Failed to parse camera_orientation");
                let radians = degrees * PI / 180.0;
                let y = radians.cos();
                let x = radians.sin();
                camera_up = Vec3 {x:x, y:y, z:0.0};
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

    SceneParams {
        image_size,
        background_color,
        camera: Camera::new(camera_position, camera_look_at, camera_up, camera_fov, camera_aspect_ratio),
        lights: vec![Light {
            position: light_position,
            intensity: light_intensity,
            color: light_color,
        }],
        objects,
    }
}


pub fn save_image(filename: &str, image: &[Vec<Color>]) {
    let width = image[0].len();
    let height = image.len();

    let mut file = File::create(filename).expect("Unable to create file");
    writeln!(file, "P3").expect("Unable to write header");
    writeln!(file, "{} {}", width, height).expect("Unable to write dimensions");
    writeln!(file, "255").expect("Unable to write max color value");

    for row in image.iter().rev() {
        for color in row.iter() {
            let (r, g,b ) = color.to_ppm_values();            
            writeln!(file, "{} {} {}", r, g, b).expect("Unable to write pixel data");
        }
    }
}




