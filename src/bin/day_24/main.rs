use advent_2020::tile;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_24/input.txt").unwrap();
    let coords = input
        .trim()
        .lines()
        .map(|l| l.parse::<tile::Coord>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut map = tile::Map::new();
    map.flip_all(&coords);
    println!("Part 1: {}", map.color_count(&tile::Color::Black));

    map.flip_days(100);
    println!("Part 2: {}", map.color_count(&tile::Color::Black));
}
