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
                let mut tally = std::collections::HashMap::new();

                lines.iter().for_each(|line| {
                    line.chars().for_each(|char| {
                        let count = tally.get(&char).cloned();
                        tally.insert(char, count.map(|count| count + 1).unwrap_or(1));
                    });
                });

                tally
                    .iter()
                    .filter(|(_key, count)| **count == lines.len())
                    .count()
            })
            .sum::<usize>()
    );
}
