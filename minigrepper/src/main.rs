use std::{env, error::Error, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let Config { query, filename } = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    println!("Searching for {query}");
    println!("In file {filename:?}");

    if let Err(e) = run(Config { query, filename }) {
        println!("Apllication error: {e}");
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{contents}");
    Ok(())
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
