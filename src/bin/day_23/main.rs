use advent_2020::cups;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_23/input.txt").unwrap();
    let cup_list = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut cups = cups::Cups::new(&cup_list);
    cups.run(100);
    println!("Part 1: {}", cups.order_after(1));

    let cup_list = cup_list
        .iter()
        .cloned()
        .chain(cup_list.len() as u32 + 1..=1000000)
        .collect::<Vec<_>>();
    let mut cups = cups::Cups::new(&cup_list);
    cups.run(10000000);
    println!(
        "Part 2: {}",
        cups.first_n_after(1, 2).iter().product::<u64>()
    );
}
