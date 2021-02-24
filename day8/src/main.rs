use shared::{read_first_arg, MyError};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let mut instructions: Vec<(&str, i32)> = Vec::new();

    let nop = "nop";
    let acc = "acc";
    let jmp = "jmp";

    for line in f.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();

        instructions.push((
            match parts[0] {
                "nop" => nop,
                "acc" => acc,
                "jmp" => jmp,
                op => panic!("Unsupported operation: {}", op),
            },
            parts[1].parse::<i32>().unwrap(),
        ));
    }

    let mut accumulator = 0;
    let mut index: usize = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    while !visited.contains(&index) {
        visited.insert(index);
        match &instructions[index] {
            ("nop", _) => index += 1,
            ("acc", arg) => {
                accumulator += arg;
                index += 1;
            }
            ("jmp", arg) => index = (index as i32 + arg) as usize,
            (op, _) => panic!("Unsupported operation: {}", op),
        }
    }

    println!("Accumulator: {}", accumulator);

    Ok(())
}
