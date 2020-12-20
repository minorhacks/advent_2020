use advent_2020::sea_monster;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_19/input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let (rules_section, input_section) = (sections.next().unwrap(), sections.next().unwrap());
    let mut rule_map = rules_section.parse::<sea_monster::RuleMap>().unwrap();

    let input_strs = input_section.trim().lines().collect::<Vec<_>>();
    let valid = input_strs.iter().filter(|&&s| rule_map.matches(s));
    println!("Part 1: {}", valid.count());

    rule_map.rewrite(8, "42 | 42 8").unwrap();
    rule_map.rewrite(11, "42 31 | 42 11 31").unwrap();
    let valid = input_strs.iter().filter(|&&s| rule_map.matches(s));
    println!("Part 2: {}", valid.count());
}
