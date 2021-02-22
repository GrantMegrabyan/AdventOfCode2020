use shared::{read_first_arg, MyError};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut group: HashMap<char, usize> = HashMap::new();
    let mut group_size: usize = 0;
    let mut result = 0;
    for line in f.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result += group.iter().filter(|(_, v)| **v == group_size).count();
            group.clear();
            group_size = 0;
        } else {
            group_size += 1;
            line.chars().into_iter().for_each(|ch| {
                let counter = group.entry(ch).or_insert(0);
                *counter += 1;
            });
        }
    }
    result += group.iter().filter(|(_, v)| **v == group_size).count();

    println!("Sum: {}", result);

    Ok(())
}
