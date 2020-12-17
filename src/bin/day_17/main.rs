use advent_2020::cube;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_17/input.txt").unwrap();
    let mut space = cube::Space::from_initial_slice(&input).unwrap();
    for _ in 0..6 {
        space = space.step();
    }
    println!("Part 1: {}", space.active_count());
}
