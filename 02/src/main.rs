use std::io::BufRead;

#[derive(Debug)]
struct Policy {
    min: usize,
    max: usize,
    letter: char,
}

#[derive(Debug)]
struct PasswordLine {
    policy: Policy,
    password: String,
}

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let lines = std::io::BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut parts = line.split(":");
        let policy: Policy = parse_policy_string(parts.next().unwrap().to_string());

        let password: String = parts.next().unwrap().to_string().split_off(1);

        return PasswordLine { policy, password };
    });

    println!(
        "{} passwords are valid!",
        lines.filter(|line| is_valid(line)).count()
    );
}

fn parse_policy_string(policy: String) -> Policy {
    let mut parts = policy.split(" ");

    let min_max = parts.next().unwrap();
    let mut min_max_parts = min_max.split("-");
    let min = min_max_parts.next().unwrap().parse::<usize>().unwrap();
    let max = min_max_parts.next().unwrap().parse::<usize>().unwrap();

    let letter = parts.next().unwrap().parse::<char>().unwrap();

    return Policy { min, max, letter };
}

fn is_valid(PasswordLine { policy, password }: &PasswordLine) -> bool {
    let count = password.chars().filter(|&c| c == policy.letter).count();
    policy.min <= count && count <= policy.max
}
