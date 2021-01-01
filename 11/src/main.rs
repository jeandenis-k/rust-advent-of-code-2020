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

#[derive(Debug, PartialEq)]
struct Part2Iter<'a> {
    area: &'a mut WaitingArea,
}

#[derive(Copy, Clone)]
enum Direction {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let area = WaitingArea::new(handle.lines().filter_map(Result::ok));
    let mut area1 = area.clone();
    let mut area2 = area.clone();

    println!(
        "Solution of part 1 is {}",
        area1.iter_part1().find_map(|option| option).unwrap(),
    );

    println!(
        "Solution of part 2 is {}",
        area2.iter_part2().find_map(|option| option).unwrap(),
    );
}

impl WaitingArea {
    fn new(it: impl Iterator<Item = String>) -> WaitingArea {
        let cells: Vec<_> = it.collect();
        WaitingArea { cells }
    }

    fn get(self: &WaitingArea, i: usize, j: usize) -> u8 {
        self.cells[i].as_bytes()[j]
    }

    fn iter_part1(self: &mut WaitingArea) -> Part1Iter {
        Part1Iter { area: self }
    }

    fn iter_part2(self: &mut WaitingArea) -> Part2Iter {
        Part2Iter { area: self }
    }

    fn occupied_seat_count(self: &WaitingArea) -> usize {
        self.cells
            .iter()
            .map(|line| line.as_bytes().iter().filter(|c| **c == b'#').count())
            .sum()
    }

    fn adjacent_cells<'a>(self: &WaitingArea, i: usize, j: usize) -> Vec<u8> {
        let i = i as isize;
        let j = j as isize;
        let width = self.cells[0].len();
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
                    && *l < self.cells.len() as isize
            })
            .map(|(i, j)| self.get(i as usize, j as usize))
            .collect()
    }

    fn visible_cells<'a>(self: &'a WaitingArea, i: usize, j: usize) -> Vec<u8> {
        use Direction::*;
        [N, Ne, E, Se, S, Sw, W, Nw]
            .iter()
            .by_ref()
            .filter_map(|dir| {
                self.visible_cells_in_direction(i, j, *dir)
                    .find(|c| *c != b'.')
            })
            .collect()
    }

    fn visible_cells_in_direction<'a>(
        self: &'a WaitingArea,
        i: usize,
        j: usize,
        dir: Direction,
    ) -> Box<dyn Iterator<Item = u8> + 'a> {
        let width = self.cells[0].len() as isize;
        let height = self.cells.len() as isize;
        let i = i as isize;
        let j = j as isize;

        let validate_coords = move |(k, l)| {
            if (k, l) != (i, j)
                && k >= 0
                && k < width as isize
                && l >= 0
                && l < self.cells.len() as isize
            {
                Some((k as usize, l as usize))
            } else {
                None
            }
        };

        match dir {
            Direction::E => Box::new(
                (j + 1..width)
                    .into_iter()
                    .map(move |j| (i, j))
                    .filter_map(validate_coords)
                    .map(move |(k, l)| self.get(k, l)),
            ),
            Direction::Ne => Box::new(
                { 0..=i - 1 }
                    .into_iter()
                    .rev()
                    .zip((j + 1..width).into_iter())
                    .filter_map(validate_coords)
                    .map(move |(i, j)| self.get(i, j)),
            ),
            Direction::N => Box::new(
                { 0..=i - 1 }
                    .into_iter()
                    .rev()
                    .map(move |i| (i, j))
                    .filter_map(validate_coords)
                    .map(move |(k, l)| self.get(k, l)),
            ),
            Direction::Nw => Box::new(
                { 0..=i - 1 }
                    .into_iter()
                    .rev()
                    .zip((0..=j - 1).into_iter().rev())
                    .filter_map(validate_coords)
                    .map(move |(i, j)| self.get(i, j)),
            ),
            Direction::W => Box::new(
                (0..=j - 1)
                    .into_iter()
                    .rev()
                    .map(move |l| (i, l))
                    .filter_map(validate_coords)
                    .map(move |(k, l)| self.get(k, l)),
            ),
            Direction::Sw => Box::new(
                (i + 1..height)
                    .into_iter()
                    .zip((0..=j - 1).into_iter().rev())
                    .filter_map(validate_coords)
                    .map(move |(i, j)| self.get(i, j)),
            ),
            Direction::S => Box::new(
                (i + 1..height)
                    .into_iter()
                    .map(move |k| (k, j))
                    .filter_map(validate_coords)
                    .map(move |(k, l)| self.get(k, l)),
            ),
            Direction::Se => Box::new(
                (i + 1..height)
                    .into_iter()
                    .zip((j + 1..width).into_iter())
                    .filter_map(validate_coords)
                    .map(move |(i, j)| self.get(i, j)),
            ),
        }
    }
}

