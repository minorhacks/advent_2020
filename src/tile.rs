use std::collections::HashMap;
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
            self.tiles.entry(c.clone()).or_insert(Color::White).flip();
        }
    }

    pub fn color_count(&self, color: &Color) -> usize {
        self.tiles.iter().filter(|(_c, t)| *t == color).count()
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
}
