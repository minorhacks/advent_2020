use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to parse rule number")]
    RuleNumParseError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("failed to parse terminal rule: {0}")]
    TerminalParseError(String),

    #[error("attempted to parse empty rule")]
    EmptyRuleError,

    #[error("failed to parse fields in rule: {0}")]
    RuleParseFieldsError(String),
}

type Result<T> = std::result::Result<T, Error>;

type RuleNum = usize;

#[derive(Debug)]
pub struct RuleMap(HashMap<RuleNum, Rule>);

#[derive(Debug)]
struct AlternativeList(Vec<RuleNum>);

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Nonterminal(Vec<AlternativeList>),
}

impl std::str::FromStr for AlternativeList {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(AlternativeList(
            s.trim()
                .split(' ')
                .map(|s| s.parse::<RuleNum>())
                .collect::<std::result::Result<_, _>>()?,
        ))
    }
}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();
        match s.chars().next() {
            Some('"') => Ok(Rule::Terminal(
                s.chars()
                    .nth(1)
                    .ok_or_else(|| Error::TerminalParseError(s.to_string()))?,
            )),
            Some(_) => {
                let alternatives = s
                    .split(" | ")
                    .map(|alt_str| alt_str.parse::<AlternativeList>())
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                Ok(Rule::Nonterminal(alternatives))
            }
            None => Err(Error::EmptyRuleError),
        }
    }
}

impl std::str::FromStr for RuleMap {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(RuleMap(
            s.trim()
                .lines()
                .map(|line| {
                    let fields = line.split(": ").collect::<Vec<_>>();
                    if fields.len() != 2 {
                        return Err(Error::RuleParseFieldsError(line.to_string()));
                    }
                    let rule_num = fields.get(0).unwrap().parse::<RuleNum>()?;
                    let rule = fields.get(1).unwrap().parse::<Rule>()?;
                    Ok((rule_num, rule))
                })
                .collect::<std::result::Result<HashMap<_, _>, _>>()?,
        ))
    }
}

impl RuleMap {
    pub fn rewrite(&mut self, rule_num: RuleNum, rule_str: &str) -> Result<()> {
        let rule = rule_str.parse::<Rule>()?;
        self.0.insert(rule_num, rule);
        Ok(())
    }

    pub fn matches(&self, s: &str) -> bool {
        // Keep a list of AlternativeLists, as well as an iter to the string it needs to match
        let mut alternatives: Vec<(AlternativeList, std::str::Chars)> = Vec::new();
        alternatives.push((AlternativeList(vec![0]), s.chars()));
        // While list of AlternativeLists is not empty
        while !alternatives.is_empty() {
            // Pop an AlternativeList and string iter
            let (alternative, mut chars) = alternatives.pop().unwrap();
            // If rule list is empty
            if alternative.0.is_empty() {
                // Success iff no more chars as well
                return chars.next().is_none();
            }
            // For each rule in alternative list
            let rule_num = alternative.0.get(0).unwrap();
            let rule = self.0.get(rule_num).unwrap();
            match rule {
                // If terminal:
                Rule::Terminal(c) => {
                    match chars.next() {
                        // If matches next char in string
                        Some(t) if t == *c => {
                            // New AlternativeList with first elem missing
                            let new_list = AlternativeList(alternative.0[1..].to_vec());
                            // Push new list, iter
                            alternatives.push((new_list, chars.clone()));
                        }
                        // If not matches
                        _ => {
                            // Do nothing to loop back around to next AlternativeList
                        }
                    }
                }
                // If nonterminal:
                Rule::Nonterminal(nt) => {
                    // For each alternative in this rule:
                    for at in nt.iter().rev() {
                        // New AlternativeList = rule_alternatives + current
                        let new_list = AlternativeList(
                            at.0.iter()
                                .cloned()
                                .chain(alternative.0.iter().skip(1).cloned())
                                .collect(),
                        );
                        // Push new list, iter
                        alternatives.push((new_list, chars.clone()));
                    }
                }
            }
        }
        // No more viable alternatives; failure
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RULES: &str = &r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;

    static TEST_RULES_2: &str = &r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;

    #[test]
    fn test_parse_rule_map() {
        let rule_map = TEST_RULES.parse::<RuleMap>();
        assert_eq!(true, rule_map.is_ok());
    }

    #[test]
    fn test_rule_map_matches() {
        let rule_map = TEST_RULES.parse::<RuleMap>().unwrap();
        assert_eq!(true, rule_map.matches("ababbb"));
        assert_eq!(false, rule_map.matches("bababa"));
        assert_eq!(true, rule_map.matches("abbbab"));
        assert_eq!(false, rule_map.matches("aaabbb"));
        assert_eq!(false, rule_map.matches("aaaabbb"));
    }

    #[test]
    fn test_rule_map_matches_looping() {
        let test_strs = vec![
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        let mut rule_map = TEST_RULES_2.parse::<RuleMap>().unwrap();
        assert_eq!(
            3,
            test_strs.iter().filter(|&&s| rule_map.matches(s)).count()
        );
        rule_map.rewrite(8, "42 | 42 8").unwrap();
        rule_map.rewrite(11, "42 31 | 42 11 31").unwrap();
        assert_eq!(
            12,
            test_strs.iter().filter(|&&s| rule_map.matches(s)).count()
        );
    }
}
