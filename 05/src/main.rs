use arrayvec::ArrayVec;
use std::io::BufRead;
use ColumnHalf::*;
use RowHalf::*;

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let seats: Vec<_> = std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut chars = line.chars();
            let row = chars
                .by_ref()
                .take(7)
                .map(|char| if char == 'F' { F } else { B })
                .collect();
            let col = chars
                .take(3)
                .map(|char| if char == 'L' { L } else { R })
                .collect();
            Seat(row, col)
        })
        .collect();

    println!(
        "Highest seat id is {}",
        seats.iter().map(|seat| seat.id()).max().unwrap()
    );
}

fn test_seat() {
    // Seat FBFBBFFRLR
    let seat = Seat(
        ArrayVec::from([F, B, F, B, B, F, F]),
        ArrayVec::from([R, L, R]),
    );
    println!("Seat row is {}", seat.row());
    println!("Seat col is {}", seat.col());
    println!("Seat id is {}", seat.id());
}

#[derive(Debug)]
struct Seat(ArrayVec<[RowHalf; 7]>, ArrayVec<[ColumnHalf; 3]>);

#[derive(PartialEq, Debug)]
enum RowHalf {
    F, // Lower
    B, // Upper
}

#[derive(PartialEq, Debug)]
enum ColumnHalf {
    L, // Lower
    R, // Upper
}

impl Seat {
    fn id(self: &Seat) -> i32 {
        self.row() * 8 + self.col()
    }

    fn row(self: &Seat) -> i32 {
        self.0
            .iter()
            .fold((0, 127), |(from, to), half| {
                if *half == F {
                    (from, to - (to - from + 1) / 2)
                } else {
                    (from + (to - from + 1) / 2, to)
                }
            })
            .0
    }

    fn col(self: &Seat) -> i32 {
        self.1
            .iter()
            .fold((0, 7), |(from, to), half| {
                if *half == L {
                    (from, to - (to - from + 1) / 2)
                } else {
                    (from + (to - from + 1) / 2, to)
                }
            })
            .0
    }
}
