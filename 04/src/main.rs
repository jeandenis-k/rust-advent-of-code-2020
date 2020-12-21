use regex::Regex;
use std::io::BufRead;
#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
    let file = std::fs::File::open(&args[1]).unwrap();
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

    println!(
        "There are {} valid passwords for part 2",
        passports
            .iter()
            .filter(|passport| passport.is_valid_2())
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
        return self.valid_field_with("byr", |byr| {
            let value: usize = byr.parse().unwrap();
            return 1920 <= value && value <= 2002;
        }) && self.valid_field_with("iyr", |iyr| {
            let value: usize = iyr.parse().unwrap();
            return 2010 <= value && value <= 2020;
        }) && self.valid_field_with("eyr", |eyr| {
            let value: usize = eyr.parse().unwrap();
            return 2020 <= value && value <= 2030;
        }) && self.valid_field_with("hgt", |hgt| {
            if hgt.ends_with("cm") {
                let mut hgt = hgt.clone();
                hgt.truncate(hgt.len() - 2);
                let num = hgt.parse().unwrap();
                return 150 <= num && num <= 193;
            } else if hgt.ends_with("in") {
                let mut hgt = hgt.clone();
                hgt.truncate(hgt.len() - 2);
                let num = hgt.parse().unwrap();
                return 59 <= num && num <= 76;
            } else {
                false
            }
        }) && self.valid_field_with("hcl", |hcl| {
            lazy_static! {
                static ref RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
            }
            return RE.is_match(hcl);
        }) && self.valid_field_with("ecl", |ecl| {
            lazy_static! {
                static ref RE: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            }
            return RE.is_match(ecl);
        }) && self.valid_field_with("pid", |pid| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            return RE.is_match(pid);
        });
    }

    fn valid_field_with(self: &Passport, field: &str, f: fn(&String) -> bool) -> bool {
        let value = self
            .0
            .iter()
            .find_map(|pair| if pair.0 == field { Some(&pair.1) } else { None });
        return value.map(|value| f(value)).unwrap_or(false);
    }
}
