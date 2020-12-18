use advent_2020::password;

fn main() {
    let contents = std::fs::read_to_string("src/bin/day_02/input.txt").unwrap();
    let lines = contents.trim().lines().collect::<Vec<_>>();

    let acceptable_passwords_count = lines
        .iter()
        .map(|line| password::parse_password_policy_line(line).unwrap())
        .filter(|pair| password::meets_policy_frequency(&pair.0, &pair.1))
        .count();
    println!("Part 1: {}", acceptable_passwords_count);

    let acceptable_passwords_count = lines
        .iter()
        .map(|line| password::parse_password_policy_line(line).unwrap())
        .filter(|pair| password::meets_policy_position(&pair.0, &pair.1))
        .count();
    println!("Part 2: {}", acceptable_passwords_count);
}
