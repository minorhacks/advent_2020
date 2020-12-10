use advent_2020::charger;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_10/input.txt").unwrap();
    let charger_list = input
        .trim()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let distribution = charger::difference_distribution(&charger_list);
    println!("Part 1: {}", distribution[&1] * distribution[&3]);

    //println!("1's: {}\t3's: {}", distribution[&1], distribution[&3]);

    println!(
        "Part 2: {}",
        charger::valid_combinations_count(&charger_list)
    );
}
