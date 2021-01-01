use std::collections::HashMap;
use std::io::BufRead;
use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines: Vec<_> = handle
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    lines.sort();
    println!("{:?}", lines);
    let mut lines_minus_first = lines.iter();
    let &first = lines_minus_first.next().unwrap();
    let (d1, d3) = lines
        .iter()
        .zip(lines_minus_first)
        .map(|(i1, i2)| i2 - i1)
        // .inspect(|diff| println!("{}", diff))
        .fold(reduce((0, 0), first), reduce);
    println!("{:?}", (d1, d3 + 1));
    println!("Solution of part 1 is {}", d1 * (d3 + 1));

    let mut lines_with_zero = vec![0];
    lines_with_zero.extend(lines);
    println!(
        "Solution of part 2 is {}",
        count_arrangements(&lines_with_zero)
    );
}

fn reduce((d1, d3): (i32, i32), d: i32) -> (i32, i32) {
    match d {
        1 => (d1 + 1, d3),
        3 => (d1, d3 + 1),
        _ => (d1, d3),
    }
}

fn count_arrangements(numbers: &[i32]) -> i64 {
    fn rec_count_arrangements(map: &mut HashMap<i32, i64>, numbers: &[i32], index: usize) -> i64 {
        let n1 = numbers[index];
        if index == numbers.len() - 1 {
            1
        } else {
            let count = map.get(&n1);
            match count {
                Some(count) => *count,
                None => {
                    let count = numbers[index + 1..]
                        .iter()
                        .enumerate()
                        .take_while(|(_, n2)| (**n2 - n1) <= 3)
                        .map(|(i, _)| rec_count_arrangements(map, &numbers, i + index + 1))
                        .sum();
                    map.insert(n1, count);
                    count
                }
            }
        }
    }
    let mut map: HashMap<i32, i64> = HashMap::new();
    let result = rec_count_arrangements(&mut map, numbers, 0);
    println!(
        "{:?}",
        numbers
            .iter()
            .map(|n| (n, map.get(n).unwrap_or(&0)))
            .collect::<Vec<_>>()
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::*;

    #[test]
    fn test_count_arrangements_on_example1() {
        let numbers = read_example_file("./input_example1");
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 8);
    }

    #[test]
    fn test_count_arrangements_on_example2() {
        let numbers = read_example_file("./input_example2");
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 19208);
    }

    #[test]
    fn test_count_arrangements_on_simple_example() {
        let numbers: Vec<_> = vec![1, 2, 4, 5];
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 3);

        let numbers: Vec<_> = vec![1, 2, 4, 7];
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 2);

        let numbers: Vec<_> = vec![1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18];
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 28);
    }

    fn read_example_file(path: &str) -> Vec<i32> {
        let file = File::open(path).unwrap();
        let mut numbers = vec![0];
        numbers.extend(
            BufReader::new(file)
                .lines()
                .filter_map(Result::ok)
                .map(|line| line.parse::<i32>().unwrap()),
        );
        numbers.sort();
        numbers
    }
}
