use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse mask: {0}")]
    MaskParseError(String),

    #[error("failed to parse write: {0}")]
    WriteParseError(String),
}

type Result<T> = std::result::Result<T, Error>;

pub struct Memory {
    words: HashMap<usize, u64>,
    mask: Mask,
}

pub enum Input {
    Write(Write),
    Mask(Mask),
}

pub struct Write {
    addr: usize,
    val: u64,
}

#[derive(Clone)]
pub struct Mask {
    bits_set: u64,
    bits_cleared: u64,
}

impl std::str::FromStr for Mask {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mask_str =
            scan_fmt!(s, "mask = {}", String).map_err(|_| Error::MaskParseError(s.to_string()))?;
        let and_mask = u64::from_str_radix(
            &mask_str
                .chars()
                .map(|c| match c {
                    '0' => '0',
                    c => '1',
                })
                .collect::<String>(),
            2,
        )
        .map_err(|_| Error::MaskParseError(s.to_string()))?;
        let or_mask = u64::from_str_radix(
            &mask_str
                .chars()
                .map(|c| match c {
                    '1' => '1',
                    c => '0',
                })
                .collect::<String>(),
            2,
        )
        .map_err(|_| Error::MaskParseError(s.to_string()))?;
        Ok(Mask {
            bits_set: or_mask,
            bits_cleared: and_mask,
        })
    }
}

impl std::str::FromStr for Write {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (addr, val) = scan_fmt!(s, "mem[{}] = {}", usize, u64)
            .map_err(|_| Error::WriteParseError(s.to_string()))?;
        Ok(Write { addr, val })
    }
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Input::Mask(s.parse::<Mask>()?))
        } else {
            Ok(Input::Write(s.parse::<Write>()?))
        }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            words: HashMap::new(),
            mask: Mask {
                bits_set: 0,
                bits_cleared: 0,
            },
        }
    }

    pub fn apply(&mut self, i: &Input) {
        match i {
            Input::Write(w) => {
                let val = w.val | self.mask.bits_set;
                let val = val & self.mask.bits_cleared;
                self.words.insert(w.addr, val);
            }
            Input::Mask(m) => self.mask = m.clone(),
        };
    }

    pub fn sum(&self) -> u64 {
        self.words.iter().map(|(_, v)| v).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = &r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_writes() {
        let mut mem = Memory::new();
        let _ = TEST_INPUT
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .map(|i| mem.apply(&i))
            .collect::<Vec<_>>();
        assert_eq!(165, mem.sum());
    }
}
