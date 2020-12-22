use advent_2020::cards;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_22/input.txt").unwrap();
    let mut game = input.parse::<cards::Combat>().unwrap();
    println!("Part 1: {}", game.play().1);

    let mut game = input.parse::<cards::Combat>().unwrap();
    println!("Part 2: {}", game.play_recursive().1);
}
