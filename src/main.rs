use std::env;
use std::process;
use rt::utils::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run configs/audit00.txt output.ppm OR cargo run configs/audit00.txt output.png");
        process::exit(1);
    }

    let config_file = &args[1];
    let output_file = &args[2];

    // Vérifier que `output_file` ne contient pas de chiffre
    if output_file.chars().any(|c| c.is_digit(10)) {
        eprintln!("Error: The output file name should not contain any digits.");
        process::exit(1);
    }

    let scene_params = parse_config_file(&config_file);
    
    scene_params.render(&output_file);
}