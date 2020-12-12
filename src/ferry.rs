use itertools::Itertools;
use std::convert::TryFrom;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse seat '{0}'")]
    SeatParseError(String),

    #[error("seat not found")]
    SeatNotFoundError,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WaitingArea {
    seats: Vec<Vec<Seat>>,
}

impl std::str::FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::SeatParseError(s.to_string()));
        }
        match s
            .chars()
            .nth(0)
            .ok_or(Error::SeatParseError(s.to_string()))?
        {
            'L' => Ok(Seat::Empty),
            '#' => Ok(Seat::Occupied),
            '.' => Ok(Seat::Floor),
            _ => Err(Error::SeatParseError(s.to_string())),
        }
    }
}

impl std::str::FromStr for WaitingArea {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let seats = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<Seat>())
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(WaitingArea { seats })
    }
}

impl WaitingArea {
    pub fn stabilize_adjacent(&self) -> WaitingArea {
        let mut old = self.clone();
        let mut new = old.step_adjacent();
        while old != new {
            old = new;
            new = old.step_adjacent();
        }
        new
    }

    pub fn stabilize_first_visible(&self) -> WaitingArea {
        let mut old = self.clone();
        let mut new = old.step_first_visible();
        while old != new {
            old = new;
            new = old.step_first_visible();
        }
        new
    }

    pub fn num_occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|row| {
                row.iter()
                    .map(|s| match s {
                        Seat::Occupied => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn step_adjacent(&self) -> WaitingArea {
        let mut new = self.clone();
        let _ = (0..self.seats.len())
            .map(|row| {
                (0..self.seats[row].len())
                    .map(|col| {
                        let surrounding_seats = (-1..=1 as i32)
                            .cartesian_product(-1..=1 as i32)
                            .filter(|&coord| coord != (0, 0))
                            .map(|(delta_row, delta_col)| {
                                self.get_seat(usize_add(row, delta_row), usize_add(col, delta_col))
                            })
                            .filter_map(Result::ok);
                        let num_occupied =
                            surrounding_seats.filter(|&s| s == &Seat::Occupied).count();
                        let new_seat = match self.get_seat(row, col).unwrap() {
                            Seat::Floor => Seat::Floor,
                            Seat::Occupied if num_occupied >= 4 => Seat::Empty,
                            Seat::Empty if num_occupied == 0 => Seat::Occupied,
                            old_seat => old_seat.clone(),
                        };
                        new.set_seat(row, col, new_seat);
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        new
    }

    fn step_first_visible(&self) -> WaitingArea {
        let mut new = self.clone();
        let directions = (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&coord| coord != (0, 0))
            .collect::<Vec<_>>();
        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let mut seats = Vec::new();
                for dir in &directions {
                    let mut coords = (usize_add(row, dir.0), usize_add(col, dir.1));
                    let mut next = self.get_seat(coords.0, coords.1);
                    while next.is_ok() && next.as_ref().unwrap() == &&Seat::Floor {
                        coords = (usize_add(coords.0, dir.0), usize_add(coords.1, dir.1));
                        next = self.get_seat(coords.0, coords.1);
                    }
                    if next.is_ok() {
                        seats.push(next.unwrap());
                    }
                }
                let num_occupied = seats.iter().filter(|&&s| s == &Seat::Occupied).count();
                let new_seat = match self.get_seat(row, col).unwrap() {
                    Seat::Floor => Seat::Floor,
                    Seat::Occupied if num_occupied >= 5 => Seat::Empty,
                    Seat::Empty if num_occupied == 0 => Seat::Occupied,
                    old_seat => old_seat.clone(),
                };
                new.set_seat(row, col, new_seat);
            }
        }
        new
    }

    fn get_seat(&self, row: usize, col: usize) -> Result<&Seat> {
        self.seats
            .get(row)
            .and_then(|r| r.get(col))
            .ok_or(Error::SeatNotFoundError)
    }

    fn set_seat(&mut self, row: usize, col: usize, seat: Seat) {
        self.seats[row][col] = seat;
    }
}

fn usize_add(u: usize, i: i32) -> usize {
    if i < 0 && (-i as usize) > u {
        usize::MAX
    } else {
        usize::try_from(i32::try_from(u).unwrap() + i as i32).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_stabilize() {
        let waiting_area = TEST_INPUT.parse::<WaitingArea>().unwrap();
        let waiting_area = waiting_area.stabilize_adjacent();
        assert_eq!(37, waiting_area.num_occupied());
    }

    #[test]
    fn test_first_visible() {
        let waiting_area = TEST_INPUT.parse::<WaitingArea>().unwrap();
        let waiting_area = waiting_area.stabilize_first_visible();
        assert_eq!(26, waiting_area.num_occupied());
    }
}
