use std::collections::HashMap;
use std::collections::HashSet;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Eq, PartialEq)]
pub enum Error {
    #[error("color {0} not found")]
    ColorNotFoundError(String),

    #[error("color string is empty")]
    EmptyColorError,
}

type Result<T> = std::result::Result<T, Error>;

fn split_once<'a>(s: &'a str, delimiter: &'a str) -> Option<(&'a str, &'a str)> {
    let pieces = s.splitn(2, delimiter).collect::<Vec<_>>();
    match pieces.len() {
        2 => Some((pieces[0], pieces[1])),
        _ => None,
    }
}
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
    pub fn bags_containing(&self, color: &Color) -> Result<HashSet<Color>> {
        let mut colors = HashSet::new();
        let mut node_queue = std::collections::VecDeque::new();
        let node = self
            .0
            .get(&color)
            .ok_or_else(|| Error::ColorNotFoundError(color.0.clone()))?;
        node_queue.push_back(node);
        while !node_queue.is_empty() {
            let node = node_queue.pop_front().unwrap();
            for c in node.held_by.iter() {
                if colors.insert(c.clone()) {
                    node_queue.push_back(self.0.get(&c).unwrap());
                }
            }
        }
        Ok(colors)
    }

    pub fn bags_inside(&self, color: &Color) -> Result<usize> {
        let num = self.0.get(color).unwrap().holds.iter().fold(
            Ok(0),
            |acc: Result<usize>, (quantity, color)| {
                Ok(acc? + quantity + quantity * self.bags_inside(color)?)
            },
        );
        num
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
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut rules_graph = RulesGraph(HashMap::new());
        let _ = s
            .trim()
            .lines()
            .map(|line| parse_bag_rule(line))
            .map(|pair| {
                let pair = pair?;
                rules_graph.insert(pair.0, pair.1);
                Ok(())
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(rules_graph)
    }
}

fn parse_bag_str(mut s: &str) -> Result<Option<(usize, Color)>> {
    match s.trim() {
        "no other bags" => Ok(None),
        _ => {
            let quantity = match s.trim().chars().next().ok_or(Error::EmptyColorError)? {
                '0'..='9' => {
                    let (quantity_str, color_str) = split_once(s.trim(), " ").unwrap();
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
            Ok(Some((quantity, color)))
        }
    }
}

fn parse_bag_rule(s: &str) -> Result<(Color, Vec<(usize, Color)>)> {
    let (container, contained) = split_once(s, "contain").unwrap();
    let container_color = parse_bag_str(container)?.unwrap().1;
    let contained = contained
        .trim_end_matches('.')
        .split(',')
        .map(|s| parse_bag_str(s))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter_map(|i| i)
        .collect::<Vec<_>>();
    Ok((container_color, contained))
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
        let bags_containing_shiny_gold = graph
            .bags_containing(&Color("shiny gold".to_string()))
            .unwrap();
        assert_eq!(4, bags_containing_shiny_gold.len());
    }

    #[test]
    fn test_bags_contained_count() {
        let graph = TEST_INPUT_2.parse::<RulesGraph>().unwrap();
        println!("{:?}", graph);
        assert_eq!(126, graph.bags_inside(&Color::new("shiny gold")).unwrap());
    }

    #[test]
    fn test_parse_bag_str() {
        assert_eq!(
            Some((0, Color("light red".to_string()))),
            parse_bag_str("light red bags").unwrap()
        );
        assert_eq!(
            Some((1, Color("bright white".to_string()))),
            parse_bag_str("1 bright white bag").unwrap()
        );
        assert_eq!(
            Some((2, Color("muted yellow".to_string()))),
            parse_bag_str("2 muted yellow bags").unwrap()
        );
        assert_eq!(None, parse_bag_str("no other bags").unwrap());
    }

    #[test]
    fn test_parse_bag_rule() {
        assert_eq!(
            Ok((
                Color("light red".to_string()),
                vec![
                    (1, Color("bright white".to_string())),
                    (2, Color("muted yellow".to_string()))
                ]
            )),
            parse_bag_rule("light red bags contain 1 bright white bag, 2 muted yellow bags."),
        )
    }
}
