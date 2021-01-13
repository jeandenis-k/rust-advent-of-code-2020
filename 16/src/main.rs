use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;

static INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq)]
struct Input {
    rules: Vec<Rule>,
    ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

#[derive(Debug, PartialEq)]
struct Rule {
    field: String,
    range1: RangeInclusive<i32>,
    range2: RangeInclusive<i32>,
}

fn main() {
    let input = Input::parse(INPUT);
    println!("{}", input.solve_part1());
    println!("{}", input.solve_part2());
}

impl Input {
    fn solve_part1(&self) -> i32 {
        self.nearby_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|n| self.rules.iter().all(|r| !r.validate(**n)))
            })
            .sum()
    }

    fn solve_part2(&self) -> i64 {
        let field_indices = self.solve_field_indices();
        field_indices
            .iter()
            .filter_map(|(field, index)| {
                if field.starts_with("departure") {
                    let value: i64 = i64::from(self.ticket[*index as usize]);
                    Some(value)
                } else {
                    None
                }
            })
            .fold(1_i64, std::ops::Mul::mul)
    }

    fn solve_field_indices(&self) -> Vec<(&str, i32)> {
        // comment ?
        // looper sur tableau en marquant les items qui n'ont qu'une possibilité
        let mut possibilities = self.find_possible_field_for_each_index();
        let mut found: HashSet<i32> = HashSet::new();
        while possibilities.iter().any(|(_, indices)| indices.len() > 1) {
            let first_found = possibilities.iter().find_map(|(field, indices)| {
                if indices.len() == 1 && !found.contains(indices.iter().next().unwrap()) {
                    Some((field.to_string(), indices))
                } else {
                    None
                }
            });
            if let Some((found_field, indices)) = first_found {
                let found_index = indices.iter().cloned().next().unwrap();
                found.insert(found_index);
                for (field, candidates) in possibilities.iter_mut() {
                    if field != &found_field {
                        candidates.remove(&found_index);
                    }
                }
            }
        }
        possibilities
            .iter()
            .map(|(field, indices)| (*field, *indices.iter().next().unwrap()))
            .collect()
    }

    fn find_possible_field_for_each_index(self: &Input) -> Vec<(&'_ str, HashSet<i32>)> {
        let valid_tickets: Vec<&Vec<i32>> = self.valid_tickets().collect();
        self.rules
            .iter()
            .map(|rule| {
                let possible_indices = (0_i32..(self.rules.len() as i32))
                    .into_iter()
                    .filter(|&possible_index| {
                        valid_tickets
                            .iter()
                            .by_ref()
                            .all(|&t| rule.validate(t[possible_index as usize]))
                    })
                    .collect();
                (&rule.field as &str, possible_indices)
            })
            .collect()
    }

    fn valid_tickets(self: &Input) -> impl Iterator<Item = &'_ Vec<i32>> {
        self.nearby_tickets.iter().filter(move |&ticket| {
            !ticket
                .iter()
                .any(|&n| self.rules.iter().all(|r| !r.validate(n)))
        })
    }

    fn parse(s: &str) -> Input {
        let mut lines = s.lines();
        let rules = lines
            .by_ref()
            .take_while(|l| *l != "your ticket:")
            .filter_map(|l| {
                if !l.is_empty() {
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
    fn validate(&self, n: i32) -> bool {
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
    fn test_valid_tickets() {
        assert_eq!(
            Input::parse(INPUT).valid_tickets().collect::<Vec<_>>(),
            vec![&vec![7_i32, 3, 47]]
        )
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

#[cfg(test)]
mod tests_part2 {
    use super::*;

    static INPUT: &str = include_str!("input_example2");

    #[test]
    fn test_solve_field_indices() {
        assert_eq!(
            Input::parse(INPUT).solve_field_indices(),
            vec![("class", 1), ("row", 0), ("seat", 2)]
        )
    }

    #[test]
    fn test_find_possible_field_indices() {
        assert_eq!(
            Input::parse(INPUT).find_possible_field_for_each_index(),
            vec![
                ("class", [1, 2].iter().cloned().collect()),
                ("row", [0, 1, 2].iter().cloned().collect()),
                ("seat", [2].iter().cloned().collect())
            ]
        )
    }
}
