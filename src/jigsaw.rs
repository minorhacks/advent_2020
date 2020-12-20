use itertools::Itertools;
use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse tile: {0}")]
    TileParseError(String),

    #[error("invalid corner spec")]
    InvalidCornerSpec,

    #[error("tile not found")]
    TileNotFound,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
struct Border(String);

#[derive(Debug)]
struct Tile {
    borders: HashMap<Direction, Border>,
}

type TileId = usize;

#[derive(Debug)]
pub struct Tiles(HashMap<TileId, Tile>);

impl std::str::FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        let mut borders = HashMap::new();
        borders.insert(
            Direction::Top,
            Border(s.lines().next().unwrap().to_string()),
        );
        borders.insert(
            Direction::Bottom,
            Border(s.lines().rev().next().unwrap().to_string()),
        );
        borders.insert(
            Direction::Left,
            Border(
                s.lines()
                    .map(|line| line.chars().next().unwrap())
                    .collect::<String>(),
            ),
        );
        borders.insert(
            Direction::Right,
            Border(
                s.lines()
                    .map(|line| line.chars().rev().next().unwrap())
                    .collect::<String>(),
            ),
        );
        Ok(Tile { borders })
    }
}

impl std::str::FromStr for Tiles {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let id_and_tile_map = s
            .trim()
            .split("\n\n")
            .map(|tile_str| {
                let mut lines = tile_str.lines();
                let id = lines
                    .next()
                    .ok_or_else(|| Error::TileParseError(tile_str.to_string()))?
                    .trim_start_matches("Tile ")
                    .trim_end_matches(':')
                    .parse::<TileId>()
                    .map_err(|_| Error::TileParseError(tile_str.to_string()))?;
                let tile = lines.join("\n").parse::<Tile>()?;
                Ok((id, tile))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        Ok(Tiles(id_and_tile_map))
    }
}

impl Border {
    fn matches(&self, other: &Border) -> bool {
        self.0 == other.0 || self.0 == other.0.chars().rev().collect::<String>()
    }
}

impl Tile {
    fn matches(&self, border: &Border) -> bool {
        self.borders.iter().any(|(_dir, b)| b.matches(border))
    }
}

impl Tiles {
    pub fn find_corners(&self) -> Vec<TileId> {
        self.0
            .iter()
            .filter_map(|(&id, t)| {
                match t
                    .borders
                    .iter()
                    .filter(|(_dir, b)| self.find_tile(id, b).is_none())
                    .count()
                {
                    2 => Some(id),
                    _ => None,
                }
            })
            .collect::<Vec<_>>()
    }

    fn find_tile(&self, tile: TileId, border: &Border) -> Option<TileId> {
        let found = self
            .0
            .iter()
            .find(|(&id, t)| id != tile && t.matches(&border))
            .map(|(id, _t)| *id);
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = &r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_tiles_parse() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        assert_eq!(9, tiles.0.len());
    }

    #[test]
    fn test_tiles_find_corner() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        let corners = tiles.find_corners();
        assert!(corners.contains(&1951));
        assert!(corners.contains(&3079));
        assert!(corners.contains(&2971));
        assert!(corners.contains(&1171));
    }
}
