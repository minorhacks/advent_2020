pub fn find_sum(preamble: &[i64], sum: i64) -> Option<(i64, i64)> {
    let mut preamble = preamble.to_vec();
    preamble.sort_unstable();
    let (mut i, mut j) = (0, preamble.len() - 1);
    while i < j {
        let current_sum = preamble[i] + preamble[j];
        match current_sum {
            _ if current_sum < sum => i += 1,
            _ if current_sum > sum => j -= 1,
            _ if current_sum == sum => return Some((preamble[i], preamble[j])),
            _ => panic!("should not get here"),
        };
    }
    None
}

pub fn first_missing_sum(seq: &[i64], preamble_len: usize) -> Option<i64> {
    (preamble_len..seq.len())
        .find(|&i| find_sum(&seq[i - preamble_len..i], seq[i]).is_none())
        .map(|i| seq[i])
}

pub fn find_sum_range(seq: &[i64], sum: i64) -> Option<&[i64]> {
    for start in 0..seq.len() - 1 {
        for end in start + 1..seq.len() - 1 {
            match seq[start..=end].iter().sum::<i64>() {
                seq_sum if seq_sum == sum => return Some(&seq[start..=end]),
                seq_sum if seq_sum > sum => break,
                seq_sum if seq_sum < sum => continue,
                _ => panic!("should not get here"),
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NUMS: &[i64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_sum() {
        assert!(find_sum(&TEST_NUMS[0..5], 40).is_some());
        assert!(find_sum(&TEST_NUMS[1..6], 62).is_some());
        assert!(find_sum(&TEST_NUMS[9..14], 127).is_none());
    }

    #[test]
    fn test_first_missing_sum() {
        assert_eq!(Some(127), first_missing_sum(TEST_NUMS, 5));
    }

    #[test]
    fn test_find_sum_range() {
        assert_eq!(
            Some(vec![15, 25, 47, 40].as_slice()),
            find_sum_range(TEST_NUMS, 127)
        );
    }
}
