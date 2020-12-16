use advent_2020::train;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_16/input.txt").unwrap();
    let mut sections = input.trim().split("\n\n");
    let rules_section = sections.next().unwrap();
    let _my_ticket_section = sections.next().unwrap();
    let nearby_tickets_section = sections.next().unwrap();

    let rules = rules_section
        .lines()
        .map(|line| line.parse::<train::Rule>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let nearby_tickets = nearby_tickets_section
        .trim()
        .lines()
        .skip(1)
        .map(|line| line.parse::<train::Ticket>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let error_rate = nearby_tickets
        .iter()
        .map(|t| t.error_rate(&rules))
        .sum::<i32>();
    println!("Part 1: {}", error_rate);
}
