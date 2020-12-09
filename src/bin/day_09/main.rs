use advent_2020::xmas;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_09/input.txt").unwrap();
    let seq = input
        .trim()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let first_missing_sum = xmas::first_missing_sum(&seq, 25).unwrap();
    println!("Part 1: {}", first_missing_sum);

    let range = xmas::find_sum_range(&seq, first_missing_sum).unwrap();
    println!(
        "Part 2: {}",
        range.iter().min().unwrap() + range.iter().max().unwrap()
    );
}
