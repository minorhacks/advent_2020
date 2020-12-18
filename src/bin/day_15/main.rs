use advent_2020::memory;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_15/input.txt").unwrap();
    let nums: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>())
        .collect::<Result<_, _>>()
        .unwrap();
    let mut game = memory::Game::new(nums.clone());
    println!("Part 1: {}", game.play_until(2020));

    let mut game = memory::Game::new(nums);
    println!("Part 2: {}", game.play_until(30000000));
}
