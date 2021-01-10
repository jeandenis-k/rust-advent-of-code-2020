use regex::Regex;
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Input {
    rules: Vec<Rule>,
    ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    field: String,
    range1: RangeInclusive<i32>,
    range2: RangeInclusive<i32>,
}

fn main() {
    let input = Input::parse(INPUT);
    println!("{}", input.solve_part1());
}

impl Input {
    fn solve_part1(self) -> i32 {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .inspect(|n| {
                        dbg!(n);
                        dbg!(self.rules.iter().all(|r| !r.clone().validate(**n)));
                    })
                    .filter(|n| self.rules.iter().all(|r| !r.clone().validate(**n)))
            })
            .sum()
    }
    fn parse(s: &str) -> Input {
        let mut lines = s.lines();
        let rules = lines
            .by_ref()
            .take_while(|l| *l != "your ticket:")
            .filter_map(|l| {
                if l != "" {
                    let re = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
                    let matches = re.captures(&l).unwrap();
                    Some(Rule {
                        field: matches.get(1).unwrap().as_str().to_string(),
                        range1: RangeInclusive::new(
                            matches.get(2).unwrap().as_str().parse().unwrap(),
                            matches.get(3).unwrap().as_str().parse().unwrap(),
                        ),
                        range2: RangeInclusive::new(
                            matches.get(4).unwrap().as_str().parse().unwrap(),
                            matches.get(5).unwrap().as_str().parse().unwrap(),
                        ),
                    })
                } else {
                    None
                }
            })
            .collect();
        let ticket = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        lines.next();
        lines.next();
        let nearby_tickets = lines
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();
        Input {
            rules,
            ticket,
            nearby_tickets,
        }
    }
}

impl Rule {
    fn validate(self, n: i32) -> bool {
        self.range1.contains(&n) || self.range2.contains(&n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input_example");

    #[test]
    fn test_solve_part1() {
        assert_eq!(Input::parse(INPUT).solve_part1(), 71)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Input::parse(INPUT),
            Input {
                rules: vec![
                    Rule {
                        field: "class".to_string(),
                        range1: 1..=3,
                        range2: 5..=7
                    },
                    Rule {
                        field: "row".to_string(),
                        range1: 6..=11,
                        range2: 33..=44
                    },
                    Rule {
                        field: "seat".to_string(),
                        range1: 13..=40,
                        range2: 45..=50
                    }
                ],
                ticket: vec![7, 1, 14],
                nearby_tickets: vec![
                    vec![7, 3, 47],
                    vec![40, 4, 50],
                    vec![55, 2, 20],
                    vec![38, 6, 12]
                ]
            }
        )
    }
}
