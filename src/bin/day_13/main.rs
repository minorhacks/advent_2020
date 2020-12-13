use advent_2020::shuttle;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_13/input.txt").unwrap();
    let input = input.trim();
    let now = input.lines().nth(0).unwrap().parse::<u64>().unwrap();
    let shuttles = input
        .lines()
        .nth(1)
        .unwrap()
        .parse::<shuttle::ShuttleList>()
        .unwrap();
    let next_id = shuttles.next_shuttle(now);
    println!(
        "Part 1: {}",
        next_id * shuttle::minutes_to_wait(now, next_id)
    );

    println!("Part 2: {}", shuttles.leaving_consecutively());
}
