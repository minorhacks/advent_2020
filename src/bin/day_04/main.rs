use advent_2020::credentials;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_04/input.txt").unwrap();

    let passports = input
        .split("\n\n")
        .map(|s| credentials::Passport::from_string(s))
        .collect::<Vec<_>>();
    println!("Num passports: {}", passports.len());

    let valid_passports: usize = input
        .trim()
        .split("\n\n")
        .map(|s| credentials::Passport::from_string(s))
        .map(
            |p| match p.has_fields(&credentials::valid_passport_fields()) {
                true => 1,
                false => 0,
            },
        )
        .sum();
    println!("Part 1: {}", valid_passports);
}
