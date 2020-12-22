use advent_2020::food;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_21/input.txt").unwrap();
    let menu = input.parse::<food::Menu>().unwrap();
    println!(
        "Part 1: {}",
        menu.count_ingredients_usage(menu.deduce_hypoallergens())
    );

    println!("Part 2: {}", menu.canonical_dangerous_ingredients());
}
