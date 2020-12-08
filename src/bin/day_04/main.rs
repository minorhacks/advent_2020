use advent_2020::credentials;
use std::convert::TryFrom;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_04/input.txt").unwrap();

    let passports = input
        .split("\n\n")
        .map(|s| s.parse::<credentials::UnvalidatedPassport>().unwrap())
        .collect::<Vec<_>>();
    println!("Num passports: {}", passports.len());

    let have_correct_fields = passports
        .iter()
        .filter(|p| p.has_fields(&credentials::valid_passport_fields()))
        .collect::<Vec<_>>();
    println!("Part 1: {}", have_correct_fields.len());

    // Dump part 1 passports to file for debugging
    let output = have_correct_fields
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write("src/bin/day_04/output.txt", output).unwrap();

    let valid_passports = passports
        .into_iter()
        .map(|unvalidated| credentials::Passport::try_from(unvalidated))
        .filter(|result| result.is_ok())
        .count();
    println!("Part 2: {}", valid_passports);
}
