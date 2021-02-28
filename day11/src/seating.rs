use std::cmp;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _ => panic!("Unsupported char: '{}'", ch),
        }
    }
}

impl Seat {
    pub fn to_char(&self) -> char {
        match self {
            Seat::Empty => 'L',
            Seat::Floor => '.',
            Seat::Occupied => '#',
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Seating {
    seats: Vec<Vec<Seat>>,
}

impl From<&str> for Seating {
    fn from(input: &str) -> Self {
        let mut seats = vec![];

        for line in input.lines() {
            let row = line.trim().chars().map(Seat::from).collect::<Vec<Seat>>();
            seats.push(row);
        }

        Seating { seats }
    }
}

impl fmt::Display for Seating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.seats {
            write!(
                f,
                "{}\n",
                row.iter().map(|s| s.to_char()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Seating {
    pub fn occupied(&self) -> usize {
        self.seats
            .iter()
            .flatten()
            .filter(|&s| *s == Seat::Occupied)
            .count()
    }

    pub fn next_gen(&mut self) -> Option<Seating> {
        let mut new_seats: Vec<Vec<Seat>> = Vec::with_capacity(self.seats.len());
        for i in 0..self.seats.len() {
            new_seats.push(Vec::with_capacity(self.seats[i].len()));
            for j in 0..self.seats[i].len() {
                let adj = self.get_adjacent_seats(i, j);
                let occupied_adj = adj.iter().filter(|&a| **a == Seat::Occupied).count();

                let mut new_seat = self.seats[i][j];
                if self.seats[i][j] == Seat::Empty && occupied_adj == 0 {
                    new_seat = Seat::Occupied;
                }
                if self.seats[i][j] == Seat::Occupied && occupied_adj >= 4 {
                    new_seat = Seat::Empty;
                }
                new_seats[i].push(new_seat);
            }
        }
        let new_seating = Seating { seats: new_seats };
        if new_seating == *self {
            None
        } else {
            Some(new_seating)
        }
    }

    fn get_adjacent_seats<'a>(&'a self, row: usize, col: usize) -> Vec<&'a Seat> {
        let mut adj = Vec::with_capacity(8);

        for i in cmp::max(row as i32 - 1, 0) as usize..=row + 1 {
            if let Some(line) = self.seats.get(i) {
                for j in cmp::max(col as i32 - 1, 0) as usize..=col + 1 {
                    if i != row || j != col {
                        adj.push(line.get(j));
                    }
                }
            }
        }
        adj.iter()
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_from_char() {
        assert_eq!(Seat::Empty, Seat::from('L'));
        assert_eq!(Seat::Occupied, Seat::from('#'));
        assert_eq!(Seat::Floor, Seat::from('.'));
    }

    #[test]
    fn test_seating_from_string() {
        let input = "L.L#
            .#L#";

        let seating = Seating::from(input);
        assert_eq!(
            vec![
                Seat::Empty,
                Seat::Floor,
                Seat::Empty,
                Seat::Occupied,
                Seat::Floor,
                Seat::Occupied,
                Seat::Empty,
                Seat::Occupied
            ],
            seating.seats.into_iter().flatten().collect::<Vec<Seat>>(),
        )
    }
}
