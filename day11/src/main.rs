mod seating;

use seating::Seating;
use shared::{read_first_arg, MyError};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), MyError> {
    let input = read_first_arg()?;

    let mut f = File::open(input)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    part1(&contents);

    Ok(())
}

fn part1(input: &str) {
    println!("Count number of occupied seats when stable");
    println!("Answer: {}", count_occupied(input));
}

fn count_occupied(input: &str) -> usize {
    let mut seating = Seating::from(input);
    while let Some(next_gen) = seating.next_gen() {
        seating = next_gen;
    }
    seating.occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_occupied() {
        let input = "L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL";
        let occupied = count_occupied(input);
        assert_eq!(37, occupied);
    }
}
