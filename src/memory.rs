use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    last_seen: HashMap<usize, usize>,
    latest: usize,
}

impl Game {
    pub fn new(nums: Vec<usize>) -> Game {
        let latest = *nums.iter().rev().next().unwrap();
        let nums = nums
            .into_iter()
            .enumerate()
            .rev()
            .skip(1)
            .map(|(turn, num)| (num, turn + 1))
            .collect::<HashMap<_, _>>();
        Game {
            last_seen: nums,
            latest,
        }
    }

    pub fn play_until(&mut self, final_turn: usize) -> usize {
        for turn in self.last_seen.len() + 2..=final_turn {
            let new_latest = match self.last_seen.get(&self.latest) {
                Some(i) => turn - i - 1,
                None => 0,
            };
            self.last_seen.insert(self.latest, turn - 1);
            self.latest = new_latest;
        }
        self.latest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_until() {
        let mut game = Game::new(vec![0, 3, 6]);
        println!("{:?}", game);
        assert_eq!(436, game.play_until(2020));
    }

    #[test]
    #[cfg_attr(not(feature = "expensive_tests"), ignore)]
    fn test_play_until_long() {
        let mut game = Game::new(vec![0, 3, 6]);
        assert_eq!(175594, game.play_until(30000000));
    }
}
