use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// fn main() {
//     // panic!("crash and burn!");

//     let _file = File::open("hello.txt");

//     // let file = match file {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(file) => file,
//     //             Err(error) => panic!("Problem creating file: {error:?}"),
//     //         },
//     //         other_error => {
//     //             panic!("Problem opening file: {other_error:?}")
//     //         },
//     //     },
//     // };

//     // let file = File::open("world.txt").unwrap_or_else(|ex| {
//     //     if ex.kind() == ErrorKind::NotFound {
//     //         File::create("world.txt").unwrap_or_else(|error| {
//     //             panic!("Problem creating file: {error:?}")
//     //         })
//     //     } else {
//     //         panic!("Problem opening file: {ex:?}")
//     //     }
//     // });
//     // let file = File::open("hello.txt").expect("Failed to open file");
// }

fn main() -> Result<(), Box<dyn Error>> {
    File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file_v4(pathname: &str) -> Result<String, io::Error> {
    fs::read_to_string(pathname)
}

fn read_username_from_file_v3(pathname: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(pathname)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_v2(pathname: &str) -> Result<String, io::Error> {
    let mut file = File::open(pathname)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_v1(pathname: &str) -> Result<String, io::Error> {
    let file = File::open(pathname);

    let mut f = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
