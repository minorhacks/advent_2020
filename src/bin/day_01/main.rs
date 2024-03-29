use advent_2020::expense;
fn main() {
    let expenses = std::fs::read_to_string("src/bin/day_01/input.txt")
        .expect("can't read input file")
        .trim()
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let report = expense::Report::new(&expenses);

    let nums = report.pair_with_sum(2020).unwrap();
    let product = nums.into_iter().product::<i32>();
    println!("Product for Part 1: {}", product);

    let nums = report.triple_with_sum(2020).unwrap();
    let product = nums.into_iter().product::<i32>();
    println!("Product for Part 2: {}", product);
}
