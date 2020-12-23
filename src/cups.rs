use std::collections::VecDeque;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse char for cup: {0}")]
    CupParse(char),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Cups {
    c: VecDeque<u32>,
    max: u32,
    front: u32,
}

impl std::str::FromStr for Cups {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let c = s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).ok_or(Error::CupParse(c)))
            .collect::<Result<VecDeque<u32>>>()?;
        let &max = c.iter().max().unwrap();
        Ok(Cups { c, max, front: 0 })
    }
}

impl Cups {
    pub fn step(&mut self) {
        let current = *self.c.iter().next().unwrap();
        let temp = self.c.pop_front().unwrap();
        self.c.push_back(temp);
        // Pick up 3 cups after current cupub p
        let mut picked_up = VecDeque::new();
        for _ in 0..3 {
            picked_up.push_back(self.c.pop_front().unwrap())
        }
        // Calculate new cup, which is src - 1, or keep decrementing mod #cups
        // until we find a cup that isn't picked up
        let mut dest = ((current as i32 - 2).rem_euclid(self.max as i32) + 1) as u32;
        while picked_up.contains(&dest) {
            dest = ((dest as i32 - 2).rem_euclid(self.max as i32) + 1) as u32;
        }
        // new array:
        let mut new_cups = VecDeque::new();
        // * current cup
        new_cups.push_back(self.c.pop_back().unwrap());
        // * everything between current cup up to and including dest cup, except
        //   for cups that are picked up
        let mut temp = None;
        while temp != Some(dest) {
            temp = Some(self.c.pop_front().unwrap());
            new_cups.push_back(temp.unwrap());
        }
        // * cups that were picked up
        while !picked_up.is_empty() {
            new_cups.push_back(picked_up.pop_front().unwrap());
        }
        // * any remaining cups
        while !self.c.is_empty() {
            new_cups.push_back(self.c.pop_front().unwrap());
        }
        self.c = new_cups;
        // Shuffle around until next current is in front
        let temp = self.c.pop_front().unwrap();
        self.c.push_back(temp);
        self.front = (self.front + 1).rem_euclid(self.max);
    }

    pub fn run(&mut self, num_steps: usize) {
        for _ in 0..num_steps {
            self.step()
        }
    }

    #[cfg(test)]
    fn order(&mut self) -> String {
        for _ in 0..self.front {
            let temp = self.c.pop_back().unwrap();
            self.c.push_front(temp);
        }
        let s = self
            .c
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("");
        for _ in 0..self.front {
            let temp = self.c.pop_front().unwrap();
            self.c.push_back(temp);
        }
        s
    }

    pub fn order_after(&self, c: u32) -> String {
        let mut copy = self.c.clone();
        while c != *copy.front().unwrap() {
            let temp = copy.pop_front().unwrap();
            copy.push_back(temp);
        }
        copy.into_iter()
            .skip(1)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_order() {
        let mut cups = "389125467".parse::<Cups>().unwrap();
        cups.step();
        assert_eq!("328915467", cups.order());
        cups.step();
        assert_eq!("325467891", cups.order());
        cups.step();
        assert_eq!("725891346", cups.order());
        cups.step();
        assert_eq!("325846791", cups.order());
        cups.step();
        assert_eq!("925841367", cups.order());
        cups.step();
        assert_eq!("725841936", cups.order());
        cups.step();
        assert_eq!("836741925", cups.order());
        cups.step();
        assert_eq!("741583926", cups.order());
        cups.step();
        assert_eq!("574183926", cups.order());
        cups.step();
        assert_eq!("583741926", cups.order());
    }

    #[test]
    fn test_run_order_after() {
        let mut cups = "389125467".parse::<Cups>().unwrap();
        cups.run(10);
        assert_eq!("92658374", cups.order_after(1));
        cups.run(90);
        assert_eq!("67384529", cups.order_after(1));
    }
}
