use advent_2020::newmath;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_18/input.txt").unwrap();
    let expr_list = input
        .trim()
        .lines()
        .map(|line| newmath::Expr::from_str(line))
        .collect::<Vec<_>>();

    let expr_sum: u64 = expr_list.iter().map(|e| e.result()).sum();
    println!("Part 1: {}", expr_sum);
}
