#[derive(Debug, PartialEq)]
struct Notes {
    earliest: i32,
    bus_ids: Vec<i32>,
}
fn main() {
    println!("{:?}", parse(INPUT).unwrap().solve_part1());
}

static INPUT: &str = include_str!("../input");

fn parse(input: &str) -> Option<Notes> {
    let mut lines = input.lines();
    let timestamp: i32 = lines.next()?.parse::<i32>().ok()?;
    let bus_ids = lines
        .next()?
        .split(",")
        .filter(|s| *s != "x")
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
            bus_ids.iter().find_map(|bus_id| {
                if time % bus_id == 0 {
                    Some((time - earliest) * *bus_id)
                } else {
                    None
                }
            })
        })
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
                bus_ids: vec![7, 13, 59, 31, 19]
            })
        )
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(parse(INPUT_EXAMPLE).unwrap().solve_part1(), Some(295))
    }
}
