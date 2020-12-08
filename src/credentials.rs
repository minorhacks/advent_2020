use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::Error;
use std::io::ErrorKind;

struct BirthYear(i32);
struct IssueYear(i32);
struct ExpirationYear(i32);
struct Height(Length);
struct HairColor(String);
struct EyeColor(String);
struct PassportId(String);
struct CountryId(String);

enum Length {
    Centimeters(i32),
    Inches(i32),
}

pub struct UnvalidatedPassport {
    fields: HashMap<String, String>,
}

#[allow(dead_code)]
pub struct Passport {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportId,
    cid: Option<CountryId>,
}

impl std::str::FromStr for BirthYear {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let byr = s.parse::<i32>()?;
        if byr >= 1920 && byr <= 2002 {
            Ok(BirthYear(byr))
        } else {
            Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "byr out of range",
            )))
        }
    }
}

impl std::str::FromStr for IssueYear {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let iyr = s.parse::<i32>()?;
        if iyr >= 2010 && iyr <= 2020 {
            Ok(IssueYear(iyr))
        } else {
            Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "iyr out of range",
            )))
        }
    }
}

impl std::str::FromStr for ExpirationYear {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let eyr = s.parse::<i32>()?;
        if eyr >= 2020 && eyr <= 2030 {
            Ok(ExpirationYear(eyr))
        } else {
            Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "eyr out of range",
            )))
        }
    }
}

impl std::str::FromStr for Height {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match Regex::new(r"^(\d+)in$").unwrap().captures(s) {
            Some(c) => {
                let val = c.get(1).unwrap().as_str().parse::<i32>()?;
                if val >= 59 && val <= 76 {
                    Ok(Height(Length::Inches(val)))
                } else {
                    Err(Box::new(Error::new(
                        ErrorKind::InvalidInput,
                        "hgt out of range",
                    )))
                }
            }
            None => match Regex::new(r"^(\d+)cm$").unwrap().captures(s) {
                Some(c) => {
                    let val = c.get(1).unwrap().as_str().parse::<i32>()?;
                    if val >= 150 && val <= 193 {
                        Ok(Height(Length::Centimeters(val)))
                    } else {
                        Err(Box::new(Error::new(
                            ErrorKind::InvalidInput,
                            "hgt out of range",
                        )))
                    }
                }
                None => Err(Box::new(Error::new(ErrorKind::InvalidInput, "invalid hgt"))),
            },
        }
    }
}

impl std::str::FromStr for HairColor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(s) {
            true => Ok(HairColor(s.to_string())),
            false => Err(Box::new(Error::new(ErrorKind::InvalidInput, "hcl invalid"))),
        }
    }
}

impl std::str::FromStr for EyeColor {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match valid_eye_colors.contains(&s) {
            true => Ok(EyeColor(s.to_string())),
            false => Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "unrecognized ecl",
            ))),
        }
    }
}

impl std::str::FromStr for PassportId {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match Regex::new(r"^\d{9}$").unwrap().is_match(s) {
            true => Ok(PassportId(s.to_string())),
            false => Err(Box::new(Error::new(ErrorKind::InvalidInput, "pid invalid"))),
        }
    }
}

impl std::str::FromStr for UnvalidatedPassport {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let fields = s
            .trim()
            .split_whitespace()
            .map(|s| {
                let field_parts = s.splitn(2, ":").collect::<Vec<_>>();
                (field_parts[0].to_string(), field_parts[1].to_string())
            })
            .collect::<HashMap<_, _>>();
        Ok(UnvalidatedPassport { fields })
    }
}

impl UnvalidatedPassport {
    pub fn has_fields(&self, field_names: &HashSet<String>) -> bool {
        let keys = self.fields.keys().cloned().collect::<HashSet<_>>();
        keys.is_superset(field_names)
    }
}

impl std::fmt::Display for UnvalidatedPassport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pairs = self.fields.iter().collect::<Vec<_>>();
        pairs.sort_by(|a, b| a.0.cmp(b.0));
        let pair_str = pairs
            .iter()
            .map(|pair| format!("{}:{}", pair.0, pair.1).to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", pair_str)
    }
}

impl TryFrom<UnvalidatedPassport> for Passport {
    type Error = Box<dyn std::error::Error>;

    fn try_from(val: UnvalidatedPassport) -> std::result::Result<Self, Self::Error> {
        let passport = Passport {
            byr: val.fields.get("byr").ok_or("no byr field")?.parse()?,
            iyr: val.fields.get("iyr").ok_or("no iyr field")?.parse()?,
            eyr: val.fields.get("eyr").ok_or("no eyr field")?.parse()?,
            hgt: val.fields.get("hgt").ok_or("no hgt field")?.parse()?,
            hcl: val.fields.get("hcl").ok_or("no eyr field")?.parse()?,
            ecl: val.fields.get("ecl").ok_or("no ecl field")?.parse()?,
            pid: val.fields.get("pid").ok_or("no pid field")?.parse()?,
            cid: val.fields.get("cid").map(|cid| CountryId(cid.to_string())),
        };
        Ok(passport)
    }
}

impl std::str::FromStr for Passport {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let passport: UnvalidatedPassport = s.parse()?;
        Ok(Passport::try_from(passport)?)
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
        let passport: UnvalidatedPassport = passport_str.parse().unwrap();
        assert_eq!(true, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_hgt() {
        let passport_str = r#"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"#;
        let passport: UnvalidatedPassport = passport_str.parse().unwrap();
        assert_eq!(false, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_cid() {
        let passport_str = r#"hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"#;
        let passport: UnvalidatedPassport = passport_str.parse().unwrap();
        assert_eq!(true, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn passport_missing_cid_byr() {
        let passport_str = r#"hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let passport: UnvalidatedPassport = passport_str.parse().unwrap();
        assert_eq!(false, passport.has_fields(&valid_passport_fields()))
    }

    #[test]
    fn invalid_passports() {
        let passports = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:2003 eyr:2021 pid:093154719

iyr:2010 hgt:190in hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:190 hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#123abz ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:123abc ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652a ecl:wat byr:1944 eyr:2021 pid:0123456789
";
        let (valid, invalid): (Vec<_>, Vec<_>) = passports
            .split("\n\n")
            .map(|s| s.parse::<Passport>())
            .partition(Result::is_ok);
        assert_eq!(0, valid.len());
        assert_eq!(11, invalid.len());
    }

    #[test]
    fn valid_passports() {
        let passports = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let (valid, invalid): (Vec<_>, Vec<_>) = passports
            .split("\n\n")
            .map(|s| s.parse::<Passport>())
            .partition(Result::is_ok);
        assert_eq!(4, valid.len());
        assert_eq!(0, invalid.len());
    }
}
