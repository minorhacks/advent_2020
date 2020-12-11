use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("seat string is length {0}; want length 10")]
    SeatLengthError(usize),

    #[error("unexpected char {0} in seat string")]
    SeatCharError(char),

    #[error("error while parsing seat")]
    SeatParseError {
        #[from]
        source: std::num::ParseIntError,
    },
}

type Result<T> = std::result::Result<T, Error>;

pub struct Seat {
    id: i32,
}

impl Seat {
    pub fn id(&self) -> i32 {
        self.id
    }
}

impl std::str::FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Error::SeatLengthError(s.len()));
        }

        Ok(Seat {
            id: i32::from_str_radix(
                &s.chars()
                    .map(|c| match c {
                        'F' => Ok('0'),
                        'B' => Ok('1'),
                        'L' => Ok('0'),
                        'R' => Ok('1'),
                        _ => Err(Error::SeatCharError(c)),
                    })
                    .collect::<Result<String>>()?,
                2,
            )?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seats() {
        let seat_strs = vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];
        let seat_ids = seat_strs
            .into_iter()
            .map(|s| s.parse::<Seat>().unwrap())
            .map(|seat| seat.id())
            .collect::<Vec<_>>();
        assert_eq!(vec![357, 567, 119, 820], seat_ids)
    }
}
