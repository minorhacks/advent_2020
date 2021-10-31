use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("failed to parse ticket: '{0}'")]
    TicketParseError(String),

    #[error("failed to parse field of rule: '{0}'")]
    RuleFieldError(String),

    #[error("failed to parse ranges of rule: '{0}'")]
    RuleRangesError(String),
}

pub struct Ticket {
    nums: Vec<i64>,
}

#[derive(Debug)]
pub struct Rule {
    pub field: String,
    ranges: Vec<(i64, i64)>,
}

impl std::str::FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::TicketParseError(s.to_string()))?;
        Ok(Ticket { nums })
    }
}

impl Ticket {
    pub fn is_valid(&self, rules: &[Rule]) -> bool {
        self.nums.iter().all(|&v| rules.iter().any(|r| r.test(v)))
    }

    pub fn error_rate(&self, rules: &[Rule]) -> i64 {
        self.nums
            .iter()
            .filter(|&&v| rules.iter().all(|r| !r.test(v)))
            .sum()
    }

    pub fn sum(&self) -> i64 {
        self.nums.iter().sum()
    }

    pub fn get(&self, i: usize) -> i64 {
        self.nums[i]
    }

    pub fn num_fields(&self) -> usize {
        self.nums.len()
    }
}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut colon_split = s.split(": ");
        let field = colon_split
            .next()
            .ok_or_else(|| Error::RuleFieldError(s.to_string()))?
            .to_string();
        let ranges = colon_split
            .next()
            .ok_or_else(|| Error::RuleRangesError(s.to_string()))?
            .split(" or ")
            .map(|range_str| {
                let mut iter = range_str.split('-');
                (
                    iter.next().unwrap().parse::<i64>().unwrap(),
                    iter.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Rule { field, ranges })
    }
}

impl Rule {
    fn test(&self, val: i64) -> bool {
        self.ranges
            .iter()
            .any(|(begin, end)| &val >= begin && &val <= end)
    }

    pub fn possible_fields(&self, tickets: &[Ticket]) -> HashSet<usize> {
        (0..tickets[0].num_fields())
            .filter(|&i| tickets.iter().all(|t| self.test(t.get(i))))
            .collect::<HashSet<_>>()
    }
}

pub fn resolve_field_map(mut m: HashMap<String, HashSet<usize>>) -> HashMap<String, usize> {
    let mut ret = HashMap::new();
    loop {
        if m.is_empty() {
            break;
        }
        let (field, set) = match m.iter().find(|(_, v)| v.len() == 1) {
            Some(pair) => pair,
            None => return ret,
        };
        let &val = set.iter().next().unwrap();
        ret.insert(field.clone(), val);
        m = m
            .into_iter()
            .map(|(k, mut v)| {
                v.remove(&val);
                (k, v)
            })
            .collect::<HashMap<_, _>>();
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RULES: &str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50";

    static TEST_RULES_2: &str = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19";

    #[test]
    fn test_ticket_valid() {
        let rules = TEST_RULES
            .trim()
            .lines()
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        assert!("7,1,14".parse::<Ticket>().unwrap().is_valid(&rules));
        assert!("7,3,47".parse::<Ticket>().unwrap().is_valid(&rules));
        assert!("40,4,50".parse::<Ticket>().unwrap().is_valid(&rules));
        assert!("55,2,20".parse::<Ticket>().unwrap().is_valid(&rules));
        assert!("38,6,12".parse::<Ticket>().unwrap().is_valid(&rules));
    }

    #[test]
    fn test_ticket_error_rate() {
        let rules = TEST_RULES
            .trim()
            .lines()
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(0, "7,1,14".parse::<Ticket>().unwrap().error_rate(&rules));
        assert_eq!(0, "7,3,47".parse::<Ticket>().unwrap().error_rate(&rules));
        assert_eq!(4, "40,4,50".parse::<Ticket>().unwrap().error_rate(&rules));
        assert_eq!(55, "55,2,20".parse::<Ticket>().unwrap().error_rate(&rules));
        assert_eq!(12, "38,6,12".parse::<Ticket>().unwrap().error_rate(&rules));
    }

    #[test]
    fn test_find_field() {
        let rules = TEST_RULES_2
            .trim()
            .lines()
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        let tickets = vec!["3,9,18", "15,1,5", "5,14,9"]
            .into_iter()
            .map(|str| str.parse::<Ticket>().unwrap())
            .collect::<Vec<_>>();
        assert!(rules[1].possible_fields(&tickets).contains(&0));
        assert!(rules[0].possible_fields(&tickets).contains(&1));
        assert!(rules[2].possible_fields(&tickets).contains(&2));
    }
}
