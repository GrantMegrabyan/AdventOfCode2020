use shared::read_first_arg;
use shared::MyError;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let f = File::open(input)?;
    let f = BufReader::new(f);

    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let mut trees = vec![0; slopes.len()];
    let mut pos = vec![0; slopes.len()];

    let mut line_num = 0;
    for line in f.lines().skip(1) {
        line_num += 1;
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();

        for i in 0..slopes.len() {
            if line_num % slopes[i].down == 0 {
                pos[i] = (pos[i] + slopes[i].right) % chars.len();
                if chars[pos[i]] == '#' {
                    trees[i] += 1;
                }
            }
        }
    }

    println!("Trees: {:?}", trees);
    println!(
        "{} = {}",
        trees
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" * "),
        trees.iter().product::<u32>()
    );
    Ok(())
}

struct Slope {
    right: usize,
    down: usize,
}
