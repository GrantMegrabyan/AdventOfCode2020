use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use shared::{MyError, read_first_arg};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    println!("Valid passwords, policy 1: {}", policy1(&input)?);
    println!("Valid passwords, policy 2: {}", policy2(&input)?);

    Ok(())
}

fn policy1(file_path: &str) -> Result<i32, MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut valid_cnt = 0;
    for line in f.lines() {
        let s = line.unwrap();
        let parts: Vec<&str> = s.split(' ').collect();
        let limits: Vec<usize> = parts[0].split('-').map(|n| n.parse().unwrap()).collect();
        let target = parts[1].chars().next().unwrap();
        let cnt = parts[2].chars().filter(|ch| *ch == target).count();
        if limits[0] <= cnt && cnt <= limits[1] {
            valid_cnt += 1;
        }
    }
    Ok(valid_cnt)
}

fn policy2(file_path: &str) -> Result<i32, MyError> {
    let f = File::open(file_path)?;
    let f = BufReader::new(f);

    let mut valid_cnt = 0;
    for line in f.lines() {
        let s = line.unwrap();
        let parts: Vec<&str> = s.split(' ').collect();
        let pos: Vec<usize> = parts[0].split('-').map(|n| n.parse().unwrap()).collect();
        let target = parts[1].chars().next().unwrap();
        let chars: Vec<char> = parts[2].chars().collect();

        match (chars[pos[0]-1] == target, chars[pos[1]-1] == target) {
            (true, false) | (false, true) => valid_cnt += 1,
            (_, _) => (),
        }
    }
    Ok(valid_cnt)
}
