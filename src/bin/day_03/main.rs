use advent_2020::toboggan;

fn main() {
    let map =
        toboggan::Map::from_string(&std::fs::read_to_string("src/bin/day_03/input.txt").unwrap());
    println!("Part 1: {}", toboggan::count_trees(&map, 3, 1));

    let trees_product =
        toboggan::count_trees_batch(&map, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
            .into_iter()
            .fold(1, |acc, n| acc * n);
    println!("Part 2: {}", trees_product);
}
