use shared::{read_first_arg, MyError};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let adapters: Vec<usize> = f
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect();

    part1(&adapters);
    part2(&adapters);
    Ok(())
}

fn part1(adapters: &[usize]) {
    let diffs = get_joltage_diffs(adapters.to_owned());

    println!("Part 1: {}", diffs.one_jolt * diffs.three_jolts);
}

fn part2(adapters: &[usize]) {
    let result = count_paths(adapters.to_owned());

    println!("Part 2: {}", result);
}

fn count_paths(mut adapters: Vec<usize>) -> usize {
    adapters.sort_unstable();
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut paths: HashMap<usize, usize> = HashMap::new();
    paths.insert(0, 1);
    paths.insert(1, 1);

    let mut path: usize = 1;
    for &adapter in &adapters[1..] {
        path = paths.get(&(adapter - 2)).unwrap_or(&0) + paths.get(&(adapter - 1)).unwrap_or(&0);
        if adapter > 2 {
            path += paths.get(&(adapter - 3)).unwrap_or(&0);
        }
        paths.insert(adapter, path);
    }
    path
}

#[derive(Debug)]
struct JoltageDiffs {
    one_jolt: usize,
    two_jolts: usize,
    three_jolts: usize,
}

fn get_joltage_diffs(mut adapters: Vec<usize>) -> JoltageDiffs {
    let mut diffs = vec![0; 3];
    adapters.sort_unstable();

    let mut prev: usize = 0;
    for adapter in adapters {
        diffs[adapter - prev - 1] += 1;
        prev = adapter;
    }
    diffs[2] += 1;

    JoltageDiffs {
        one_jolt: diffs[0],
        two_jolts: diffs[1],
        three_jolts: diffs[2],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltage_diffs_one() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let diffs = get_joltage_diffs(input);
        assert_eq!(7, diffs.one_jolt);
        assert_eq!(5, diffs.three_jolts);
    }

    #[test]
    fn test_get_joltage_diffs_two() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let diffs = get_joltage_diffs(input);
        assert_eq!(22, diffs.one_jolt);
        assert_eq!(10, diffs.three_jolts);
    }

    #[test]
    fn test_ways_to_arrange_one() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = count_paths(input);
        assert_eq!(8, result);
    }

    #[test]
    fn test_ways_to_arrange_two() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result = count_paths(input);
        assert_eq!(19208, result);
    }
}
