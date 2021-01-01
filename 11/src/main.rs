use std::io::BufRead;
use std::io::{self};

#[derive(Debug, Clone)]
struct WaitingArea {
    cells: Vec<String>,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let area = WaitingArea::new(handle.lines().filter_map(Result::ok));
    dbg!(area.take(10).map(|(old, _new)| old).collect::<Vec<_>>());
}

impl WaitingArea {
    fn new(it: impl Iterator<Item = String>) -> WaitingArea {
        let cells: Vec<_> = it.collect();
        WaitingArea { cells }
    }
}

impl Iterator for WaitingArea {
    type Item = (WaitingArea, WaitingArea);

    fn next(self: &mut WaitingArea) -> Option<Self::Item> {
        let old = self.clone();
        for (i, line) in self.cells.iter_mut().enumerate() {
            unsafe {
                for (j, cell) in line.as_bytes_mut().iter_mut().enumerate() {
                    if *cell == b'L' {
                        let no_adjacent_occupied_seats: bool =
                            adjacent_cells(&old.cells, i, j).iter().all(|c| *c != b'#');
                        if no_adjacent_occupied_seats {
                            *cell = b'#';
                        }
                    } else if *cell == b'#' {
                        let adjacent_occupied_count = adjacent_cells(&old.cells, i, j)
                            .iter()
                            .filter(|c| **c == b'#')
                            .count();
                        if adjacent_occupied_count >= 4 {
                            *cell = b'L';
                        }
                    }
                }
            }
        }
        Some((old, self.clone()))
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
