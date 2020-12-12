use advent_2020::ferry;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_11/input.txt").unwrap();
    let waiting_area = input.parse::<ferry::WaitingArea>().unwrap();
    println!(
        "Part 1: {}",
        waiting_area.stabilize_adjacent().num_occupied()
    );

    println!(
        "Part 2: {}",
        waiting_area.stabilize_first_visible().num_occupied()
    )
}
