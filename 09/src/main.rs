use std::collections::HashSet;
use std::io::BufRead;
use std::io::{self};

#[derive(Debug)]
struct XmasIterator<'a> {
    numbers: &'a Vec<i64>,
    preamble_size: usize,
    valid_sums: HashSet<i64>,
    counter: usize,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines: Vec<_> = handle
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut xmas_it = XmasIterator::new(&lines, 25);
    let part1_solution = xmas_it
        .find_map(|(value, valid)| if !valid { Some(value) } else { None })
        .unwrap();
    println!("Solution of part 1 is {}", part1_solution);

    let contiguous_nums =
        find_contiguous_numbers_adding_up_to_step1(&lines, part1_solution).unwrap();
    println!(
        "Solution of part 2 is {}",
        contiguous_nums.iter().min().unwrap() + contiguous_nums.iter().max().unwrap()
    )
}

fn generate_valid_sums(preamble: &[i64]) -> HashSet<i64> {
    preamble
        .iter()
        .enumerate()
        .flat_map(|(index, &n1)| preamble[index + 1..].iter().map(move |n2| n1 + n2))
        .collect()
}

fn find_contiguous_numbers_adding_up_to_step1(numbers: &Vec<i64>, step1: i64) -> Option<Vec<i64>> {
    numbers.iter().enumerate().find_map(|(index, n)| {
        println!("Testing starting from  {}", n);
        let mut sum = *n;
        let contiguous_chain: Vec<_> = numbers[index + 1..]
            .iter()
            .cloned()
            .take_while(|n2| {
                sum = sum + *n2;
                println!("Sum is now {}", sum);
                sum < step1
            })
            .collect();
        if sum == step1 {
            Some(numbers[index..=index + contiguous_chain.len() + 1].to_vec())
        } else {
            None
        }
    })
}

impl<'a> Iterator for XmasIterator<'a> {
    type Item = (i64, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == self.numbers.len() {
            None
        } else {
            let num = self.numbers[self.counter];
            let valid = self.valid_sums.contains(&num);

            let new_preamble = &self.numbers[self.counter - self.preamble_size + 1..=self.counter];
            self.valid_sums = generate_valid_sums(new_preamble);
            self.counter += 1;

            Some((num, valid))
        }
    }
}

impl<'a> XmasIterator<'a> {
    fn new(numbers: &Vec<i64>, preamble_size: i64) -> XmasIterator {
        let preamble_size = preamble_size as usize;
        let preamble = &numbers[0..preamble_size];
        XmasIterator {
            numbers,
            preamble_size,
            valid_sums: generate_valid_sums(preamble),
            counter: preamble_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::*;

    #[test]
    fn test_generate_valid_sums() {
        let values = [1, 2, 5, 6];
        assert_eq!(
            generate_valid_sums(&values),
            vec![3, 6, 7, 7, 8, 11].into_iter().collect()
        );
    }

    #[test]
    fn test_part2() {
        let values = parse_input_example();
        assert_eq!(
            find_contiguous_numbers_adding_up_to_step1(&values, 127).unwrap(),
            vec![15, 25, 47, 40]
        );
    }

    #[test]
    fn find_invalid_number_in_example() {
        let values = parse_input_example();
        let mut xmas_it = XmasIterator::new(&values, 5);
        assert_eq!(
            xmas_it.find_map(|(value, valid)| if !valid { Some(value) } else { None }),
            Some(127)
        );
    }

    fn parse_input_example() -> Vec<i64> {
        let file = File::open("./input_example").unwrap();
        BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }
}
