use advent_2020::dock;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_14/input.txt").unwrap();
    let input = input
        .trim()
        .lines()
        .map(|line| line.parse::<dock::Input>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut mem = dock::Memory::new();
    let _ = input.iter().map(|i| mem.apply(i)).collect::<Vec<_>>();
    println!("Part 1: {}", mem.sum());
}
