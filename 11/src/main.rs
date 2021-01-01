use std::io::BufRead;
use std::io::{self};

#[derive(Debug)]
struct WaitingArea {
    cells: Vec<String>,
    width: usize,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut area = WaitingArea::new(handle.lines().filter_map(Result::ok));
    let next = area.next();
    dbg!(area);
    dbg!(next);
}

impl WaitingArea {
    fn new(it: impl Iterator<Item = String>) -> WaitingArea {
        let cells: Vec<_> = it.collect();
        let width = cells[0].len();
        WaitingArea { cells, width }
    }
}

impl Iterator for WaitingArea {
    type Item = (Vec<String>, Vec<String>);

    fn next(self: &mut WaitingArea) -> Option<Self::Item> {
        let old = self.cells.clone();
        for (i, line) in self.cells.iter_mut().enumerate() {
            unsafe {
                for (j, cell) in line.as_bytes_mut().iter_mut().enumerate() {
                    if *cell == b'L' {
                        let no_occupied_seats: bool =
                            adjacent_cells(&old, i, j).iter().all(|c| *c != b'#');
                        if no_occupied_seats {
                            *cell = b'#';
                        }
                    }
                }
            }
        }
        Some((old, self.cells.clone()))
    }
}

fn adjacent_cells<'a>(cells: &'a Vec<String>, i: usize, j: usize) -> Vec<u8> {
    let lines = if i == 0 {
        vec![0, 1]
    } else {
        vec![i - 1, i, i + 1]
    };
    let cols = if j == 0 {
        vec![0, 1]
    } else {
        vec![j - 1, j, j + 1]
    };
    lines
        .iter()
        .flat_map(|line| cols.iter().map(move |col| (*line, *col)))
        .filter(|(i, j)| i != j)
        .filter_map(|(i, j)| {
            cells
                .get(i)
                .and_then(|line| line.as_bytes().get(j).map(|c| *c))
        })
        .collect()
}
