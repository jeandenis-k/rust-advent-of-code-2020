use std::io::BufRead;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let file = std::fs::File::open(&args[1]).unwrap();
    let mut lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut group_answers = Vec::new();

    loop {
        let mut answers = std::collections::HashSet::new();
        let group_lines = lines
            .by_ref()
            .take_while(|line| line.len() != 0)
            .collect::<Vec<_>>();
        group_lines.iter().for_each(|line| {
            line.chars().for_each(|char| {
                answers.insert(char);
            })
        });

        if group_lines.len() != 0 {
            println!("{:?}", answers);
            group_answers.push(answers);
        } else {
            break;
        }
    }

    println!(
        "Solution is {}",
        group_answers
            .iter()
            .map(|answers| answers.len())
            .sum::<usize>()
    );
}