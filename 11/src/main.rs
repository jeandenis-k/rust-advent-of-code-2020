use std::io::BufRead;
use std::io::{self};

#[derive(Debug, Clone, PartialEq)]
struct WaitingArea {
    cells: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Part1Iter<'a> {
    area: &'a mut WaitingArea,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut area = WaitingArea::new(handle.lines().filter_map(Result::ok));
    println!(
        "Solution of part 1 is {}",
        area.occupied_seat_count_on_fixed_configuration(),
    )
}

impl WaitingArea {
    fn new(it: impl Iterator<Item = String>) -> WaitingArea {
        let cells: Vec<_> = it.collect();
        WaitingArea { cells }
    }

    fn iter_part1(self: &mut WaitingArea) -> Part1Iter {
        Part1Iter { area: self }
    }

    fn occupied_seat_count_on_fixed_configuration(self: &mut WaitingArea) -> usize {
        self.iter_part1().find_map(|option| option).unwrap()
    }

    fn occupied_seat_count(self: &WaitingArea) -> usize {
        self.cells
            .iter()
            .map(|line| line.as_bytes().iter().filter(|c| **c == b'#').count())
            .sum()
    }
}

impl<'a> Iterator for Part1Iter<'a> {
    type Item = Option<usize>;

    fn next(self: &mut Part1Iter<'a>) -> Option<Self::Item> {
        let old = self.area.clone();
        let mut did_update = false;

        for (i, line) in self.area.cells.iter_mut().enumerate() {
            unsafe {
                for (j, cell) in line.as_bytes_mut().iter_mut().enumerate() {
                    if *cell == b'L' {
                        let no_adjacent_occupied_seats: bool =
                            adjacent_cells(&old.cells, i, j).iter().all(|c| *c != b'#');
                        if no_adjacent_occupied_seats {
                            *cell = b'#';
                            did_update = true;
                        }
                    } else if *cell == b'#' {
                        let adjacent_occupied_count = adjacent_cells(&old.cells, i, j)
                            .iter()
                            .filter(|c| **c == b'#')
                            .count();
                        if adjacent_occupied_count >= 4 {
                            *cell = b'L';
                            did_update = true;
                        }
                    }
                }
            }
        }

        if did_update {
            Some(None)
        } else {
            Some(Some(self.area.occupied_seat_count()))
        }
    }
}

fn adjacent_cells<'a>(cells: &'a Vec<String>, i: usize, j: usize) -> Vec<u8> {
    let i = i as isize;
    let j = j as isize;
    let width = cells[0].len();
    let lines: Vec<isize> = vec![i - 1, i, i + 1];
    let cols: Vec<isize> = vec![j - 1, j, j + 1];
    lines
        .iter()
        .flat_map(|line| cols.iter().map(move |col| (*line, *col)))
        .filter(|(k, l)| {
            (*k, *l) != (i, j)
                && *k >= 0
                && *k < width as isize
                && *l >= 0
                && *l < cells.len() as isize
        })
        .map(|(i, j)| cells[i as usize].as_bytes()[j as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_cells() {
        let area_string = include_str!("../input_example");
        let area = WaitingArea::new(area_string.lines().map(|line| line.to_string()));
        assert_eq!(adjacent_cells(&area.cells, 0, 9), [b'L', b'L', b'L'])
    }
}
