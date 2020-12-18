use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse mask: {0}")]
    MaskParseError(String),

    #[error("failed to parse write: {0}")]
    WriteParseError(String),
}

pub struct Memory {
    words: HashMap<u64, u64>,
    mask: Mask,
}

pub enum Input {
    WriteVal(Write),
    SetMask(Mask),
}

pub enum InputV2 {
    WriteValV2(Write),
    SetMask(Mask),
}

pub struct Write {
    addr: u64,
    val: u64,
}

#[derive(Clone, Debug)]
pub struct Mask {
    bits_set: u64,
    bits_cleared: u64,
    dont_cares: Vec<usize>,
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
                    _ => '1',
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
                    _ => '0',
                })
                .collect::<String>(),
            2,
        )
        .map_err(|_| Error::MaskParseError(s.to_string()))?;
        let dont_cares = mask_str
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(i, c)| match c {
                'X' => Some(i),
                _ => None,
            })
            .collect::<Vec<_>>();
        Ok(Mask {
            bits_set: or_mask,
            bits_cleared: and_mask,
            dont_cares,
        })
    }
}

impl std::str::FromStr for Write {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (addr, val) = scan_fmt!(s, "mem[{}] = {}", u64, u64)
            .map_err(|_| Error::WriteParseError(s.to_string()))?;
        Ok(Write { addr, val })
    }
}

impl std::str::FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Input::SetMask(s.parse::<Mask>()?))
        } else {
            Ok(Input::WriteVal(s.parse::<Write>()?))
        }
    }
}

impl std::str::FromStr for InputV2 {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(InputV2::SetMask(s.parse::<Mask>()?))
        } else {
            Ok(InputV2::WriteValV2(s.parse::<Write>()?))
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            words: HashMap::new(),
            mask: Mask::new(),
        }
    }

    pub fn apply(&mut self, i: &Input) {
        match i {
            Input::WriteVal(w) => {
                let val = w.val | self.mask.bits_set;
                let val = val & self.mask.bits_cleared;
                self.words.insert(w.addr, val);
            }
            Input::SetMask(m) => self.mask = m.clone(),
        };
    }

    pub fn apply_v2(&mut self, i: &InputV2) {
        match i {
            InputV2::SetMask(m) => self.mask = m.clone(),
            InputV2::WriteValV2(w) => {
                for addr in self.mask.gen_addresses(w.addr) {
                    self.words.insert(addr, w.val);
                }
            }
        }
    }

    pub fn sum(&self) -> u64 {
        self.words.iter().map(|(_, v)| v).sum()
    }
}

impl Mask {
    fn new() -> Mask {
        Mask {
            bits_set: 0,
            bits_cleared: 0,
            dont_cares: Vec::new(),
        }
    }
    fn gen_addresses(&self, addr: u64) -> Vec<u64> {
        (0..=2u32.pow(self.dont_cares.len() as u32) - 1)
            .map(Self::bits_set)
            .map(|set_idx| {
                let mut addr = addr;
                addr |= self.bits_set;
                for (i, val) in self.dont_cares.iter().enumerate() {
                    addr = match val {
                        val if set_idx.contains(&i) => addr | (1 << val),
                        val => addr & (!(1 << val)),
                    };
                }
                addr
            })
            .collect::<Vec<_>>()
    }
    fn bits_set(mut i: u32) -> Vec<usize> {
        let mut v = Vec::new();
        let mut pos = 0;
        while i != 0 {
            if i & 1 == 1 {
                v.push(pos);
            }
            i >>= 1;
            pos += 1;
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = &r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static TEST_INPUT_V2: &str = &r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_writes() {
        let mut mem: Memory = Default::default();
        let _ = TEST_INPUT
            .trim()
            .lines()
            .map(|line| line.parse::<Input>().unwrap())
            .map(|i| mem.apply(&i))
            .collect::<Vec<_>>();
        assert_eq!(165, mem.sum());
    }

    #[test]
    fn test_writes_v2() {
        let mut mem: Memory = Default::default();
        let _ = TEST_INPUT_V2
            .trim()
            .lines()
            .map(|line| line.parse::<InputV2>().unwrap())
            .map(|i| mem.apply_v2(&i))
            .collect::<Vec<_>>();
        assert_eq!(208, mem.sum());
    }
}
