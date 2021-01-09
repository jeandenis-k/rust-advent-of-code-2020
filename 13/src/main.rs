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
    Id(i32),
    X,
}
fn main() {
    println!("{:?}", parse(INPUT).unwrap().solve_part1());
    println!("{:?}", parse(INPUT).unwrap().solve_part2());
}

static INPUT: &str = include_str!("../input");

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
        let Notes { earliest, bus_ids } = self;
        (*earliest..).find_map(|time| {
            bus_ids.iter().find_map(|bus_entry| match bus_entry {
                BusEntry::Id(bus_id) => {
                    if time % bus_id == 0 {
                        Some((time - earliest) * *bus_id)
                    } else {
                        None
                    }
                }
                BusEntry::X => None,
            })
        })
    }
    fn solve_part2(self: &Notes) -> Option<i64> {
        let first = match self.bus_ids[0] {
            Id(id) => id,
            X => 1,
        };
        (100000000000000_i64..).find_map(|n| {
            dbg!(n);
            if self
                .bus_ids
                .iter()
                .enumerate()
                .all(|(i, entry)| match entry {
                    BusEntry::Id(bus_id) => (n + i64::try_from(i).unwrap()) % (*bus_id as i64) == 0,
                    BusEntry::X => true,
                })
            {
                Some(n)
            } else {
                None
            }
        })
    }

    fn max(self: &Notes) -> i32 {
        self.bus_ids
            .iter()
            .map(|entry| match entry {
                X => 0,
                Id(id) => *id,
            })
            .max()
            .unwrap()
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
        assert_eq!(parse("12\n17,x,13,19").unwrap().solve_part2(), Some(3417))
    }
}
