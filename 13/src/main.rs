type Notes = (i32, Vec<i32>);
fn main() {
    println!("{:?}", solve_part1(parse(INPUT).unwrap()));
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
    Some((timestamp, bus_ids))
}

fn solve_part1((timestamp, buses): Notes) -> Option<i32> {
    (timestamp..).find_map(|time| {
        buses.iter().find_map(|bus_id| {
            if time % bus_id == 0 {
                Some((time - timestamp) * *bus_id)
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = include_str!("../input_example");

    #[test]
    fn test_parse() {
        assert_eq!(parse(INPUT_EXAMPLE), Some((939, vec![7, 13, 59, 31, 19])))
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(parse(INPUT_EXAMPLE).unwrap()), Some(295))
    }
}
