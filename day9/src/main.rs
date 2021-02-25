use std::collections::HashMap;
use shared::{read_first_arg, MyError};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let numbers: Vec<i64> = f
        .lines()
        .map(|line| {line.unwrap().parse::<i64>().unwrap()})
        .collect();

    const PREAMBLE_SIZE: usize = 25;
    if let Some(invalid) = find_invalid(&numbers, PREAMBLE_SIZE) {
        println!("Invalid: {}", invalid);
        if let Some(seq) = find_contiguous_seq(&numbers, invalid) {
            println!("Sequence: {:?}", seq);
            
            let min = seq.iter().min().unwrap();
            let max = seq.iter().max().unwrap();
            println!("min + max: {}", min + max);
        }

    } else {
        println!("Everything is valid");
    }

    Ok(())
}

fn find_contiguous_seq(numbers: &[i64], target: i64) -> Option<&[i64]> {
    let mut map: HashMap<i64, usize> = HashMap::new();
    let mut sum = 0;
    for (i, num) in numbers.iter().enumerate() {
        sum += num;
        let remainder = sum - target;
        if map.contains_key(&remainder) && i - map[&remainder] > 1 {
            return Some(&numbers[map[&remainder]+1..i+1])
        }
        map.insert(sum, i);
    }
    None
}

fn find_invalid(numbers: &[i64], preamble_size: usize) -> Option<i64> {
    let mut pre_start = 0;
    let mut pre_end = preamble_size;
    for num in numbers.iter().skip(preamble_size) {
        if !satisfies_rules(&numbers[pre_start..pre_end], *num) {
            return Some(*num)
        } else {
            pre_start += 1;
            pre_end += 1;
        }
    }
    None
}

fn satisfies_rules(preamble: &[i64], num: i64) -> bool {
    let mut set: HashSet<i64> = HashSet::new();
    for item in preamble {
        if set.contains(&(num - item)) {
            return true;
        }
        set.insert(*item);
    }
    false
}
