use std::fs::{self, File};
use std::io::ErrorKind;
use std::io;
use std::error::Error;
use std::io::Read;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("The guess must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let f = File::open("hello.txt")?;

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open the file");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    let f = File::open("hello.txt").unwrap_or_else(|error | {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error | {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // By adding ?, if fails it will return the Error
    let mut f = File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
