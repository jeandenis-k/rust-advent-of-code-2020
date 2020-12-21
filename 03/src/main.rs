use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let file = std::fs::File::open("input").unwrap();
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

    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let solution2 = slopes
        .iter()
        .map(|slope| map.count_trees(&slope))
        .fold(1, |acc, count| acc * count);
    println!("Slope counts multiplied equals {:?}", solution2);
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
            .step_by(slope.down)
            .enumerate()
            // .for_each(|(index, line)| println!("{}", index * 3 % line.len()))
            .map(|(index, line)| {
                let location = line[index * slope.right % line.len()];
                println!(
                    "{}, {}: {:?}",
                    index,
                    index * slope.right % line.len(),
                    location
                );
                return location;
            })
            .filter(|&location| location == MapLocation::Tree)
            .count();
    }
}
