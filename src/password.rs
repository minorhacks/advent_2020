use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Policy {
    required_char: char,
    num_1: usize,
    num_2: usize,
}

impl std::str::FromStr for Policy {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (num_1, num_2, c) = scan_fmt!(s, "{}-{} {}", usize, usize, char)?;
        Ok(Policy {
            required_char: c,
            num_1: num_1,
            num_2: num_2,
        })
    }
}

pub struct Password {
    password: String,
}

impl Password {
    fn new(password_str: &str) -> Password {
        Password {
            password: password_str.to_string(),
        }
    }
}

pub fn parse_password_policy_line(line: &str) -> Result<(Policy, Password)> {
    let elements = line.splitn(2, ": ").collect::<Vec<_>>();
    let policy: Policy = elements[0].parse()?;
    let password = Password::new(elements[1]);
    Ok((policy, password))
}

pub fn meets_policy_frequency(policy: &Policy, password: &Password) -> bool {
    let freq_map = password
        .password
        .chars()
        .fold(HashMap::new(), |mut freq_map, c| {
            let counter = freq_map.entry(c).or_insert(0);
            *counter += 1;
            freq_map
        });
    let &freq = freq_map.get(&policy.required_char).or(Some(&0)).unwrap();
    policy.num_1 <= freq && policy.num_2 >= freq
}

pub fn meets_policy_position(policy: &Policy, password: &Password) -> bool {
    let in_first_pos =
        password.password.chars().nth(policy.num_1 - 1).unwrap() == policy.required_char;
    let in_second_pos =
        password.password.chars().nth(policy.num_2 - 1).unwrap() == policy.required_char;
    in_first_pos ^ in_second_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_frequency_one_a_passes() {
        let line = "1-3 a: abcde";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(true, meets_policy_frequency(&policy, &password));
    }

    #[test]
    fn test_password_frequency_no_b_fails() {
        let line = "1-3 b: cdefg";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(false, meets_policy_frequency(&policy, &password));
    }

    #[test]
    fn test_password_frequency_many_c_passes() {
        let line = "2-9 c: ccccccccc";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(true, meets_policy_frequency(&policy, &password));
    }

    #[test]
    fn test_password_posiiton_one_a_passes() {
        let line = "1-3 a: abcde";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(true, meets_policy_position(&policy, &password));
    }

    #[test]
    fn test_password_posiiton_no_b_fails() {
        let line = "1-3 b: cdefg";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(false, meets_policy_position(&policy, &password));
    }

    #[test]
    fn test_password_posiiton_many_c_fails() {
        let line = "2-9 c: ccccccccc";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(false, meets_policy_position(&policy, &password));
    }
}
