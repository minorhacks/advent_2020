use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unknown direction char: '{0}'")]
    UnknownDir(char),
}

#[cfg(test)]
type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
enum Dir {
    None,
    NorthEast,
    NorthWest,
    East,
    West,
    SouthEast,
    SouthWest,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::Black => *self = Color::White,
            Color::White => *self = Color::Black,
        };
    }
}

#[derive(Default)]
pub struct Map {
    tiles: HashMap<Coord, Color>,
}

impl Coord {
    fn new() -> Coord {
        Coord { x: 0, y: 0, z: 0 }
    }

    fn step(self, dir: &Dir) -> Coord {
        match &dir {
            Dir::None => self,
            Dir::NorthEast => Coord {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
            },
            Dir::NorthWest => Coord {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
            },
            Dir::East => Coord {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
            },
            Dir::West => Coord {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
            },
            Dir::SouthEast => Coord {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
            },
            Dir::SouthWest => Coord {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
            },
        }
    }
}

impl std::str::FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s = s.chars();
        let mut coords = Vec::new();
        loop {
            match s.next() {
                None => break,
                Some(n_or_s) if n_or_s == 'n' || n_or_s == 's' => match n_or_s {
                    'n' => match s.next() {
                        Some('e') => coords.push(Dir::NorthEast),
                        Some('w') => coords.push(Dir::NorthWest),
                        Some(c) => return Err(Error::UnknownDir(c)),
                        None => return Err(Error::UnknownDir(n_or_s)),
                    },
                    's' => match s.next() {
                        Some('e') => coords.push(Dir::SouthEast),
                        Some('w') => coords.push(Dir::SouthWest),
                        Some(c) => return Err(Error::UnknownDir(c)),
                        None => return Err(Error::UnknownDir(n_or_s)),
                    },
                    c => panic!(format!("unreachable: {}", c)),
                },
                Some(e_or_w) if e_or_w == 'e' || e_or_w == 'w' => match e_or_w {
                    'e' => coords.push(Dir::East),
                    'w' => coords.push(Dir::West),
                    c => panic!(format!("unreachable: {}", c)),
                },
                Some(c) => return Err(Error::UnknownDir(c)),
            };
        }
        Ok(coords.iter().fold(Coord::new(), |acc, dir| acc.step(dir)))
    }
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: HashMap::new(),
        }
    }

    pub fn flip_all(&mut self, coords: &[Coord]) {
        for c in coords {
            self.get(c).flip();
        }
    }

    pub fn color_count(&self, color: &Color) -> usize {
        self.tiles.iter().filter(|(_c, t)| *t == color).count()
    }

    fn get(&mut self, c: &Coord) -> &mut Color {
        self.tiles.entry(c.clone()).or_insert(Color::White)
    }

    fn color_adjacent(&mut self, c: &Coord, color: &Color) -> usize {
        [
            Dir::East,
            Dir::West,
            Dir::NorthEast,
            Dir::NorthWest,
            Dir::SouthEast,
            Dir::SouthWest,
        ]
        .iter()
        .map(|d| {
            if self.get(&c.clone().step(d)) == color {
                1
            } else {
                0
            }
        })
        .sum()
    }

    fn flip_day(&mut self) {
        let mut new_tiles = HashMap::new();
        let mut coords_to_process = HashSet::new();
        for c in self.tiles.iter().filter_map(|(k, v)| match v {
            Color::Black => Some(k),
            Color::White => None,
        }) {
            coords_to_process.insert(c.clone().step(&Dir::None));
            coords_to_process.insert(c.clone().step(&Dir::East));
            coords_to_process.insert(c.clone().step(&Dir::West));
            coords_to_process.insert(c.clone().step(&Dir::NorthEast));
            coords_to_process.insert(c.clone().step(&Dir::NorthWest));
            coords_to_process.insert(c.clone().step(&Dir::SouthEast));
            coords_to_process.insert(c.clone().step(&Dir::SouthWest));
        }
        for c in coords_to_process {
            match self.get(&c) {
                Color::White => {
                    if self.color_adjacent(&c, &Color::Black) == 2 {
                        *new_tiles.entry(c).or_insert(Color::White) = Color::Black;
                    }
                }
                Color::Black => {
                    let num_adjacent = self.color_adjacent(&c, &Color::Black);
                    if (1..=2).contains(&num_adjacent) {
                        *new_tiles.entry(c).or_insert(Color::White) = Color::Black;
                    }
                }
            }
        }
        self.tiles = new_tiles;
    }

    pub fn flip_days(&mut self, num_days: usize) {
        for _ in 0..num_days {
            self.flip_day();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = &r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_tile_flip() {
        let coords = TEST_INPUT
            .trim()
            .lines()
            .map(|l| l.parse::<Coord>())
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let mut map = Map::new();
        map.flip_all(&coords);
        assert_eq!(10, map.color_count(&Color::Black));
    }

    #[test]
    fn test_coord_parse() {
        let c = "esew".parse::<Coord>().unwrap();
        assert_eq!(Coord { x: -1, y: 0, z: 1 }, c);
        let c = "nwwswee".parse::<Coord>().unwrap();
        assert_eq!(Coord { x: 0, y: 0, z: 0 }, c);
    }

    #[test]
    fn test_flip_day() {
        let coords = TEST_INPUT
            .trim()
            .lines()
            .map(|l| l.parse::<Coord>())
            .collect::<Result<Vec<_>>>()
            .unwrap();
        let mut map = Map::new();
        map.flip_all(&coords);
        assert_eq!(10, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(15, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(12, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(25, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(14, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(23, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(28, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(41, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(37, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(49, map.color_count(&Color::Black));
        map.flip_day();
        assert_eq!(37, map.color_count(&Color::Black));
        map.flip_days(90);
        assert_eq!(2208, map.color_count(&Color::Black));
    }
}
