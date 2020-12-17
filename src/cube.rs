use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse space: {0}")]
    SpaceParseError(String),

    #[error("failed to parse state: {0}")]
    UnknownStateError(char),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
enum State {
    Active,
    Inactive,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug)]
pub struct Space {
    states: HashMap<Coord, State>,
}

impl Space {
    pub fn from_initial_slice(slice_str: &str) -> Result<Space> {
        let x = slice_str
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(State::Inactive),
                        '#' => Ok(State::Active),
                        c => Err(Error::UnknownStateError(c)),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;
        let mut active = HashMap::new();
        let _ = x
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                v.into_iter()
                    .enumerate()
                    .map(|(j, state)| {
                        match state {
                            State::Active => {
                                let c = Coord {
                                    x: i as i32,
                                    y: j as i32,
                                    z: 0,
                                };
                                active.insert(c, state);
                            }
                            State::Inactive => (),
                        };
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut s = Space { states: active };
        s.pad();
        Ok(s)
    }

    pub fn active_count(&self) -> usize {
        self.states
            .iter()
            .map(|(_, v)| match *v {
                State::Active => 1,
                State::Inactive => 0,
            })
            .sum()
    }

    pub fn neighbors_active_count(&self, coord: &Coord) -> usize {
        let neighbors = coord.neighbors();
        neighbors
            .into_iter()
            .map(|n| match self.states.get(&n) {
                Some(&State::Active) => 1,
                _ => 0,
            })
            .sum()
    }

    pub fn step(&self) -> Space {
        let mut new = self.clone();
        let _ = self
            .states
            .iter()
            .map(|(coord, state)| {
                let new_state = match self.neighbors_active_count(coord) {
                    2..=3 if state == &State::Active => State::Active,
                    3 if state == &State::Inactive => State::Active,
                    _ => State::Inactive,
                };
                new.states.insert(coord.clone(), new_state);
            })
            .collect::<Vec<_>>();
        new.pad();
        new
    }

    fn pad(&mut self) {
        let mut new_states = self.states.clone();
        for (coord, _val) in self.states.iter() {
            let neighbors = coord.neighbors();
            for n in neighbors.into_iter() {
                if !new_states.contains_key(&n) {
                    new_states.insert(n, State::Inactive);
                }
            }
        }
        self.states = new_states;
    }
}

impl Coord {
    fn neighbors(&self) -> Vec<Coord> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|c| (c.0 .0, c.0 .1, c.1))
            .filter(|c| c != &(0, 0, 0))
            .map(|(x, y, z)| Coord { x, y, z })
            .map(|c| self.add(c))
            .collect::<Vec<_>>()
    }

    fn add(&self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let space = Space::from_initial_slice(".#.\n..#\n###").unwrap();
        assert_eq!(5, space.active_count());
        let space = space.step();
        assert_eq!(11, space.active_count());
        let space = space.step();
        assert_eq!(21, space.active_count());
        let space = space.step();
        assert_eq!(38, space.active_count());
        let space = space.step();
        let space = space.step();
        let space = space.step();
        assert_eq!(112, space.active_count());
    }

    #[test]
    fn test_neighbors() {
        let n = Coord { x: 0, y: 0, z: 0 }.neighbors();
        assert_eq!(26, n.len());
    }
}
