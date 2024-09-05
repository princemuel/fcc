use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {query}");
    println!("In file {filename:?}");

    let _contents =
        fs::read_to_string(filename).expect("something went wrong while reading the file");
}
