#[macro_use]
extern crate lazy_static;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use shared::{read_first_arg, MyError};
use std::collections::HashMap;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut max_seat_id = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let seat_id = calc_seat_id(&line);
        max_seat_id = std::cmp::max(max_seat_id, seat_id);
    }

    println!("Max seat id: {}", max_seat_id);
    
    Ok(())
}

fn calc_seat_id(s: &str) -> u32 {
    lazy_static! {
        static ref MAP: HashMap<char, u32> = [('F', 0), ('B', 1), ('L', 0), ('R', 1)]
            .iter()
            .cloned()
            .collect();
    }

    let mut row = 0;
    for ch in s[0..7].chars() {
        let bit = MAP.get(&ch).unwrap();
        row = (row << 1) | bit;
    }
    
    let mut col = 0;
    for ch in s[0..3].chars() {
        let bit = MAP.get(&ch).unwrap();
        col = (col << 1) | bit;
    }

    row * 8 + col
}
