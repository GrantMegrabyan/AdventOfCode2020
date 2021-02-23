use regex::Regex;
use shared::{read_first_arg, MyError};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let bag_re: Regex = Regex::new(r"(\w+ \w+) bag").unwrap();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in f.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" contain ").collect();

        let outer = match bag_re.captures_iter(parts[0]).next() {
            Some(cap) => String::from(&cap[1]),
            None => panic!("Failed to parse outer bag"),
        };

        let inner_bags: Vec<String> = bag_re
            .captures_iter(parts[1])
            .map(|cap| String::from(&cap[1]))
            .collect();

        for inner in inner_bags {
            let entry = map.entry(inner).or_insert_with(HashSet::new);
            entry.insert(outer.clone());
        }
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue = vec!["shiny gold"];
    while let Some(current) = queue.pop() {
        if let Some(outer_bags) = map.get(current) {
            for outer_bag in outer_bags {
                if visited.contains(&(*outer_bag)[..]) {
                    continue;
                }
                queue.push(outer_bag);
                visited.insert(outer_bag);
            }
        }
    }

    println!("Answer: {}", visited.len());

    Ok(())
}
