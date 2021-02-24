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
    let mut stack: Vec<(usize, &str)> = Vec::new();
    let mut is_changed = false;
    loop {
        let (mut op, arg) = instructions[index];

        // If current op was already visited,
        // start backtracking. Pop the last operation from the stack,
        // if it's "acc" - subtract the value from accumulator,
        // if it's "nop" or "jmp" - look at the original instructions list.
        // If the same op is there, it means we haven't tried changing it
        // to the opposite one - do it. Otherwise skip and go to the next
        // element of the stack.
        if visited.contains(&index) {
            while let Some((ind, prev_op)) = stack.pop() {
                visited.remove(&ind);
                if prev_op == "acc" {
                    accumulator -= instructions[ind].1;
                } else if instructions[ind].0 == prev_op {
                    if !is_changed {
                        index = ind;
                        op = if prev_op == "nop" { "jmp" } else { "nop" };
                        is_changed = true;
                        break;
                    }
                } else {
                    is_changed = false;
                }
            }
        }

        visited.insert(index);
        stack.push((index, op));
        match op {
            "nop" => index += 1,
            "acc" => {
                accumulator += arg;
                index += 1;
            }
            "jmp" => index = (index as i32 + arg) as usize,
            _ => panic!("Unsupported operation: {}", op),
        }

        if index >= instructions.len() {
            break;
        }
    }

    // println!("Stack: {:?}", stack);
    println!("Accumulator: {}", accumulator);

    Ok(())
}
