//Module utilitaire pour des fonctions diverses
use std::io::Write;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::camera::Camera;
use crate::color::*;
use crate::vec3::*;
use crate::objects::*;
use crate::scene::*;


pub fn parse_config_file(file_path: &str) -> Scene_params {
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
    let mut light_intensity = 1.5;
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
                    light_position =  Vec3 { x: 25.0, y: 25.0, z: 25.0 };
                }else{
                    light_position = parse_vec3(&next_line)
                }
            }
        }

        // if line.contains("$$$ light_intensity") {
        //     if let Some(Ok(next_line)) = lines.next(){
        //         light_intensity = next_line;
        //     }
        // }
        
        if line.contains("$$$ light_color") {
            if let Some(Ok(next_line)) = lines.next(){
                light_color = get_color(&next_line);
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

    Scene_params {
        image_size,
        background_color,
        camera: Camera::new(camera_position, camera_look_at, camera_up, camera_fov, camera_aspect_ratio),
        light: Light {
            position: light_position,
            intensity: light_intensity,
            color: light_color,
        },
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




