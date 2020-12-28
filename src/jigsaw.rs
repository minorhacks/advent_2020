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

pub struct Image(Vec<Vec<i8>>);

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

fn rotate<T: Default + Clone + Copy>(v: &[Vec<T>]) -> Vec<Vec<T>> {
    let (h, w) = (v.len(), v[0].len());
    let mut rot = vec![vec![Default::default(); w]; h];
    for (i, j) in (0..h).cartesian_product(0..w) {
        rot[j][w - 1 - i] = v[i][j];
    }
    rot
}

fn flip_horizontal<T: Clone>(v: &[Vec<T>]) -> Vec<Vec<T>> {
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

    fn data(&self) -> Vec<Vec<i8>> {
        self.data
            .iter()
            .map(|v| {
                v.iter()
                    .rev()
                    .skip(1)
                    .rev()
                    .skip(1)
                    .map(|c| match c {
                        '.' => -1,
                        '#' => 0,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .rev()
            .skip(1)
            .rev()
            .skip(1)
            .collect::<Vec<_>>()
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

    pub fn images(&self) -> Vec<Image> {
        // For each vert group
        let mut data = Vec::new();
        for vert_group in self.0.iter() {
            let mut line = Vec::new();
            for tile in vert_group {
                line = data_join(line, tile.data());
            }
            data.append(&mut line);
        }
        let mut images = Vec::new();
        images.push(Image(flip_horizontal(&data)));
        images.push(Image(data));
        for _i in 0..3 {
            let last = images.last().unwrap();
            let new_image_rotated = Image(flip_horizontal(&rotate(&last.0)));
            let new_image = Image(rotate(&last.0));
            images.push(new_image_rotated);
            images.push(new_image);
        }
        images
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|v| {
                v.iter()
                    .map(|i| match i {
                        -1 => ".".to_string(),
                        _ => i.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Image {
    pub fn mark_sea_monsters(&mut self) -> usize {
        static SEA_MONSTER: &[&str] = &[
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ];
        let mut sea_monster_count = 0;
        for line in 0..self.0.len() - 3 {
            for offset in 0..self.0[line].len() - SEA_MONSTER[0].len() {
                if self.sea_monster_line_matches(line, offset, SEA_MONSTER[0])
                    && self.sea_monster_line_matches(line + 1, offset, SEA_MONSTER[1])
                    && self.sea_monster_line_matches(line + 2, offset, SEA_MONSTER[2])
                {
                    sea_monster_count += 1;
                    self.mark_monster_pieces(line, offset, SEA_MONSTER[0]);
                    self.mark_monster_pieces(line + 1, offset, SEA_MONSTER[1]);
                    self.mark_monster_pieces(line + 2, offset, SEA_MONSTER[2]);
                }
            }
        }
        sea_monster_count
    }

    fn sea_monster_line_matches(&self, line: usize, col: usize, sea_monster_line: &str) -> bool {
        self.0
            .get(line)
            .unwrap()
            .iter()
            .skip(col)
            .take(sea_monster_line.len())
            .enumerate()
            .find(|(i, &v)| sea_monster_line.chars().nth(*i).unwrap() == '#' && v == -1)
            .is_none()
    }

    fn mark_monster_pieces(&mut self, line: usize, col: usize, sea_monster_line: &str) {
        for i in 0..sea_monster_line.len() {
            if sea_monster_line.chars().nth(i).unwrap() == '#' {
                self.0[line][col + i] += 1;
            }
        }
    }

    pub fn roughness(&self) -> usize {
        self.0
            .iter()
            .map(|v| v.iter().filter(|&&v| v == 0).count())
            .sum::<usize>()
    }
}

fn data_join(data: Vec<Vec<i8>>, next_tile: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut new_data = Vec::new();
    for i in 0..next_tile.len() {
        let mut data_line = data.get(i).unwrap_or(&Vec::new()).clone();
        let mut tile_line = next_tile[i].clone();
        data_line.append(&mut tile_line);
        new_data.push(data_line);
    }
    new_data
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

    #[test]
    fn test_find_sea_monsters() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        let mut images = tiles.assemble(3).images();
        let sea_monsters = images
            .iter_mut()
            .filter_map(|i| match i.mark_sea_monsters() {
                0 => None,
                i => Some(i),
            })
            .next()
            .unwrap();
        assert_eq!(2, sea_monsters);
    }

    #[test]
    fn test_roughness() {
        let tiles = TEST_INPUT.trim().parse::<Tiles>().unwrap();
        let mut images = tiles.assemble(3).images();
        let image = images
            .iter_mut()
            .filter_map(|i| match i.mark_sea_monsters() {
                0 => None,
                _ => Some(i),
            })
            .next()
            .unwrap();
        assert_eq!(273, image.roughness());
    }
}
