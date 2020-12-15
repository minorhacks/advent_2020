pub struct Game(Vec<usize>);

impl Game {
    pub fn new(nums: Vec<usize>) -> Game {
        Game(nums)
    }

    pub fn play_until(&self, iter: usize) -> usize {
        let mut nums = self.0.clone();
        while nums.len() < iter {
            let mut nums_iter = nums.iter().rev();
            let current = nums_iter.nth(0).unwrap();
            let next = match nums_iter.position(|v| v == current) {
                Some(i) => i + 1,
                None => 0,
            };
            nums.push(next);
        }
        nums.iter().rev().nth(0).unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_until() {
        let game = Game::new(vec![0, 3, 6]);
        assert_eq!(436, game.play_until(2020));
    }
}
