use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let Config { query, filename } = parse_config(&args);

    println!("Searching for {query}");
    println!("In file {filename:?}");

    let contents =
        fs::read_to_string(filename).expect("something went wrong while reading the file");

    println!("With text:\n{contents}");
}

pub struct Config {
    query: String,
    filename: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
