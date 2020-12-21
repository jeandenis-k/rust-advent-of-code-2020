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
            passports.push(Passport(passport));
        } else {
            break;
        }
    }

    println!(
        "There are {} valid passwords",
        passports
            .iter()
            .filter(|passport| passport.is_valid_1())
            .count()
    );
}

struct Passport(Vec<(String, String)>);

impl Passport {
    fn is_valid_1(self: &Passport) -> bool {
        self.0.iter().any(|pair| pair.0 == "byr")
            && self.0.iter().any(|pair| pair.0 == "iyr")
            && self.0.iter().any(|pair| pair.0 == "eyr")
            && self.0.iter().any(|pair| pair.0 == "hgt")
            && self.0.iter().any(|pair| pair.0 == "hcl")
            && self.0.iter().any(|pair| pair.0 == "ecl")
            && self.0.iter().any(|pair| pair.0 == "pid")
    }

    fn is_valid_2(self: &Passport) -> bool {
        return self.valid_byr();
    }

    fn valid_byr(self: &Passport) -> bool {
        let byr = self.0.iter().find(|pair| pair.0 == "byr");
        match byr {
            Some(byr) => {
                let value: usize = byr.1.parse().unwrap();
                return 1920 <= value && value <= 2002;
            }
            None => false,
        }
    }

    fn valid_field_with(self: &Passport, field: &str, f: fn(&String) -> bool) -> bool {
        let value = self
            .0
            .iter()
            .find_map(|pair| if pair.0 == field { Some(&pair.1) } else { None });
        match value {
            Some(value) => {
                return f(value);
            }
            None => false,
        }
    }
}
