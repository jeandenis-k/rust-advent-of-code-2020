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
    println!("Solution of part 2 is {}", count_arrangements(&lines));
}

fn reduce((d1, d3): (i32, i32), d: i32) -> (i32, i32) {
    match d {
        1 => (d1 + 1, d3),
        3 => (d1, d3 + 1),
        _ => (d1, d3),
    }
}

fn count_arrangements(numbers: &[i32]) -> i32 {
    fn rec_count_arrangements(map: &mut HashMap<usize, i32>, numbers: &[i32], index: usize) -> i32 {
        let n1 = numbers[index];
        if index == numbers.len() - 1 {
            1
        } else {
            let count = map.get(&index);
            match count {
                Some(count) => *count,
                None => {
                    let count = numbers[index + 1..]
                        .iter()
                        .enumerate()
                        .take_while(|(_, n2)| (**n2 - n1) <= 3)
                        .map(|(i, _)| rec_count_arrangements(map, &numbers, i + index + 1))
                        .sum();
                    map.insert(index, count);
                    count
                }
            }
        }
    }
    let mut map: HashMap<usize, i32> = HashMap::new();
    rec_count_arrangements(&mut map, numbers, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::*;

    #[test]
    fn test_count_arrangements_on_example1() {
        let file = File::open("./input_example1").unwrap();
        let mut numbers: Vec<_> = BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        numbers.sort();
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 8);
    }

    #[test]
    fn test_count_arrangements_on_simple_example() {
        let numbers: Vec<_> = vec![1, 2, 4, 5];
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 3);

        let numbers: Vec<_> = vec![1, 2, 4, 7];
        println!("{:?}", numbers);
        assert_eq!(count_arrangements(&numbers), 2);
    }
}
