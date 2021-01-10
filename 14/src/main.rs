mod mask;
use mask::*;
use regex::Regex;
use std::collections::HashMap;
use Instruction::*;

#[derive(PartialEq, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Debug)]
enum Instruction {
    SetMask(String),
    Write((usize, u64)),
}

static PROGRAM: &str = include_str!("input");

fn main() {
    let program = Program::parse(PROGRAM);
    println!("{}", program.execute());
    println!("{}", program.execute2());
}

impl Program {
    fn parse(s: &str) -> Program {
        let instructions = s
            .lines()
            .map(|line| {
                let re_mask = Regex::new(r"mask = (.*)").unwrap();
                let re_write = Regex::new(r"mem\[(.*)\] = (.*)").unwrap();
                if let Some(matches) = re_write.captures(&line) {
                    let address = matches.get(1).unwrap().as_str().parse().unwrap();
                    let value = matches.get(2).unwrap().as_str().parse().unwrap();
                    Write((address, value))
                } else if let Some(matches) = re_mask.captures(&line) {
                    let mask = matches.get(1).unwrap().as_str().parse().unwrap();
                    SetMask(mask)
                } else {
                    panic!("Invalid input line: {}", line);
                }
            })
            .collect();
        Program { instructions }
    }

    fn execute(self: &Program) -> u64 {
        let mut memory: HashMap<usize, u64> = HashMap::new();
        let mut current_mask = Mask("");
        for instruction in self.instructions.iter().by_ref() {
            match instruction {
                Write((addr, value)) => {
                    memory.insert(*addr, current_mask.apply(*value));
                }
                SetMask(mask) => current_mask = Mask(mask),
            }
        }
        memory.values().sum()
    }

    fn execute2(self: &Program) -> u64 {
        let mut memory: HashMap<usize, u64> = HashMap::new();
        let mut current_mask = Mask("");
        for instruction in self.instructions.iter().by_ref() {
            match instruction {
                Write((addr, value)) => {
                    for addr in current_mask.apply2(*addr as u64) {
                        memory.insert(addr, *value);
                    }
                }
                SetMask(mask) => current_mask = Mask(mask),
            }
        }
        memory.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROGRAM: &str = include_str!("input_example");
    static PROGRAM2: &str = include_str!("input_example2");

    #[test]
    fn test_execute_program() {
        assert_eq!(Program::parse(PROGRAM).execute(), 165)
    }

    #[test]
    fn test_execute_version2() {
        assert_eq!(Program::parse(PROGRAM2).execute2(), 208)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Program::parse(PROGRAM),
            Program {
                instructions: vec![
                    SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
                    Write((8, 11)),
                    Write((7, 101)),
                    Write((8, 0))
                ]
            }
        )
    }
}
