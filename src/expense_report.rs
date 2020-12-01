use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

pub struct Report {
    items: BTreeSet<i32>,
}

impl Report {
    pub fn new(items: &[i32]) -> Report {
        let num_map: BTreeSet<i32> = items.iter().cloned().collect();
        Report { items: num_map }
    }

    pub fn pair_with_sum(&self, sum: i32) -> Result<HashSet<i32>> {
        for num in self.items.iter().cloned() {
            if self.items.contains(&(sum - num)) {
                return Ok(vec![num, sum - num].into_iter().collect());
            }
        }
        Err(Error::new(
            ErrorKind::NotFound,
            format!("pair with sum {} not found", sum),
        ))
    }

    pub fn triple_with_sum(self, sum: i32) -> Result<HashSet<i32>> {
        for num in self.items.iter().cloned() {
            match self.pair_with_sum(sum - num) {
                Ok(mut num_map) => {
                    num_map.insert(num);
                    return Ok(num_map);
                }
                Err(_) => continue,
            }
        }
        Err(Error::new(
            ErrorKind::NotFound,
            format!("triple with sum {} not found", sum),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_with_sum() {
        let report = Report::new(&vec![1721, 979, 366, 299, 675, 1456]);
        let nums = report.pair_with_sum(2020).unwrap();
        assert_eq!(true, nums.contains(&1721));
        assert_eq!(true, nums.contains(&299));
    }

    #[test]
    fn test_triple_with_sum() {
        let report = Report::new(&vec![1721, 979, 366, 299, 675, 1456]);
        let nums = report.triple_with_sum(2020).unwrap();
        assert_eq!(true, nums.contains(&979));
        assert_eq!(true, nums.contains(&366));
        assert_eq!(true, nums.contains(&675));
    }
}
