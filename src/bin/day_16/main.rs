use advent_2020::train;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/bin/day_16/input.txt").unwrap();
    let mut sections = input.trim().split("\n\n");
    let rules_section = sections.next().unwrap();
    let my_ticket_section = sections.next().unwrap();
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
        .sum::<i64>();
    println!("Part 1: {}", error_rate);

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|t| t.is_valid(&rules))
        .collect::<Vec<_>>();

    let my_ticket = my_ticket_section
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .parse::<train::Ticket>()
        .unwrap();

    let possible_field_map = rules
        .iter()
        .map(|r| (r.field.clone(), r.possible_fields(&valid_tickets)))
        .collect::<HashMap<_, _>>();

    let field_map = train::resolve_field_map(possible_field_map);

    let departure_product = field_map
        .into_iter()
        .filter_map(|(k, v)| match k.starts_with("departure") {
            true => Some(my_ticket.get(v)),
            false => None,
        })
        .map(|v| {
            println!("{}", v);
            v
        })
        .product::<i64>();

    println!("Part 2: {}", departure_product);
}
