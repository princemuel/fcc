use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let Config { query, filename } = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

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

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
