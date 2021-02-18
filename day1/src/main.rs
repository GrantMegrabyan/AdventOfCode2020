use shared::{read_first_arg, MyError};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    two_numbers(2020, &input)?;
    three_numbers(2020, &input)?;

    Ok(())
}

fn two_numbers(target: i32, file_path: &str) -> Result<(), MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut set: HashSet<i32> = HashSet::new();

    for line in f.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        let remainder = target - num;

        if set.contains(&remainder) {
            println!("{}", num * remainder);
            break;
        } else {
            set.insert(num);
        }
    }
    Ok(())
}

fn three_numbers(target: i32, file_path: &str) -> Result<(), MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let numbers: Vec<i32> = f.lines().map(|n| n.unwrap().parse().unwrap()).collect();
    let mut set: HashSet<i32> = HashSet::with_capacity(numbers.len());

    for i in 0..numbers.len() {
        set.insert(numbers[i]);
        for j in 1..numbers.len() {
            let sum = numbers[i] + numbers[j];
            let remainder = target - sum;

            if set.contains(&remainder) {
                println!("{}", numbers[i] * numbers[j] * remainder);
                return Ok(());
            }
        }
    }

    Ok(())
}
