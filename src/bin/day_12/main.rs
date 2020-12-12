use advent_2020::ferry;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_12/input.txt").unwrap();
    let instructions = input.parse::<ferry::Instructions>().unwrap();
    let mut ferry = ferry::Ferry::new();
    ferry.mov(&instructions);
    println!("Part 1: {}", ferry.distance_from_origin());
}
