use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse shuttle ids")]
    ShuttleParseError,
}

enum Shuttle {
    OutOfService,
    Id(i32),
}

type Result<T> = std::result::Result<T, Error>;

pub struct ShuttleList(Vec<Shuttle>);

impl std::str::FromStr for Shuttle {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.trim() == "x" {
            Ok(Shuttle::OutOfService)
        } else {
            Ok(Shuttle::Id(
                s.trim()
                    .parse::<i32>()
                    .map_err(|_| Error::ShuttleParseError)?,
            ))
        }
    }
}

impl std::str::FromStr for ShuttleList {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let ids = s
            .trim()
            .split(",")
            .map(|s| s.parse::<Shuttle>())
            .collect::<Result<Vec<_>>>()?;
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
            .filter_map(|s| match s {
                Shuttle::Id(id) => Some(id),
                Shuttle::OutOfService => None,
            })
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
