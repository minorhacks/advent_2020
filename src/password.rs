use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Policy {
    required_char: char,
    min: u32,
    max: u32,
}

impl std::str::FromStr for Policy {
    type Err = scan_fmt::parse::ScanError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (min, max, c) = scan_fmt!(s, "{}-{} {}", u32, u32, char)?;
        Ok(Policy {
            required_char: c,
            min: min,
            max: max,
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

pub fn password_meets_policy(policy: &Policy, password: &Password) -> bool {
    let freq_map = password
        .password
        .chars()
        .fold(HashMap::new(), |mut freq_map, c| {
            let counter = freq_map.entry(c).or_insert(0);
            *counter += 1;
            freq_map
        });
    let &freq = freq_map.get(&policy.required_char).or(Some(&0)).unwrap();
    policy.min <= freq && policy.max >= freq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_one_a_passes() {
        let line = "1-3 a: abcde";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(true, password_meets_policy(&policy, &password));
    }

    #[test]
    fn test_password_no_b_fails() {
        let line = "1-3 b: cdefg";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(false, password_meets_policy(&policy, &password));
    }

    #[test]
    fn test_password_many_c_passes() {
        let line = "2-9 c: ccccccccc";
        let (policy, password) = parse_password_policy_line(line).unwrap();
        assert_eq!(true, password_meets_policy(&policy, &password));
    }
}
