use std::io::BufRead;
use std::io::{self};
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
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut ship = Ship::new();
    for instr in NavInstruction::parse(handle.lines().filter_map(Result::ok)) {
        ship.apply1(&instr);
    }
    println!("Part 1 manhattan distance is {}", ship.manhattan_distance())
}

impl Ship {
    fn new() -> Ship {
        Ship {
            east_pos: 0,
            north_pos: 0,
            dir_faced: East,
        }
    }

    fn apply1(self: &mut Ship, instr: &NavInstruction) {
        match instr.action {
            F => self.apply1(&NavInstruction {
                action: Move(self.dir_faced),
                value: instr.value,
            }),
            Move(East) => self.east_pos = self.east_pos + instr.value,
            Move(South) => self.north_pos = self.north_pos - instr.value,
            Move(West) => self.east_pos = self.east_pos - instr.value,
            Move(North) => self.north_pos = self.north_pos + instr.value,
            L => self.dir_faced = self.dir_faced.turn_left(instr.value),
            R => self.dir_faced = self.dir_faced.turn_right(instr.value),
        }
    }

    fn manhattan_distance(self: &Ship) -> i32 {
        self.east_pos.abs() + self.north_pos.abs()
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

impl From<i32> for Direction {
    fn from(n: i32) -> Self {
        let n = if n < 0 { n % 4 + 4 } else { n % 4 };
        match n {
            0 => East,
            1 => South,
            2 => West,
            3 => North,
            _ => unreachable!("Invalid integer for direction"),
        }
    }
}

impl From<Direction> for i32 {
    fn from(d: Direction) -> Self {
        match d {
            East => 0,
            South => 1,
            West => 2,
            North => 3,
        }
    }
}

impl Direction {
    fn turn_right(self: &Direction, degrees: i32) -> Direction {
        let delta = degrees / 90;
        Direction::from(i32::from(*self) + delta % 4)
    }

    fn turn_left(self: &Direction, degrees: i32) -> Direction {
        let delta = degrees / 90;
        Direction::from(i32::from(*self) - delta % 4)
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
        ship.apply1(&NavInstruction {
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
            ship.apply1(&instr);
        }
        assert_eq!(ship.east_pos, 17);
        assert_eq!(ship.north_pos, 3);
        assert_eq!(ship.dir_faced, East);
    }

    #[test]
    fn turn_ship_right() {
        let mut ship = Ship::new();
        ship.apply1(&NavInstruction {
            action: R,
            value: 90,
        });
        assert_eq!(ship.dir_faced, South);
    }

    #[test]
    fn execute_example_instructions() {
        let mut ship = Ship::new();
        for instr in NavInstruction::parse(EXAMPLE.lines().map(|l| l.to_string())) {
            ship.apply1(&instr);
        }
        assert_eq!(ship.east_pos, 17);
        assert_eq!(ship.north_pos, -8);
        assert_eq!(ship.dir_faced, South);
        assert_eq!(ship.manhattan_distance(), 25);
    }
}
