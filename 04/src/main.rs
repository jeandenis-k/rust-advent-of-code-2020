use std::io::BufRead;

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let mut lines = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap());
    let mut passports: Vec<Passport> = Vec::new();

    loop {
        let passport_lines = lines
            .by_ref()
            .take_while(|line| line.len() != 0)
            .collect::<Vec<_>>();

        if passport_lines.len() != 0 {
            let passport = passport_lines
                .iter()
                .flat_map(|line| line.split_whitespace())
                .map(|pair| {
                    let mut pair_it = pair.split(":");
                    let key = pair_it.next().unwrap().to_string();
                    let value = pair_it.next().unwrap().to_string();
                    (key, value)
                })
                .collect::<Vec<_>>();
            println!("{:?}", passport);
            passports.push(passport);
        } else {
            break;
        }
    }

    println!(
        "There are {} valid passwords",
        passports
            .iter()
            .filter(|passport| is_valid(passport))
            .count()
    );
}

type Passport = Vec<(String, String)>;

fn is_valid(passport: &Passport) -> bool {
    passport.iter().any(|pair| pair.0 == "byr")
        && passport.iter().any(|pair| pair.0 == "iyr")
        && passport.iter().any(|pair| pair.0 == "eyr")
        && passport.iter().any(|pair| pair.0 == "hgt")
        && passport.iter().any(|pair| pair.0 == "hcl")
        && passport.iter().any(|pair| pair.0 == "ecl")
        && passport.iter().any(|pair| pair.0 == "pid")
}

// while let passport_lines = lines.take_while(|line| line

//     std::io::BufReader::new(file).lines().for_each(|line| {
//         let line = line.unwrap();
//         println!("{}", line);
//     })

// struct Passport {
//     byr: Option<String>,
//     iyr: Option<String>,
//     eyr: Option<String>,
//     hgt: Option<String>,
//     hcl: Option<String>,
//     ecl: Option<String>,
//     pid: Option<String>,
//     cid: Option<String>,
// }
