use advent_2020::dock;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_14/input.txt").unwrap();
    let v1 = input
        .trim()
        .lines()
        .map(|line| line.parse::<dock::Input>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut mem = dock::Memory::new();
    let _ = v1.iter().map(|i| mem.apply(i)).collect::<Vec<_>>();
    println!("Part 1: {}", mem.sum());

    let v2 = input
        .trim()
        .lines()
        .map(|line| line.parse::<dock::InputV2>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut mem = dock::Memory::new();
    let _ = v2.iter().map(|i| mem.apply_v2(i)).collect::<Vec<_>>();
    println!("Part 2: {}", mem.sum());
}
