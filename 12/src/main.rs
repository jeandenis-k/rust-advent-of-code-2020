use std::str::FromStr;
use Direction::*;
use NavAction::*;

#[derive(Debug, PartialEq)]
struct Ship {
    east_pos: i32,
    north_pos: i32,
    dir_faced: Direction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Debug, PartialEq)]
struct NavInstruction {
    action: NavAction,
    value: i32,
}

#[derive(Debug, PartialEq)]
enum NavAction {
    Move(Direction),
    F,
    L,
    R,
}

impl FromStr for NavAction {
    type Err = ();
    fn from_str(s: &str) -> Result<NavAction, ()> {
        match s {
            "N" => Ok(Move(North)),
            "S" => Ok(Move(South)),
            "E" => Ok(Move(East)),
            "W" => Ok(Move(West)),
            "L" => Ok(L),
            "R" => Ok(R),
            "F" => Ok(F),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

impl Ship {
    fn new() -> Ship {
        Ship {
            east_pos: 0,
            north_pos: 0,
            dir_faced: East,
        }
    }

    fn apply(self: &mut Ship, instr: &NavInstruction) {
        match instr.action {
            F => self.apply(&NavInstruction {
                action: Move(self.dir_faced),
                value: instr.value,
            }),
            Move(East) => self.east_pos = self.east_pos + instr.value,
            Move(South) => self.north_pos = self.north_pos - instr.value,
            Move(West) => self.east_pos = self.east_pos - instr.value,
            Move(North) => self.north_pos = self.north_pos + instr.value,
            _ => unimplemented!(),
        }
    }
}

impl NavInstruction {
    fn parse(it: impl Iterator<Item = String>) -> impl Iterator<Item = NavInstruction> {
        it.map(|line| {
            let action: NavAction = line[0..=0].parse().unwrap();
            let value = line[1..].parse().unwrap();
            NavInstruction { action, value }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "\
F10
N3
F7
R90
F11
";

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            NavInstruction::parse(EXAMPLE.lines().map(|str| str.to_string())).collect::<Vec<_>>(),
            vec![
                NavInstruction {
                    action: F,
                    value: 10
                },
                NavInstruction {
                    action: Move(North),
                    value: 3
                },
                NavInstruction {
                    action: F,
                    value: 7
                },
                NavInstruction {
                    action: R,
                    value: 90
                },
                NavInstruction {
                    action: F,
                    value: 11
                },
            ]
        )
    }

    #[test]
    fn test_execute_one_instruction() {
        let mut ship = Ship::new();
        ship.apply(&NavInstruction {
            action: F,
            value: 10,
        });
        assert_eq!(ship.east_pos, 10);
        assert_eq!(ship.north_pos, 0);
        assert_eq!(ship.dir_faced, East);
    }

    #[test]
    fn execute_simple_instructions() {
        let mut ship = Ship::new();
        for instr in vec![
            NavInstruction {
                action: F,
                value: 10,
            },
            NavInstruction {
                action: Move(North),
                value: 3,
            },
            NavInstruction {
                action: F,
                value: 7,
            },
        ] {
            ship.apply(&instr);
        }
        assert_eq!(ship.east_pos, 17);
        assert_eq!(ship.north_pos, 3);
        assert_eq!(ship.dir_faced, East);
    }
}
