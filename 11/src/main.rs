use std::io::BufRead;
use std::io::{self};

#[derive(Debug)]
struct WaitingArea {
    cells: Vec<Vec<char>>,
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
        let cells: Vec<Vec<char>> = it.map(|s| s.chars().collect()).collect();
        let width = cells[0].len();
        WaitingArea { cells, width }
    }
}

impl Iterator for WaitingArea {
    type Item = (Vec<Vec<char>>, Vec<Vec<char>>);

    fn next(self: &mut WaitingArea) -> Option<Self::Item> {
        let old = self.cells.clone();
        for (i, line) in self.cells.iter_mut().enumerate() {
            for (j, cell) in line.iter_mut().enumerate() {
                *cell = 'X';
            }
        }
        Some((old, self.cells.clone()))
    }
}
