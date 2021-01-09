use regex::Regex;
use std::collections::HashMap;
use Instruction::*;

#[derive(PartialEq, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Debug)]
enum Instruction {
    Mask(String),
    Write((usize, u64)),
}

static PROGRAM: &str = include_str!("input");

fn main() {
    println!("{}", Program::parse(PROGRAM).execute());
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
                    Mask(mask)
                } else {
                    panic!("Invalid input line: {}", line);
                }
            })
            .collect();
        Program { instructions }
    }

    fn execute(self: &Program) -> u64 {
        let mut memory: HashMap<usize, u64> = HashMap::new();
        let mut current_mask = "".to_string();
        for instruction in self.instructions.iter().by_ref() {
            match instruction {
                Write((addr, value)) => {
                    memory.insert(*addr, apply_mask(*value, &current_mask));
                }
                Mask(mask) => current_mask = mask.clone(),
            }
        }
        memory.values().sum()
    }
}

fn apply_mask(n: u64, mask: &str) -> u64 {
    n & gen_and_mask(mask) | gen_or_mask(mask)
}

fn gen_and_mask(s: &str) -> u64 {
    let s = str::replace(s, "X", "1");
    u64::from_str_radix(&s.to_string(), 2).unwrap()
}

fn gen_or_mask(s: &str) -> u64 {
    let s = str::replace(s, "X", "0");
    u64::from_str_radix(&s.to_string(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static PROGRAM: &str = include_str!("input_example");
    static MASK: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";

    #[test]
    fn test_execute_program() {
        assert_eq!(Program::parse(PROGRAM).execute(), 165)
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            Program::parse(PROGRAM),
            Program {
                instructions: vec![
                    Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()),
                    Write((8, 11)),
                    Write((7, 101)),
                    Write((8, 0))
                ]
            }
        )
    }

    #[test]
    fn test_apply_mask() {
        assert_eq!(apply_mask(11, MASK), 73);
        assert_eq!(apply_mask(101, MASK), 101);
        assert_eq!(apply_mask(0, MASK), 64);
    }

    #[test]
    fn test_gen_and_mask() {
        assert_eq!(
            gen_and_mask(MASK),
            0b111111111111111111111111111111111101_u64
        )
    }

    #[test]
    fn test_gen_or_mask() {
        assert_eq!(
            gen_or_mask(MASK),
            0b000000000000000000000000000001000000_u64
        )
    }
}
