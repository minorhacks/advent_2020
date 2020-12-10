use std::collections::BTreeSet;
use std::collections::HashSet;
use thiserror::Error as ThisError;

pub struct Report {
    items: BTreeSet<i32>,
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("pair with sum {0} not found")]
    PairNotFound(i32),

    #[error("triple with sum {0} not found")]
    TripleNotFound(i32),
}

type Result<T> = std::result::Result<T, Error>;

impl Report {
    pub fn new(items: &[i32]) -> Report {
        let num_map: BTreeSet<i32> = items.iter().cloned().collect();
        Report { items: num_map }
    }

    pub fn pair_with_sum(&self, sum: i32) -> Result<HashSet<i32>> {
        self.items
            .iter()
            .find(|&&item| self.items.contains(&(sum - item)))
            .ok_or(Error::PairNotFound(sum))
            .map(|&item| [item, sum - item].iter().cloned().collect())
    }

    pub fn triple_with_sum(&self, sum: i32) -> Result<HashSet<i32>> {
        self.items
            .iter()
            .find(|&&item| self.pair_with_sum(sum - item).is_ok())
            .ok_or(Error::TripleNotFound(sum))
            .map(|&item| {
                let mut pair = self.pair_with_sum(sum - item).unwrap();
                pair.insert(item);
                pair
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_with_sum() {
        let report = Report::new(&vec![1721, 979, 366, 299, 675, 1456]);
        let nums = report.pair_with_sum(2020).unwrap();
        assert_eq!(true, nums.contains(&1721));
        assert_eq!(true, nums.contains(&299));
    }

    #[test]
    fn test_triple_with_sum() {
        let report = Report::new(&vec![1721, 979, 366, 299, 675, 1456]);
        let nums = report.triple_with_sum(2020).unwrap();
        assert_eq!(true, nums.contains(&979));
        assert_eq!(true, nums.contains(&366));
        assert_eq!(true, nums.contains(&675));
    }
}
