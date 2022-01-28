use std::error::Error;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem with the file {:?}", error),
            },
            other_error => {
                panic! {"Problem opening the file {:?}", other_error}
            }
        },
    };

    println!("{:?}", f);
    Ok(())
}

fn read_userame_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
