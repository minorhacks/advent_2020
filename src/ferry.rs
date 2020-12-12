use itertools::Itertools;
use ndarray::{arr1, arr2, Array1};
use std::convert::TryFrom;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse seat '{0}'")]
    SeatParseError(String),

    #[error("seat not found")]
    SeatNotFoundError,

    #[error("failed to parse instruction")]
    InstructionFormatError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("malformed instruction: '{0}'")]
    InstructionParseError(String),
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

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

pub struct Instructions(Vec<Instruction>);

pub struct Ferry {
    position: (i32, i32),
    orientation: i32,
}

pub struct FerryAndWaypoint {
    ferry_position: (i32, i32),
    waypoint_position: ndarray::Array1<i32>,
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match (
            s.chars()
                .nth(0)
                .ok_or(Error::InstructionParseError(s.to_string()))?,
            s[1..].parse::<i32>()?,
        ) {
            ('N', num) => Ok(Instruction::North(num)),
            ('S', num) => Ok(Instruction::South(num)),
            ('E', num) => Ok(Instruction::East(num)),
            ('W', num) => Ok(Instruction::West(num)),
            ('L', num) => Ok(Instruction::Left(num)),
            ('R', num) => Ok(Instruction::Right(num)),
            ('F', num) => Ok(Instruction::Forward(num)),
            _ => Err(Error::InstructionParseError(s.to_string())),
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::North(num) => write!(f, "North {}", num),
            Instruction::South(num) => write!(f, "South {}", num),
            Instruction::East(num) => write!(f, "East {}", num),
            Instruction::West(num) => write!(f, "West {}", num),
            Instruction::Forward(num) => write!(f, "Forward {}", num),
            Instruction::Left(num) => write!(f, "Left {}", num),
            Instruction::Right(num) => write!(f, "Right {}", num),
        }?;
        Ok(())
    }
}

impl std::str::FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Instructions(instructions))
    }
}

impl Ferry {
    pub fn new() -> Ferry {
        Ferry {
            orientation: 0,
            position: (0, 0),
        }
    }
    pub fn mov(&mut self, instructions: &Instructions) {
        let _ = instructions
            .0
            .iter()
            .map(|i| self.step(i))
            .collect::<Vec<_>>();
    }

    fn step(&mut self, i: &Instruction) {
        match i {
            Instruction::North(num) => self.position.1 += num,
            Instruction::South(num) => self.position.1 -= num,
            Instruction::East(num) => self.position.0 += num,
            Instruction::West(num) => self.position.0 -= num,
            Instruction::Forward(num) => {
                self.position = (
                    self.position.0 + self.orientation_vector().0 * num,
                    self.position.1 + self.orientation_vector().1 * num,
                )
            }
            Instruction::Left(num) => self.orientation = (self.orientation - num).rem_euclid(360),
            Instruction::Right(num) => self.orientation = (self.orientation + num).rem_euclid(360),
        }
    }

    pub fn distance_from_origin(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn orientation_vector(&self) -> (i32, i32) {
        static ORIENTATION_VEC: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1)];
        ORIENTATION_VEC[((self.orientation / 90).rem_euclid(4)) as usize]
    }
}

impl FerryAndWaypoint {
    pub fn new() -> FerryAndWaypoint {
        FerryAndWaypoint {
            ferry_position: (0, 0),
            waypoint_position: arr1(&[10, 1]),
        }
    }

    pub fn mov(&mut self, instructions: &Instructions) {
        let _ = instructions
            .0
            .iter()
            .map(|i| self.step(i))
            .collect::<Vec<_>>();
    }

    fn step(&mut self, i: &Instruction) {
        let right_rotations = [
            arr2(&[[0, 1], [1, 0]]),
            arr2(&[[0, -1], [1, 0]]),
            arr2(&[[-1, 0], [0, -1]]),
            arr2(&[[0, 1], [-1, 0]]),
        ];
        let left_rotations = [
            arr2(&[[0, 1], [1, 0]]),
            arr2(&[[0, 1], [-1, 0]]),
            arr2(&[[-1, 0], [0, -1]]),
            arr2(&[[0, -1], [1, 0]]),
        ];
        match i {
            Instruction::North(num) => self.waypoint_position[1] += num,
            Instruction::South(num) => self.waypoint_position[1] -= num,
            Instruction::East(num) => self.waypoint_position[0] += num,
            Instruction::West(num) => self.waypoint_position[0] -= num,
            Instruction::Forward(num) => {
                self.ferry_position = (
                    self.ferry_position.0 + self.waypoint_position[0] * num,
                    self.ferry_position.1 + self.waypoint_position[1] * num,
                )
            }
            Instruction::Left(num) => {
                self.waypoint_position = self
                    .waypoint_position
                    .dot(&left_rotations[(num / 90).rem_euclid(4) as usize])
            }
            Instruction::Right(num) => {
                self.waypoint_position = self
                    .waypoint_position
                    .dot(&right_rotations[(num / 90).rem_euclid(4) as usize])
            }
        }
    }

    pub fn distance_from_origin(&self) -> i32 {
        self.ferry_position.0.abs() + self.ferry_position.1.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static SEAT_INPUT: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    static INSTRUCTIONS_INPUT: &str = r"F10
N3
F7
R90
F11";

    #[test]
    fn test_stabilize() {
        let waiting_area = SEAT_INPUT.parse::<WaitingArea>().unwrap();
        let waiting_area = waiting_area.stabilize_adjacent();
        assert_eq!(37, waiting_area.num_occupied());
    }

    #[test]
    fn test_first_visible() {
        let waiting_area = SEAT_INPUT.parse::<WaitingArea>().unwrap();
        let waiting_area = waiting_area.stabilize_first_visible();
        assert_eq!(26, waiting_area.num_occupied());
    }

    #[test]
    fn test_ferry_move() {
        let instructions = INSTRUCTIONS_INPUT.parse::<Instructions>().unwrap();
        let mut ferry = Ferry::new();
        ferry.mov(&instructions);
        assert_eq!(25, ferry.distance_from_origin());
    }
    #[test]
    fn test_ferry_waypoint_move() {
        let instructions = INSTRUCTIONS_INPUT.parse::<Instructions>().unwrap();
        let mut ferry = FerryAndWaypoint::new();
        ferry.mov(&instructions);
        assert_eq!(286, ferry.distance_from_origin());
    }
}
