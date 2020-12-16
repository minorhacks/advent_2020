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
    nums: Vec<i32>,
}

#[derive(Debug)]
pub struct Rule {
    field: String,
    ranges: Vec<(i32, i32)>,
}

impl std::str::FromStr for Ticket {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split(",")
            .map(|s| s.parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::TicketParseError(s.to_string()))?;
        Ok(Ticket { nums })
    }
}

impl Ticket {
    pub fn is_valid(&self, rules: &[Rule]) -> bool {
        self.nums.iter().all(|&v| rules.iter().any(|r| r.test(v)))
    }

    pub fn error_rate(&self, rules: &[Rule]) -> i32 {
        self.nums
            .iter()
            .filter(|&&v| rules.iter().all(|r| !r.test(v)))
            .sum()
    }

    pub fn sum(&self) -> i32 {
        self.nums.iter().sum()
    }
}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut colon_split = s.split(": ");
        let field = colon_split
            .next()
            .ok_or(Error::RuleFieldError(s.to_string()))?
            .to_string();
        let ranges = colon_split
            .next()
            .ok_or(Error::RuleRangesError(s.to_string()))?
            .split(" or ")
            .map(|range_str| {
                let mut iter = range_str.split("-");
                (
                    iter.next().unwrap().parse::<i32>().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Rule { field, ranges })
    }
}

impl Rule {
    fn test(&self, val: i32) -> bool {
        match self
            .ranges
            .iter()
            .find(|(begin, end)| &val >= begin && &val <= end)
        {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RULES: &str = &r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50";

    #[test]
    fn test_ticket_valid() {
        let rules = TEST_RULES
            .trim()
            .lines()
            .map(|line| line.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(true, "7,1,14".parse::<Ticket>().unwrap().is_valid(&rules));
        assert_eq!(true, "7,3,47".parse::<Ticket>().unwrap().is_valid(&rules));
        assert_eq!(false, "40,4,50".parse::<Ticket>().unwrap().is_valid(&rules));
        assert_eq!(false, "55,2,20".parse::<Ticket>().unwrap().is_valid(&rules));
        assert_eq!(false, "38,6,12".parse::<Ticket>().unwrap().is_valid(&rules));
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
}
