use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse shuttle ids")]
    ShuttleParseError,
}

enum Shuttle {
    OutOfService,
    Id(u64),
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
                    .parse::<u64>()
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
            .split(',')
            .map(|s| s.parse::<Shuttle>())
            .collect::<Result<Vec<_>>>()?;
        Ok(ShuttleList(ids))
    }
}

pub fn minutes_to_wait(now: u64, id: u64) -> u64 {
    id - (now.rem_euclid(id))
}

impl ShuttleList {
    pub fn next_shuttle(&self, now: u64) -> u64 {
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

    pub fn leaving_consecutively(&self) -> u64 {
        let mut shuttle_iter = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, shuttle)| match shuttle {
                Shuttle::Id(id) => Some((i, id)),
                _ => None,
            });
        let (time, step) = shuttle_iter.next().unwrap();
        let mut time = time as u64;
        let mut step = *step;
        for (offset, &id) in shuttle_iter {
            let offset = offset as u64;
            while (time + offset) % id != 0 {
                time += step;
            }
            step *= id;
        }
        time
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

    #[test]
    fn test_leaving_consecutively() {
        assert_eq!(
            1068781,
            TEST_INPUT
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
        assert_eq!(
            3417,
            "17,x,13,19"
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
        assert_eq!(
            754018,
            "67,7,59,61"
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
        assert_eq!(
            779210,
            "67,x,7,59,61"
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
        assert_eq!(
            1261476,
            "67,7,x,59,61"
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
        assert_eq!(
            1202161486,
            "1789,37,47,1889"
                .parse::<ShuttleList>()
                .unwrap()
                .leaving_consecutively()
        );
    }
}
