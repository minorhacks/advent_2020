use advent_2020::luggage;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_07/input.txt").unwrap();
    let graph = input.parse::<luggage::RulesGraph>().unwrap();
    let bags_containing_shiny_gold = graph.bags_containing(&luggage::Color::new("shiny gold"));
    println!("Part 1: {}", bags_containing_shiny_gold.len());

    let num_bags_in_shiny_gold = graph.bags_inside(&luggage::Color::new("shiny gold"));
    println!("Part 2: {}", num_bags_in_shiny_gold);
}
