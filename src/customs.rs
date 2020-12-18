use std::collections::HashSet;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {}

pub struct FormResponses(Vec<HashSet<char>>);

impl std::str::FromStr for FormResponses {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let answer_set = s
            .trim()
            .lines()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>();
        Ok(FormResponses(answer_set))
    }
}

impl FormResponses {
    pub fn num_unique_questions(&self) -> usize {
        self.0
            .iter()
            .fold(HashSet::new(), |acc, set| {
                acc.union(&set).cloned().collect::<HashSet<_>>()
            })
            .len()
    }

    pub fn num_common_questions(&self) -> usize {
        self.0
            .iter()
            .skip(1)
            .fold(self.0.get(0).unwrap().clone(), |acc, set| {
                acc.intersection(&set).cloned().collect::<HashSet<_>>()
            })
            .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b
    ";

    #[test]
    fn test_num_unique_questions() {
        let unique_count = TEST_INPUT
            .trim()
            .split("\n\n")
            .map(|s| s.parse::<FormResponses>().unwrap().num_unique_questions())
            .collect::<Vec<_>>();
        assert_eq!(vec![3, 3, 3, 1, 1], unique_count);
    }

    #[test]
    fn test_num_common_questions() {
        let common_count = TEST_INPUT
            .trim()
            .split("\n\n")
            .map(|s| s.parse::<FormResponses>().unwrap().num_common_questions())
            .collect::<Vec<_>>();
        assert_eq!(vec![3, 0, 1, 1, 1], common_count);
    }
}
