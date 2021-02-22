#[macro_use]
extern crate lazy_static;
use std::fmt;
use std::cmp;
use shared::{read_first_arg, MyError};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut heap = BinaryHeap::new();
    for line in f.lines() {
        let line = line.unwrap();
        let seat = parse_seat(&line);
        heap.push(seat);
    }

    let mut prev_seat = heap.pop().unwrap();
    while let Some(seat) = heap.pop() {
        if prev_seat.id - seat.id > 1 {
            let my_seat_id = seat.id + 1;
            println!("My seat id: {}", my_seat_id);
            break;
        }
        prev_seat = seat;
    }

    Ok(())
}

fn parse_seat(s: &str) -> Seat {
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
    for ch in s[7..].chars() {
        let bit = MAP.get(&ch).unwrap();
        col = (col << 1) | bit;
    }
    Seat::new(row, col)
}

#[derive(Eq)]
struct Seat {
    row: u32,
    col: u32,
    id: u32,
}

impl Seat {
    pub fn new(row: u32, col: u32) -> Self {
        Seat {
            row,
            col,
            id: row * 8 + col,
        }
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row {}, col {}, id {}", self.row, self.col, self.id)
    }
}

impl cmp::Ord for Seat {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl cmp::PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
