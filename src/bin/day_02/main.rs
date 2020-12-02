use advent_2020::password;

fn main() {
    let acceptable_passwords_count = std::fs::read_to_string("src/bin/day_02/input.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| password::parse_password_policy_line(line).unwrap())
        .filter(|pair| password::password_meets_policy(&pair.0, &pair.1))
        .count();
    println!("Part 1: {}", acceptable_passwords_count);
}
