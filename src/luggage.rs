use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Color(String);

impl Color {
    pub fn new(s: &str) -> Color {
        Color(s.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub struct BagQuantity(usize);

#[derive(Debug)]
pub struct RulesGraph(HashMap<Color, RulesGraphNode>);

#[derive(Debug)]
struct RulesGraphNode {
    color: Color,
    held_by: HashMap<Color, BagQuantity>,
}

impl RulesGraph {
    pub fn bags_containing(&self, color: &Color) -> HashSet<Color> {
        let mut colors = HashSet::new();
        let mut node_queue = std::collections::VecDeque::new();
        let node = self.0.get(&color).unwrap();
        node_queue.push_back(node);
        while !node_queue.is_empty() {
            let node = node_queue.pop_front().unwrap();
            for (c, q) in node.held_by.iter() {
                if colors.insert(c.clone()) {
                    node_queue.push_back(self.0.get(&c).unwrap());
                }
            }
        }
        colors
    }

    fn insert(&mut self, c: Color, contains: Vec<(BagQuantity, Color)>) {
        for (quantity, color) in contains {
            let node = self.0.entry(color.clone()).or_insert(RulesGraphNode {
                color: color.clone(),
                held_by: HashMap::new(),
            });
            node.held_by.insert(c.clone(), quantity);
        }
        let _ = self.0.entry(c.clone()).or_insert(RulesGraphNode {
            color: c,
            held_by: HashMap::new(),
        });
    }
}

impl std::str::FromStr for RulesGraph {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut rules_graph = RulesGraph(HashMap::new());
        let _ = s
            .trim()
            .lines()
            .map(|line| parse_bag_rule(line))
            .map(|pair| rules_graph.insert(pair.0, pair.1))
            .collect::<Vec<_>>();
        Ok(rules_graph)
    }
}

fn parse_bag_str(mut s: &str) -> Option<(BagQuantity, Color)> {
    match s.trim() {
        "no other bags" => None,
        _ => {
            let quantity = match s.trim().chars().nth(0).unwrap() {
                '0'..='9' => {
                    let (quantity_str, color_str) = s.trim().split_once(" ").unwrap();
                    s = color_str;
                    BagQuantity(quantity_str.parse::<usize>().unwrap())
                }
                _ => BagQuantity(0),
            };

            let color = Color(
                s.trim()
                    .strip_suffix("s")
                    .unwrap_or(s)
                    .strip_suffix("bag")
                    .unwrap()
                    .trim()
                    .to_string(),
            );
            Some((quantity, color))
        }
    }
}

fn parse_bag_rule(s: &str) -> (Color, Vec<(BagQuantity, Color)>) {
    let (container, contained) = s.split_once("contain").unwrap();
    let container_color = parse_bag_str(container).unwrap().1;
    let contained = contained
        .trim_end_matches('.')
        .split(",")
        .filter_map(|s| parse_bag_str(s))
        .collect::<Vec<_>>();
    (container_color, contained)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_graph_size() {
        let graph = TEST_INPUT.parse::<RulesGraph>().unwrap();
        assert_eq!(9, graph.0.len());
    }

    #[test]
    fn test_bags_can_contain_indirect() {
        let graph = TEST_INPUT.parse::<RulesGraph>().unwrap();
        let bags_containing_shiny_gold = graph.bags_containing(&Color("shiny gold".to_string()));
        assert_eq!(4, bags_containing_shiny_gold.len());
    }

    #[test]
    fn test_parse_bag_str() {
        assert_eq!(
            Some((BagQuantity(0), Color("light red".to_string()))),
            parse_bag_str("light red bags")
        );
        assert_eq!(
            Some((BagQuantity(1), Color("bright white".to_string()))),
            parse_bag_str("1 bright white bag")
        );
        assert_eq!(
            Some((BagQuantity(2), Color("muted yellow".to_string()))),
            parse_bag_str("2 muted yellow bags")
        );
        assert_eq!(None, parse_bag_str("no other bags"));
    }

    #[test]
    fn test_parse_bag_rule() {
        assert_eq!(
            (
                Color("light red".to_string()),
                vec![
                    (BagQuantity(1), Color("bright white".to_string())),
                    (BagQuantity(2), Color("muted yellow".to_string()))
                ]
            ),
            parse_bag_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        )
    }
}
