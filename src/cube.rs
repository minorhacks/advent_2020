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
struct Coord3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coord4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

#[derive(Clone, Debug)]
pub struct Space3 {
    states: HashMap<Coord3, State>,
}

#[derive(Clone, Debug)]
pub struct Space4 {
    states: HashMap<Coord4, State>,
}

impl Space3 {
    pub fn from_initial_slice(slice_str: &str) -> Result<Space3> {
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
                                let c = Coord3 {
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
        let mut s = Space3 { states: active };
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

    fn neighbors_active_count(&self, coord: &Coord3) -> usize {
        let neighbors = coord.neighbors();
        neighbors
            .into_iter()
            .map(|n| match self.states.get(&n) {
                Some(&State::Active) => 1,
                _ => 0,
            })
            .sum()
    }

    #[must_use]
    pub fn step(&self) -> Space3 {
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
                new_states.entry(n).or_insert(State::Inactive);
            }
        }
        self.states = new_states;
    }
}

impl Space4 {
    pub fn from_initial_slice(slice_str: &str) -> Result<Space4> {
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
                                let c = Coord4 {
                                    x: i as i32,
                                    y: j as i32,
                                    z: 0,
                                    w: 0,
                                };
                                active.insert(c, state);
                            }
                            State::Inactive => (),
                        };
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut s = Space4 { states: active };
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

    fn neighbors_active_count(&self, coord: &Coord4) -> usize {
        let neighbors = coord.neighbors();
        neighbors
            .into_iter()
            .map(|n| match self.states.get(&n) {
                Some(&State::Active) => 1,
                _ => 0,
            })
            .sum()
    }

    #[must_use]
    pub fn step(&self) -> Space4 {
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
                new_states.entry(n).or_insert(State::Inactive);
            }
        }
        self.states = new_states;
    }
}
impl Coord3 {
    fn neighbors(&self) -> Vec<Coord3> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|c| (c.0 .0, c.0 .1, c.1))
            .filter(|c| c != &(0, 0, 0))
            .map(|(x, y, z)| Coord3 { x, y, z })
            .map(|c| self.add(c))
            .collect::<Vec<_>>()
    }

    fn add(&self, other: Coord3) -> Coord3 {
        Coord3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Coord4 {
    fn neighbors(&self) -> Vec<Coord4> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|c| (c.0 .0 .0, c.0 .0 .1, c.0 .1, c.1))
            .filter(|c| c != &(0, 0, 0, 0))
            .map(|(x, y, z, w)| Coord4 { x, y, z, w })
            .map(|c| self.add(c))
            .collect::<Vec<_>>()
    }

    fn add(&self, other: Coord4) -> Coord4 {
        Coord4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space3_step() {
        let space = Space3::from_initial_slice(".#.\n..#\n###").unwrap();
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
    #[cfg_attr(not(feature = "expensive_tests"), ignore)]
    fn test_space4_step() {
        let space = Space4::from_initial_slice(".#.\n..#\n###").unwrap();
        assert_eq!(5, space.active_count());
        let space = space.step();
        assert_eq!(29, space.active_count());
        let space = space.step();
        let space = space.step();
        let space = space.step();
        let space = space.step();
        let space = space.step();
        assert_eq!(848, space.active_count());
    }

    #[test]
    fn test_neighbors() {
        let n = Coord3 { x: 0, y: 0, z: 0 }.neighbors();
        assert_eq!(26, n.len());
    }
}
