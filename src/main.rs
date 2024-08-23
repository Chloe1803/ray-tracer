use std::env;
use std::process;
use rt::utils::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    // if args.len() != 3 {
    //     eprintln!("Usage: cargo run > output.ppm config.txt");
    //     process::exit(1);
    // }

    // let config_file = &args[2];

    if args.len() != 2 {
        eprintln!("Usage: cargo run configs/audit00.txt");
        process::exit(1);
    }

    let config_file = &args[1];

    let scene_params = parse_config_file(&config_file);
    
    scene_params.render();

}
