use std::collections::HashMap;
use std::collections::HashSet;

pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn from_string(s: &str) -> Passport {
        let fields = s
            .trim()
            .split_whitespace()
            .map(|s| {
                let field_parts = s.splitn(2, ":").collect::<Vec<_>>();
                (field_parts[0].to_string(), field_parts[1].to_string())
            })
            .collect::<HashMap<_, _>>();
        Passport { fields }
    }

    pub fn has_fields(&self, field_names: &HashSet<String>) -> bool {
        let keys = self.fields.keys().cloned().collect::<HashSet<_>>();
        keys.is_superset(field_names)
    }
}

pub fn valid_passport_fields() -> HashSet<String> {
    vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_all_fields() {
        let passport_str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"#;
        let passport = Passport::from_string(passport_str);
        assert_eq!(true, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_hgt() {
        let passport_str = r#"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"#;
        let passport = Passport::from_string(passport_str);
        assert_eq!(false, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_cid() {
        let passport_str = r#"hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"#;
        let passport = Passport::from_string(passport_str);
        assert_eq!(true, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_cid_byr() {
        let passport_str = r#"hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let passport = Passport::from_string(passport_str);
        assert_eq!(false, passport.has_fields(&valid_passport_fields()))
    }
}
