use advent_2020::airplane;

fn main() {
    let seat_strs = std::fs::read_to_string("src/bin/day_05/input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let all_ids = seat_strs
        .iter()
        .map(|s| s.parse::<airplane::Seat>().unwrap())
        .map(|seat| seat.id())
        .collect::<Vec<_>>();
    let max_seat_id = all_ids.iter().max().unwrap();
    println!("Part 1: {}", max_seat_id);

    let first_missing_id = (0..*max_seat_id)
        .rev()
        .find_map(|id| {
            if !all_ids.contains(&id) {
                Some(id)
            } else {
                None
            }
        })
        .unwrap();
    println!("Part 2: {}", first_missing_id);
}
