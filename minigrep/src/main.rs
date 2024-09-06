use minigrepper::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrepper::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    };
}
