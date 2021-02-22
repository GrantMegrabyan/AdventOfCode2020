use shared::{read_first_arg, MyError};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut group: HashSet<char> = HashSet::new();
    let mut result = 0;
    for line in f.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result += group.len();
            group.clear();
        } else {
            line.chars().into_iter().for_each(|ch| {
                let _ = group.insert(ch);
            });
        }
    }
    result += group.len();

    println!("Sum: {}", result);

    Ok(())
}
