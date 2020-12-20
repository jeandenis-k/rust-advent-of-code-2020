use std::fs::File;
use std::io::BufRead;

fn main() {
    println!("Hello advent 01!");
    let file = File::open("input").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let numbers: Vec<i64> = lines
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    println!("Solution is {}", solve_puzzle(numbers).unwrap());
}

fn solve_puzzle(numbers: Vec<i64>) -> Option<i64> {
    return numbers.iter().find_map(|i1| {
        numbers
            .iter()
            .find_map(|i2| if i1 + i2 == 2020 { Some(i1 * i2) } else { None })
    });
}
