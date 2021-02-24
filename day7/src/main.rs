use regex::Regex;
use shared::{read_first_arg, MyError};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let outer_bag_re: Regex = Regex::new(r"(\w+ \w+) bag").unwrap();
    let inner_bag_re: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in f.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" contain ").collect();

        let outer = match outer_bag_re.captures_iter(parts[0]).next() {
            Some(cap) => String::from(&cap[1]),
            None => panic!("Failed to parse outer bag"),
        };

        let inner_bags: HashMap<String, usize> = inner_bag_re
            .captures_iter(parts[1])
            .map(|cap| (String::from(&cap[2]), cap[1].parse::<usize>().unwrap()))
            .collect();

        bags.insert(outer, inner_bags);
    }

    println!("Answer: {}", get_inner_count(&bags, "shiny gold"));

    Ok(())
}

fn get_inner_count(bags: &HashMap<String, HashMap<String, usize>>, outer_bag: &str) -> usize {
    let mut result = 0;

    if let Some(inner_bags) = bags.get(outer_bag) {
        for (inner_bag, count) in inner_bags {
            result += count + count * get_inner_count(bags, inner_bag);
        }
    }

    result
}
