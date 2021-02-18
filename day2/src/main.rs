use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use shared::{MyError, read_first_arg};

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
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

    println!("{}", valid_cnt);

    Ok(())
}