impl<'a> Iterator for Part1Iter<'a> {
    type Item = Option<usize>;

    fn next(self: &mut Part1Iter<'a>) -> Option<Self::Item> {
        let old_area = self.area.clone();
        let mut did_update = false;

        for (i, line) in self.area.cells.iter_mut().enumerate() {
            unsafe {
                for (j, cell) in line.as_bytes_mut().iter_mut().enumerate() {
                    if *cell == b'L' {
                        let no_adjacent_occupied_seats: bool =
                            old_area.adjacent_cells(i, j).iter().all(|c| *c != b'#');
                        if no_adjacent_occupied_seats {
                            *cell = b'#';
                            did_update = true;
                        }
                    } else if *cell == b'#' {
                        let adjacent_occupied_count = old_area
                            .adjacent_cells(i, j)
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

impl<'a> Iterator for Part2Iter<'a> {
    type Item = Option<usize>;

    fn next(self: &mut Part2Iter<'a>) -> Option<Self::Item> {
        let old_area = self.area.clone();
        let mut did_update = false;

        for (i, line) in self.area.cells.iter_mut().enumerate() {
            unsafe {
                for (j, cell) in line.as_bytes_mut().iter_mut().enumerate() {
                    if *cell == b'L' {
                        let no_visible_occupied_seats: bool =
                            old_area.visible_cells(i, j).iter().all(|c| *c != b'#');
                        if no_visible_occupied_seats {
                            *cell = b'#';
                            did_update = true;
                        }
                    } else if *cell == b'#' {
                        let visible_occupied_count = old_area
                            .visible_cells(i, j)
                            .iter()
                            .filter(|c| **c == b'#')
                            .count();
                        if visible_occupied_count >= 5 {
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
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_adjacent_cells() {
        let area_string = include_str!("../input_example");
        let area = WaitingArea::new(area_string.lines().map(|line| line.to_string()));
        assert_eq!(area.adjacent_cells(0, 9), [b'L', b'L', b'L'])
    }

    static AREA_EXAMPLE: &str = indoc! {r#"
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        "#};

    static SIMPLE_AREA: &str = indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "};

    fn parse_example(str: &str) -> WaitingArea {
        WaitingArea::new(str.lines().map(|line| line.to_string()))
    }

    #[test]
    fn test_next_with_part1_rules() {
        let mut area = parse_example(SIMPLE_AREA);
        area.iter_part1().next();
        assert_eq!(
            area.cells,
            vec![
                "#.##.##.##",
                "#######.##",
                "#.#.#..#..",
                "####.##.##",
                "#.##.##.##",
                "#.#####.##",
                "..#.#.....",
                "##########",
                "#.######.#",
                "#.#####.##",
            ]
        )
    }

    #[test]
    fn test_next_with_part2_rules() {
        let mut area = parse_example(SIMPLE_AREA);
        let mut iter = area.iter_part2();
        iter.next();
        assert_eq!(
            area.cells,
            vec![
                "#.##.##.##",
                "#######.##",
                "#.#.#..#..",
                "####.##.##",
                "#.##.##.##",
                "#.#####.##",
                "..#.#.....",
                "##########",
                "#.######.#",
                "#.#####.##",
            ]
        );
        assert_eq!(area.visible_cells(0, 0), "###".as_bytes());
    }

    #[test]
    fn test_next_2_with_part2_rules() {
        let mut area = parse_example(SIMPLE_AREA);
        let mut iter = area.iter_part2();
        iter.next();
        iter.next();
        assert_eq!(
            area.cells,
            vec![
                "#.LL.LL.L#",
                "#LLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLL#",
                "#.LLLLLL.L",
                "#.LLLLL.L#",
            ]
        );
    }

    #[test]
    fn test_visible_cells_in_direction() {
        let area = parse_example(AREA_EXAMPLE);
        assert_eq!(area.visible_cells(4, 3), "########".as_bytes());
        assert_eq!(area.visible_cells(0, 0), "##".as_bytes());

        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::E)
                .collect::<Vec<_>>(),
            "....#".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::Ne)
                .collect::<Vec<_>>(),
            "...#".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::N)
                .collect::<Vec<_>>(),
            "..#.".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::Nw)
                .collect::<Vec<_>>(),
            ".#.".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::W)
                .collect::<Vec<_>>(),
            "#..".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::Sw)
                .collect::<Vec<_>>(),
            "..#".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::S)
                .collect::<Vec<_>>(),
            "...#".as_bytes()
        );
        assert_eq!(
            area.visible_cells_in_direction(4, 3, Direction::Se)
                .collect::<Vec<_>>(),
            "#...".as_bytes()
        );
    }
}
