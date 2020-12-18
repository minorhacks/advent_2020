use std::result::Result;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("unrecognized character in map: {0}")]
    IllegalMapCharacter(char),
}

pub enum Space {
    Open,
    Tree,
}
pub struct Map {
    spaces: Vec<Vec<Space>>,
}

impl std::str::FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let spaces = s
            .trim()
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => Ok(Space::Open),
                        '#' => Ok(Space::Tree),
                        _ => Err(Error::IllegalMapCharacter(c)),
                    })
                    .collect()
            })
            .collect::<Result<_, _>>()?;
        Ok(Map { spaces })
    }
}

impl Map {
    pub fn at(&self, x: usize, y: usize) -> &Space {
        let line = &self.spaces[y];
        &line[x % line.len()]
    }

    pub fn height(&self) -> usize {
        self.spaces.len()
    }
}

pub fn count_trees(map: &Map, delta_x: usize, delta_y: usize) -> usize {
    (0..)
        .step_by(delta_x)
        .zip((0..map.height()).step_by(delta_y))
        .map(|(x, y)| match map.at(x, y) {
            Space::Open => 0,
            Space::Tree => 1,
        })
        .sum()
}

pub fn count_trees_batch(map: &Map, slopes: Vec<(usize, usize)>) -> Vec<usize> {
    slopes
        .into_iter()
        .map(|(x, y)| count_trees(map, x, y))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_MAP: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn test_over_three_down_one() {
        let map: Map = TEST_MAP.parse().unwrap();
        assert_eq!(7, count_trees(&map, 3, 1))
    }

    #[test]
    fn test_count_trees_batch() {
        let map = TEST_MAP.parse().unwrap();
        assert_eq!(
            336,
            count_trees_batch(&map, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
                .into_iter()
                .product::<usize>()
        )
    }
}
