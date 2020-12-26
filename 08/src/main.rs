use std::collections::HashSet;
use std::io::BufRead;
use std::io::{self};

#[derive(Debug)]
struct Console {
    instructions: Vec<(String, i32)>, // nop, acc or jmp
    instructions_executed: HashSet<i32>,
    program_counter: i32,
    accumulator: i32,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let instructions: Vec<_> = handle
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            (parts[0].to_string(), parts[1].parse::<i32>().unwrap())
        })
        .collect();
    println!("{:?}", instructions);

    let console = Console::new(instructions);
    println!("{:?}", console.last());
}

impl Console {
    fn new(instructions: Vec<(String, i32)>) -> Console {
        Console {
            instructions,
            instructions_executed: HashSet::new(),
            program_counter: 0,
            accumulator: 0,
        }
    }
}

impl Iterator for Console {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.instructions_executed.contains(&self.program_counter) {
            return None;
        }

        let instruction = &self.instructions[self.program_counter as usize];
        self.instructions_executed.insert(self.program_counter);

        self.accumulator = if instruction.0 == "acc" {
            self.accumulator + instruction.1
        } else {
            self.accumulator
        };

        println!("{}: {}", self.program_counter + 1, self.accumulator);

        self.program_counter = if instruction.0 == "jmp" {
            self.program_counter + instruction.1
        } else {
            self.program_counter + 1
        };

        Some(self.accumulator)
    }
}
