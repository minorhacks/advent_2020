use advent_2020::customs;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_06/input.txt").unwrap();
    let response_groups = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<customs::FormResponses>().unwrap())
        .collect::<Vec<_>>();

    let total_unique = response_groups
        .iter()
        .map(|r| r.num_unique_questions())
        .sum::<usize>();
    println!("Part 1: {}", total_unique);

    let total_common = response_groups
        .iter()
        .map(|r| r.num_common_questions())
        .sum::<usize>();
    println!("Part 2: {}", total_common);
}
