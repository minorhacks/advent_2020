pub fn find_loop_size(subject_num: u64, public_key: u64) -> u64 {
    (1..)
        .scan(1, |acc, i| {
            *acc = (*acc * subject_num).rem_euclid(20201227);
            Some((i, *acc))
        })
        .find(|(_i, val)| *val == public_key)
        .map(|(i, _val)| i)
        .unwrap()
}

pub fn transform(subject_num: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |acc, _i| (acc * subject_num).rem_euclid(20201227))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        assert_eq!(8, find_loop_size(7, 5764801));
        assert_eq!(11, find_loop_size(7, 17807724));
    }

    #[test]
    fn test_transform() {
        assert_eq!(14897079, transform(17807724, 8));
        assert_eq!(14897079, transform(5764801, 11));
    }
}
