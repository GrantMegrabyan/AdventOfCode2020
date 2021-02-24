use shared::{read_first_arg, MyError};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    const PREAMBLE_SIZE: usize = 25;
    let mut preamble: Vec<i32> = Vec::with_capacity(PREAMBLE_SIZE);
    let mut leftmost = 0;

    for line in f.lines() {
        let line = line.unwrap();
        let num = line.parse::<i32>().unwrap();

        if preamble.len() < PREAMBLE_SIZE {
            preamble.push(num);
        } else if !satisfies_rules(&preamble, num) {
            println!("Wrong: {}", num);
            break;
        } else {
            preamble[leftmost] = num;
            leftmost = (leftmost + 1) % PREAMBLE_SIZE;
        }
    }

    Ok(())
}

fn satisfies_rules(preamble: &[i32], num: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for item in preamble {
        if set.contains(&(num - item)) {
            return true;
        }
        set.insert(*item);
    }
    false
}
