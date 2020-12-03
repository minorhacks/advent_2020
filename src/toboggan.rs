pub enum Space {
    Open,
    Tree,
}
pub struct Map {
    spaces: Vec<Vec<Space>>,
}

impl Map {
    pub fn from_string(s: &str) -> Map {
        let spaces = s
            .trim()
            .split("\n")
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => Space::Open,
                        '#' => Space::Tree,
                        _ => panic!(format!("unrecognized character in map: {}", c)),
                    })
                    .collect()
            })
            .collect();
        Map { spaces: spaces }
    }

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
        let map = Map::from_string(TEST_MAP);
        assert_eq!(7, count_trees(&map, 3, 1))
    }

    #[test]
    fn test_count_trees_batch() {
        let map = Map::from_string(TEST_MAP);
        assert_eq!(
            336,
            count_trees_batch(&map, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
                .into_iter()
                .fold(1, |acc, n| acc * n)
        )
    }
}
