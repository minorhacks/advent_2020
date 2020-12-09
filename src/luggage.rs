use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Color(String);

impl Color {
    pub fn new(s: &str) -> Color {
        Color(s.to_string())
    }
}

#[derive(Debug)]
pub struct RulesGraph(HashMap<Color, RulesGraphNode>);

#[derive(Debug)]
struct RulesGraphNode {
    color: Color,
    held_by: HashSet<Color>,
    holds: Vec<(usize, Color)>,
}

impl RulesGraph {
    pub fn bags_containing(&self, color: &Color) -> HashSet<Color> {
        let mut colors = HashSet::new();
        let mut node_queue = std::collections::VecDeque::new();
        let node = self.0.get(&color).unwrap();
        node_queue.push_back(node);
        while !node_queue.is_empty() {
            let node = node_queue.pop_front().unwrap();
            for c in node.held_by.iter() {
                if colors.insert(c.clone()) {
                    node_queue.push_back(self.0.get(&c).unwrap());
                }
            }
        }
        colors
    }

    pub fn bags_inside(&self, color: &Color) -> usize {
        self.0
            .get(color)
            .unwrap()
            .holds
            .iter()
            .fold(0, |acc, (quantity, color)| {
                acc + quantity + quantity * self.bags_inside(color)
            })
    }

    fn insert(&mut self, c: Color, contains: Vec<(usize, Color)>) {
        for (_quantity, color) in &contains {
            let node = self.0.entry(color.clone()).or_insert(RulesGraphNode {
                color: color.clone(),
                held_by: HashSet::new(),
                holds: Vec::new(),
            });
            node.held_by.insert(c.clone());
        }
        let node = self.0.entry(c.clone()).or_insert(RulesGraphNode {
            color: c,
            held_by: HashSet::new(),
            holds: Vec::new(),
        });
        node.holds = contains;
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

fn parse_bag_str(mut s: &str) -> Option<(usize, Color)> {
    match s.trim() {
        "no other bags" => None,
        _ => {
            let quantity = match s.trim().chars().nth(0).unwrap() {
                '0'..='9' => {
                    let (quantity_str, color_str) = s.trim().split_once(" ").unwrap();
                    s = color_str;
                    quantity_str.parse::<usize>().unwrap()
                }
                _ => 0,
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

fn parse_bag_rule(s: &str) -> (Color, Vec<(usize, Color)>) {
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

    static TEST_INPUT_2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

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
    fn test_bags_contained_count() {
        let graph = TEST_INPUT_2.parse::<RulesGraph>().unwrap();
        println!("{:?}", graph);
        assert_eq!(126, graph.bags_inside(&Color::new("shiny gold")));
    }

    #[test]
    fn test_parse_bag_str() {
        assert_eq!(
            Some((0, Color("light red".to_string()))),
            parse_bag_str("light red bags")
        );
        assert_eq!(
            Some((1, Color("bright white".to_string()))),
            parse_bag_str("1 bright white bag")
        );
        assert_eq!(
            Some((2, Color("muted yellow".to_string()))),
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
                    (1, Color("bright white".to_string())),
                    (2, Color("muted yellow".to_string()))
                ]
            ),
            parse_bag_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        )
    }
}
