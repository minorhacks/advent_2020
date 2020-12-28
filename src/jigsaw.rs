use itertools::Itertools;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse tile data")]
    TileParseError {
        #[from]
        source: ndarray::ShapeError,
    },

    #[error("invalid corner spec")]
    InvalidCornerSpec,

    #[error("tile not found")]
    TileNotFound,

    #[error("invalid Tile ID header")]
    InvalidTileHeader,
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

#[derive(Debug, Clone)]
struct Tile {
    id: TileId,
    data: Vec<Vec<char>>,
}

type TileId = usize;

#[derive(Debug)]
pub struct Tiles(Vec<Tile>);

pub struct Assembly(Vec<Vec<Tile>>);

impl std::str::FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let id = lines
            .next()
            .ok_or_else(|| Error::InvalidTileHeader)?
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<TileId>()
            .map_err(|_| Error::InvalidTileHeader)?;
        let data = lines
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Tile { id, data })
    }
}

impl std::str::FromStr for Tiles {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tiles = s
            .trim()
            .split("\n\n")
            .map(|tile_str| {
                let tile = tile_str.parse::<Tile>()?;
                Ok(tile)
            })
            .collect::<Result<Vec<_>>>()?;
        let tiles = tiles
            .iter()
            .map(|t| t.rotations())
            .flatten()
            .collect::<Vec<_>>();
        Ok(Tiles(tiles))
    }
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Bottom => Direction::Top,
            Direction::Top => Direction::Bottom,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn rotate(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let (h, w) = (v.len(), v[0].len());
    let mut rot = vec![vec!['\0'; w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        rot[j][w - 1 - i] = v[i][j];
    }
    rot
}

fn flip_horizontal(v: &[Vec<char>]) -> Vec<Vec<char>> {
    v.iter()
        .map(|inner| {
            let mut rev = inner.clone();
            rev.reverse();
            rev
        })
        .collect::<Vec<_>>()
}

impl Tile {
    fn matches(&self, other: &Tile, side: &Direction) -> bool {
        self.border(side) == other.border(&side.opposite())
    }

    fn border(&self, dir: &Direction) -> String {
        match dir {
            Direction::Top => self.data.first().unwrap().iter().collect::<String>(),
            Direction::Bottom => self.data.last().unwrap().iter().collect::<String>(),
            Direction::Left => self
                .data
                .iter()
                .map(|l| l.first().unwrap())
                .collect::<String>(),
            Direction::Right => self
                .data
                .iter()
                .map(|l| l.last().unwrap())
                .collect::<String>(),
        }
    }

    fn rotations(&self) -> Vec<Tile> {
        let mut tiles = Vec::new();
        tiles.push(Tile {
            id: self.id,
            data: flip_horizontal(&self.data),
        });
        tiles.push(self.clone());
        for _i in 0..3 {
            let last = tiles.last().unwrap();
            let new_tile_rotated = Tile {
                id: last.id,
                data: flip_horizontal(&rotate(&last.data)),
            };
            let new_tile = Tile {
                id: last.id,
                data: rotate(&last.data),
            };
            tiles.push(new_tile_rotated);
            tiles.push(new_tile);
        }
        tiles
    }
}

impl Tiles {
    fn find_top_left(&self) -> &Tile {
        self.0
            .iter()
            .find(|t| {
                self.find_tile(t, &Direction::Top).is_none()
                    && self.find_tile(t, &Direction::Left).is_none()
            })
            .unwrap()
    }

    pub fn assemble(&self, n: usize) -> Assembly {
        let mut line_travel = self.find_top_left();
        let mut lines = Vec::new();
        let mut line = Vec::new();
        let mut travel = line_travel;
        line.push(travel.clone());
        for _j in 1..n {
            travel = self.find_tile(travel, &Direction::Right).unwrap();
            line.push(travel.clone());
        }
        lines.push(line);
        for _i in 1..n {
            line_travel = self.find_tile(line_travel, &Direction::Bottom).unwrap();
            let mut line = Vec::new();
            let mut travel = line_travel;
            line.push(travel.clone());
            for _j in 1..n {
                travel = self.find_tile(travel, &Direction::Right).unwrap();
                line.push(travel.clone());
            }
            lines.push(line);
        }
        Assembly(lines)
    }

    fn find_tile(&self, tile: &Tile, side: &Direction) -> Option<&Tile> {
        let found = self
            .0
            .iter()
            .find(|other| other.id != tile.id && tile.matches(other, side));
        found
    }
}

impl std::fmt::Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .0
            .iter()
            .map(|line| {
                line.iter()
                    .map(|i| i.id.to_string())
                    .collect::<Vec<_>>()
                    .join("\t")
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", output)
    }
}

impl Assembly {
    pub fn corner_ids(&self) -> Vec<TileId> {
        vec![
            self.0.first().unwrap().first().unwrap().id,
            self.0.first().unwrap().last().unwrap().id,
            self.0.last().unwrap().first().unwrap().id,
            self.0.last().unwrap().last().unwrap().id,
        ]
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
        assert_eq!(9 * 8, tiles.0.len());
    }

    #[test]
    fn test_tiles_assemble() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        let assembly = tiles.assemble(3);
        assert_eq!(3, assembly.0.len());
    }

    #[test]
    fn test_tiles_find_corner() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        let assembly = tiles.assemble(3);
        println!("assembly:\n{}", assembly);
        let corners = assembly.corner_ids();
        println!("corners: {:?}", corners);
        assert!(corners.contains(&1951));
        assert!(corners.contains(&3079));
        assert!(corners.contains(&2971));
        assert!(corners.contains(&1171));
    }
}
