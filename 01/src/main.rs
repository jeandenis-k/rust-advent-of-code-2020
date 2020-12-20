use std::fs::File;
use std::io::BufRead;

fn main() {
    println!("Hello advent 01!");
    let file = File::open("input").unwrap();
    let lines = std::io::BufReader::new(file).lines();
    let numbers: Vec<i64> = lines
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    println!("Solution of part 1 is {}", solve_puzzle(&numbers).unwrap());
    println!("Solution of part 2 is {}", solve_part2(&numbers).unwrap());
}

fn solve_puzzle(numbers: &Vec<i64>) -> Option<i64> {
    return numbers.iter().find_map(|i1| {
        numbers
            .iter()
            .find_map(|i2| if i1 + i2 == 2020 { Some(i1 * i2) } else { None })
    });
}

fn solve_part2(numbers: &Vec<i64>) -> Option<i64> {
    return numbers.iter().find_map(|i1| {
        numbers.iter().find_map(|i2| {
            numbers.iter().find_map(|i3| {
                if i1 + i2 + i3 == 2020 {
                    Some(i1 * i2 * i3)
                } else {
                    None
                }
            })
        })
    });
}
