use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use shared::MyError;
use shared::read_first_arg;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut pos = 0;
    let mut trees = 0;
    for line in f.lines().skip(1) {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        pos = (pos + 3) % chars.len();
        if chars[pos] == '#' {
            trees += 1;
        }
    }

    println!("Trees: {}", trees);

    Ok(())
}
