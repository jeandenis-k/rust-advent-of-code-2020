use std::convert::TryFrom;
use std::str::FromStr;
use BusEntry::*;

#[derive(Debug, PartialEq)]
struct Notes {
    earliest: i32,
    bus_ids: Vec<BusEntry>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BusEntry {
    Id(i64),
    X,
}

static INPUT: &str = include_str!("../input");

fn main() {
    println!("{:?}", parse(INPUT).unwrap().solve_part1());
    println!("{:?}", parse(INPUT).unwrap().solve_part2());
}

fn parse(input: &str) -> Option<Notes> {
    let mut lines = input.lines();
    let timestamp: i32 = lines.next()?.parse::<i32>().ok()?;
    let bus_ids = lines
        .next()?
        .split(",")
        .map(|s| s.parse())
        .filter_map(Result::ok)
        .collect();
    Some(Notes {
        earliest: timestamp,
        bus_ids,
    })
}

impl Notes {
    fn solve_part1(self: &Notes) -> Option<i32> {
        (self.earliest..).find_map(|time| {
            self.numbers().find_map(|(_, bus_id)| {
                if time % (bus_id as i32) == 0 {
                    Some((time - self.earliest) * (bus_id as i32))
                } else {
                    None
                }
            })
        })
    }

    fn solve_part2(self: &Notes) -> i64 {
        // For each bus id n, find a number m that satisfies the two following conditions :
        // - it is a multiple of the product of all other numbers
        // - m % n == - index of bus id
        let product = self.numbers().map(|(_, n)| n).fold(1, std::ops::Mul::mul);
        let sum = self
            .numbers()
            .map(|(index, n)| {
                let product_of_others = product / n;
                (1_i64..)
                    .find_map(|m| {
                        let chinese = m * product_of_others;
                        if index == 0 && chinese % n == 0 {
                            assert!(chinese % n == 0);
                            Some(chinese)
                        } else if index != 0 && chinese % n == 1 {
                            let index = i64::try_from(index).unwrap();
                            let reminder = (-index % n) + n;
                            let chinese = chinese * reminder;
                            assert!(chinese % n == reminder);
                            Some(chinese)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .sum::<i64>();
        sum % product
    }

    fn numbers(self: &Notes) -> impl Iterator<Item = (usize, i64)> + '_ {
        self.bus_ids
            .iter()
            .enumerate()
            .filter_map(|(index, entry)| match entry {
                Id(id) => Some((index, *id)),
                X => None,
            })
    }
}

impl FromStr for BusEntry {
    type Err = ();
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "x" => Ok(BusEntry::X),
            _ => Ok(BusEntry::Id(str.parse().map_err(|_| ())?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT_EXAMPLE),
            Some(Notes {
                earliest: 939,
                bus_ids: vec![Id(7), Id(13), X, X, Id(59), X, Id(31), Id(19)]
            })
        )
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(parse(INPUT_EXAMPLE).unwrap().solve_part1(), Some(295))
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(parse("12\n17,x,13,19").unwrap().solve_part2(), 3417)
    }
}
