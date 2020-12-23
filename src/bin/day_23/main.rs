use advent_2020::cups;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_23/input.txt").unwrap();
    let mut cups = input.trim().parse::<cups::Cups>().unwrap();
    cups.run(100);
    println!("Part 1: {}", cups.order_after(1));
}
