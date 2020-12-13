use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse shuttle ids")]
    ShuttleParseError,
}

pub struct ShuttleList(Vec<i32>);

impl std::str::FromStr for ShuttleList {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let ids = s
            .trim()
            .split(",")
            .filter(|&s| s != "x")
            .map(|s| s.parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_err| Error::ShuttleParseError)?;
        Ok(ShuttleList(ids))
    }
}

pub fn minutes_to_wait(now: i32, id: i32) -> i32 {
    id - (now.rem_euclid(id))
}

impl ShuttleList {
    pub fn next_shuttle(&self, now: i32) -> i32 {
        *self
            .0
            .iter()
            .min_by_key(|&&id| minutes_to_wait(now, id))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"7,13,x,x,59,x,31,19";

    #[test]
    fn test_next_shuttle() {
        let shuttles = TEST_INPUT.parse::<ShuttleList>().unwrap();
        assert_eq!(59, shuttles.next_shuttle(939));
    }
}
