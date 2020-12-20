use advent_2020::jigsaw;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_20/input.txt").unwrap();
    let tiles = input.trim().parse::<jigsaw::Tiles>().unwrap();
    println!("Part 1: {}", tiles.find_corners().iter().product::<usize>());
}
