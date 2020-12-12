use itertools::Itertools;
use std::convert::TryFrom;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse seat '{0}'")]
    SeatParseError(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq)]
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
    pub fn stabilize(&self) -> WaitingArea {
        let mut old = self.clone();
        let mut new = old.step();
        while old != new {
            old = new;
            new = old.step();
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

    fn step(&self) -> WaitingArea {
        let mut new = self.clone();
        let _ = (0..self.seats.len())
            .map(|row| {
                (0..self.seats[row].len())
                    .map(|col| {
                        let surrounding_seats = (-1..=1 as i32)
                            .cartesian_product(-1..=1 as i32)
                            .filter(|&coord| coord != (0, 0))
                            .map(|(delta_row, delta_col)| {
                                self.get_seat(
                                    usize::try_from(i32::try_from(row).unwrap() + delta_row)
                                        .unwrap_or(usize::MAX),
                                    usize::try_from(i32::try_from(col).unwrap() + delta_col)
                                        .unwrap_or(usize::MAX),
                                )
                            });
                        let num_occupied =
                            surrounding_seats.filter(|&s| s == &Seat::Occupied).count();
                        let new_seat = match self.get_seat(row, col) {
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

    fn get_seat(&self, row: usize, col: usize) -> &Seat {
        self.seats
            .get(row)
            .and_then(|r| r.get(col))
            .or(Some(&Seat::Floor))
            .unwrap()
    }

    fn set_seat(&mut self, row: usize, col: usize, seat: Seat) {
        self.seats[row][col] = seat;
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
        let waiting_area = waiting_area.stabilize();
        assert_eq!(37, waiting_area.num_occupied());
    }
}
