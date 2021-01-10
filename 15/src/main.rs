use std::collections::HashMap;

static PUZZLE_INPUT: [i32; 6] = [2, 0, 1, 9, 5, 19];

struct Recitation<'a> {
    starting_numbers: &'a [i32],
    count: i32,
    spoken: HashMap<i32, i32>,
    last_spoken: i32,
}

fn main() {
    println!("{}", solve_part1(&PUZZLE_INPUT));
    println!("{}", solve_part2(&PUZZLE_INPUT));
}

fn solve_part1(input: &[i32]) -> i32 {
    Recitation::new(input).nth(2019).unwrap()
}

fn solve_part2(input: &[i32]) -> i32 {
    Recitation::new(input).nth(30000000 - 1).unwrap()
}

impl<'a> Recitation<'a> {
    fn new(starting_numbers: &'a [i32]) -> Recitation<'a> {
        Recitation {
            starting_numbers,
            count: 0,
            spoken: HashMap::new(),
            last_spoken: 0,
        }
    }
}

impl<'a> Iterator for Recitation<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if (self.count as usize) < self.starting_numbers.len() {
            self.starting_numbers[self.count as usize]
        } else {
            let previously_spoken_at = self.spoken.get(&self.last_spoken);
            match previously_spoken_at {
                Some(i) => self.count - i,
                None => 0,
            }
        };
        if self.count != 0 {
            self.spoken.insert(self.last_spoken, self.count);
        }
        self.count += 1;
        self.last_spoken = next;
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [i32; 3] = [0, 3, 6];

    #[test]
    fn test_recitation() {
        assert_eq!(
            Recitation::new(&INPUT).take(10).collect::<Vec<i32>>(),
            vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0],
        );
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&INPUT), 436);
        assert_eq!(solve_part1(&[1, 3, 2]), 1);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&INPUT), 175594);
        assert_eq!(solve_part2(&[1, 3, 2]), 2578);
    }
}
