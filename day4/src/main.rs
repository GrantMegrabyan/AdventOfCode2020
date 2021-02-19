use shared::read_first_arg;
use shared::MyError;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let req_fields: HashSet<&'static str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();
    let req_fields_cnt = req_fields.len();

    let mut fields_cnt = 0;
    let mut passport_cnt = 0;
    for line in f.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if fields_cnt == req_fields_cnt {
                passport_cnt += 1;
            }
            fields_cnt = 0;
        } else {
            fields_cnt += line
                .split(" ")
                .filter_map(
                    |p| match req_fields.contains(p.split(":").next().unwrap()) {
                        true => Some(true),
                        false => None,
                    },
                )
                .count();
        }
    }

    println!("Passports: {}", passport_cnt);

    Ok(())
}
