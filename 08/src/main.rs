use std::collections::HashSet;
use std::io::BufRead;
use std::io::{self};

#[derive(Debug)]
struct ConsoleProgram {
    instructions: Vec<(String, i32)>, // nop, acc or jmp
    flipped_instruction: Option<(i32, (String, i32))>,
}

#[derive(Debug)]
struct ConsoleInterpreter<'a> {
    program: &'a ConsoleProgram,
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

    let program = ConsoleProgram::new(instructions);
    println!("Part 1: {:?}", program.iter().last());

    let fixed_program = program
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(index, instruction)| {
            if instruction.0 == "jmp" || instruction.0 == "nop" {
                Some(program.flip_instruction(index as i32))
            } else {
                None
            }
        })
        .find_map(|candidate| {
            let result = candidate.run();
            if result.1 {
                Some(result)
            } else {
                None
            }
        });
    println!("Part 2: {:?}", fixed_program)
}

impl ConsoleProgram {
    fn new(instructions: Vec<(String, i32)>) -> ConsoleProgram {
        ConsoleProgram {
            instructions,
            flipped_instruction: None,
        }
    }

    fn run(self: &ConsoleProgram) -> (i32, bool) {
        self.iter().last().unwrap()
    }

    fn flip_instruction(self: &ConsoleProgram, index: i32) -> ConsoleProgram {
        let instruction = &self.instructions[index as usize];
        let flipped_instruction = if instruction.0 == "nop" {
            (index, ("jmp".to_string(), instruction.1))
        } else {
            (index, ("nop".to_string(), instruction.1))
        };
        ConsoleProgram {
            instructions: self.instructions.clone(),
            flipped_instruction: Some(flipped_instruction),
        }
    }

    fn iter(self: &ConsoleProgram) -> ConsoleInterpreter {
        ConsoleInterpreter {
            program: self,
            instructions_executed: HashSet::new(),
            program_counter: 0,
            accumulator: 0,
        }
    }
}

impl Iterator for ConsoleInterpreter<'_> {
    type Item = (i32, bool);

    fn next(&mut self) -> Option<(i32, bool)> {
        if self.instructions_executed.contains(&self.program_counter)
            || self.program_counter == self.program.instructions.len() as i32
        {
            return None;
        }

        let instruction = &self.program.instructions[self.program_counter as usize];
        let instruction = match &self.program.flipped_instruction {
            Some((index, flipped_instruction)) => {
                if *index == self.program_counter {
                    flipped_instruction
                } else {
                    instruction
                }
            }
            None => instruction,
        };

        let terminates = self.program_counter == self.program.instructions.len() as i32 - 1;
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

        Some((self.accumulator, terminates))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_program() -> ConsoleProgram {
        ConsoleProgram::new(vec![
            ("nop".to_string(), 0),
            ("acc".to_string(), 1),
            ("jmp".to_string(), 4),
            ("acc".to_string(), 3),
            ("jmp".to_string(), 3),
            ("acc".to_string(), -99),
            ("acc".to_string(), 1),
            ("jmp".to_string(), -4),
            ("acc".to_string(), 6),
        ])
    }

    #[test]
    fn test_does_not_terminate() {
        assert_eq!(example_program().run(), (5, false))
    }

    #[test]
    fn test_terminate() {
        println!("{:?}", example_program().flip_instruction(7));
        assert_eq!(example_program().flip_instruction(7).run(), (8, true))
    }
}
