#[macro_use]
extern crate lazy_static;
use regex::Regex;
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
    let mut is_valid = true;
    for line in f.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if fields_cnt == req_fields_cnt && is_valid {
                passport_cnt += 1;
            }
            is_valid = true;
            fields_cnt = 0;
        } else {
            if !is_valid {
                continue;
            }
            for (name, value) in line.split(' ').filter_map(|p| {
                let field_parts: Vec<&str> = p.split(':').collect();
                match req_fields.contains(field_parts[0]) {
                    true => Some((field_parts[0], field_parts[1])),
                    false => None,
                }
            }) {
                fields_cnt += 1;
                if !validate(name, value) {
                    is_valid = false;
                    break;
                }
            }
        }
    }

    println!("Passports: {}", passport_cnt);

    Ok(())
}

fn validate(name: &str, value: &str) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[\d]{9}$").unwrap();
    }

    match name {
        "byr" => match value.parse::<i32>() {
            Ok(val) => (1920..=2002).contains(&val),
            Err(_) => false,
        },
        "iyr" => match value.parse::<i32>() {
            Ok(val) => (2010..=2020).contains(&val),
            Err(_) => false,
        },
        "eyr" => match value.parse::<i32>() {
            Ok(val) => (2020..=2030).contains(&val),
            Err(_) => false,
        },
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "hcl" => HCL_RE.is_match(value),
        "hgt" => match HGT_RE.captures_iter(value).next() {
            Some(cap) => match &cap[2] {
                "cm" => match cap[1].parse::<i32>() {
                    Ok(val) => (150..=193).contains(&val),
                    Err(_) => false,
                },
                "in" => match cap[1].parse::<i32>() {
                    Ok(val) => (59..=76).contains(&val),
                    Err(_) => false,
                },
                _ => false,
            },
            None => false,
        },
        "pid" => PID_RE.is_match(value),
        _ => false,
    }
}
