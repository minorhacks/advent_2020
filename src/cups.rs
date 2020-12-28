use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse char for cup: {0}")]
    CupParse(char),
}

pub struct Cups {
    next_map: Vec<u32>,
    max: u32,
    current: u32,
}

impl Cups {
    pub fn new(cup_list: &[u32]) -> Cups {
        let first = *cup_list.iter().next().unwrap();
        let mut i = cup_list.iter().peekable();
        let mut next_map = vec![0; cup_list.len() + 1];
        while let Some(v) = i.next() {
            next_map[*v as usize] = **i.peek().unwrap_or(&&first);
        }
        let &max = cup_list.iter().max().unwrap();
        Cups {
            next_map,
            max,
            current: first,
        }
    }

    pub fn step(&mut self) {
        // Take current cup
        let taken_1 = self.next_map[self.current as usize];
        let taken_2 = self.next_map[taken_1 as usize];
        let taken_3 = self.next_map[taken_2 as usize];

        // Calculate new cup, which is src - 1, or keep decrementing mod #cups
        // until we find a cup that isn't picked up
        let mut dest = ((self.current as i32 - 2).rem_euclid(self.max as i32) + 1) as u32;
        while [taken_1, taken_2, taken_3].contains(&dest) {
            dest = ((dest as i32 - 2).rem_euclid(self.max as i32) + 1) as u32;
        }

        // current's next is taken_3's old next
        self.next_map[self.current as usize] = self.next_map[taken_3 as usize];
        // taken_3's next is dest's old next
        self.next_map[taken_3 as usize] = self.next_map[dest as usize];
        // dest's next is taken_1
        self.next_map[dest as usize] = taken_1;

        // Update current
        self.current = self.next_map[self.current as usize];
    }

    pub fn run(&mut self, num_steps: usize) {
        for _ in 0..num_steps {
            self.step()
        }
    }

    pub fn order_after(&self, c: u32) -> String {
        let nums = self.first_n_after(c, self.next_map.len() - 2);
        nums.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn first_n_after(&self, c: u32, n: usize) -> Vec<u64> {
        let mut first_n = Vec::new();
        let mut travel = c;
        for _i in 0..n {
            travel = self.next_map[travel as usize];
            first_n.push(travel as u64);
        }
        first_n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_ORDER: &[u32] = &[3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn test_step_order() {
        let mut cups = Cups::new(TEST_ORDER);
        cups.step();
        assert_eq!("28915467", cups.order_after(3));
        cups.step();
        assert_eq!("25467891", cups.order_after(3));
        cups.step();
        assert_eq!("25891346", cups.order_after(7));
        cups.step();
        assert_eq!("25846791", cups.order_after(3));
        cups.step();
        assert_eq!("25841367", cups.order_after(9));
        cups.step();
        assert_eq!("25841936", cups.order_after(7));
        cups.step();
        assert_eq!("36741925", cups.order_after(8));
        cups.step();
        assert_eq!("41583926", cups.order_after(7));
        cups.step();
        assert_eq!("74183926", cups.order_after(5));
        cups.step();
        assert_eq!("83741926", cups.order_after(5));
    }

    #[test]
    fn test_run_order_after() {
        let mut cups = Cups::new(TEST_ORDER);
        cups.run(10);
        assert_eq!("92658374", cups.order_after(1));
        cups.run(90);
        assert_eq!("67384529", cups.order_after(1));
    }

    #[test]
    fn test_million_cups() {
        let all_cups = TEST_ORDER
            .iter()
            .cloned()
            .chain(TEST_ORDER.len() as u32 + 1..=1000000)
            .collect::<Vec<_>>();
        let mut cups = Cups::new(&all_cups);
        cups.run(10000000);
        let two_front = cups.first_n_after(1, 2);
        assert_eq!(934001, two_front[0]);
        assert_eq!(159792, two_front[1]);
    }
}
