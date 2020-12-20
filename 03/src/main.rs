use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = std::fs::File::open("example_map").unwrap();
    let map: Map = Map(std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            return line
                .chars()
                .map(|c| {
                    if c == '.' {
                        MapLocation::OpenSquare
                    } else {
                        MapLocation::Tree
                    }
                })
                .collect::<Vec<_>>();
        })
        .collect());

    let solution = map.count_trees(&Slope { right: 3, down: 1 });
    println!("Encountered {} trees", solution);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapLocation {
    OpenSquare,
    Tree,
}

struct Map(Vec<Vec<MapLocation>>);

struct Slope {
    right: usize,
    down: usize,
}

impl Map {
    fn count_trees(self: &Map, slope: &Slope) -> usize {
        return self
            .0
            .iter()
            .enumerate()
            // .for_each(|(index, line)| println!("{}", index * 3 % line.len()))
            .map(|(index, line)| line[index * slope.right % line.len()])
            .filter(|&location| location == MapLocation::Tree)
            .count();
    }
}
