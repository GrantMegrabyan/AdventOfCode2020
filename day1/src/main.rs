use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

fn main() -> Result<(), MyError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 1 {
        return Err(MyError::InputNotProvided);
    }

    two_numbers(2020, &args[0])?;

    Ok(())
}

fn two_numbers(target: i32, file_path: &str) -> Result<(), MyError>{
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut set: HashSet<i32> = HashSet::new();

    for line in f.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        let remainder = target - num;

        if set.contains(&remainder) {
            println!("{}", num * remainder);
            break;
        }
        else {
            set.insert(num);
        }
    }
    Ok(())
}

#[derive(Debug)]
enum MyError {
    InputNotProvided,
    FileNotFound(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> MyError {
        MyError::FileNotFound(error)
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InputNotProvided => write!(f, "Input file must be provided"),
            MyError::FileNotFound(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for MyError {}
