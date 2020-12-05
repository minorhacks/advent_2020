use std::io::Error;
use std::io::ErrorKind;
pub struct Seat {
    row: i32,
    seat: i32,
}

impl Seat {
    pub fn id(&self) -> i32 {
        self.row * 8 + self.seat
    }
}

impl std::str::FromStr for Seat {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "boarding pass seat is wrong length",
            )));
        }

        let (row_str, seat_str) = (&s[0..7], &s[7..10]);
        Ok(Seat {
            row: i32::from_str_radix(
                &row_str
                    .chars()
                    .map(|c| match c {
                        'F' => '0',
                        'B' => '1',
                        _ => panic!(format!("unrecognized char: {}", c)),
                    })
                    .collect::<String>(),
                2,
            )?,
            seat: i32::from_str_radix(
                &seat_str
                    .chars()
                    .map(|c| match c {
                        'L' => '0',
                        'R' => '1',
                        _ => panic!(format!("unrecognized char: {}", c)),
                    })
                    .collect::<String>(),
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
