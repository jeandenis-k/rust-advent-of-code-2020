use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("Hello advent 01!");
    println!("Solution is {}", solve_puzzle().unwrap());
}

fn solve_puzzle() -> Result<i64, io::Error> {
    let file = File::open("input")?;
    let lines = std::io::BufReader::new(file).lines();
    let numbers: Vec<i64> = lines
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let solution = numbers.iter().find_map(|i1| {
        numbers
            .iter()
            .find_map(|i2| if i1 + i2 == 2020 { Some(i1 * i2) } else { None })
    });
    return Ok(solution.unwrap());

    // let mut iter = numbers.iter().zip(numbers.iter());
    // let mut iter2 = numbers.iter().zip(numbers.iter());
    // // for (i1, i2) in iter2 {
    // //     println!("{} {} {}", i1, i2, i1 + i2);
    // // }

    // for (i1, i2) in numbers.iter().zip(numbers.iter()) {
    //     let sum = i1 + i2;
    //     if sum > 2000 && sum <= 2020 {
    //         println!("{}", sum);
    //     }
    // }

    // println!("{:?}", iter2.find(|(&i1, &i2)| i1 + i2 == 2020));
    // return Ok(iter
    //     // .find_map(|(i1, i2)| if i1 + i2 == 2020 { Some(i1 * i2) } else { None })
    //     .find_map(|(i1, i2)| if i1 + i2 == 2020 { Some(i1 * i2) } else { None })
    //     .unwrap());
}

// for (i1, i2) in iter {
//     println!("{} {}", i1, i2);
// }
// return Ok(0);

// let mut number = 0;
// for line in lines {
//     number = line?.parse().unwrap();
//     println!("{}", number);
//     for line2 in lines {}
// }
