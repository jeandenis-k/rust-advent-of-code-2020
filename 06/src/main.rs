use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let mut lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut groups = Vec::new();

    loop {
        let group_lines = lines
            .by_ref()
            .take_while(|line| line.len() != 0)
            .collect::<Vec<_>>();

        if group_lines.len() != 0 {
            groups.push(group_lines);
        } else {
            break;
        }
    }

    println!(
        "Solution of part 1 is {}",
        groups
            .iter()
            .map(|lines| lines
                .iter()
                .flat_map(|line| line.chars())
                .collect::<std::collections::HashSet<_>>()
                .len())
            .sum::<usize>()
    );

    println!(
        "Solution of part 2 is {}",
        groups
            .iter()
            .map(|lines| {
                lines
                    .iter()
                    .fold(HashSet::new(), |set, line| {
                        let line_set = line.chars().collect::<std::collections::HashSet<_>>();
                        set.intersection(&line_set).cloned().collect()
                    })
                    .len()
            })
            .sum::<usize>()
    );
}
